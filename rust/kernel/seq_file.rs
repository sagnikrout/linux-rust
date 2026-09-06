
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

//! Seq file bindings.
//!
//! C header: [`include/linux/seq_file.h`](srctree/include/linux/seq_file.h)

use crate::{bindings, fmt, str::CStrExt as _, types::NotThreadSafe, types::Opaque};

/// A utility for generating the contents of a seq file.
#[repr(transparent)]
pub struct SeqFile {
    inner: Opaque<bindings::seq_file>,
    _not_send: NotThreadSafe,
}

impl SeqFile {
    /// Creates a new [`SeqFile`] from a raw pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that for the duration of `'a` the following is satisfied:
    /// * The pointer points at a valid `struct seq_file`.
    /// * The `struct seq_file` is not accessed from any other thread.
    pub unsafe fn from_raw<'a>(ptr: *mut bindings::seq_file) -> &'a SeqFile {
        // SAFETY: The caller ensures that the reference is valid for 'a. There's no way to trigger
        // a data race by using the `&SeqFile` since this is the only thread accessing the seq_file.
        //
        // CAST: The layout of `struct seq_file` and `SeqFile` is compatible.
        unsafe { &*ptr.cast() }
    }

    /// Used by the [`seq_print`] macro.
    #[inline]
    pub fn call_printf(&self, args: fmt::Arguments<'_>) {
        // SAFETY: Passing a void pointer to `Arguments` is valid for `%pA`.
        unsafe {
            bindings::seq_printf(
                self.inner.get(),
                c"%pA".as_char_ptr(),
                core::ptr::from_ref(&args).cast::<crate::ffi::c_void>(),
            );
        }
    }
}

/// Write to a [`SeqFile`] with the ordinary Rust formatting syntax.
#[macro_export]
macro_rules! seq_print {
    ($m:expr, $($arg:tt)+) => (
        $m.call_printf($crate::prelude::fmt!($($arg)+))
    );
}
pub use seq_print;
