
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

//! IRQ abstractions.
//!
//! An IRQ is an interrupt request from a device. It is used to get the CPU's
//! attention so it can service a hardware event in a timely manner.
//!
//! The current abstractions handle IRQ requests and handlers, i.e.: it allows
//! drivers to register a handler for a given IRQ line.
//!
//! C header: [`include/linux/interrupt.h`](srctree/include/linux/interrupt.h)

/// Flags to be used when registering IRQ handlers.
mod flags;

/// IRQ allocation and handling.
mod request;

pub use flags::Flags;

pub use request::{
    Handler, IrqRequest, IrqReturn, Registration, ThreadedHandler, ThreadedIrqReturn,
    ThreadedRegistration,
};
