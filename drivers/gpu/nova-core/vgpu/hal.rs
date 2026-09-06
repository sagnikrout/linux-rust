
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

use crate::gpu::{
    Architecture,
    Chipset, //
};

mod gb202;
mod tu102;

pub(super) trait VgpuHal {
    /// Returns whether this chipset can support vGPU.
    fn supports_vgpu(&self) -> bool;
}

pub(super) fn vgpu_hal(chipset: Chipset) -> &'static dyn VgpuHal {
    match chipset.arch() {
        Architecture::BlackwellGB20x => gb202::GB202_HAL,
        Architecture::Turing
        | Architecture::Ampere
        | Architecture::Hopper
        | Architecture::Ada
        | Architecture::BlackwellGB10x => tu102::TU102_HAL,
    }
}
