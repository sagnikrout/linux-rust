//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/memblock.h
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
// Logical memory blocks.
//
// Copyright (C) 2001 Peter Bergner, IBM Corp.
//

//
// highest page
//
// highest possible page
//
// enum memblock_flags - definition of memory region attributes
// @MEMBLOCK_NONE: no special request
// @MEMBLOCK_HOTPLUG: memory region indicated in the firmware-provided memory
// map during early boot as hot(un)pluggable system RAM (e.g., memory range
// that might get hotunplugged later). With "movable_node" set on the kernel
// commandline, try keeping this memory region hotunpluggable. Does not apply
// to memblocks added ("hotplugged") after early boot.
// @MEMBLOCK_MIRROR: mirrored region
// @MEMBLOCK_NOMAP: don't add to kernel direct mapping and treat as
// reserved in the memory map; refer to memblock_mark_nomap() description
// for further details
// @MEMBLOCK_DRIVER_MANAGED: memory region that is always detected and added
// via a driver, and never indicated in the firmware-provided memory map as
// system RAM. This corresponds to IORESOURCE_SYSRAM_DRIVER_MANAGED in the
// kernel resource tree.
// @MEMBLOCK_RSRV_NOINIT: reserved memory region for which struct pages are not
// fully initialized. Users of this flag are responsible to properly initialize
// struct pages of this region
// @MEMBLOCK_RSRV_KERN: memory region that is reserved for kernel use,
// either explictitly with memblock_reserve_kern() or via memblock
// allocation APIs. All memblock allocations set this flag.
// @MEMBLOCK_KHO_SCRATCH: memory region that kexec can pass to the next
// kernel in handover mode. During early boot, we do not know about all
// memory reservations yet, so we get scratch memory from the previous
// kernel that we know is good to use. It is the only memory that
// allocations may happen from in this phase.
// @MEMBLOCK_RSRV_HUGETLB: memory is reserved for hugetlb pages
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum memblock_flags {
    MEMBLOCK_NONE		= 0x0,	/* No special request */
    MEMBLOCK_HOTPLUG	= 0x1,	/* hotpluggable region */
    MEMBLOCK_MIRROR		= 0x2,	/* mirrored region */
    MEMBLOCK_NOMAP		= 0x4,	/* don't add to kernel direct mapping */
    MEMBLOCK_DRIVER_MANAGED = 0x8,	/* always detected via a driver */
    MEMBLOCK_RSRV_NOINIT	= 0x10,	/* don't initialize struct pages */
    MEMBLOCK_RSRV_KERN	= 0x20,	/* memory reserved for kernel use */
    MEMBLOCK_KHO_SCRATCH	= 0x40,	/* scratch memory for kexec handover */
    MEMBLOCK_RSRV_HUGETLB	= 0x80, /* memory reserved for hugetlb pages */
}

//
// struct memblock_region - represents a memory region
// @base: base address of the region
// @size: size of the region
// @flags: memory region attributes
// @nid: NUMA node id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct memblock_region {
    pub base: phys_addr_t,
    pub size: phys_addr_t,
    pub flags: memblock_flags,

    pub nid: c_int,

}

//
// struct memblock_type - collection of memory regions of certain type
// @cnt: number of regions
// @max: size of the allocated array
// @total_size: size of all regions
// @regions: array of regions
// @name: the memory type symbolic name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct memblock_type {
    pub cnt: c_ulong,
    pub max: c_ulong,
    pub total_size: phys_addr_t,
    pub regions: *mut memblock_region,
    pub name: *mut c_char,
}

//
// struct memblock - memblock allocator metadata
// @bottom_up: is bottom up direction?
// @current_limit: physical address of the current allocation limit
// @memory: usable memory regions
// @reserved: reserved memory regions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct memblock {
    pub /: *mut *mut bool bottom_up; / is bottom up direction?,
    pub current_limit: phys_addr_t,
    pub memory: memblock_type,
    pub reserved: memblock_type,
}

