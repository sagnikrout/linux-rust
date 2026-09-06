
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

use kernel::{
    io::Io,
    sizes::SizeConstants, //
};

use crate::{
    driver::Bar0,
    fsp::hal::FspHal,
    regs, //
};

struct Gh100;

/// Reads the FSP secure boot status from the Hopper/GB10x thermal scratch register.
pub(super) fn fsp_boot_status_gh100(bar: Bar0<'_>) -> u32 {
    bar.read(regs::gh100::NV_THERM_I2CS_SCRATCH_FSP_BOOT_COMPLETE)
        .fsp_boot_complete()
        .into()
}

impl FspHal for Gh100 {
    fn fsp_boot_status(&self, bar: Bar0<'_>) -> u32 {
        fsp_boot_status_gh100(bar)
    }

    fn cot_version(&self) -> u16 {
        1
    }

    fn fb_end_reserved_size(&self) -> u64 {
        u64::SZ_2M
    }
}

const GH100: Gh100 = Gh100;
pub(super) const GH100_HAL: &dyn FspHal = &GH100;
