
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
// SPDX-FileCopyrightText: Copyright (c) 2026 NVIDIA CORPORATION & AFFILIATES. All rights reserved.

use crate::{
    driver::Bar0,
    gpu::{
        Architecture,
        Chipset, //
    },
};

mod gb100;
mod gb202;
mod gh100;

pub(super) trait FspHal {
    /// Returns the secure boot status from the architecture-specific `NV_THERM_I2CS_SCRATCH` register.
    fn fsp_boot_status(&self, bar: Bar0<'_>) -> u32;

    /// Returns the FSP Chain of Trust protocol version this chipset advertises.
    fn cot_version(&self) -> u16;

    // TODO: consider moving this into the TLV firmware metadata when ready
    /// Returns the size reserved at the end of the framebuffer, in bytes.
    fn fb_end_reserved_size(&self) -> u64;
}

/// Returns the FSP HAL, or `None` if the architecture doesn't support FSP.
pub(super) fn fsp_hal(chipset: Chipset) -> Option<&'static dyn FspHal> {
    match chipset.arch() {
        Architecture::Turing | Architecture::Ampere | Architecture::Ada => None,
        Architecture::Hopper => Some(gh100::GH100_HAL),
        Architecture::BlackwellGB10x => Some(gb100::GB100_HAL),
        Architecture::BlackwellGB20x => Some(gb202::GB202_HAL),
    }
}
