initSidebarItems({"enum":[["CaptureBufferError","Errors that are possible from a `CaptureBuffer`."]],"fn":[["async_read_capture_buffer","Call `f` with a `AsyncCaptureBuffer`, and trigger the buffer done call back after. `f` can read the capture data from the given `AsyncCaptureBuffer`."]],"struct":[["AsyncCaptureBuffer","Async version of 'CaptureBuffer`"],["CaptureBuffer","`CaptureBuffer` contains a block of audio samples got from capture stream. It provides temporary view to those samples and will notifies capture stream when dropped. Note that it'll always send `buffer.len() / frame_size` to drop function when it got destroyed since `CaptureBufferStream` assumes that users get all the samples from the buffer."],["NoopCaptureStream","Stream that provides null capture samples."]],"trait":[["AsyncCaptureBufferStream",""],["CaptureBufferStream","`CaptureBufferStream` provides `CaptureBuffer`s to read with audio samples from capture."]]});