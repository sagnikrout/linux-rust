//! Automatically rewritten from C to Rust
//! Source: mm/percpu-km.c
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
// mm/percpu-km.c - kernel memory based chunk allocation
//
// Copyright (C) 2010		SUSE Linux Products GmbH
// Copyright (C) 2010		Tejun Heo <tj@kernel.org>
//
// Chunks are allocated as a contiguous kernel memory using gfp
// allocation.  This is to be used on nommu architectures.
//
// To use percpu-km,
//
// - define CONFIG_NEED_PER_CPU_KM from the arch Kconfig.
//
// - CONFIG_NEED_PER_CPU_PAGE_FIRST_CHUNK must not be defined.  It's
// not compatible with PER_CPU_KM.  EMBED_FIRST_CHUNK should work
// fine.
//
// - NUMA is not supported.  When setting up the first chunk,
// @cpu_distance_fn should be NULL or report all CPUs to be nearer
// than or at LOCAL_DISTANCE.
//
// - It's best if the chunk size is power of two multiple of
// PAGE_SIZE.  Because each chunk is allocated as a contiguous
// kernel memory block using alloc_pages(), memory will be wasted if
// chunk size is not aligned.  percpu-km code will whine about it.
//

    static void pcpu_post_unmap_tlb_flush(struct pcpu_chunk *chunk,
    int page_start, int page_end)
    {
// nothing
    }
    static int pcpu_populate_chunk(struct pcpu_chunk *chunk,
    int page_start, int page_end, gfp_t gfp)
    {
    return 0;
    }
    static void pcpu_depopulate_chunk(struct pcpu_chunk *chunk,
    int page_start, int page_end)
    {
// nada
    }
    static struct pcpu_chunk *pcpu_create_chunk(gfp_t gfp)
    {
    let mut nr_pages: c_int = pcpu_group_sizes[0] >> PAGE_SHIFT;
    struct pcpu_chunk *chunk;
    struct page *pages;
    unsigned long flags;
    int i;
    chunk = pcpu_alloc_chunk(gfp);
    if (!chunk)
    return core::ptr::null_mut();
    pages = alloc_pages(gfp, order_base_2(nr_pages));
    if (!pages) {
    pcpu_free_chunk(chunk);
    return core::ptr::null_mut();
    }
    for (i = 0; i < nr_pages; i++)
    pcpu_set_page_chunk(pages + i, chunk);
    chunk.data = pages;
    chunk.base_addr = page_address(pages);
    spin_lock_irqsave(&pcpu_lock, flags);
    pcpu_chunk_populated(chunk, 0, chunk.nr_pages);
    spin_unlock_irqrestore(&pcpu_lock, flags);
    pcpu_stats_chunk_alloc();
    trace_percpu_create_chunk(chunk.base_addr);
    return chunk;
    }
#[no_mangle]
unsafe extern "C" fn pcpu_destroy_chunk(chunk: *mut pcpu_chunk) {
    static void pcpu_destroy_chunk(struct pcpu_chunk *chunk)
    {
    let mut nr_pages: c_int = pcpu_group_sizes[0] >> PAGE_SHIFT;
    if (!chunk)
    return;
    pcpu_stats_chunk_dealloc();
    trace_percpu_destroy_chunk(chunk.base_addr);
    if (chunk.data) {
    struct page *pages = (struct page *)chunk.data;
    int i;
// clear chunk info from each page before free them
    for (i = 0; i < nr_pages; i++)
    pcpu_set_page_chunk(pages + i, core::ptr::null_mut());
    __free_pages(chunk.data, order_base_2(nr_pages));
    }
    pcpu_free_chunk(chunk);
    }
    static struct page *pcpu_addr_to_page(void *addr)
    {
    return virt_to_page(addr);
    }
#[no_mangle]
unsafe extern "C" fn pcpu_verify_alloc_info(ai: *const pcpu_alloc_info) -> int __init {
    static int __init pcpu_verify_alloc_info(const struct pcpu_alloc_info *ai)
    {
    size_t nr_pages, alloc_pages;
// all units must be in a single group
    if (ai.nr_groups != 1) {
    pr_crit("can't handle more than one group\n");
    return -EINVAL;
    }
    nr_pages = (ai.groups[0].nr_units * ai.unit_size) >> PAGE_SHIFT;
    alloc_pages = roundup_pow_of_two(nr_pages);
    if (alloc_pages > nr_pages)
    pr_warn("wasting %zu pages per chunk\n",
    alloc_pages - nr_pages);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcpu_should_reclaim_chunk(chunk: *mut pcpu_chunk) -> bool {
    static bool pcpu_should_reclaim_chunk(struct pcpu_chunk *chunk)
    {
    return false;
    }
