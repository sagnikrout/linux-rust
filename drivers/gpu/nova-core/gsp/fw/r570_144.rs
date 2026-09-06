
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

//! Firmware bindings.
//!
//! Imports the generated bindings by `bindgen`.
//!
//! This module may not be directly used. Please abstract or re-export the needed symbols in the
//! parent module instead.

#![allow(
    dead_code,
    clippy::all,
    clippy::undocumented_unsafe_blocks,
    clippy::ptr_as_ptr,
    clippy::ref_as_ptr,
    missing_docs,
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    improper_ctypes,
    unreachable_pub,
    unsafe_op_in_unsafe_fn
)]
use kernel::ffi;
use pin_init::MaybeZeroable;

include!("r570_144/bindings.rs");

// SAFETY: This type has a size of zero, so its inclusion into another type should not affect their
// ability to implement `Zeroable`.
unsafe impl<T> kernel::prelude::Zeroable for __IncompleteArrayField<T> {}
