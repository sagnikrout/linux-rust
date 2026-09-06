//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/dma-noncoherent.c
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
// PowerPC version derived from arch/arm/mm/consistent.c
// Copyright (C) 2001 Dan Malek (dmalek@jlc.net)
//
// Copyright (C) 2000 Russell King
//

//
// make an area consistent.
//
#[no_mangle]
unsafe extern "C" fn __dma_sync(vaddr: *mut c_void, size: usize, direction: c_int) {
    static void __dma_sync(void *vaddr, size_t size, int direction)
    {
    let mut start: c_ulong = (unsigned long)vaddr;
    let mut end: c_ulong = start + size;
    switch (direction) {
    case DMA_NONE:
    BUG();
    case DMA_FROM_DEVICE:
//
// invalidate only when cache-line aligned otherwise there is
// the potential for discarding uncommitted data from the cache
//
    if ((start | end) & (L1_CACHE_BYTES - 1))
    flush_dcache_range(start, end);
    else
    invalidate_dcache_range(start, end);
    break;
    case DMA_TO_DEVICE:		/* writeback only */
    clean_dcache_range(start, end);
    break;
    case DMA_BIDIRECTIONAL:	/* writeback and invalidate */
    flush_dcache_range(start, end);
    break;
    }
    }

//
// __dma_sync_page() implementation for systems using highmem.
// In this case, each page of a buffer must be kmapped/kunmapped
// in order to have a virtual address for __dma_sync(). This must
// not sleep so kmap_atomic()/kunmap_atomic() are used.
//
// Note: yes, it is possible and correct to have a buffer extend
// beyond the first page.
//
    static inline void __dma_sync_page_highmem(struct page *page,
    unsigned long offset, size_t size, int direction)
    {
    let mut seg_size: usize = min((size_t)(PAGE_SIZE - offset), size);
    let mut cur_size: usize = seg_size;
    unsigned long flags, start, seg_offset = offset;
    let mut nr_segs: c_int = 1 + ((size - seg_size) + PAGE_SIZE - 1)/PAGE_SIZE;
    let mut seg_nr: c_int = 0;
    local_irq_save(flags);
    do {
    start = (unsigned long)kmap_atomic(page + seg_nr) + seg_offset;
// Sync this buffer segment
    __dma_sync((void *)start, seg_size, direction);
    kunmap_atomic((void *)start);
    seg_nr++;
// Calculate next buffer segment size
    seg_size = min((size_t)PAGE_SIZE, size - cur_size);
// Add the segment size to our running total
    cur_size += seg_size;
    seg_offset = 0;
    } while (seg_nr < nr_segs);
    local_irq_restore(flags);
    }

//
// __dma_sync_page makes memory consistent. identical to __dma_sync, but
// takes a struct page instead of a virtual address
//
#[no_mangle]
unsafe extern "C" fn __dma_sync_page(paddr: phys_addr_t, size: usize, dir: c_int) {
    static void __dma_sync_page(phys_addr_t paddr, size_t size, int dir)
    {
    struct page *page = pfn_to_page(paddr >> PAGE_SHIFT);
    let mut offset: unsigned = paddr & ~PAGE_MASK;

    __dma_sync_page_highmem(page, offset, size, dir);

    let mut start: c_ulong = (unsigned long)page_address(page) + offset;
    __dma_sync((void *)start, size, dir);

    }
    void arch_sync_dma_for_device(phys_addr_t paddr, size_t size,
    enum dma_data_direction dir)
    {
    __dma_sync_page(paddr, size, dir);
    }
    void arch_sync_dma_for_cpu(phys_addr_t paddr, size_t size,
    enum dma_data_direction dir)
    {
    __dma_sync_page(paddr, size, dir);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_dma_prep_coherent(page: *mut page, size: usize) {
    void arch_dma_prep_coherent(struct page *page, size_t size)
    {
    let mut kaddr: c_ulong = (unsigned long)page_address(page);
    flush_dcache_range(kaddr, kaddr + size);
    }
