// Copyright 2020 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::virtio::snd::constants::*;
use crate::virtio::snd::layout::*;

use base::{
    error, net::UnixSeqpacket, AsRawDescriptor, Error as BaseError, Event, FromRawDescriptor,
    IntoRawDescriptor, MemoryMapping, MemoryMappingBuilder, MmapError, PollToken, SafeDescriptor,
    ScmSocket, WaitContext,
};
use data_model::{DataInit, Le32, Le64, VolatileMemory, VolatileMemoryError, VolatileSlice};

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{Error as IOError, ErrorKind as IOErrorKind, Seek, SeekFrom};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::sync::Arc;
use std::thread::JoinHandle;

use sync::Mutex;

use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Failed to connect to VioS server: {0:?}")]
    ServerConnectionError(IOError),
    #[error("Failed to communicate with VioS server: {0:?}")]
    ServerError(BaseError),
    #[error("Failed to communicate with VioS server: {0:?}")]
    ServerIOError(IOError),
    #[error("Failed to get size of tx shared memory: {0}")]
    FileSizeError(IOError),
    #[error("Error duplicating file descriptor: {0}")]
    DupError(BaseError),
    #[error("Error accessing VioS server's shared memory: {0}")]
    ServerMmapError(MmapError),
    #[error("Error accessing guest's shared memory: {0}")]
    GuestMmapError(MmapError),
    #[error("Error memory mapping client_shm: {0}")]
    BaseMmapError(BaseError),
    #[error("Error accessing volatile memory: {0}")]
    VolatileMemoryError(VolatileMemoryError),
    #[error("{0}")]
    ProtocolError(ProtocolErrorKind),
    #[error("No PCM streams available")]
    NoStreamsAvailable,
    #[error("No stream with id {0}")]
    InvalidStreamId(u32),
    #[error("Stream is in unexpected state: {0:?}")]
    UnexpectedState(StreamState),
    #[error("Invalid operation for stream direction: {0}")]
    WrongDirection(u8),
    #[error("Insuficient space for the new buffer in the queue's buffer area")]
    OutOfSpace,
    #[error("Unsupported frame rate: {0}")]
    UnsupportedFrameRate(u32),
    #[error("Platform not supported")]
    PlatformNotSupported,
    #[error("Command failed with status {0}")]
    CommandFailed(u32),
    #[error("IO buffer operation failed: status = {0}")]
    IOBufferError(u32),
    #[error("Failed to duplicate UnixSeqpacket: {0}")]
    UnixSeqpacketDupError(IOError),
    #[error("Sender was dropped without sending buffer status, the recv thread may have exited")]
    BufferStatusSenderLost(RecvError),
    #[error("Failed to create Recv event: {0}")]
    EventCreateError(BaseError),
    #[error("Failed to dup Recv event: {0}")]
    EventDupError(BaseError),
    #[error("Failed to signal event: {0}")]
    EventWriteError(BaseError),
    #[error("Failed to create Recv thread's WaitContext: {0}")]
    WaitContextCreateError(BaseError),
    #[error("Error waiting for events")]
    WaitError(BaseError),
}

#[derive(ThisError, Debug)]
pub enum ProtocolErrorKind {
    #[error("The server sent a config of the wrong size: {0}")]
    UnexpectedConfigSize(usize),
    #[error("Received {1} file descriptors from the server, expected {0}")]
    UnexpectedNumberOfFileDescriptors(usize, usize), // expected, received
    #[error("Server's version ({0}) doesn't match client's")]
    VersionMismatch(u32),
    #[error("Received a msg with an unexpected size: expected {0}, received {1}")]
    UnexpectedMessageSize(usize, usize), // expected, received
}

