//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/ttm/ttm_resource.h
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


//
// Copyright 2020 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: Christian König
//

pub const TTM_NUM_MEM_TYPES: c_int = 9;
//
// define TTM_NUM_MOVE_FENCES - How many entities can be used for evictions
//
// Pipelined evictions can be spread on multiple entities. This
// is the max number of entities that can be used by the driver
// for that purpose.
//
pub const TTM_NUM_MOVE_FENCES: c_int = 8;
//
// enum ttm_lru_item_type - enumerate ttm_lru_item subclasses
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ttm_lru_item_type {
// @TTM_LRU_RESOURCE: The resource subclass
    TTM_LRU_RESOURCE,
// @TTM_LRU_HITCH: The iterator hitch subclass
    TTM_LRU_HITCH
}

//
// struct ttm_lru_item - The TTM lru list node base class
// @link: The list link
// @type: The subclass type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_lru_item {
    pub link: list_head,
    pub type: ttm_lru_item_type,
}

//
// ttm_lru_item_init() - initialize a struct ttm_lru_item
// @item: The item to initialize
// @type: The subclass type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_resource_manager_func {
//
// struct ttm_resource_manager_func member alloc
//
// @man: Pointer to a memory type manager.
// @bo: Pointer to the buffer object we're allocating space for.
// @place: Placement details.
// @res: Resulting pointer to the ttm_resource.
//
// This function should allocate space in the memory type managed
// by @man. Placement details if applicable are given by @place. If
// successful, a filled in ttm_resource object should be returned in
// @res. @res::start should be set to a value identifying the beginning
// of the range allocated, and the function should return zero.
// If the manager can't fulfill the request -ENOSPC should be returned.
// If a system error occurred, preventing the request to be fulfilled,
// the function should return a negative error code.
//
// This function may not be called from within atomic context and needs
// to take care of its own locking to protect any data structures
// managing the space.
//
    pub res): *mut ttm_resource,
//
// struct ttm_resource_manager_func member free
//
// @man: Pointer to a memory type manager.
// @res: Pointer to a struct ttm_resource to be freed.
//
// This function frees memory type resources previously allocated.
// May not be called from within atomic context.
//
    pub res): *mut ttm_resource,
//
// struct ttm_resource_manager_func member intersects
//
// @man: Pointer to a memory type manager.
// @res: Pointer to a struct ttm_resource to be checked.
// @place: Placement to check against.
// @size: Size of the check.
//
// Test if @res intersects with @place + @size. Used to judge if
// evictions are valueable or not.
//
    pub size): usize,
//
// struct ttm_resource_manager_func member compatible
//
// @man: Pointer to a memory type manager.
// @res: Pointer to a struct ttm_resource to be checked.
// @place: Placement to check against.
// @size: Size of the check.
//
// Test if @res compatible with @place + @size. Used to check of
// the need to move the backing store or not.
//
    pub size): usize,
//
// struct ttm_resource_manager_func member debug
//
// @man: Pointer to a memory type manager.
// @printer: Prefix to be used in printout to identify the caller.
//
// This function is called to print out the state of the memory
// type manager to aid debugging of out-of-memory conditions.
// It may not be called from within atomic context.
//
    pub printer): *mut drm_printer,
}

//
// struct ttm_resource_manager
//
// @use_type: The memory type is enabled.
// @use_tt: If a TT object should be used for the backing store.
// @size: Size of the managed region.
// @bdev: ttm device this manager belongs to
// @func: structure pointer implementing the range manager. See above
// @eviction_lock: lock for eviction fences
// @eviction_fences: The fences of the last pipelined move operation.
// @lru: The lru list for this memory type.
//
// This structure is used to identify and manage memory types for a device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_resource_manager {
//
// No protection. Constant from start.
//
    pub use_type: bool,
    pub use_tt: bool,
    pub bdev: *mut ttm_device,
    pub size: u64,
    pub func: *const ttm_resource_manager_func,
// This is very similar to a dma_resv object, but locking rules make
// it difficult to use one in this context.
//
    pub eviction_lock: spinlock_t,
    pub eviction_fences: [*mut dma_fence; TTM_NUM_MOVE_FENCES],
//
// Protected by the bdev->lru_lock.
//
    pub lru: [list_head; TTM_MAX_BO_PRIORITY],
//
// @usage: How much of the resources are used, protected by the
// bdev->lru_lock.
//
    pub usage: u64,
//
// @cg: &dmem_cgroup_region used for memory accounting, if not NULL.
//
    pub cg: *mut dmem_cgroup_region,
}

//
// struct ttm_bus_placement
//
// @addr:		mapped virtual address
// @offset:		physical addr
// @is_iomem:		is this io memory ?
// @caching:		See enum ttm_caching
//
// Structure indicating the bus placement of an object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_bus_placement {
    pub addr: *mut c_void,
    pub offset: phys_addr_t,
    pub is_iomem: bool,
    pub caching: ttm_caching,
}

//
// struct ttm_resource
//
// @start: Start of the allocation.
// @size: Actual size of resource in bytes.
// @mem_type: Resource type of the allocation.
// @placement: Placement flags.
// @bus: Placement on io bus accessible to the CPU
// @bo: weak reference to the BO, protected by ttm_device::lru_lock
// @css: cgroup state this resource is charged to
//
// Structure indicating the placement and space resources used by a
// buffer object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_resource {
    pub start: c_ulong,
    pub size: usize,
    pub mem_type: u32,
    pub placement: u32,
    pub bus: ttm_bus_placement,
    pub bo: *mut ttm_buffer_object,
    pub css: *mut dmem_cgroup_pool_state,
