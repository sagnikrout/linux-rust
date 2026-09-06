//! Automatically rewritten from C to Rust
//! Source: arch/arm64/mm/dma-mapping.c
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
//
// Copyright (C) 2012 ARM Ltd.
// Author: Catalin Marinas <catalin.marinas@arm.com>
//

    void arch_sync_dma_for_device(phys_addr_t paddr, size_t size,
    enum dma_data_direction dir)
    {
    let mut start: c_ulong = (unsigned long)phys_to_virt(paddr);
    dcache_clean_poc_nosync(start, start + size);
    }
    void arch_sync_dma_for_cpu(phys_addr_t paddr, size_t size,
    enum dma_data_direction dir)
    {
    let mut start: c_ulong = (unsigned long)phys_to_virt(paddr);
    if (dir == DMA_TO_DEVICE)
    return;
    dcache_inval_poc_nosync(start, start + size);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_dma_prep_coherent(page: *mut page, size: usize) {
    void arch_dma_prep_coherent(struct page *page, size_t size)
    {
    let mut start: c_ulong = (unsigned long)page_address(page);
    dcache_clean_poc(start, start + size);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_setup_dma_ops(dev: *mut device, coherent: bool) {
    void arch_setup_dma_ops(struct device *dev, bool coherent)
    {
    let mut cls: c_int = cache_line_size_of_cpu();
    if (!coherent && !CLIDR_LOC(read_sysreg(clidr_el1))) {
    dev_warn(dev, "CLIDR_EL1.LoC == 0, treating as coherent\n");
    coherent = true;
    }
    WARN_TAINT(!coherent && cls > ARCH_DMA_MINALIGN,
    TAINT_CPU_OUT_OF_SPEC,
    "%s %s: ARCH_DMA_MINALIGN smaller than CTR_EL0.CWG (%d < %d)",
    dev_driver_string(dev), dev_name(dev),
    ARCH_DMA_MINALIGN, cls);
    dev_assign_dma_coherent(dev, coherent);
    xen_setup_dma_ops(dev);
    }