/// The client for the VioS backend
///
/// Uses a protocol equivalent to virtio-snd over a shared memory file and a unix socket for
/// notifications. It's thread safe, it can be encapsulated in an Arc smart pointer and shared
/// between threads.
pub struct VioSClient {
    config: VioSConfig,
    // These mutexes should almost never be held simultaneously. If at some point they have to the
    // locking order should match the order in which they are declared here.
    streams: Mutex<Vec<VioSStreamInfo>>,
    chmaps: Mutex<Vec<virtio_snd_chmap_info>>,
    control_socket: Mutex<UnixSeqpacket>,
    event_socket: Mutex<UnixSeqpacket>,
    tx: Mutex<IoBufferQueue>,
    rx: Mutex<IoBufferQueue>,
    tx_subscribers: Arc<Mutex<HashMap<usize, Sender<BufferReleaseMsg>>>>,
    rx_subscribers: Arc<Mutex<HashMap<usize, Sender<BufferReleaseMsg>>>>,
    events: Arc<Mutex<VecDeque<virtio_snd_event>>>,
    event_notifier: Arc<Mutex<Option<Event>>>,
    recv_running: Arc<Mutex<bool>>,
    recv_event: Mutex<Event>,
    recv_thread: Mutex<Option<JoinHandle<Result<()>>>>,
}

impl VioSClient {
    /// Create a new client given the path to the audio server's socket.
    pub fn try_new<P: AsRef<Path>>(server: P) -> Result<VioSClient> {
        let client_socket = UnixSeqpacket::connect(server).map_err(Error::ServerConnectionError)?;
        let mut config: VioSConfig = Default::default();
        let mut fds: Vec<RawFd> = Vec::new();
        const NUM_FDS: usize = 5;
        fds.resize(NUM_FDS, 0);
        let (recv_size, fd_count) = client_socket
            .recv_with_fds(config.as_mut_slice(), &mut fds)
            .map_err(Error::ServerError)?;

        // Resize the vector to the actual number of file descriptors received and wrap them in
        // SafeDescriptors to prevent leaks
        fds.resize(fd_count, -1);
        let mut safe_fds: Vec<SafeDescriptor> = fds
            .into_iter()
            .map(|fd| unsafe {
                // safe because the SafeDescriptor object completely assumes ownership of the fd.
                SafeDescriptor::from_raw_descriptor(fd)
            })
            .collect();

        if recv_size != std::mem::size_of::<VioSConfig>() {
            return Err(Error::ProtocolError(
                ProtocolErrorKind::UnexpectedConfigSize(recv_size),
            ));
        }

        if config.version != VIOS_VERSION {
            return Err(Error::ProtocolError(ProtocolErrorKind::VersionMismatch(
                config.version,
            )));
        }

        fn pop<T: FromRawFd>(
            safe_fds: &mut Vec<SafeDescriptor>,
            expected: usize,
            received: usize,
        ) -> Result<T> {
            unsafe {
                // Safe because we transfer ownership from the SafeDescriptor to T
                Ok(T::from_raw_fd(
                    safe_fds
                        .pop()
                        .ok_or(Error::ProtocolError(
                            ProtocolErrorKind::UnexpectedNumberOfFileDescriptors(
                                expected, received,
                            ),
                        ))?
                        .into_raw_descriptor(),
                ))
            }
        }

        let rx_shm_file = pop::<File>(&mut safe_fds, NUM_FDS, fd_count)?;
        let tx_shm_file = pop::<File>(&mut safe_fds, NUM_FDS, fd_count)?;
        let rx_socket = pop::<UnixSeqpacket>(&mut safe_fds, NUM_FDS, fd_count)?;
        let tx_socket = pop::<UnixSeqpacket>(&mut safe_fds, NUM_FDS, fd_count)?;
        let event_socket = pop::<UnixSeqpacket>(&mut safe_fds, NUM_FDS, fd_count)?;

        if !safe_fds.is_empty() {
            return Err(Error::ProtocolError(
                ProtocolErrorKind::UnexpectedNumberOfFileDescriptors(NUM_FDS, fd_count),
            ));
        }

        let tx_subscribers: Arc<Mutex<HashMap<usize, Sender<BufferReleaseMsg>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let rx_subscribers: Arc<Mutex<HashMap<usize, Sender<BufferReleaseMsg>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let recv_running = Arc::new(Mutex::new(true));
        let recv_event = Event::new().map_err(Error::EventCreateError)?;

        let mut client = VioSClient {
            config,
            streams: Mutex::new(Vec::new()),
            chmaps: Mutex::new(Vec::new()),
            control_socket: Mutex::new(client_socket),
            event_socket: Mutex::new(event_socket),
            tx: Mutex::new(IoBufferQueue::new(tx_socket, tx_shm_file)?),
            rx: Mutex::new(IoBufferQueue::new(rx_socket, rx_shm_file)?),
            events: Arc::new(Mutex::new(VecDeque::new())),
            event_notifier: Arc::new(Mutex::new(None)),
            tx_subscribers,
            rx_subscribers,
            recv_running,
            recv_event: Mutex::new(recv_event),
            recv_thread: Mutex::new(None),
        };
        client.request_and_cache_info()?;
        Ok(client)
    }