//
// @lru: Least recently used list, see &ttm_resource_manager.lru
//
    pub lru: ttm_lru_item,
}

//
// ttm_lru_item_to_res() - Downcast a struct ttm_lru_item to a struct ttm_resource
// @item: The struct ttm_lru_item to downcast
//
// Return: Pointer to the embedding struct ttm_resource
//
extern "C" {
    pub fn container_of(_arg: item, ttm_resource: struct, _arg: lru) -> return;
}
//
// struct ttm_lru_bulk_move_pos
//
// @first: first res in the bulk move range
// @last: last res in the bulk move range
//
// Range of resources for a lru bulk move.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_lru_bulk_move_pos {
    pub first: *mut ttm_resource,
    pub last: *mut ttm_resource,
}

//
// struct ttm_lru_bulk_move
// @pos: first/last lru entry for resources in the each domain/priority
// @cursor_list: The list of cursors currently traversing any of
// the sublists of @pos. Protected by the ttm device's lru_lock.
//
// Container for the current bulk move state. Should be used with
// ttm_lru_bulk_move_init() and ttm_bo_set_bulk_move().
// All BOs in a bulk_move structure need to share the same reservation object to
// ensure that the bulk as a whole is locked for eviction even if only one BO of
// the bulk is evicted.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_lru_bulk_move {
    pub pos: [ttm_lru_bulk_move_pos; TTM_NUM_MEM_TYPES][TTM_MAX_BO_PRIORITY],
    pub cursor_list: list_head,
}

//
// struct ttm_resource_cursor
// @man: The resource manager currently being iterated over
// @hitch: A hitch list node inserted before the next resource
// to iterate over.
// @bulk_link: A list link for the list of cursors traversing the
// bulk sublist of @bulk. Protected by the ttm device's lru_lock.
// @bulk: Pointer to struct ttm_lru_bulk_move whose subrange @hitch is
// inserted to. NULL if none. Never dereference this pointer since
// the struct ttm_lru_bulk_move object pointed to might have been
// freed. The pointer is only for comparison.
// @mem_type: The memory type of the LRU list being traversed.
// This field is valid iff @bulk != NULL.
// @priority: the current priority
//
// Cursor to iterate over the resources in a manager.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_resource_cursor {
    pub man: *mut ttm_resource_manager,
    pub hitch: ttm_lru_item,
    pub bulk_link: list_head,
    pub bulk: *mut ttm_lru_bulk_move,
    pub mem_type: c_uint,
    pub priority: c_uint,
}

extern "C" {
    pub fn ttm_resource_cursor_fini(cursor: *mut ttm_resource_cursor);
}
//
// struct ttm_kmap_iter_iomap - Specialization for a struct io_mapping +
// struct sg_table backed struct ttm_resource.
// @base: Embedded struct ttm_kmap_iter providing the usage interface.
// @iomap: struct io_mapping representing the underlying linear io_memory.
// @st: sg_table into @iomap, representing the memory of the struct ttm_resource.
// @start: Offset that needs to be subtracted from @st to make
// sg_dma_address(st->sgl) - @start == 0 for @iomap start.
// @cache: Scatterlist traversal cache for fast lookups.
// @cache.sg: Pointer to the currently cached scatterlist segment.
// @cache.i: First index of @sg. PAGE_SIZE granularity.
// @cache.end: Last index + 1 of @sg. PAGE_SIZE granularity.
// @cache.offs: First offset into @iomap of @sg. PAGE_SIZE granularity.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_kmap_iter_iomap {
    pub base: ttm_kmap_iter,
    pub iomap: *mut io_mapping,
    pub st: *mut sg_table,
    pub start: resource_size_t,
    pub sg: *mut scatterlist,
    pub i: pgoff_t,
    pub end: pgoff_t,
    pub offs: pgoff_t,
    pub cache: },
}

//
// struct ttm_kmap_iter_linear_io - Iterator specialization for linear io
// @base: The base iterator
// @dmap: Points to the starting address of the region
// @needs_unmap: Whether we need to unmap on fini
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_kmap_iter_linear_io {
    pub base: ttm_kmap_iter,
    pub dmap: iosys_map,
    pub needs_unmap: bool,
}

//
// ttm_resource_manager_set_used
//
// @man: A memory manager object.
// @used: usage state to set.
//
// Set the manager in use flag. If disabled the manager is no longer
// used for object placement.
//
// ttm_resource_manager_used
//
// @man: Manager to get used state for
//
// Get the in use flag for a manager.
// Returns:
// true is used, false if not.
//
// ttm_resource_manager_cleanup
//
// @man: A memory manager object.
//
// Cleanup the move fences from the memory manager object.
//
extern "C" {
    pub fn ttm_lru_bulk_move_init(bulk: *mut ttm_lru_bulk_move);
}
extern "C" {
    pub fn ttm_lru_bulk_move_tail(bulk: *mut ttm_lru_bulk_move);
}
extern "C" {
    pub fn ttm_resource_move_to_lru_tail(res: *mut ttm_resource);
}
extern "C" {
    pub fn ttm_resource_free(bo: *mut ttm_buffer_object, res: *mut ttm_resource);
}
extern "C" {
    pub fn ttm_resource_manager_usage(man: *mut ttm_resource_manager) -> u64;
}
//
// ttm_resource_manager_for_each_res - iterate over all resources
// @cursor: struct ttm_resource_cursor for the current position
// @res: the current resource
//
// Iterate over all the evictable resources in a resource manager.
//

