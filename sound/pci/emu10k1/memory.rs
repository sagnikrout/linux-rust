//! Automatically rewritten from C to Rust
//! Source: sound/pci/emu10k1/memory.c
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
//
// EMU10K1 memory page allocation (PTB area)
//

// page arguments of these two macros are Emu page (4096 bytes), not like
// aligned pages in others
//

    (((__le32 *)(emu).ptb_pages.area)[page] = \
    cpu_to_le32(((addr) << (emu.address_mode)) | (page)))

    (le32_to_cpu(((__le32 *)(emu).ptb_pages.area)[page]))

// get aligned page from offset address

// get offset address from aligned page

// fill PTB entrie(s) corresponding to page with addr

// fill PTB entrie(s) corresponding to page with silence pointer

// fill PTB entries -- we need to fill UNIT_PAGES entries
#[no_mangle]
pub unsafe extern "C" fn set_ptb_entry(emu: *mut snd_emu10k1, page: c_int, addr: dma_addr_t) {
    static inline void set_ptb_entry(struct snd_emu10k1 *emu, int page, dma_addr_t addr)
    {
    int i;
    page *= UNIT_PAGES;
    for (i = 0; i < UNIT_PAGES; i++, page++) {
    __set_ptb_entry(emu, page, addr);
    dev_dbg(emu.card.dev, "mapped page %d to entry %.8x\n", page,
    (unsigned int)__get_ptb_entry(emu, page));
    addr += EMUPAGESIZE;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn set_silent_ptb(emu: *mut snd_emu10k1, page: c_int) {
    static inline void set_silent_ptb(struct snd_emu10k1 *emu, int page)
    {
    int i;
    page *= UNIT_PAGES;
    for (i = 0; i < UNIT_PAGES; i++, page++) {
// do not increment ptr
    __set_ptb_entry(emu, page, emu.silent_page.addr);
    dev_dbg(emu.card.dev, "mapped silent page %d to entry %.8x\n",
    page, (unsigned int)__get_ptb_entry(emu, page));
    }
    }

//
    static int synth_alloc_pages(struct snd_emu10k1 *hw, struct snd_emu10k1_memblk *blk);
    static int synth_free_pages(struct snd_emu10k1 *hw, struct snd_emu10k1_memblk *blk);

// initialize emu10k1 part
#[no_mangle]
unsafe extern "C" fn emu10k1_memblk_init(blk: *mut snd_emu10k1_memblk) {
    static void emu10k1_memblk_init(struct snd_emu10k1_memblk *blk)
    {
    blk.mapped_page = -1;
    INIT_LIST_HEAD(&blk.mapped_link);
    INIT_LIST_HEAD(&blk.mapped_order_link);
    blk.map_locked = 0;
    blk.first_page = get_aligned_page(blk.mem.offset);
    blk.last_page = get_aligned_page(blk.mem.offset + blk.mem.size - 1);
    blk.pages = blk.last_page - blk.first_page + 1;
    }
//
// search empty region on PTB with the given size
//
// if an empty region is found, return the page and store the next mapped block
// in nextp
// if not found, return a negative error code.
//
#[no_mangle]
unsafe extern "C" fn search_empty_map_area(emu: *mut snd_emu10k1, npages: c_int, nextp: *mut list_head) -> c_int {
    static int search_empty_map_area(struct snd_emu10k1 *emu, int npages, struct list_head **nextp)
    {
    let mut page: c_int = 1, found_page = -ENOMEM;
    let mut max_size: c_int = npages;
    int size;
    struct list_head *candidate = &emu.mapped_link_head;
    struct list_head *pos;
    list_for_each (pos, &emu.mapped_link_head) {
    struct snd_emu10k1_memblk *blk = get_emu10k1_memblk(pos, mapped_link);
    if (blk.mapped_page < 0)
    continue;
    size = blk.mapped_page - page;
    if (size == npages) {
// nextp = pos;
    return page;
    }
#[no_mangle]
pub unsafe extern "C" fn if(max_size: size >) -> else {
// we look for the maximum empty hole
    max_size = size;
    candidate = pos;
    found_page = page;
    }
    page = blk.mapped_page + blk.pages;
    }
    size = (emu.address_mode ? MAX_ALIGN_PAGES1 : MAX_ALIGN_PAGES0) - page;
    if (size >= max_size) {
// nextp = pos;
    return page;
    }
// nextp = candidate;
    return found_page;
    }
//
// map a memory block onto emu10k1's PTB
//
// call with memblk_lock held
//
#[no_mangle]
unsafe extern "C" fn map_memblk(emu: *mut snd_emu10k1, blk: *mut snd_emu10k1_memblk) -> c_int {
    static int map_memblk(struct snd_emu10k1 *emu, struct snd_emu10k1_memblk *blk)
    {
    int page, pg;
    struct list_head *next;
    page = search_empty_map_area(emu, blk.pages, &next);
    if (page < 0) /* not found */
    return page;
    if (page == 0) {
    dev_err(emu.card.dev, "trying to map zero (reserved) page\n");
    return -EINVAL;
    }
// insert this block in the proper position of mapped list
    list_add_tail(&blk.mapped_link, next);
// append this as a newest block in order list
    list_add_tail(&blk.mapped_order_link, &emu.mapped_order_link_head);
    blk.mapped_page = page;
// fill PTB
    for (pg = blk.first_page; pg <= blk.last_page; pg++) {
    set_ptb_entry(emu, page, emu.page_addr_table[pg]);
    page++;
    }
    return 0;
    }
//
// unmap the block
// return the size of resultant empty pages
//
// call with memblk_lock held
//
#[no_mangle]
unsafe extern "C" fn unmap_memblk(emu: *mut snd_emu10k1, blk: *mut snd_emu10k1_memblk) -> c_int {
    static int unmap_memblk(struct snd_emu10k1 *emu, struct snd_emu10k1_memblk *blk)
    {
    int start_page, end_page, mpage, pg;
    struct list_head *p;
    struct snd_emu10k1_memblk *q;
// calculate the expected size of empty region
    p = blk.mapped_link.prev;
    if (p != &emu.mapped_link_head) {
    q = get_emu10k1_memblk(p, mapped_link);
    start_page = q.mapped_page + q.pages;
    } else {
    start_page = 1;
    }
    p = blk.mapped_link.next;
    if (p != &emu.mapped_link_head) {
    q = get_emu10k1_memblk(p, mapped_link);
    end_page = q.mapped_page;
    } else {
    end_page = (emu.address_mode ? MAX_ALIGN_PAGES1 : MAX_ALIGN_PAGES0);
    }
// remove links
    list_del(&blk.mapped_link);
    list_del(&blk.mapped_order_link);
// clear PTB
    mpage = blk.mapped_page;
    for (pg = blk.first_page; pg <= blk.last_page; pg++) {
    set_silent_ptb(emu, mpage);
    mpage++;
    }
    blk.mapped_page = -1;
    return end_page - start_page; /* return the new empty size */
    }
//
// search empty pages with the given size, and create a memory block
//
// unlike synth_alloc the memory block is aligned to the page start
//
    static struct snd_emu10k1_memblk *
    search_empty(struct snd_emu10k1 *emu, int size)
    {
    struct list_head *p;
    struct snd_emu10k1_memblk *blk;
    int page, psize;
    psize = get_aligned_page(size + PAGE_SIZE -1);
    page = 0;
    list_for_each(p, &emu.memhdr.block) {
    blk = get_emu10k1_memblk(p, mem.list);
    if (page + psize <= blk.first_page)
    goto __found_pages;
    page = blk.last_page + 1;
    }
    if (page + psize > emu.max_cache_pages)
    return core::ptr::null_mut();
    __found_pages:
// create a new memory block
    blk = (struct snd_emu10k1_memblk *)__snd_util_memblk_new(emu.memhdr, psize << PAGE_SHIFT, p.prev);
    if (blk == core::ptr::null_mut())
    return core::ptr::null_mut();
    blk.mem.offset = aligned_page_offset(page); /* set aligned offset */
    emu10k1_memblk_init(blk);
    return blk;
    }
//
// check if the given pointer is valid for pages
//
#[no_mangle]
unsafe extern "C" fn is_valid_page(emu: *mut snd_emu10k1, addr: dma_addr_t) -> c_int {
    static int is_valid_page(struct snd_emu10k1 *emu, dma_addr_t addr)
    {
    if (addr & ~emu.dma_mask) {
    dev_err_ratelimited(emu.card.dev,
    "max memory size is 0x%lx (addr = 0x%lx)!!\n",
    emu.dma_mask, (unsigned long)addr);
    return 0;
    }
    if (addr & (EMUPAGESIZE-1)) {
    dev_err_ratelimited(emu.card.dev, "page is not aligned\n");
    return 0;
    }
    return 1;
    }
//
// map the given memory block on PTB.
// if the block is already mapped, update the link order.
// if no empty pages are found, tries to release unused memory blocks
// and retry the mapping.
//
#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_memblk_map(emu: *mut snd_emu10k1, blk: *mut snd_emu10k1_memblk) -> c_int {
    int snd_emu10k1_memblk_map(struct snd_emu10k1 *emu, struct snd_emu10k1_memblk *blk)
    {
    int err;
    int size;
    struct list_head *p, *nextp;
    struct snd_emu10k1_memblk *deleted;
    guard(spinlock_irqsave)(&emu.memblk_lock);
    if (blk.mapped_page >= 0) {
// update order link
    list_move_tail(&blk.mapped_order_link,
    &emu.mapped_order_link_head);
    return 0;
    }
    err = map_memblk(emu, blk);
    if (err < 0) {
// no enough page - try to unmap some blocks
// starting from the oldest block
    p = emu.mapped_order_link_head.next;
    for (; p != &emu.mapped_order_link_head; p = nextp) {
    nextp = p.next;
    deleted = get_emu10k1_memblk(p, mapped_order_link);
    if (deleted.map_locked)
    continue;
    size = unmap_memblk(emu, deleted);
    if (size >= blk.pages) {
// ok the empty region is enough large
    err = map_memblk(emu, blk);
    break;
    }
    }
    }
    return err;
    }
    EXPORT_SYMBOL(snd_emu10k1_memblk_map);
//
// page allocation for DMA
//
    struct snd_util_memblk *
    snd_emu10k1_alloc_pages(struct snd_emu10k1 *emu, struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct snd_util_memhdr *hdr;
    struct snd_emu10k1_memblk *blk;
    int page, err, idx;
    if (snd_BUG_ON(!emu))
    return core::ptr::null_mut();
    if (snd_BUG_ON(runtime.dma_bytes <= 0 ||
    runtime.dma_bytes >= (emu.address_mode ? MAXPAGES1 : MAXPAGES0) * EMUPAGESIZE))
    return core::ptr::null_mut();
    hdr = emu.memhdr;
    if (snd_BUG_ON(!hdr))
    return core::ptr::null_mut();
    guard(mutex)(&hdr.block_mutex);
    blk = search_empty(emu, runtime.dma_bytes);
    if (blk == core::ptr::null_mut())
    return core::ptr::null_mut();
// fill buffer addresses but pointers are not stored so that
// snd_free_pci_page() is not called in synth_free()
//
    idx = 0;
    for (page = blk.first_page; page <= blk.last_page; page++, idx++) {
    let mut ofs: c_ulong = idx << PAGE_SHIFT;
    dma_addr_t addr;
    if (ofs >= runtime.dma_bytes)
    addr = emu.silent_page.addr;
    else
    addr = snd_pcm_sgbuf_get_addr(substream, ofs);
    if (! is_valid_page(emu, addr)) {
    dev_err_ratelimited(emu.card.dev,
    "emu: failure page = %d\n", idx);
    return core::ptr::null_mut();
    }
    emu.page_addr_table[page] = addr;
    emu.page_ptr_table[page] = core::ptr::null_mut();
    }
// set PTB entries
    blk.map_locked = 1; /* do not unmap this block! */
    err = snd_emu10k1_memblk_map(emu, blk);
    if (err < 0) {
    __snd_util_mem_free(hdr, (struct snd_util_memblk *)blk);
    return core::ptr::null_mut();
    }
    return (struct snd_util_memblk *)blk;
    }
//
// release DMA buffer from page table
//
#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_free_pages(emu: *mut snd_emu10k1, blk: *mut snd_util_memblk) -> c_int {
    int snd_emu10k1_free_pages(struct snd_emu10k1 *emu, struct snd_util_memblk *blk)
    {
    if (snd_BUG_ON(!emu || !blk))
    return -EINVAL;
    return snd_emu10k1_synth_free(emu, blk);
    }
//
// allocate DMA pages, widening the allocation if necessary
//
// See the comment above snd_emu10k1_detect_iommu() in emu10k1_main.c why
// this might be needed.
//
// If you modify this function check whether __synth_free_pages() also needs
// changes.
//
    int snd_emu10k1_alloc_pages_maybe_wider(struct snd_emu10k1 *emu, size_t size,
    struct snd_dma_buffer *dmab)
    {
    if (emu.iommu_workaround) {
    let mut npages: usize = DIV_ROUND_UP(size, PAGE_SIZE);
    let mut size_real: usize = npages * PAGE_SIZE;
//
// The device has been observed to accesses up to 256 extra
// bytes, but use 1k to be safe.
//
    if (size_real < size + 1024)
    size += PAGE_SIZE;
    }
    return snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV,
    &emu.pci.dev, size, dmab);
    }
//
// memory allocation using multiple pages (for synth)
// Unlike the DMA allocation above, non-contiguous pages are assined.
//
// allocate a synth sample area
//
    struct snd_util_memblk *
    snd_emu10k1_synth_alloc(struct snd_emu10k1 *hw, unsigned int size)
    {
    struct snd_emu10k1_memblk *blk;
    struct snd_util_memhdr *hdr = hw.memhdr;
    guard(mutex)(&hdr.block_mutex);
    blk = (struct snd_emu10k1_memblk *)__snd_util_mem_alloc(hdr, size);
    if (blk == core::ptr::null_mut())
    return core::ptr::null_mut();
    if (synth_alloc_pages(hw, blk)) {
    __snd_util_mem_free(hdr, (struct snd_util_memblk *)blk);
    return core::ptr::null_mut();
    }
    snd_emu10k1_memblk_map(hw, blk);
    return (struct snd_util_memblk *)blk;
    }
    EXPORT_SYMBOL(snd_emu10k1_synth_alloc);
//
// free a synth sample area
//
    int
    snd_emu10k1_synth_free(struct snd_emu10k1 *emu, struct snd_util_memblk *memblk)
    {
    struct snd_util_memhdr *hdr = emu.memhdr;
    struct snd_emu10k1_memblk *blk = (struct snd_emu10k1_memblk *)memblk;
    guard(mutex)(&hdr.block_mutex);
    scoped_guard(spinlock_irqsave, &emu.memblk_lock) {
    if (blk.mapped_page >= 0)
    unmap_memblk(emu, blk);
    }
    synth_free_pages(emu, blk);
    __snd_util_mem_free(hdr, memblk);
    return 0;
    }
    EXPORT_SYMBOL(snd_emu10k1_synth_free);
// check new allocation range
    static void get_single_page_range(struct snd_util_memhdr *hdr,
    struct snd_emu10k1_memblk *blk,
    int *first_page_ret, int *last_page_ret)
    {
    struct list_head *p;
    struct snd_emu10k1_memblk *q;
    int first_page, last_page;
    first_page = blk.first_page;
    p = blk.mem.list.prev;
    if (p != &hdr.block) {
    q = get_emu10k1_memblk(p, mem.list);
    if (q.last_page == first_page)
    first_page++;  /* first page was already allocated */
    }
    last_page = blk.last_page;
    p = blk.mem.list.next;
    if (p != &hdr.block) {
    q = get_emu10k1_memblk(p, mem.list);
    if (q.first_page == last_page)
    last_page--; /* last page was already allocated */
    }
// first_page_ret = first_page;
// last_page_ret = last_page;
    }
// release allocated pages
    static void __synth_free_pages(struct snd_emu10k1 *emu, int first_page,
    int last_page)
    {
    struct snd_dma_buffer dmab;
    int page;
    dmab.dev.type = SNDRV_DMA_TYPE_DEV;
    dmab.dev.dev = &emu.pci.dev;
    for (page = first_page; page <= last_page; page++) {
    if (emu.page_ptr_table[page] == core::ptr::null_mut())
    continue;
    dmab.area = emu.page_ptr_table[page];
    dmab.addr = emu.page_addr_table[page];
//
// please keep me in sync with logic in
// snd_emu10k1_alloc_pages_maybe_wider()
//
    dmab.bytes = PAGE_SIZE;
    if (emu.iommu_workaround)
    dmab.bytes *= 2;
    snd_dma_free_pages(&dmab);
    emu.page_addr_table[page] = 0;
    emu.page_ptr_table[page] = core::ptr::null_mut();
    }
    }
//
// allocate kernel pages
//
#[no_mangle]
unsafe extern "C" fn synth_alloc_pages(emu: *mut snd_emu10k1, blk: *mut snd_emu10k1_memblk) -> c_int {
    static int synth_alloc_pages(struct snd_emu10k1 *emu, struct snd_emu10k1_memblk *blk)
    {
    int page, first_page, last_page;
    struct snd_dma_buffer dmab;
    emu10k1_memblk_init(blk);
    get_single_page_range(emu.memhdr, blk, &first_page, &last_page);
// allocate kernel pages
    for (page = first_page; page <= last_page; page++) {
    if (snd_emu10k1_alloc_pages_maybe_wider(emu, PAGE_SIZE,
    &dmab) < 0)
    goto __fail;
    if (!is_valid_page(emu, dmab.addr)) {
    snd_dma_free_pages(&dmab);
    goto __fail;
    }
    emu.page_addr_table[page] = dmab.addr;
    emu.page_ptr_table[page] = dmab.area;
    }
    return 0;
    __fail:
// release allocated pages
    last_page = page - 1;
    __synth_free_pages(emu, first_page, last_page);
    return -ENOMEM;
    }
//
// free pages
//
#[no_mangle]
unsafe extern "C" fn synth_free_pages(emu: *mut snd_emu10k1, blk: *mut snd_emu10k1_memblk) -> c_int {
    static int synth_free_pages(struct snd_emu10k1 *emu, struct snd_emu10k1_memblk *blk)
    {
    int first_page, last_page;
    get_single_page_range(emu.memhdr, blk, &first_page, &last_page);
    __synth_free_pages(emu, first_page, last_page);
    return 0;
    }
// calculate buffer pointer from offset address
    static inline void *offset_ptr(struct snd_emu10k1 *emu, int page, int offset)
    {
    char *ptr;
    if (snd_BUG_ON(page < 0 || page >= emu.max_cache_pages))
    return core::ptr::null_mut();
    ptr = emu.page_ptr_table[page];
    if (! ptr) {
    dev_err(emu.card.dev,
    "access to core::ptr::null_mut() ptr: page = %d\n", page);
    return core::ptr::null_mut();
    }
    ptr += offset & (PAGE_SIZE - 1);
    return (void*)ptr;
    }
//
// memset(blk + offset, value, size)
//
    int snd_emu10k1_synth_memset(struct snd_emu10k1 *emu, struct snd_util_memblk *blk,
    int offset, int size, u8 value)
    {
    int page, nextofs, end_offset, temp, temp1;
    void *ptr;
    struct snd_emu10k1_memblk *p = (struct snd_emu10k1_memblk *)blk;
    if (snd_BUG_ON(offset + size > p.mem.size))
    return -EFAULT;
    offset += blk.offset & (PAGE_SIZE - 1);
    end_offset = offset + size;
    page = get_aligned_page(offset);
    do {
    nextofs = aligned_page_offset(page + 1);
    temp = nextofs - offset;
    temp1 = end_offset - offset;
    if (temp1 < temp)
    temp = temp1;
    ptr = offset_ptr(emu, page + p.first_page, offset);
    if (ptr)
    memset(ptr, value, temp);
    offset = nextofs;
    page++;
    } while (offset < end_offset);
    return 0;
    }
    EXPORT_SYMBOL(snd_emu10k1_synth_memset);
// Note that the value is assumed to be suitably repetitive.
#[no_mangle]
unsafe extern "C" fn xor_range(ptr: *mut c_void, size: c_int, value: u32) {
    static void xor_range(void *ptr, int size, u32 value)
    {
    if ((long)ptr & 1) {
// (u8 *)ptr ^= (u8)value;
    ptr++;
    size--;
    }
    if (size > 1 && ((long)ptr & 2)) {
// (u16 *)ptr ^= (u16)value;
    ptr += 2;
    size -= 2;
    }
    while (size > 3) {
// (u32 *)ptr ^= value;
    ptr += 4;
    size -= 4;
    }
    if (size > 1) {
// (u16 *)ptr ^= (u16)value;
    ptr += 2;
    size -= 2;
    }
    if (size > 0)
// (u8 *)ptr ^= (u8)value;
    }
//
// copy_from_user(blk + offset, data, size) ^ xor
//
    int snd_emu10k1_synth_copy_from_user(struct snd_emu10k1 *emu, struct snd_util_memblk *blk,
    int offset, const char __user *data, int size, u32 xor)
    {
    int page, nextofs, end_offset, temp, temp1;
    void *ptr;
    struct snd_emu10k1_memblk *p = (struct snd_emu10k1_memblk *)blk;
    if (snd_BUG_ON(offset + size > p.mem.size))
    return -EFAULT;
    offset += blk.offset & (PAGE_SIZE - 1);
    end_offset = offset + size;
    page = get_aligned_page(offset);
    do {
    nextofs = aligned_page_offset(page + 1);
    temp = nextofs - offset;
    temp1 = end_offset - offset;
    if (temp1 < temp)
    temp = temp1;
    ptr = offset_ptr(emu, page + p.first_page, offset);
    if (ptr) {
    if (copy_from_user(ptr, data, temp))
    return -EFAULT;
    if (xor)
    xor_range(ptr, temp, xor);
    }
    offset = nextofs;
    data += temp;
    page++;
    } while (offset < end_offset);
    return 0;
    }
    EXPORT_SYMBOL(snd_emu10k1_synth_copy_from_user);