    /// Get the number of jacks
    pub fn num_jacks(&self) -> u32 {
        self.config.jacks
    }

    /// Get the number of pcm streams
    pub fn num_streams(&self) -> u32 {
        self.config.streams
    }

    /// Get the number of channel maps
    pub fn num_chmaps(&self) -> u32 {
        self.config.chmaps
    }

    /// Get the configuration information on a pcm stream
    pub fn stream_info(&self, idx: u32) -> Option<virtio_snd_pcm_info> {
        self.streams
            .lock()
            .get(idx as usize)
            .map(virtio_snd_pcm_info::from)
    }

    /// Get the configuration information on a channel map
    pub fn chmap_info(&self, idx: u32) -> Option<virtio_snd_chmap_info> {
        self.chmaps.lock().get(idx as usize).copied()
    }

    /// Starts the background thread that receives release messages from the server. If the thread
    /// was already started this function does nothing.
    /// This thread must be started prior to attempting any stream IO operation or the calling
    /// thread would block.
    pub fn start_bg_thread(&self) -> Result<()> {
        if self.recv_thread.lock().is_some() {
            return Ok(());
        }
        let recv_event = self
            .recv_event
            .lock()
            .try_clone()
            .map_err(Error::EventDupError)?;
        let tx_socket = self
            .tx
            .lock()
            .socket
            .try_clone()
            .map_err(Error::UnixSeqpacketDupError)?;
        let rx_socket = self
            .rx
            .lock()
            .socket
            .try_clone()
            .map_err(Error::UnixSeqpacketDupError)?;
        let event_socket = self
            .event_socket
            .lock()
            .try_clone()
            .map_err(Error::UnixSeqpacketDupError)?;
        let mut opt = self.recv_thread.lock();
        // The lock on recv_thread was released above to avoid holding more than one lock at a time
        // while duplicating the fds. So we have to check again the condition.
        if opt.is_none() {
            *opt = Some(spawn_recv_thread(
                self.tx_subscribers.clone(),
                self.rx_subscribers.clone(),
                self.event_notifier.clone(),
                self.events.clone(),
                recv_event,
                self.recv_running.clone(),
                tx_socket,
                rx_socket,
                event_socket,
            ));
        }
        Ok(())
    }

    /// Stops the background thread.
    pub fn stop_bg_thread(&self) -> Result<()> {
        if self.recv_thread.lock().is_none() {
            return Ok(());
        }
        *self.recv_running.lock() = false;
        self.recv_event
            .lock()
            .write(1u64)
            .map_err(Error::EventWriteError)?;
        if let Some(handle) = self.recv_thread.lock().take() {
            return match handle.join() {
                Ok(r) => r,
                Err(e) => {
                    error!("Recv thread panicked: {:?}", e);
                    Ok(())
                }
            };
        }
        Ok(())
    }

    /// Gets an Event object that will trigger every time an event is received from the server
    pub fn get_event_notifier(&self) -> Result<Event> {
        let mut event_notifier = self.event_notifier.lock();
        Ok(match &*event_notifier {
            Some(evt) => evt.try_clone().map_err(Error::EventDupError)?,
            None => {
                let evt = Event::new().map_err(Error::EventCreateError)?;
                *event_notifier = Some(evt.try_clone().map_err(Error::EventDupError)?);
                evt
            }
        })
    }

    /// Retrieves one event. Callers should have received a notification through the event notifier
    /// before calling this function.
    pub fn pop_event(&self) -> Option<virtio_snd_event> {
        self.events.lock().pop_front()
    }

    /// Gets an unused stream id of the specified direction. `direction` must be one of
    /// VIRTIO_SND_D_INPUT OR VIRTIO_SND_D_OUTPUT.
    pub fn get_unused_stream_id(&self, direction: u8) -> Option<u32> {
        self.streams
            .lock()
            .iter()
            .filter(|s| s.state == StreamState::Available && s.direction == direction as u8)
            .map(|s| s.id)
            .next()
    }

