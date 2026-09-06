//! Automatically rewritten from C to Rust
//! Source: sound/pci/trident/trident_memory.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
// Copyright (c) by Takashi Iwai <tiwai@suse.de>
// Copyright (c) by Scott McNab <sdm@fractalgraphics.com.au>
//
// Trident 4DWave-NX memory page allocation (TLB area)
// Trident chip can handle only 16MByte of the memory at the same time.
//

// page arguments of these two macros are Trident page (4096 bytes), not like
// aligned pages in others
//

    (trident).tlb.entries[page] = cpu_to_le32((addr) & ~(SNDRV_TRIDENT_PAGE_SIZE-1))

    (dma_addr_t)le32_to_cpu((trident.tlb.entries[page]) & ~(SNDRV_TRIDENT_PAGE_SIZE - 1))

// page size == SNDRV_TRIDENT_PAGE_SIZE

// fill TLB entrie(s) corresponding to page with ptr

// fill TLB entrie(s) corresponding to page with silence pointer

// get aligned page from offset address

// get offset address from aligned page

// get PCI physical address from aligned page

// page size == SNDRV_TRIDENT_PAGE_SIZE x 2

// fill TLB entries -- we need to fill two entries
    static inline void set_tlb_bus(struct snd_trident *trident, int page,
    dma_addr_t addr)
    {
    page <<= 1;
    __set_tlb_bus(trident, page, addr);
    __set_tlb_bus(trident, page+1, addr + SNDRV_TRIDENT_PAGE_SIZE);
    }
