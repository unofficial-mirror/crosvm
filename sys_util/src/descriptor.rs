// Copyright 2020 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::mem;
use std::ops::Drop;
use std::os::unix::io::{AsRawFd, RawFd};

use crate::{errno_result, Result};

pub type RawDescriptor = RawFd;

/// Trait for forfeiting ownership of the current raw descriptor, and returning the raw descriptor
pub trait IntoRawDescriptor {
    fn into_raw_descriptor(self) -> RawDescriptor;
}

/// Trait for returning the underlying raw descriptor, without giving up ownership of the
/// descriptor.
pub trait AsRawDescriptor {
    fn as_raw_descriptor(&self) -> RawDescriptor;
}

pub trait FromRawDescriptor {
    /// # Safety
    /// Safe only if the caller ensures nothing has access to the descriptor after passing it to
    /// `from_raw_descriptor`
    unsafe fn from_raw_descriptor(descriptor: RawDescriptor) -> Self;
}

/// Wraps a RawDescriptor and safely closes it when self falls out of scope.
#[derive(Debug, Eq)]
pub struct SafeDescriptor {
    descriptor: RawDescriptor,
}

const KCMP_FILE: u32 = 0;

impl PartialEq for SafeDescriptor {
    fn eq(&self, other: &Self) -> bool {
        // If RawFd numbers match then we can return early without calling kcmp
        if self.descriptor == other.descriptor {
            return true;
        }

        // safe because we only use the return value and libc says it's always successful
        let pid = unsafe { libc::getpid() };
        // safe because we are passing everything by value and checking the return value
        let ret = unsafe {
            libc::syscall(
                libc::SYS_kcmp,
                pid,
                pid,
                KCMP_FILE,
                self.descriptor,
                other.descriptor,
            )
        };

        ret == 0
    }
}

impl Drop for SafeDescriptor {
    fn drop(&mut self) {
        let _ = unsafe { libc::close(self.descriptor) };
    }
}

impl AsRawDescriptor for SafeDescriptor {
    fn as_raw_descriptor(&self) -> RawDescriptor {
        self.descriptor
    }
}

impl IntoRawDescriptor for SafeDescriptor {
    fn into_raw_descriptor(self) -> RawDescriptor {
        let descriptor = self.descriptor;
        mem::forget(self);
        descriptor
    }
}

impl FromRawDescriptor for SafeDescriptor {
    unsafe fn from_raw_descriptor(descriptor: RawDescriptor) -> Self {
        SafeDescriptor { descriptor }
    }
}

impl AsRawFd for SafeDescriptor {
    fn as_raw_fd(&self) -> RawFd {
        self.as_raw_descriptor()
    }
}

impl SafeDescriptor {
    /// Clones this descriptor, internally creating a new descriptor. The new SafeDescriptor will
    /// share the same underlying count within the kernel.
    pub fn try_clone(&self) -> Result<SafeDescriptor> {
        // Safe because self.as_raw_descriptor() returns a valid value
        let copy_fd = unsafe { libc::dup(self.as_raw_descriptor()) };
        if copy_fd < 0 {
            return errno_result();
        }
        // Safe becuase we just successfully duplicated and this object will uniquely
        // own the raw descriptor.
        Ok(unsafe { SafeDescriptor::from_raw_descriptor(copy_fd) })
    }
}

/// For use cases where a simple wrapper around a RawDescriptor is needed.
/// This is a simply a wrapper and does not manage the lifetime of the descriptor.
/// Most usages should prefer SafeDescriptor or using a RawDescriptor directly
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Descriptor(pub RawDescriptor);
impl AsRawDescriptor for Descriptor {
    fn as_raw_descriptor(&self) -> RawDescriptor {
        self.0
    }
}

#[test]
fn clone_equality() {
    let ret = unsafe { libc::eventfd(0, 0) };
    if ret < 0 {
        panic!("failed to create eventfd");
    }
    let descriptor = unsafe { SafeDescriptor::from_raw_descriptor(ret) };

    assert_eq!(descriptor, descriptor);

    assert_eq!(
        descriptor,
        descriptor.try_clone().expect("failed to clone eventfd")
    );

    let ret = unsafe { libc::eventfd(0, 0) };
    if ret < 0 {
        panic!("failed to create eventfd");
    }
    let another = unsafe { SafeDescriptor::from_raw_descriptor(ret) };

    assert_ne!(descriptor, another);
}