    /// Configures a stream with the given parameters.
    pub fn set_stream_parameters(&self, stream_id: u32, params: VioSStreamParams) -> Result<()> {
        self.validate_stream_id(
            stream_id,
            &[StreamState::Available, StreamState::Acquired],
            None,
        )?;
        let raw_params: virtio_snd_pcm_set_params = (stream_id, params).into();
        self.send_cmd(raw_params)?;
        self.streams.lock()[stream_id as usize].state = StreamState::Acquired;
        Ok(())
    }

    /// Configures a stream with the given parameters.
    pub fn set_stream_parameters_raw(&self, raw_params: virtio_snd_pcm_set_params) -> Result<()> {
        let stream_id = raw_params.hdr.stream_id.to_native();
        self.validate_stream_id(
            stream_id,
            &[StreamState::Available, StreamState::Acquired],
            None,
        )?;
        self.send_cmd(raw_params)?;
        self.streams.lock()[stream_id as usize].state = StreamState::Acquired;
        Ok(())
    }

    /// Send the PREPARE_STREAM command to the server.
    pub fn prepare_stream(&self, stream_id: u32) -> Result<()> {
        self.common_stream_op(
            stream_id,
            &[StreamState::Available, StreamState::Acquired],
            StreamState::Acquired,
            VIRTIO_SND_R_PCM_PREPARE,
        )
    }

    /// Send the RELEASE_STREAM command to the server.
    pub fn release_stream(&self, stream_id: u32) -> Result<()> {
        self.common_stream_op(
            stream_id,
            &[StreamState::Acquired],
            StreamState::Available,
            VIRTIO_SND_R_PCM_RELEASE,
        )
    }

    /// Send the START_STREAM command to the server.
    pub fn start_stream(&self, stream_id: u32) -> Result<()> {
        self.common_stream_op(
            stream_id,
            &[StreamState::Acquired],
            StreamState::Active,
            VIRTIO_SND_R_PCM_START,
        )
    }

    /// Send the STOP_STREAM command to the server.
    pub fn stop_stream(&self, stream_id: u32) -> Result<()> {
        self.common_stream_op(
            stream_id,
            &[StreamState::Active],
            StreamState::Acquired,
            VIRTIO_SND_R_PCM_STOP,
        )
    }

    /// Send audio frames to the server. Blocks the calling thread until the server acknowledges
    /// the data.
    pub fn inject_audio_data<R, Cb: FnOnce(VolatileSlice) -> R>(
        &self,
        stream_id: u32,
        size: usize,
        callback: Cb,
    ) -> Result<(u32, R)> {
        self.validate_stream_id(stream_id, &[StreamState::Active], Some(VIRTIO_SND_D_OUTPUT))?;
        let (status_promise, ret) = {
            let mut tx_lock = self.tx.lock();
            let tx = &mut *tx_lock;
            let dst_offset = tx.allocate_buffer(size)?;
            let buffer_slice = tx.buffer_at(dst_offset, size)?;
            let ret = callback(buffer_slice);
            // Register to receive the status before sending the buffer to the server
            let (sender, receiver): (Sender<BufferReleaseMsg>, Receiver<BufferReleaseMsg>) =
                channel();
            // It's OK to acquire tx_subscriber's lock after tx_lock
            self.tx_subscribers.lock().insert(dst_offset, sender);
            let msg = IoTransferMsg::new(stream_id, dst_offset, size);
            seq_socket_send(&tx.socket, msg)?;
            (receiver, ret)
        };
        let (_, latency) = await_status(status_promise)?;
        Ok((latency, ret))
    }

