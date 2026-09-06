
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

use core::ops::Range;

use kernel::{
    dma::DmaMask,
    prelude::*, //
};

use crate::driver::Bar0;

use super::GpuHal;

struct Gh100;

impl GpuHal for Gh100 {
    fn wait_gfw_boot_completion(&self, _bar: Bar0<'_>) -> Result {
        Ok(())
    }

    fn dma_mask(&self) -> DmaMask {
        DmaMask::new::<52>()
    }

    fn pci_config_mirror_range(&self) -> Range<u32> {
        const PCI_CONFIG_MIRROR_START: u32 = 0x092000;
        const PCI_CONFIG_MIRROR_SIZE: u32 = 0x001000;

        PCI_CONFIG_MIRROR_START..PCI_CONFIG_MIRROR_START + PCI_CONFIG_MIRROR_SIZE
    }
}

const GH100: Gh100 = Gh100;
pub(super) const GH100_HAL: &dyn GpuHal = &GH100;
