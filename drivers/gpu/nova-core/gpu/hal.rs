
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

use crate::{
    driver::Bar0,
    gpu::{
        Architecture,
        Chipset, //
    },
};

mod gh100;
mod tu102;

pub(crate) trait GpuHal {
    /// Waits for GFW_BOOT completion if required by this hardware family.
    fn wait_gfw_boot_completion(&self, bar: Bar0<'_>) -> Result;

    /// Returns the DMA mask for the current architecture.
    fn dma_mask(&self) -> DmaMask;

    /// Returns the address range of the PCI config mirror space.
    fn pci_config_mirror_range(&self) -> Range<u32>;
}

pub(super) fn gpu_hal(chipset: Chipset) -> &'static dyn GpuHal {
    match chipset.arch() {
        Architecture::Turing | Architecture::Ampere | Architecture::Ada => tu102::TU102_HAL,
        Architecture::Hopper | Architecture::BlackwellGB10x | Architecture::BlackwellGB20x => {
            gh100::GH100_HAL
        }
    }
}