    /// Request audio frames from the server. It blocks until the data is available.
    pub fn request_audio_data<R, Cb: FnOnce(&VolatileSlice) -> R>(
        &self,
        stream_id: u32,
        size: usize,
        callback: Cb,
    ) -> Result<(u32, R)> {
        self.validate_stream_id(stream_id, &[StreamState::Active], Some(VIRTIO_SND_D_INPUT))?;
        let (src_offset, status_promise) = {
            let mut rx_lock = self.rx.lock();
            let rx = &mut *rx_lock;
            let src_offset = rx.allocate_buffer(size)?;
            // Register to receive the status before sending the buffer to the server
            let (sender, receiver): (Sender<BufferReleaseMsg>, Receiver<BufferReleaseMsg>) =
                channel();
            // It's OK to acquire rx_subscriber's lock after rx_lock
            self.rx_subscribers.lock().insert(src_offset, sender);
            let msg = IoTransferMsg::new(stream_id, src_offset, size);
            seq_socket_send(&rx.socket, msg)?;
            (src_offset, receiver)
        };
        // Make sure no mutexes are held while awaiting for the buffer to be written to
        let (recv_size, latency) = await_status(status_promise)?;
        {
            let mut rx_lock = self.rx.lock();
            let buffer_slice = rx_lock.buffer_at(src_offset, recv_size)?;
            Ok((latency, callback(&buffer_slice)))
        }
    }

    /// Get a list of file descriptors used by the implementation.
    pub fn keep_fds(&self) -> Vec<RawFd> {
        let control_fd = self.control_socket.lock().as_raw_fd();
        let event_fd = self.event_socket.lock().as_raw_fd();
        let (tx_socket_fd, tx_shm_fd) = {
            let lock = self.tx.lock();
            (lock.socket.as_raw_fd(), lock.file.as_raw_fd())
        };
        let (rx_socket_fd, rx_shm_fd) = {
            let lock = self.rx.lock();
            (lock.socket.as_raw_fd(), lock.file.as_raw_fd())
        };
        let recv_event = self.recv_event.lock().as_raw_descriptor();
        vec![
            control_fd,
            event_fd,
            tx_socket_fd,
            tx_shm_fd,
            rx_socket_fd,
            rx_shm_fd,
            recv_event,
        ]
    }

    fn send_cmd<T: DataInit>(&self, data: T) -> Result<()> {
        let mut control_socket_lock = self.control_socket.lock();
        seq_socket_send(&*control_socket_lock, data)?;
        recv_cmd_status(&mut *control_socket_lock)
    }

    fn validate_stream_id(
        &self,
        stream_id: u32,
        permitted_states: &[StreamState],
        direction: Option<u8>,
    ) -> Result<()> {
        let streams_lock = self.streams.lock();
        let stream_idx = stream_id as usize;
        if stream_idx >= streams_lock.len() {
            return Err(Error::InvalidStreamId(stream_id));
        }
        if !permitted_states.contains(&streams_lock[stream_idx].state) {
            return Err(Error::UnexpectedState(streams_lock[stream_idx].state));
        }
        match direction {
            None => Ok(()),
            Some(d) => {
                if d == streams_lock[stream_idx].direction {
                    Ok(())
                } else {
                    Err(Error::WrongDirection(streams_lock[stream_idx].direction))
                }
            }
        }
    }

    fn common_stream_op(
        &self,
        stream_id: u32,
        expected_states: &[StreamState],
        new_state: StreamState,
        op: u32,
    ) -> Result<()> {
        self.validate_stream_id(stream_id, expected_states, None)?;
        let msg = virtio_snd_pcm_hdr {
            hdr: virtio_snd_hdr { code: op.into() },
            stream_id: stream_id.into(),
        };
        self.send_cmd(msg)?;
        self.streams.lock()[stream_id as usize].state = new_state;
        Ok(())
    }

    fn request_and_cache_info(&mut self) -> Result<()> {
        self.request_and_cache_streams_info()?;
        self.request_and_cache_chmaps_info()?;
        Ok(())
    }