extern "C" {
    pub fn memblock_discard();
}

// Macro flag: #define __init_memblock
// Macro flag: #define __initdata_memblock

extern "C" {
    pub fn memblock_allow_resize();
}
extern "C" {
    pub fn memblock_add(base: phys_addr_t, size: phys_addr_t) -> c_int;
}
extern "C" {
    pub fn memblock_remove(base: phys_addr_t, size: phys_addr_t) -> c_int;
}
extern "C" {
    pub fn memblock_phys_free(base: phys_addr_t, size: phys_addr_t) -> c_int;
}
extern "C" {
    pub fn __memblock_reserve(_arg: base, _arg: size, _arg: NUMA_NO_NODE, _arg: 0) -> return;
}
extern "C" {
    pub fn __memblock_reserve(_arg: base, _arg: size, _arg: NUMA_NO_NODE, _arg: MEMBLOCK_RSRV_KERN) -> return;
}

extern "C" {
    pub fn memblock_physmem_add(base: phys_addr_t, size: phys_addr_t) -> c_int;
}

extern "C" {
    pub fn memblock_trim_memory(align: phys_addr_t);
}
extern "C" {
    pub fn memblock_validate_numa_coverage(threshold_bytes: c_ulong) -> bool;
}
extern "C" {
    pub fn memblock_mark_hotplug(base: phys_addr_t, size: phys_addr_t) -> c_int;
}
extern "C" {
    pub fn memblock_clear_hotplug(base: phys_addr_t, size: phys_addr_t) -> c_int;
}
extern "C" {
    pub fn memblock_mark_mirror(base: phys_addr_t, size: phys_addr_t) -> c_int;
}
extern "C" {
    pub fn memblock_mark_nomap(base: phys_addr_t, size: phys_addr_t) -> c_int;
}
extern "C" {
    pub fn memblock_clear_nomap(base: phys_addr_t, size: phys_addr_t) -> c_int;
}
extern "C" {
    pub fn memblock_reserved_mark_noinit(base: phys_addr_t, size: phys_addr_t) -> c_int;
}
extern "C" {
    pub fn memblock_reserved_mark_kern(base: phys_addr_t, size: phys_addr_t) -> c_int;
}
extern "C" {
    pub fn memblock_mark_kho_scratch(base: phys_addr_t, size: phys_addr_t) -> c_int;
}
extern "C" {
    pub fn memblock_clear_kho_scratch(base: phys_addr_t, size: phys_addr_t) -> c_int;
}
extern "C" {
    pub fn memblock_free(ptr: *mut c_void, size: usize);
}
extern "C" {
    pub fn reset_all_zones_managed_pages();
}
// Low level functions

//
// for_each_physmem_range - iterate through physmem areas not included in type.
// @i: u64 used as loop variable
// @type: ptr to memblock_type which excludes from the iteration, can be %NULL
// @p_start: ptr to phys_addr_t for start address of the range, can be %NULL
// @p_end: ptr to phys_addr_t for end address of the range, can be %NULL
//

//
// __for_each_mem_range - iterate through memblock areas from type_a and not
// included in type_b. Or just type_a if type_b is NULL.
// @i: u64 used as loop variable
// @type_a: ptr to memblock_type to iterate
// @type_b: ptr to memblock_type which excludes from the iteration
// @nid: node selector, %NUMA_NO_NODE for all nodes
// @flags: pick from blocks based on memory attributes
// @p_start: ptr to phys_addr_t for start address of the range, can be %NULL
// @p_end: ptr to phys_addr_t for end address of the range, can be %NULL
// @p_nid: ptr to int for nid of the range, can be %NULL
//

//
// __for_each_mem_range_rev - reverse iterate through memblock areas from
// type_a and not included in type_b. Or just type_a if type_b is NULL.
// @i: u64 used as loop variable
// @type_a: ptr to memblock_type to iterate
// @type_b: ptr to memblock_type which excludes from the iteration
// @nid: node selector, %NUMA_NO_NODE for all nodes
// @flags: pick from blocks based on memory attributes
// @p_start: ptr to phys_addr_t for start address of the range, can be %NULL
// @p_end: ptr to phys_addr_t for end address of the range, can be %NULL
// @p_nid: ptr to int for nid of the range, can be %NULL
//

