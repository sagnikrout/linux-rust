
// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------

// SPDX-License-Identifier: GPL-2.0

// Copyright (C) 2025 Google LLC.

use kernel::fmt;
use kernel::prelude::*;

use crate::defs::*;

pub(crate) type BinderResult<T = ()> = core::result::Result<T, BinderError>;

/// An error that will be returned to userspace via the `BINDER_WRITE_READ` ioctl rather than via
/// errno.
pub(crate) struct BinderError {
    pub(crate) reply: u32,
    pub(crate) source: Option<Error>,
}

impl BinderError {
    pub(crate) fn new_dead() -> Self {
        Self {
            reply: BR_DEAD_REPLY,
            source: None,
        }
    }

    pub(crate) fn new_frozen() -> Self {
        Self {
            reply: BR_FROZEN_REPLY,
            source: None,
        }
    }

    pub(crate) fn new_frozen_oneway() -> Self {
        Self {
            reply: BR_TRANSACTION_PENDING_FROZEN,
            source: None,
        }
    }

    pub(crate) fn is_dead(&self) -> bool {
        self.reply == BR_DEAD_REPLY
    }
}

/// Convert an errno into a `BinderError` and store the errno used to construct it. The errno
/// should be stored as the thread's extended error when given to userspace.
impl From<Error> for BinderError {
    fn from(source: Error) -> Self {
        Self {
            reply: BR_FAILED_REPLY,
            source: Some(source),
        }
    }
}

impl From<kernel::fs::file::BadFdError> for BinderError {
    fn from(source: kernel::fs::file::BadFdError) -> Self {
        BinderError::from(Error::from(source))
    }
}

impl From<kernel::alloc::AllocError> for BinderError {
    fn from(_: kernel::alloc::AllocError) -> Self {
        Self {
            reply: BR_FAILED_REPLY,
            source: Some(ENOMEM),
        }
    }
}

impl fmt::Debug for BinderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.reply {
            BR_FAILED_REPLY => match self.source.as_ref() {
                Some(source) => source.fmt(f),
                None => f.pad("BR_FAILED_REPLY"),
            },
            BR_DEAD_REPLY => f.pad("BR_DEAD_REPLY"),
            BR_FROZEN_REPLY => f.pad("BR_FROZEN_REPLY"),
            BR_TRANSACTION_PENDING_FROZEN => f.pad("BR_TRANSACTION_PENDING_FROZEN"),
            BR_TRANSACTION_COMPLETE => f.pad("BR_TRANSACTION_COMPLETE"),
            _ => match self.source.as_ref() {
                Some(source) => source.fmt(f),
                None => self.reply.fmt(f),
            },
        }
    }
}