    fn request_and_cache_streams_info(&mut self) -> Result<()> {
        let num_streams = self.config.streams as usize;
        if num_streams == 0 {
            return Ok(());
        }
        let info_size = std::mem::size_of::<virtio_snd_pcm_info>();
        let req = virtio_snd_query_info {
            hdr: virtio_snd_hdr {
                code: VIRTIO_SND_R_PCM_INFO.into(),
            },
            start_id: 0u32.into(),
            count: (num_streams as u32).into(),
            size: (std::mem::size_of::<virtio_snd_query_info>() as u32).into(),
        };
        self.send_cmd(req)?;
        // send_cmd acquires and releases the control_socket lock and then it's acquired again
        // here, however this is not a race because this function is called once during creation of
        // the object before there is a chance for multiple threads to have access to it.
        let control_socket_lock = self.control_socket.lock();
        let info_vec = control_socket_lock
            .recv_as_vec()
            .map_err(Error::ServerIOError)?;
        if info_vec.len() != num_streams * info_size {
            return Err(Error::ProtocolError(
                ProtocolErrorKind::UnexpectedMessageSize(num_streams * info_size, info_vec.len()),
            ));
        }
        self.streams = Mutex::new(
            info_vec
                .chunks(info_size)
                .enumerate()
                .map(|(id, info_buffer)| {
                    let mut virtio_stream_info: virtio_snd_pcm_info = Default::default();
                    virtio_stream_info
                        .as_mut_slice()
                        .copy_from_slice(info_buffer);
                    VioSStreamInfo::new(id as u32, &virtio_stream_info)
                })
                .collect(),
        );
        Ok(())
    }

    fn request_and_cache_chmaps_info(&mut self) -> Result<()> {
        let num_chmaps = self.config.chmaps as usize;
        if num_chmaps == 0 {
            return Ok(());
        }
        let info_size = std::mem::size_of::<virtio_snd_chmap_info>();
        let req = virtio_snd_query_info {
            hdr: virtio_snd_hdr {
                code: VIRTIO_SND_R_CHMAP_INFO.into(),
            },
            start_id: 0u32.into(),
            count: (num_chmaps as u32).into(),
            size: (std::mem::size_of::<virtio_snd_query_info>() as u32).into(),
        };
        self.send_cmd(req)?;
        // send_cmd acquires and releases the control_socket lock and then it's acquired again
        // here, however this is not a race because this function is called once during creation of
        // the object before there is a chance for multiple threads to have access to it.
        let control_socket_lock = self.control_socket.lock();
        let info_vec = control_socket_lock
            .recv_as_vec()
            .map_err(Error::ServerIOError)?;
        if info_vec.len() != num_chmaps * info_size {
            return Err(Error::ProtocolError(
                ProtocolErrorKind::UnexpectedMessageSize(num_chmaps * info_size, info_vec.len()),
            ));
        }
        self.chmaps = Mutex::new(
            info_vec
                .chunks(info_size)
                .map(|info_buffer| {
                    let mut info: virtio_snd_chmap_info = Default::default();
                    info.as_mut_slice().copy_from_slice(info_buffer);
                    info
                })
                .collect(),
        );
        Ok(())
    }
}

impl Drop for VioSClient {
    fn drop(&mut self) {
        if let Err(e) = self.stop_bg_thread() {
            error!("Error stopping Recv thread: {}", e);
        }
    }
}

#[derive(PollToken)]
enum Token {
    Notification,
    TxBufferMsg,
    RxBufferMsg,
    EventMsg,
}

fn recv_buffer_status_msg(
    socket: &UnixSeqpacket,
    subscribers: &Arc<Mutex<HashMap<usize, Sender<BufferReleaseMsg>>>>,
) -> Result<()> {
    let mut msg: IoStatusMsg = Default::default();
    let size = socket
        .recv(msg.as_mut_slice())
        .map_err(Error::ServerIOError)?;
    if size != std::mem::size_of::<IoStatusMsg>() {
        return Err(Error::ProtocolError(
            ProtocolErrorKind::UnexpectedMessageSize(std::mem::size_of::<IoStatusMsg>(), size),
        ));
    }
    let mut status = msg.status.status.into();
    if status == u32::MAX {
        // Anyone waiting for this would continue to wait for as long as status is
        // u32::MAX
        status -= 1;
    }
    let latency = msg.status.latency_bytes.into();
    let offset = msg.buffer_offset as usize;
    let consumed_len = msg.consumed_len as usize;
    let promise_opt = subscribers.lock().remove(&offset);
    match promise_opt {
        None => error!(
            "Received an unexpected buffer status message: {}. This is a BUG!!",
            offset
        ),
        Some(sender) => {
            if let Err(e) = sender.send(BufferReleaseMsg {
                status,
                latency,
                consumed_len,
            }) {
                error!("Failed to notify waiting thread: {:?}", e);
            }
        }
    }
    Ok(())
}

