//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/swiotlb.h
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
// Maximum allowable number of contiguous slabs to map,
// must be a power of 2.  What is the appropriate value ?
// The complexity of {map,unmap}_single is linearly dependent on this value.
//
pub const IO_TLB_SEGSIZE: c_int = 128;
//
// log of the size of each IO TLB slab.  The number of slabs is command line
// controllable.
//
pub const IO_TLB_SHIFT: c_int = 11;

// compile-time default; overridable via CONFIG_SWIOTLB_DEFAULT_SIZE_MB

extern "C" {
    pub fn swiotlb_size_or_default() -> c_ulong;
}
extern "C" {
    pub fn swiotlb_update_mem_attributes() -> void __init;
}

//
// struct io_tlb_pool - IO TLB memory pool descriptor
// @start:	The start address of the swiotlb memory pool. Used to do a quick
// range check to see if the memory was in fact allocated by this
// API.
// @end:	The end address of the swiotlb memory pool. Used to do a quick
// range check to see if the memory was in fact allocated by this
// API.
// @vaddr:	The vaddr of the swiotlb memory pool. The swiotlb memory pool
// may be remapped in the memory encrypted case and store virtual
// address for bounce buffer operation.
// @nslabs:	The number of IO TLB slots between @start and @end. For the
// default swiotlb, this can be adjusted with a boot parameter,
// see setup_io_tlb_npages().
// @late_alloc:	%true if allocated using the page allocator.
// @nareas:	Number of areas in the pool.
// @area_nslabs: Number of slots in each area.
// @areas:	Array of memory area descriptors.
// @slots:	Array of slot descriptors.
// @node:	Member of the IO TLB memory pool list.
// @dyn_free:	RCU work item used to free the pool from process context.
// @transient:  %true if transient memory pool.
// @cc_shared:	%true if the pool memory is shared for confidential computing.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_tlb_pool {
    pub start: phys_addr_t,
    pub end: phys_addr_t,
    pub vaddr: *mut c_void,
    pub nslabs: c_ulong,
    pub late_alloc: bool,
    pub nareas: c_uint,
    pub area_nslabs: c_uint,
    pub areas: *mut io_tlb_area,
    pub slots: *mut io_tlb_slot,

    pub node: list_head,
    pub dyn_free: rcu_work,
    pub transient: bool,
    pub cc_shared: bool,

}

//
// struct io_tlb_mem - Software IO TLB allocator
// @defpool:	Default (initial) IO TLB memory pool descriptor.
// @pool:	IO TLB memory pool descriptor (if not dynamic).
// @nslabs:	Total number of IO TLB slabs in all pools.
// @debugfs:	The dentry to debugfs.
// @force_bounce: %true if swiotlb bouncing is forced
// @for_alloc:  %true if the pool is used for memory allocation
// @cc_shared:	%true if the pool memory is shared for confidential computing.
// @can_grow:	%true if more pools can be allocated dynamically.
// @phys_limit:	Maximum allowed physical address.
// @lock:	Lock to synchronize changes to the list.
// @pools:	List of IO TLB memory pool descriptors (if dynamic).
// @dyn_alloc:	Dynamic IO TLB pool allocation work.
// @total_used:	The total number of slots in the pool that are currently used
// across all areas. Used only for calculating used_hiwater via boot
// parameter swiotlb=track_hiwater and exposed via debugfs.
// @used_hiwater: The high water mark for total_used.  Can be enabled at boot
// time via swiotlb=track_hiwater and exposed via debugfs.
// @transient_nslabs: The total number of slots in all transient pools that
// are currently used across all areas.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_tlb_mem {
    pub defpool: io_tlb_pool,
    pub nslabs: c_ulong,
    pub debugfs: *mut dentry,
    pub force_bounce: bool,
    pub for_alloc: bool,
    pub cc_shared: bool,

    pub can_grow: bool,
    pub phys_limit: u64,
    pub lock: spinlock_t,
    pub pools: list_head,
    pub dyn_alloc: work_struct,

    pub total_used: atomic_long_t,
    pub used_hiwater: atomic_long_t,
    pub transient_nslabs: atomic_long_t,

}

//
// swiotlb_find_pool() - find swiotlb pool to which a physical address belongs
// @dev:        Device which has mapped the buffer.
// @paddr:      Physical address within the DMA buffer.
//
// Find the swiotlb pool that @paddr points into.
//
// Return:
// * pool address if @paddr points into a bounce buffer
// * NULL if @paddr does not point into a bounce buffer. As such, this function
// can be used to determine if @paddr denotes a swiotlb bounce buffer.
//

//
// All SWIOTLB buffer addresses must have been returned by
// swiotlb_tbl_map_single() and passed to a device driver.
// If a SWIOTLB address is checked on another CPU, then it was
// presumably loaded by the device driver from an unspecified private
// data structure. Make sure that this load is ordered before reading
// dev->dma_uses_io_tlb here and mem->pools in __swiotlb_find_pool().
//
// This barrier pairs with smp_mb() in swiotlb_find_slots().
//
extern "C" {
    pub fn __swiotlb_find_pool(_arg: dev, _arg: paddr) -> return;
}

extern "C" {
    pub fn swiotlb_init(addressing_limited: bool, flags: c_uint);
}
extern "C" {
    pub fn swiotlb_exit() -> void __init;
}
extern "C" {
    pub fn swiotlb_dev_init(dev: *mut device);
}
extern "C" {
    pub fn swiotlb_max_mapping_size(dev: *mut device) -> usize;
}
extern "C" {
    pub fn is_swiotlb_allocated() -> bool;
}
extern "C" {
    pub fn is_swiotlb_active(dev: *mut device) -> bool;
}
extern "C" {
    pub fn swiotlb_adjust_size(size: c_ulong) -> void __init;
}
extern "C" {
    pub fn default_swiotlb_base() -> phys_addr_t;
}
extern "C" {
    pub fn default_swiotlb_limit() -> phys_addr_t;
}

extern "C" {
    pub fn swiotlb_print_info();
}

extern "C" {
    pub fn swiotlb_free(dev: *mut device, page: *mut page, size: usize) -> bool;
}

