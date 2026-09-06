
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

// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Rust standard library vendored code.
//!
//! The contents of this file come from the Rust standard library, hosted in
//! the <https://github.com/rust-lang/rust> repository, licensed under
//! "Apache-2.0 OR MIT" and adapted for kernel use. For copyright details,
//! see <https://github.com/rust-lang/rust/blob/master/COPYRIGHT>.

use crate::sync::{arc::ArcInner, Arc};
use core::any::Any;

impl Arc<dyn Any + Send + Sync> {
    /// Attempt to downcast the `Arc<dyn Any + Send + Sync>` to a concrete type.
    pub fn downcast<T>(self) -> core::result::Result<Arc<T>, Self>
    where
        T: Any + Send + Sync,
    {
        if (*self).is::<T>() {
            // SAFETY: We have just checked that the type is correct, so we can cast the pointer.
            unsafe {
                let ptr = self.ptr.cast::<ArcInner<T>>();
                core::mem::forget(self);
                Ok(Arc::from_inner(ptr))
            }
        } else {
            Err(self)
        }
    }
}