#[no_mangle]
pub unsafe extern "C" fn set_silent_tlb(trident: *mut snd_trident, page: c_int) {
    static inline void set_silent_tlb(struct snd_trident *trident, int page)
    {
    page <<= 1;
    __set_tlb_bus(trident, page, trident.tlb.silent_page.addr);
    __set_tlb_bus(trident, page+1, trident.tlb.silent_page.addr);
    }

// arbitrary size

// Note: if alignment doesn't match to the maximum size, the last few blocks
// become unusable.  To use such blocks, you'll need to check the validity
// of accessing page in set_tlb_bus and set_silent_tlb.  search_empty()
// should also check it, too.
//

// fill TLB entries -- UNIT_PAGES entries must be filled
    static inline void set_tlb_bus(struct snd_trident *trident, int page,
    dma_addr_t addr)
    {
    int i;
    page *= UNIT_PAGES;
    for (i = 0; i < UNIT_PAGES; i++, page++) {
    __set_tlb_bus(trident, page, addr);
    addr += SNDRV_TRIDENT_PAGE_SIZE;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn set_silent_tlb(trident: *mut snd_trident, page: c_int) {
    static inline void set_silent_tlb(struct snd_trident *trident, int page)
    {
    int i;
    page *= UNIT_PAGES;
    for (i = 0; i < UNIT_PAGES; i++, page++)
    __set_tlb_bus(trident, page, trident.tlb.silent_page.addr);
    }

// first and last (aligned) pages of memory block

//
// search empty pages which may contain given size
//
    static struct snd_util_memblk *
    search_empty(struct snd_util_memhdr *hdr, int size)
    {
    struct snd_util_memblk *blk;
    int page, psize;
    struct list_head *p;
    psize = get_aligned_page(size + ALIGN_PAGE_SIZE -1);
    page = 0;
    list_for_each(p, &hdr.block) {
    blk = list_entry(p, struct snd_util_memblk, list);
    if (page + psize <= firstpg(blk))
    goto __found_pages;
    page = lastpg(blk) + 1;
    }
    if (page + psize > MAX_ALIGN_PAGES)
    return core::ptr::null_mut();
    __found_pages:
// create a new memory block
    blk = __snd_util_memblk_new(hdr, psize * ALIGN_PAGE_SIZE, p.prev);
    if (blk == core::ptr::null_mut())
    return core::ptr::null_mut();
    blk.offset = aligned_page_offset(page); /* set aligned offset */
    firstpg(blk) = page;
    lastpg(blk) = page + psize - 1;
    return blk;
    }
//
// check if the given pointer is valid for pages
//
#[no_mangle]
unsafe extern "C" fn is_valid_page(trident: *mut snd_trident, ptr: c_ulong) -> c_int {
    static int is_valid_page(struct snd_trident *trident, unsigned long ptr)
    {
    if (ptr & ~0x3fffffffUL) {
    dev_err(trident.card.dev, "max memory size is 1GB!!\n");
    return 0;
    }
    if (ptr & (SNDRV_TRIDENT_PAGE_SIZE-1)) {
    dev_err(trident.card.dev, "page is not aligned\n");
    return 0;
    }
    return 1;
    }
//
// page allocation for DMA (Scatter-Gather version)
//
    static struct snd_util_memblk *
    snd_trident_alloc_sg_pages(struct snd_trident *trident,
    struct snd_pcm_substream *substream)
    {
    struct snd_util_memhdr *hdr;
    struct snd_util_memblk *blk;
    struct snd_pcm_runtime *runtime = substream.runtime;
    int idx, page;
    if (snd_BUG_ON(runtime.dma_bytes <= 0 ||
    runtime.dma_bytes > SNDRV_TRIDENT_MAX_PAGES *
    SNDRV_TRIDENT_PAGE_SIZE))
    return core::ptr::null_mut();
    hdr = trident.tlb.memhdr;
    if (snd_BUG_ON(!hdr))
    return core::ptr::null_mut();
    guard(mutex)(&hdr.block_mutex);
    blk = search_empty(hdr, runtime.dma_bytes);
    if (blk == core::ptr::null_mut())
    return core::ptr::null_mut();
// set TLB entries
    idx = 0;
    for (page = firstpg(blk); page <= lastpg(blk); page++, idx++) {
    let mut ofs: c_ulong = idx << PAGE_SHIFT;
    let mut addr: dma_addr_t = snd_pcm_sgbuf_get_addr(substream, ofs);
    if (!is_valid_page(trident, addr)) {
    __snd_util_mem_free(hdr, blk);
    return core::ptr::null_mut();
    }
    set_tlb_bus(trident, page, addr);
    }
    return blk;
    }
//
// page allocation for DMA (contiguous version)
//
    static struct snd_util_memblk *
    snd_trident_alloc_cont_pages(struct snd_trident *trident,
    struct snd_pcm_substream *substream)
    {
    struct snd_util_memhdr *hdr;
    struct snd_util_memblk *blk;
    int page;
    struct snd_pcm_runtime *runtime = substream.runtime;
    dma_addr_t addr;
    if (snd_BUG_ON(runtime.dma_bytes <= 0 ||
    runtime.dma_bytes > SNDRV_TRIDENT_MAX_PAGES *
    SNDRV_TRIDENT_PAGE_SIZE))
    return core::ptr::null_mut();
    hdr = trident.tlb.memhdr;
    if (snd_BUG_ON(!hdr))
    return core::ptr::null_mut();
    guard(mutex)(&hdr.block_mutex);
    blk = search_empty(hdr, runtime.dma_bytes);
    if (blk == core::ptr::null_mut())
    return core::ptr::null_mut();
// set TLB entries
    addr = runtime.dma_addr;
    for (page = firstpg(blk); page <= lastpg(blk); page++,
    addr += SNDRV_TRIDENT_PAGE_SIZE) {
    if (!is_valid_page(trident, addr)) {
    __snd_util_mem_free(hdr, blk);
    return core::ptr::null_mut();
    }
    set_tlb_bus(trident, page, addr);
    }
    return blk;
    }
//
// page allocation for DMA
//
    struct snd_util_memblk *
    snd_trident_alloc_pages(struct snd_trident *trident,
    struct snd_pcm_substream *substream)
    {
    if (snd_BUG_ON(!trident || !substream))
    return core::ptr::null_mut();
    if (substream.dma_buffer.dev.type == SNDRV_DMA_TYPE_DEV_SG)
    return snd_trident_alloc_sg_pages(trident, substream);
    else
    return snd_trident_alloc_cont_pages(trident, substream);
    }
//
// release DMA buffer from page table
//
    int snd_trident_free_pages(struct snd_trident *trident,
    struct snd_util_memblk *blk)
    {
    struct snd_util_memhdr *hdr;
    int page;
    if (snd_BUG_ON(!trident || !blk))
    return -EINVAL;
    hdr = trident.tlb.memhdr;
    guard(mutex)(&hdr.block_mutex);
// reset TLB entries
    for (page = firstpg(blk); page <= lastpg(blk); page++)
    set_silent_tlb(trident, page);
// free memory block
    __snd_util_mem_free(hdr, blk);
    return 0;
    }