fn recv_event(socket: &UnixSeqpacket) -> Result<virtio_snd_event> {
    let mut msg: virtio_snd_event = Default::default();
    let size = socket
        .recv(msg.as_mut_slice())
        .map_err(Error::ServerIOError)?;
    if size != std::mem::size_of::<virtio_snd_event>() {
        return Err(Error::ProtocolError(
            ProtocolErrorKind::UnexpectedMessageSize(std::mem::size_of::<virtio_snd_event>(), size),
        ));
    }
    Ok(msg)
}

fn spawn_recv_thread(
    tx_subscribers: Arc<Mutex<HashMap<usize, Sender<BufferReleaseMsg>>>>,
    rx_subscribers: Arc<Mutex<HashMap<usize, Sender<BufferReleaseMsg>>>>,
    event_notifier: Arc<Mutex<Option<Event>>>,
    event_queue: Arc<Mutex<VecDeque<virtio_snd_event>>>,
    event: Event,
    running: Arc<Mutex<bool>>,
    tx_socket: UnixSeqpacket,
    rx_socket: UnixSeqpacket,
    event_socket: UnixSeqpacket,
) -> JoinHandle<Result<()>> {
    std::thread::spawn(move || {
        let wait_ctx: WaitContext<Token> = WaitContext::build_with(&[
            (&tx_socket, Token::TxBufferMsg),
            (&rx_socket, Token::RxBufferMsg),
            (&event_socket, Token::EventMsg),
            (&event, Token::Notification),
        ])
        .map_err(Error::WaitContextCreateError)?;
        loop {
            if !*running.lock() {
                break;
            }
            let events = wait_ctx.wait().map_err(Error::WaitError)?;
            for evt in events {
                match evt.token {
                    Token::TxBufferMsg => recv_buffer_status_msg(&tx_socket, &tx_subscribers)?,
                    Token::RxBufferMsg => recv_buffer_status_msg(&rx_socket, &rx_subscribers)?,
                    Token::EventMsg => {
                        let evt = recv_event(&event_socket)?;
                        event_queue.lock().push_back(evt);
                        if let Some(evt) = &*event_notifier.lock() {
                            evt.write(1).map_err(Error::EventWriteError)?;
                        }
                    }
                    Token::Notification => {
                        // Just consume the notification and check for termination on the next
                        // iteration
                        if let Err(e) = event.read() {
                            error!("Failed to consume notification from recv thread: {:?}", e);
                        }
                    }
                }
            }
        }
        Ok(())
    })
}

fn await_status(promise: Receiver<BufferReleaseMsg>) -> Result<(usize, u32)> {
    let BufferReleaseMsg {
        status,
        latency,
        consumed_len,
    } = promise.recv().map_err(Error::BufferStatusSenderLost)?;
    if status == VIRTIO_SND_S_OK {
        Ok((consumed_len, latency))
    } else {
        Err(Error::IOBufferError(status))
    }
}

struct IoBufferQueue {
    socket: UnixSeqpacket,
    file: File,
    mmap: MemoryMapping,
    size: usize,
    next: usize,
}

impl IoBufferQueue {
    fn new(socket: UnixSeqpacket, mut file: File) -> Result<IoBufferQueue> {
        let size = file.seek(SeekFrom::End(0)).map_err(Error::FileSizeError)? as usize;

        let mmap = MemoryMappingBuilder::new(size)
            .from_file(&file)
            .build()
            .map_err(Error::ServerMmapError)?;

        Ok(IoBufferQueue {
            socket,
            file,
            mmap,
            size,
            next: 0,
        })
    }

    fn allocate_buffer(&mut self, size: usize) -> Result<usize> {
        if size > self.size {
            return Err(Error::OutOfSpace);
        }
        let offset = if size > self.size - self.next {
            // Can't fit the new buffer at the end of the area, so put it at the beginning
            0
        } else {
            self.next
        };
        self.next = offset + size;
        Ok(offset)
    }

    fn buffer_at(&mut self, offset: usize, len: usize) -> Result<VolatileSlice> {
        self.mmap
            .get_slice(offset, len)
            .map_err(Error::VolatileMemoryError)
    }
}

/// Description of a stream made available by the server.
pub struct VioSStreamInfo {
    pub id: u32,
    pub hda_fn_nid: u32,
    pub features: u32,
    pub formats: u64,
    pub rates: u64,
    pub direction: u8,
    pub channels_min: u8,
    pub channels_max: u8,
    state: StreamState,
}

