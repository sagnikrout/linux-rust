
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

use kernel::io::register::RegisterBase;

use crate::falcon::{
    FalconEngine,
    PFalcon2Base,
    PFalconBase, //
};

/// Type specifying the `Sec2` falcon engine. Cannot be instantiated.
pub(crate) struct Sec2(());

impl RegisterBase<PFalconBase> for Sec2 {
    const BASE: usize = 0x00840000;
}

impl RegisterBase<PFalcon2Base> for Sec2 {
    const BASE: usize = 0x00841000;
}

impl FalconEngine for Sec2 {}
