//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/arm64/dma.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

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


// SPDX-License-Identifier: GPL-2.0-only

#[no_mangle]
pub unsafe extern "C" fn acpi_arch_dma_setup(dev: *mut device) {
    void acpi_arch_dma_setup(struct device *dev)
    {
    int ret;
    u64 end, mask;
    const struct bus_dma_region *map = core::ptr::null_mut();
//
// If @dev is expected to be DMA-capable then the bus code that created
// it should have initialised its dma_mask pointer by this point. For
// now, we'll continue the legacy behaviour of coercing it to the
// coherent mask if not, but we'll no longer do so quietly.
//
    if (!dev.dma_mask) {
    dev_warn(dev, "DMA mask not set\n");
    dev.dma_mask = &dev.coherent_dma_mask;
    }
    if (dev.coherent_dma_mask)
    end = dev.coherent_dma_mask;
    else
    end = (1ULL << 32) - 1;
    if (dev.dma_range_map) {
    dev_dbg(dev, "dma_range_map already set\n");
    return;
    }
    ret = acpi_dma_get_range(dev, &map);
    if (!ret && map) {
    end = dma_range_map_max(map);
    dev.dma_range_map = map;
    }
    if (ret == -ENODEV)
    ret = iort_dma_get_ranges(dev, &end);
    if (!ret) {
//
// Limit coherent and dma mask based on size retrieved from
// firmware.
//
    mask = DMA_BIT_MASK(ilog2(end) + 1);
    dev.bus_dma_limit = end;
    dev.coherent_dma_mask = min(dev.coherent_dma_mask, mask);
// dev->dma_mask = min(*dev->dma_mask, mask);
    }
    }
