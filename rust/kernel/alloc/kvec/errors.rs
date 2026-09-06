
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

//! Errors for the [`Vec`] type.

use crate::{
    fmt,
    prelude::*, //
};

/// Error type for [`Vec::push_within_capacity`].
pub struct PushError<T>(pub T);

impl<T> fmt::Debug for PushError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Not enough capacity")
    }
}

impl<T> From<PushError<T>> for Error {
    #[inline]
    fn from(_: PushError<T>) -> Error {
        // Returning ENOMEM isn't appropriate because the system is not out of memory. The vector
        // is just full and we are refusing to resize it.
        EINVAL
    }
}

/// Error type for [`Vec::remove`].
pub struct RemoveError;

impl fmt::Debug for RemoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Index out of bounds")
    }
}

impl From<RemoveError> for Error {
    #[inline]
    fn from(_: RemoveError) -> Error {
        EINVAL
    }
}

/// Error type for [`Vec::insert_within_capacity`].
pub enum InsertError<T> {
    /// The value could not be inserted because the index is out of bounds.
    IndexOutOfBounds(T),
    /// The value could not be inserted because the vector is out of capacity.
    OutOfCapacity(T),
}

impl<T> fmt::Debug for InsertError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InsertError::IndexOutOfBounds(_) => write!(f, "Index out of bounds"),
            InsertError::OutOfCapacity(_) => write!(f, "Not enough capacity"),
        }
    }
}

impl<T> From<InsertError<T>> for Error {
    #[inline]
    fn from(_: InsertError<T>) -> Error {
        EINVAL
    }
}
