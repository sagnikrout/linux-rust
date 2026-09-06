//! Automatically rewritten from C to Rust
//! Source: kernel/dma/dummy.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Dummy DMA ops that always fail.
//

    static int dma_dummy_mmap(struct device *dev, struct vm_area_struct *vma,
    void *cpu_addr, dma_addr_t dma_addr, size_t size,
    unsigned long attrs)
    {
    return -ENXIO;
    }
    static dma_addr_t dma_dummy_map_phys(struct device *dev, phys_addr_t phys,
    size_t size, enum dma_data_direction dir, unsigned long attrs)
    {
    return DMA_MAPPING_ERROR;
    }
    static void dma_dummy_unmap_phys(struct device *dev, dma_addr_t dma_handle,
    size_t size, enum dma_data_direction dir, unsigned long attrs)
    {
//
// Dummy ops doesn't support map_phys, so unmap_page should never be
// called.
//
    WARN_ON_ONCE(true);
    }
    static int dma_dummy_map_sg(struct device *dev, struct scatterlist *sgl,
    int nelems, enum dma_data_direction dir,
    unsigned long attrs)
    {
    return -EINVAL;
    }
    static void dma_dummy_unmap_sg(struct device *dev, struct scatterlist *sgl,
    int nelems, enum dma_data_direction dir,
    unsigned long attrs)
    {
//
// Dummy ops doesn't support map_sg, so unmap_sg should never be called.
//
    WARN_ON_ONCE(true);
    }
#[no_mangle]
unsafe extern "C" fn dma_dummy_supported(hwdev: *mut device, mask: u64) -> c_int {
    static int dma_dummy_supported(struct device *hwdev, u64 mask)
    {
    return 0;
    }
    const struct dma_map_ops dma_dummy_ops = {
    .mmap                   = dma_dummy_mmap,
    .map_phys               = dma_dummy_map_phys,
    .unmap_phys             = dma_dummy_unmap_phys,
    .map_sg                 = dma_dummy_map_sg,
    .unmap_sg               = dma_dummy_unmap_sg,
    .dma_supported          = dma_dummy_supported,
    };
