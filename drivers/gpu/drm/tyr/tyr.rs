
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

// SPDX-License-Identifier: GPL-2.0 or MIT

//! Arm Mali Tyr DRM driver.
//!
//! The name "Tyr" is inspired by Norse mythology, reflecting Arm's tradition of
//! naming their GPUs after Nordic mythological figures and places.

use crate::driver::TyrPlatformDriver;

mod driver;
mod file;
mod fw;
mod gem;
mod gpu;
mod mmu;
mod regs;
mod slot;
mod vm;

kernel::module_platform_driver! {
    type: TyrPlatformDriver,
    name: "tyr",
    authors: ["The Tyr driver authors"],
    description: "Arm Mali Tyr DRM driver",
    license: "Dual MIT/GPL",
}
