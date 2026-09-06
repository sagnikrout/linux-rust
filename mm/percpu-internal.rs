//! Automatically rewritten from C Header to Rust Module
//! Source: mm/percpu-internal.h
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
// pcpu_block_md is the metadata block struct.
// Each chunk's bitmap is split into a number of full blocks.
// All units are in terms of bits.
//
// The scan hint is the largest known contiguous area before the contig hint.
// It is not necessarily the actual largest contig hint though.  There is an
// invariant that the scan_hint_start > contig_hint_start iff
// scan_hint == contig_hint.  This is necessary because when scanning forward,
// we don't know if a new contig hint would be better than the current one.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcpu_block_md {
    pub /: *mut *mut int scan_hint; / scan hint for block,
    pub starting: *mut *mut int scan_hint_start; / block relative,
    pub /: *mut *mut int contig_hint; / contig hint for block,
    pub starting: *mut *mut int contig_hint_start; / block relative,
    pub along: *mut *mut int left_free; / size of free space,
    pub along: *mut *mut int right_free; / size of free space,
    pub /: *mut *mut int first_free; / block position of first free,
    pub /: *mut *mut int nr_bits; / total bits responsible for,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcpuobj_ext {

    pub cgroup: *mut obj_cgroup,

    pub tag: codetag_ref,

}

// Macro flag: #define NEED_PCPUOBJ_EXT

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcpu_chunk {

    pub /: *mut *mut int nr_alloc; / # of allocations,
    pub /: *mut *mut size_t max_alloc_size; / largest allocation size,

    pub /: *mut *mut list_head list; / linked to pcpu_slot lists,
    pub /: *mut *mut int free_bytes; / free bytes in the chunk,
    pub chunk_md: pcpu_block_md,
    pub /: *mut *mut *mut unsigned long bound_map; / boundary map,
//
// base_addr is the base address of this chunk.
// To reduce false sharing, current layout is optimized to make sure
// base_addr locate in the different cacheline with free_bytes and
// chunk_md.
//
    pub ____cacheline_aligned_in_smp: *mut *mut void base_addr,
    pub /: *mut *mut *mut unsigned long alloc_map; / allocation map,
    pub /: *mut *mut *mut pcpu_block_md md_blocks; / metadata blocks,
    pub /: *mut *mut *mut void data; / chunk data,
    pub /: *mut *mut bool immutable; / no [de]population allowed,
    pub chunk: *mut *mut bool isolated; / isolated from active,
    pub previous: *mut *mut int start_offset; / the overlap with the,
    pub to: *mut *mut int end_offset; / additional area required,
    pub /: *mut *mut int nr_pages; / # of pages served by this chunk,
    pub /: *mut *mut int nr_populated; / # of populated pages,
    pub /: *mut *mut int nr_empty_pop_pages; / # of empty populated pages,

    pub /: *mut *mut *mut pcpuobj_ext obj_exts; / vector of object cgroups,

    pub /: *mut *mut unsigned long populated[]; / populated bitmap,
}

//
// pcpu_chunk_nr_blocks - converts nr_pages to # of md_blocks
// @chunk: chunk of interest
//
// This conversion is from the number of physical pages that the chunk
// serves to the number of bitmap blocks used.
//
// pcpu_nr_pages_to_map_bits - converts the pages to size of bitmap
// @pages: number of physical pages
//
// This conversion is from physical pages to the number of bits
// required in the bitmap.
//
// pcpu_chunk_map_bits - helper to convert nr_pages to size of bitmap
// @chunk: chunk of interest
//
// This conversion is from the number of physical pages that the chunk
// serves to the number of bits in the bitmap.
//
extern "C" {
    pub fn pcpu_nr_pages_to_map_bits(_arg: chunk->nr_pages) -> return;
}
//
// pcpu_obj_full_size - helper to calculate size of each accounted object
// @size: size of area to allocate in bytes
//
// For each accounted object there is an extra space which is used to store
// obj_cgroup membership if kmemcg is not disabled. Charge it too.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_stats {
    pub /: *mut *mut u64 nr_alloc; / lifetime # of allocations,
    pub /: *mut *mut u64 nr_dealloc; / lifetime # of deallocations,
    pub /: *mut *mut u64 nr_cur_alloc; / current # of allocations,
    pub /: *mut *mut u64 nr_max_alloc; / max # of live allocations,
    pub /: *mut *mut u32 nr_chunks; / current # of live chunks,
    pub /: *mut *mut u32 nr_max_chunks; / max # of live chunks,
    pub /: *mut *mut size_t min_alloc_size; / min allocation size,
    pub /: *mut *mut size_t max_alloc_size; / max allocation size,
}

//
// For debug purposes. We don't care about the flexible array.
//
// initialize min_alloc_size to unit_size
//
// pcpu_stats_area_alloc - increment area allocation stats
// @chunk: the location of the area being allocated
// @size: size of area to allocate in bytes
//
// CONTEXT:
// pcpu_lock.
//
// pcpu_stats_area_dealloc - decrement allocation stats
// @chunk: the location of the area being deallocated
//
// CONTEXT:
// pcpu_lock.
//
// pcpu_stats_chunk_alloc - increment chunk stats
//
// pcpu_stats_chunk_dealloc - decrement chunk stats
//