//
// for_each_mem_range - iterate through memory areas.
// @i: u64 used as loop variable
// @p_start: ptr to phys_addr_t for start address of the range, can be %NULL
// @p_end: ptr to phys_addr_t for end address of the range, can be %NULL
//

//
// for_each_mem_range_rev - reverse iterate through memblock areas from
// type_a and not included in type_b. Or just type_a if type_b is NULL.
// @i: u64 used as loop variable
// @p_start: ptr to phys_addr_t for start address of the range, can be %NULL
// @p_end: ptr to phys_addr_t for end address of the range, can be %NULL
//

//
// for_each_reserved_mem_range - iterate over all reserved memblock areas
// @i: u64 used as loop variable
// @p_start: ptr to phys_addr_t for start address of the range, can be %NULL
// @p_end: ptr to phys_addr_t for end address of the range, can be %NULL
//
// Walks over reserved areas of memblock. Available as soon as memblock
// is initialized.
//

//
// for_each_mem_pfn_range - early memory pfn range iterator
// @i: an integer used as loop variable
// @nid: node selector, %MAX_NUMNODES for all nodes
// @p_start: ptr to ulong for start pfn of the range, can be %NULL
// @p_end: ptr to ulong for end pfn of the range, can be %NULL
// @p_nid: ptr to int for nid of the range, can be %NULL
//
// Walks over configured memory ranges.
//

//
// for_each_free_mem_range - iterate through free memblock areas
// @i: u64 used as loop variable
// @nid: node selector, %NUMA_NO_NODE for all nodes
// @flags: pick from blocks based on memory attributes
// @p_start: ptr to phys_addr_t for start address of the range, can be %NULL
// @p_end: ptr to phys_addr_t for end address of the range, can be %NULL
// @p_nid: ptr to int for nid of the range, can be %NULL
//
// Walks over free (memory && !reserved) areas of memblock.  Available as
// soon as memblock is initialized.
//

//
// for_each_free_mem_range_reverse - rev-iterate through free memblock areas
// @i: u64 used as loop variable
// @nid: node selector, %NUMA_NO_NODE for all nodes
// @flags: pick from blocks based on memory attributes
// @p_start: ptr to phys_addr_t for start address of the range, can be %NULL
// @p_end: ptr to phys_addr_t for end address of the range, can be %NULL
// @p_nid: ptr to int for nid of the range, can be %NULL
//
// Walks over free (memory && !reserved) areas of memblock in reverse
// order.  Available as soon as memblock is initialized.
//

// Flags for memblock allocation APIs

pub const MEMBLOCK_ALLOC_ACCESSIBLE: c_int = 0;
//
// MEMBLOCK_ALLOC_NOLEAKTRACE avoids kmemleak tracing. It implies
// MEMBLOCK_ALLOC_ACCESSIBLE
//
pub const MEMBLOCK_ALLOC_NOLEAKTRACE: c_int = 1;
// We are using top down, so it is safe to use 0 here
pub const MEMBLOCK_LOW_LIMIT: c_int = 0;

pub const ARCH_LOW_ADDRESS_LIMIT: c_uint = 0xffffffffUL;

extern "C" {
    pub fn memblock_phys_alloc_try_nid(size: phys_addr_t, align: phys_addr_t, nid: c_int) -> phys_addr_t;
}

