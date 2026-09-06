
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
// SPDX-FileCopyrightText: Copyright (c) 2025-2026 NVIDIA CORPORATION & AFFILIATES. All rights reserved.

use kernel::{
    io::Io,
    num::Bounded,
    prelude::*, //
};

use crate::{
    driver::Bar0,
    fb::{
        hal::FbHal,
        regs, //
    },
};

use super::tu102::FLUSH_SYSMEM_ADDR_SHIFT;

struct Ga100;

pub(super) fn read_sysmem_flush_page_ga100(bar: Bar0<'_>) -> u64 {
    u64::from(bar.read(regs::NV_PFB_NISO_FLUSH_SYSMEM_ADDR).adr_39_08()) << FLUSH_SYSMEM_ADDR_SHIFT
        | u64::from(bar.read(regs::NV_PFB_NISO_FLUSH_SYSMEM_ADDR_HI).adr_63_40())
            << FLUSH_SYSMEM_ADDR_SHIFT_HI
}

pub(super) fn write_sysmem_flush_page_ga100(bar: Bar0<'_>, addr: u64) {
    bar.write_reg(
        regs::NV_PFB_NISO_FLUSH_SYSMEM_ADDR_HI::zeroed().with_adr_63_40(
            Bounded::<u64, _>::from(addr)
                .shr::<FLUSH_SYSMEM_ADDR_SHIFT_HI, _>()
                .cast(),
        ),
    );

    bar.write_reg(
        regs::NV_PFB_NISO_FLUSH_SYSMEM_ADDR::zeroed()
            // CAST: `as u32` is used on purpose since we want to strip the upper bits that have
            // been written to `NV_PFB_NISO_FLUSH_SYSMEM_ADDR_HI`.
            .with_adr_39_08((addr >> FLUSH_SYSMEM_ADDR_SHIFT) as u32),
    );
}

pub(super) fn display_enabled_ga100(bar: Bar0<'_>) -> bool {
    !bar.read(crate::regs::ga100::NV_FUSE_STATUS_OPT_DISPLAY)
        .display_disabled()
}

/// Shift applied to the sysmem address before it is written into
/// `NV_PFB_NISO_FLUSH_SYSMEM_ADDR_HI`,
const FLUSH_SYSMEM_ADDR_SHIFT_HI: u32 = 40;

impl FbHal for Ga100 {
    fn read_sysmem_flush_page(&self, bar: Bar0<'_>) -> u64 {
        read_sysmem_flush_page_ga100(bar)
    }

    fn write_sysmem_flush_page(&self, bar: Bar0<'_>, addr: u64) -> Result {
        write_sysmem_flush_page_ga100(bar, addr);

        Ok(())
    }

    fn supports_display(&self, bar: Bar0<'_>) -> bool {
        display_enabled_ga100(bar)
    }

    fn vidmem_size(&self, bar: Bar0<'_>) -> u64 {
        super::tu102::vidmem_size_gp102(bar)
    }

    fn pmu_reserved_size(&self) -> u32 {
        super::tu102::pmu_reserved_size_tu102()
    }

    fn non_wpr_heap_size(&self) -> u64 {
        super::tu102::non_wpr_heap_size_tu102()
    }

    // GA100 is a special case where its FRTS region exists, but is empty.  We
    // return a size of 0 because we still need to record where the region starts.
    fn frts_size(&self) -> u64 {
        0
    }
}

const GA100: Ga100 = Ga100;
pub(super) const GA100_HAL: &dyn FbHal = &GA100;