impl VioSStreamInfo {
    fn new(id: u32, info: &virtio_snd_pcm_info) -> VioSStreamInfo {
        VioSStreamInfo {
            id,
            hda_fn_nid: info.hdr.hda_fn_nid.to_native(),
            features: info.features.to_native(),
            formats: info.formats.to_native(),
            rates: info.rates.to_native(),
            direction: info.direction,
            channels_min: info.channels_min,
            channels_max: info.channels_max,
            state: StreamState::Available,
        }
    }
}

impl std::convert::From<&VioSStreamInfo> for virtio_snd_pcm_info {
    fn from(info: &VioSStreamInfo) -> virtio_snd_pcm_info {
        virtio_snd_pcm_info {
            hdr: virtio_snd_info {
                hda_fn_nid: Le32::from(info.hda_fn_nid),
            },
            features: Le32::from(info.features),
            formats: Le64::from(info.formats),
            rates: Le64::from(info.rates),
            direction: info.direction,
            channels_min: info.channels_min,
            channels_max: info.channels_max,
            padding: [0u8; 5],
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum StreamState {
    Available,
    Acquired,
    Active,
}

/// Groups the parameters used to configure a stream prior to using it.
pub struct VioSStreamParams {
    pub buffer_bytes: u32,
    pub period_bytes: u32,
    pub features: u32,
    pub channels: u8,
    pub format: u8,
    pub rate: u8,
}

impl Into<virtio_snd_pcm_set_params> for (u32, VioSStreamParams) {
    fn into(self) -> virtio_snd_pcm_set_params {
        virtio_snd_pcm_set_params {
            hdr: virtio_snd_pcm_hdr {
                hdr: virtio_snd_hdr {
                    code: VIRTIO_SND_R_PCM_SET_PARAMS.into(),
                },
                stream_id: self.0.into(),
            },
            buffer_bytes: self.1.buffer_bytes.into(),
            period_bytes: self.1.period_bytes.into(),
            features: self.1.features.into(),
            channels: self.1.channels,
            format: self.1.format,
            rate: self.1.rate,
            padding: 0u8,
        }
    }
}

fn recv_cmd_status(control_socket: &mut UnixSeqpacket) -> Result<()> {
    let mut status: virtio_snd_hdr = Default::default();
    control_socket
        .recv(status.as_mut_slice())
        .map_err(Error::ServerIOError)?;
    if status.code.to_native() == VIRTIO_SND_S_OK {
        Ok(())
    } else {
        Err(Error::CommandFailed(status.code.to_native()))
    }
}

fn seq_socket_send<T: DataInit>(socket: &UnixSeqpacket, data: T) -> Result<()> {
    loop {
        let send_res = socket.send(data.as_slice());
        if let Err(e) = send_res {
            match e.kind() {
                // Retry if interrupted
                IOErrorKind::Interrupted => continue,
                _ => return Err(Error::ServerIOError(e)),
            }
        }
        // Success
        break;
    }
    Ok(())
}

const VIOS_VERSION: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone, Default)]
struct VioSConfig {
    version: u32,
    jacks: u32,
    streams: u32,
    chmaps: u32,
}
// Safe because it only has data and has no implicit padding.
unsafe impl DataInit for VioSConfig {}

struct BufferReleaseMsg {
    status: u32,
    latency: u32,
    consumed_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct IoTransferMsg {
    io_xfer: virtio_snd_pcm_xfer,
    buffer_offset: u32,
    buffer_len: u32,
}
// Safe because it only has data and has no implicit padding.
unsafe impl DataInit for IoTransferMsg {}

impl IoTransferMsg {
    fn new(stream_id: u32, buffer_offset: usize, buffer_len: usize) -> IoTransferMsg {
        IoTransferMsg {
            io_xfer: virtio_snd_pcm_xfer {
                stream_id: stream_id.into(),
            },
            buffer_offset: buffer_offset as u32,
            buffer_len: buffer_len as u32,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
struct IoStatusMsg {
    status: virtio_snd_pcm_status,
    buffer_offset: u32,
    consumed_len: u32,
}
// Safe because it only has data and has no implicit padding.
unsafe impl DataInit for IoStatusMsg {}