//
// Set the allocation direction to bottom-up or top-down.
//
// Check if the allocation direction is bottom-up or not.
// if this is true, that said, memblock will allocate memory
// in bottom-up direction.
//
extern "C" {
    pub fn memblock_phys_mem_size() -> phys_addr_t;
}
extern "C" {
    pub fn memblock_reserved_size() -> phys_addr_t;
}
extern "C" {
    pub fn memblock_reserved_kern_size(limit: phys_addr_t, nid: c_int) -> phys_addr_t;
}
extern "C" {
    pub fn memblock_reserved_hugetlb_size(limit: phys_addr_t, nid: c_int) -> phys_addr_t;
}
extern "C" {
    pub fn memblock_estimated_nr_free_pages() -> c_ulong;
}
extern "C" {
    pub fn memblock_start_of_DRAM() -> phys_addr_t;
}
extern "C" {
    pub fn memblock_end_of_DRAM() -> phys_addr_t;
}
extern "C" {
    pub fn memblock_enforce_memory_limit(memory_limit: phys_addr_t);
}
extern "C" {
    pub fn memblock_cap_memory_range(base: phys_addr_t, size: phys_addr_t);
}
extern "C" {
    pub fn memblock_mem_limit_remove_map(limit: phys_addr_t);
}
extern "C" {
    pub fn memblock_is_memory(addr: phys_addr_t) -> bool;
}
extern "C" {
    pub fn memblock_is_map_memory(addr: phys_addr_t) -> bool;
}
extern "C" {
    pub fn memblock_is_region_memory(base: phys_addr_t, size: phys_addr_t) -> bool;
}
extern "C" {
    pub fn memblock_is_reserved(addr: phys_addr_t) -> bool;
}
extern "C" {
    pub fn memblock_is_region_reserved(base: phys_addr_t, size: phys_addr_t) -> bool;
}
extern "C" {
    pub fn memblock_dump_all();
}
//
// memblock_set_current_limit - Set the current allocation limit to allow
// limiting allocations to what is currently
// accessible during boot
// @limit: New limit value (physical address)
//
extern "C" {
    pub fn memblock_set_current_limit(limit: phys_addr_t);
}
extern "C" {
    pub fn memblock_get_current_limit() -> phys_addr_t;
}
//
// pfn conversion functions
//
// While the memory MEMBLOCKs should always be page aligned, the reserved
// MEMBLOCKs may not be. This accessor attempt to provide a very clear
// idea of what they return for such non aligned MEMBLOCKs.
//
// memblock_region_memory_base_pfn - get the lowest pfn of the memory region
// @reg: memblock_region structure
//
// Return: the lowest pfn intersecting with the memory region
//
extern "C" {
    pub fn PFN_UP(_arg: reg->base) -> return;
}
//
// memblock_region_memory_end_pfn - get the end pfn of the memory region
// @reg: memblock_region structure
//
// Return: the end_pfn of the reserved region
//
extern "C" {
    pub fn PFN_DOWN(reg->size: reg->base +) -> return;
}
//
// memblock_region_reserved_base_pfn - get the lowest pfn of the reserved region
// @reg: memblock_region structure
//
// Return: the lowest pfn intersecting with the reserved region
//
extern "C" {
    pub fn PFN_DOWN(_arg: reg->base) -> return;
}
//
// memblock_region_reserved_end_pfn - get the end pfn of the reserved region
// @reg: memblock_region structure
//
// Return: the end_pfn of the reserved region
//
extern "C" {
    pub fn PFN_UP(reg->size: reg->base +) -> return;
}
//
// for_each_mem_region - iterate over memory regions
// @region: loop variable
//

//
// for_each_reserved_mem_region - itereate over reserved memory regions
// @region: loop variable
//

pub const HASH_EARLY: c_uint = 0x00000001	/* Allocating during early boot? */;
pub const HASH_ZERO: c_uint = 0x00000002	/* Zero allocated hash table */;
// Only NUMA needs hash distribution. 64bit NUMA architectures have
// sufficient vmalloc space.
//

extern "C" {
    pub fn early_memtest(start: phys_addr_t, end: phys_addr_t);
}
extern "C" {
    pub fn memtest_report_meminfo(m: *mut seq_file);
}

extern "C" {
    pub fn memblock_set_kho_scratch_only();
}
extern "C" {
    pub fn memblock_clear_kho_scratch_only();
}

