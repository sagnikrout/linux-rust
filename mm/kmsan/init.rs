//! Automatically rewritten from C to Rust
//! Source: mm/kmsan/init.c
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
// KMSAN initialization routines.
//
// Copyright (C) 2017-2021 Google LLC
// Author: Alexander Potapenko <glider@google.com>
//

pub const NUM_FUTURE_RANGES: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct start_end_pair {
    pub end: u64 start,,
}

    static struct start_end_pair start_end_pairs[NUM_FUTURE_RANGES] __initdata;
    static int future_index __initdata;
//
// Record a range of memory for which the metadata pages will be created once
// the page allocator becomes available.
//
#[no_mangle]
unsafe extern "C" fn kmsan_record_future_shadow_range(start: *mut c_void, end: *mut c_void) -> void __init {
    static void __init kmsan_record_future_shadow_range(void *start, void *end)
    {
    let mut nstart: u64 = (u64)start, nend = (u64)end, cstart, cend;
    let mut merged: bool = false;
    KMSAN_WARN_ON(future_index == NUM_FUTURE_RANGES);
    KMSAN_WARN_ON((nstart >= nend) ||
// Virtual address 0 is valid on s390.
    (!IS_ENABLED(CONFIG_S390) && !nstart) || !nend);
    nstart = ALIGN_DOWN(nstart, PAGE_SIZE);
    nend = ALIGN(nend, PAGE_SIZE);
//
// Scan the existing ranges to see if any of them overlaps with
// [start, end). In that case, merge the two ranges instead of
// creating a new one.
// The number of ranges is less than 20, so there is no need to organize
// them into a more intelligent data structure.
//
    for (int i = 0; i < future_index; i++) {
    cstart = start_end_pairs[i].start;
    cend = start_end_pairs[i].end;
    if ((cstart < nstart && cend < nstart) ||
    (cstart > nend && cend > nend))
// ranges are disjoint - do not merge
    continue;
    start_end_pairs[i].start = min(nstart, cstart);
    start_end_pairs[i].end = max(nend, cend);
    merged = true;
    break;
    }
    if (merged)
    return;
    start_end_pairs[future_index].start = nstart;
    start_end_pairs[future_index].end = nend;
    future_index++;
    }
//
// Initialize the shadow for existing mappings during kernel initialization.
// These include kernel text/data sections, NODE_DATA and future ranges
// registered while creating other data (e.g. percpu).
//
// Allocations via memblock can be only done before slab is initialized.
//
#[no_mangle]
pub unsafe extern "C" fn kmsan_init_shadow() -> void __init {
    void __init kmsan_init_shadow(void)
    {
    let mut nd_size: usize = sizeof(pg_data_t);
    phys_addr_t p_start, p_end;
    u64 loop;
    int nid;
    for_each_reserved_mem_range(loop, &p_start, &p_end)
    kmsan_record_future_shadow_range(phys_to_virt(p_start),
    phys_to_virt(p_end));
// Allocate shadow for .data
    kmsan_record_future_shadow_range(_sdata, _edata);
    for_each_online_node(nid)
    kmsan_record_future_shadow_range(
    NODE_DATA(nid), (char *)NODE_DATA(nid) + nd_size);
    for (int i = 0; i < future_index; i++)
    kmsan_init_alloc_meta_for_range(
    (void *)start_end_pairs[i].start,
    (void *)start_end_pairs[i].end);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct metadata_page_pair {
    pub origin: *mut *mut page shadow,,
}

    static struct metadata_page_pair held_back[NR_PAGE_ORDERS] __initdata;
//
// Eager metadata allocation. When the memblock allocator is freeing pages to
// pagealloc, we use 2/3 of them as metadata for the remaining 1/3.
// We store the pointers to the returned blocks of pages in held_back[] grouped
// by their order: when kmsan_memblock_free_pages() is called for the first
// time with a certain order, it is reserved as a shadow block, for the second
// time - as an origin block. On the third time the incoming block receives its
// shadow and origin ranges from the previously saved shadow and origin blocks,
// after which held_back[order] can be used again.
//
// At the very end there may be leftover blocks in held_back[]. They are
// collected later by kmsan_memblock_discard().
//
#[no_mangle]
pub unsafe extern "C" fn kmsan_memblock_free_pages(page: *mut page, order: c_uint) -> bool {
    bool kmsan_memblock_free_pages(struct page *page, unsigned int order)
    {
    struct page *shadow, *origin;
    if (!held_back[order].shadow) {
    held_back[order].shadow = page;
    return false;
    }
    if (!held_back[order].origin) {
    held_back[order].origin = page;
    return false;
    }
    shadow = held_back[order].shadow;
    origin = held_back[order].origin;
    kmsan_setup_meta(page, shadow, origin, order);
    held_back[order].shadow = core::ptr::null_mut();
    held_back[order].origin = core::ptr::null_mut();
    return true;
    }
pub const MAX_BLOCKS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smallstack {
    pub items: [*mut page; MAX_BLOCKS],
    pub index: c_int,
    pub order: c_int,
}

    static struct smallstack collect = {
    .index = 0,
    .order = MAX_PAGE_ORDER,
    };
#[no_mangle]
unsafe extern "C" fn smallstack_push(stack: *mut smallstack, pages: *mut page) {
    static void smallstack_push(struct smallstack *stack, struct page *pages)
    {
    KMSAN_WARN_ON(stack.index == MAX_BLOCKS);
    stack.items[stack.index] = pages;
    stack.index++;
    }

    static struct page *smallstack_pop(struct smallstack *stack)
    {
    struct page *ret;
    KMSAN_WARN_ON(stack.index == 0);
    stack.index--;
    ret = stack.items[stack.index];
    stack.items[stack.index] = core::ptr::null_mut();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn do_collection() {
    static void do_collection(void)
    {
    struct page *page, *shadow, *origin;
    while (collect.index >= 3) {
    page = smallstack_pop(&collect);
    shadow = smallstack_pop(&collect);
    origin = smallstack_pop(&collect);
    kmsan_setup_meta(page, shadow, origin, collect.order);
    __free_pages_core(page, collect.order, MEMINIT_EARLY);
    }
    }
#[no_mangle]
unsafe extern "C" fn collect_split() {
    static void collect_split(void)
    {
    struct smallstack tmp = {
    .order = collect.order - 1,
    .index = 0,
    };
    struct page *page;
    if (!collect.order)
    return;
    while (collect.index) {
    page = smallstack_pop(&collect);
    smallstack_push(&tmp, &page[0]);
    smallstack_push(&tmp, &page[1 << tmp.order]);
    }
    __memcpy(&collect, &tmp, sizeof(tmp));
    }
//
// Memblock is about to go away. Split the page blocks left over in held_back[]
// and return 1/3 of that memory to the system.
//
#[no_mangle]
unsafe extern "C" fn kmsan_memblock_discard() {
    static void kmsan_memblock_discard(void)
    {
//
// For each order=N:
// - push held_back[N].shadow and .origin to @collect;
// - while there are >= 3 elements in @collect, do garbage collection:
// - pop 3 ranges from @collect;
// - use two of them as shadow and origin for the third one;
// - repeat;
// - split each remaining element from @collect into 2 ranges of
// order=N-1,
// - repeat.
//
    collect.order = MAX_PAGE_ORDER;
    for (int i = MAX_PAGE_ORDER; i >= 0; i--) {
    if (held_back[i].shadow)
    smallstack_push(&collect, held_back[i].shadow);
    if (held_back[i].origin)
    smallstack_push(&collect, held_back[i].origin);
    held_back[i].shadow = core::ptr::null_mut();
    held_back[i].origin = core::ptr::null_mut();
    do_collection();
    collect_split();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_init_runtime() -> void __init {
    void __init kmsan_init_runtime(void)
    {
// Assuming current is init_task
    kmsan_internal_task_create(current);
    kmsan_memblock_discard();
    pr_info("Starting KernelMemorySanitizer\n");
    pr_info("ATTENTION: KMSAN is a debugging tool! Do not use it on production machines!\n");
    kmsan_enabled = true;
    }
