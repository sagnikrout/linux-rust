//! Automatically rewritten from C to Rust
//! Source: mm/percpu-stats.c
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
// Copyright (C) 2017		Facebook Inc.
// Copyright (C) 2017		Dennis Zhou <dennis@kernel.org>
//
// Prints statistics about the percpu allocator and backing chunks.
//

    seq_printf(m, "  %-20s: %12lld\n", X, (long long int)Y)
    struct percpu_stats pcpu_stats;
    struct pcpu_alloc_info pcpu_stats_ai;
#[no_mangle]
unsafe extern "C" fn cmpint(a: *const c_void, b: *const c_void) -> c_int {
    static int cmpint(const void *a, const void *b)
    {
    return *(int *)a - *(int *)b;
    }
//
// Iterates over all chunks to find the max nr_alloc entries.
//
#[no_mangle]
unsafe extern "C" fn find_max_nr_alloc() -> c_int {
    static int find_max_nr_alloc(void)
    {
    struct pcpu_chunk *chunk;
    int slot, max_nr_alloc;
    max_nr_alloc = 0;
    for (slot = 0; slot < pcpu_nr_slots; slot++)
    list_for_each_entry(chunk, &pcpu_chunk_lists[slot], list)
    max_nr_alloc = max(max_nr_alloc, chunk.nr_alloc);
    return max_nr_alloc;
    }
//
// Prints out chunk state. Fragmentation is considered between
// the beginning of the chunk to the last allocation.
//
// All statistics are in bytes unless stated otherwise.
//
    static void chunk_map_stats(struct seq_file *m, struct pcpu_chunk *chunk,
    int *buffer)
    {
    struct pcpu_block_md *chunk_md = &chunk.chunk_md;
    int i, last_alloc, as_len, start, end;
    int *alloc_sizes, *p;
// statistics
    let mut sum_frag: c_int = 0, max_frag = 0;
    let mut cur_min_alloc: c_int = 0, cur_med_alloc = 0, cur_max_alloc = 0;
    alloc_sizes = buffer;
//
// find_last_bit returns the start value if nothing found.
// Therefore, we must determine if it is a failure of find_last_bit
// and set the appropriate value.
//
    last_alloc = find_last_bit(chunk.alloc_map,
    pcpu_chunk_map_bits(chunk) -
    chunk.end_offset / PCPU_MIN_ALLOC_SIZE - 1);
    last_alloc = test_bit(last_alloc, chunk.alloc_map) ?
    last_alloc + 1 : 0;
    as_len = 0;
    start = chunk.start_offset / PCPU_MIN_ALLOC_SIZE;
//
// If a bit is set in the allocation map, the bound_map identifies
// where the allocation ends.  If the allocation is not set, the
// bound_map does not identify free areas as it is only kept accurate
// on allocation, not free.
//
// Positive values are allocations and negative values are free
// fragments.
//
    while (start < last_alloc) {
    if (test_bit(start, chunk.alloc_map)) {
    end = find_next_bit(chunk.bound_map, last_alloc,
    start + 1);
    alloc_sizes[as_len] = 1;
    } else {
    end = find_next_bit(chunk.alloc_map, last_alloc,
    start + 1);
    alloc_sizes[as_len] = -1;
    }
    alloc_sizes[as_len++] *= (end - start) * PCPU_MIN_ALLOC_SIZE;
    start = end;
    }
//
// The negative values are free fragments and thus sorting gives the
// free fragments at the beginning in largest first order.
//
    if (as_len > 0) {
    sort(alloc_sizes, as_len, sizeof(int), cmpint, core::ptr::null_mut());
// iterate through the unallocated fragments
    for (i = 0, p = alloc_sizes; *p < 0 && i < as_len; i++, p++) {
    sum_frag -= *p;
    max_frag = max(max_frag, -1 * (*p));
    }
    cur_min_alloc = alloc_sizes[i];
    cur_med_alloc = alloc_sizes[(i + as_len - 1) / 2];
    cur_max_alloc = alloc_sizes[as_len - 1];
    }
    P("nr_alloc", chunk.nr_alloc);
    P("max_alloc_size", chunk.max_alloc_size);
    P("empty_pop_pages", chunk.nr_empty_pop_pages);
    P("first_bit", chunk_md.first_free);
    P("free_bytes", chunk.free_bytes);
    P("contig_bytes", chunk_md.contig_hint * PCPU_MIN_ALLOC_SIZE);
    P("sum_frag", sum_frag);
    P("max_frag", max_frag);
    P("cur_min_alloc", cur_min_alloc);
    P("cur_med_alloc", cur_med_alloc);
    P("cur_max_alloc", cur_max_alloc);
    seq_putc(m, '\n');
    }
#[no_mangle]
unsafe extern "C" fn percpu_stats_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int percpu_stats_show(struct seq_file *m, void *v)
    {
    struct pcpu_chunk *chunk;
    int slot, max_nr_alloc;
    int *buffer;
    alloc_buffer:
    spin_lock_irq(&pcpu_lock);
    max_nr_alloc = find_max_nr_alloc();
    spin_unlock_irq(&pcpu_lock);
// there can be at most this many free and allocated fragments
    buffer = vmalloc_array(2 * max_nr_alloc + 1, sizeof(int));
    if (!buffer)
    return -ENOMEM;
    spin_lock_irq(&pcpu_lock);
// if the buffer allocated earlier is too small
    if (max_nr_alloc < find_max_nr_alloc()) {
    spin_unlock_irq(&pcpu_lock);
    vfree(buffer);
    goto alloc_buffer;
    }

    seq_printf(m, "  %-20s: %12lld\n", #X, (long long int)pcpu_stats_ai.X)
    seq_printf(m,
    "Percpu Memory Statistics\n"
    "Allocation Info:\n"
    "----------------------------------------\n");
    PL(unit_size);
    PL(static_size);
    PL(reserved_size);
    PL(dyn_size);
    PL(atom_size);
    PL(alloc_size);
    seq_putc(m, '\n');

    seq_printf(m, "  %-20s: %12llu\n", #X, (unsigned long long)pcpu_stats.X)
    seq_printf(m,
    "Global Stats:\n"
    "----------------------------------------\n");
    PU(nr_alloc);
    PU(nr_dealloc);
    PU(nr_cur_alloc);
    PU(nr_max_alloc);
    PU(nr_chunks);
    PU(nr_max_chunks);
    PU(min_alloc_size);
    PU(max_alloc_size);
    P("empty_pop_pages", pcpu_nr_empty_pop_pages);
    seq_putc(m, '\n');

    seq_printf(m,
    "Per Chunk Stats:\n"
    "----------------------------------------\n");
    if (pcpu_reserved_chunk) {
    seq_puts(m, "Chunk: <- Reserved Chunk\n");
    chunk_map_stats(m, pcpu_reserved_chunk, buffer);
    }
    for (slot = 0; slot < pcpu_nr_slots; slot++) {
    list_for_each_entry(chunk, &pcpu_chunk_lists[slot], list) {
    if (chunk == pcpu_first_chunk)
    seq_puts(m, "Chunk: <- First Chunk\n");
#[no_mangle]
pub unsafe extern "C" fn if(pcpu_to_depopulate_slot: slot ==) -> else {
    else if (slot == pcpu_to_depopulate_slot)
    seq_puts(m, "Chunk (to_depopulate)\n");
#[no_mangle]
pub unsafe extern "C" fn if(pcpu_sidelined_slot: slot ==) -> else {
    else if (slot == pcpu_sidelined_slot)
    seq_puts(m, "Chunk (sidelined):\n");
    else
    seq_puts(m, "Chunk:\n");
    chunk_map_stats(m, chunk, buffer);
    }
    }
    spin_unlock_irq(&pcpu_lock);
    vfree(buffer);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(percpu_stats);
#[no_mangle]
unsafe extern "C" fn init_percpu_stats_debugfs() -> int __init {
    static int __init init_percpu_stats_debugfs(void)
    {
    debugfs_create_file("percpu_stats", 0444, core::ptr::null_mut(), core::ptr::null_mut(),
    &percpu_stats_fops);
    return 0;
    }
    late_initcall(init_percpu_stats_debugfs);
