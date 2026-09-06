//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/ttm/ttm_device.h
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

//
// struct ttm_global - Buffer object driver global data.
//
// @dummy_read_page: Pointer to a dummy page used for mapping requests
// of unpopulated pages. Constant after init.
//
// @device_list: List of buffer object devices. Protected by
// ttm_global_mutex.
//
// @bo_count: Number of buffer objects allocated by devices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_device_funcs {
//
// ttm_tt_create
//
// @bo: The buffer object to create the ttm for.
// @page_flags: Page flags as identified by TTM_TT_FLAG_XX flags.
//
// Create a struct ttm_tt to back data with system memory pages.
// No pages are actually allocated.
// Returns:
// NULL: Out of memory.
//
    pub page_flags): u32,
//
// ttm_tt_populate
//
// @ttm: The struct ttm_tt to contain the backing pages.
//
// Allocate all backing pages
// Returns:
// -ENOMEM: Out of memory.
//
    pub ctx): *mut ttm_operation_ctx,
//
// ttm_tt_unpopulate
//
// @ttm: The struct ttm_tt to contain the backing pages.
//
// Free all backing page
//
    pub ttm): *mut ttm_tt,
//
// ttm_tt_destroy
//
// @bdev: Pointer to a ttm device
// @ttm: Pointer to a struct ttm_tt.
//
// Destroy the backend. This will be call back from ttm_tt_destroy so
// don't call ttm_tt_destroy from the callback or infinite loop.
//
    pub ttm): *mut *mut *mut void (ttm_tt_destroy)(struct ttm_device bdev, struct ttm_tt,
//
// struct ttm_bo_driver member eviction_valuable
//
// @bo: the buffer object to be evicted
// @place: placement we need room for
//
// Check with the driver if it is valuable to evict a BO to make room
// for a certain placement.
//
    pub place): *const ttm_place,
//
// struct ttm_bo_driver member evict_flags:
//
// @bo: the buffer object to be evicted
//
// Return the bo flags for a buffer which is not mapped to the hardware.
// These will be placed in proposed_flags so that when the move is
// finished, they'll end up in bo->mem.flags
// This should not cause multihop evictions, and the core will warn
// if one is proposed.
//
    pub placement): *mut ttm_placement,
//
// struct ttm_bo_driver member move:
//
// @bo: the buffer to move
// @evict: whether this motion is evicting the buffer from
// the graphics address space
// @ctx: context for this move with parameters
// @new_mem: the new memory region receiving the buffer
// @hop: placement for driver directed intermediate hop
//
// Move a buffer between two memory regions.
// Returns errno -EMULTIHOP if driver requests a hop
//
    pub hop): *mut ttm_place,
//
// Hook to notify driver about a resource delete.
//
    pub bo): *mut *mut void (delete_mem_notify)(struct ttm_buffer_object,
//
// notify the driver that we're about to swap out this bo
//
    pub bo): *mut *mut void (swap_notify)(struct ttm_buffer_object,
//
// Driver callback on when mapping io memory (for bo_move_memcpy
// for instance). TTM will take care to call io_mem_free whenever
// the mapping is not use anymore. io_mem_reserve & io_mem_free
// are balanced.
//
    pub mem): *mut ttm_resource,
    pub mem): *mut ttm_resource,
//
// Return the pfn for a given page_offset inside the BO.
//
// @bo: the BO to look up the pfn for
// @page_offset: the offset to look up
//
    pub page_offset): c_ulong,
//
// Read/write memory buffers for ptrace access
//
// @bo: the BO to access
// @offset: the offset from the start of the BO
// @buf: pointer to source/destination buffer
// @len: number of bytes to copy
// @write: whether to read (0) from or write (non-0) to BO
//
// If successful, this function should return the number of
// bytes copied, -EIO otherwise. If the number of bytes
// returned is < len, the function may be called again with
// the remainder of the buffer to copy.
//
    pub write): *mut *mut void buf, int len, int,
//
// Notify the driver that we're about to release a BO
//
// @bo: BO that is about to be released
//
// Gives the driver a chance to do any cleanup, including
// adding fences that may force a delayed delete
//
    pub bo): *mut *mut void (release_notify)(struct ttm_buffer_object,
}

//
// struct ttm_device - Buffer object driver device-specific data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_device {
//
// @device_list: Our entry in the global device list.
// Constant after bo device init
//
    pub device_list: list_head,
//
// @alloc_flags: TTM_ALLOCATION_* flags.
//
    pub alloc_flags: c_uint,
//
// @funcs: Function table for the device.
// Constant after bo device init
//
    pub funcs: *const ttm_device_funcs,
//
// @sysman: Resource manager for the system domain.
// Access via ttm_manager_type.
//
    pub sysman: ttm_resource_manager,
//
// @man_drv: An array of resource_managers, one per resource type.
//
    pub man_drv: [*mut ttm_resource_manager; TTM_NUM_MEM_TYPES],
//
// @vma_manager: Address space manager for finding BOs to mmap.
//
    pub vma_manager: *mut drm_vma_offset_manager,
//
// @pool: page pool for the device.
//
    pub pool: ttm_pool,
//
// @lru_lock: Protection for the per manager LRU and ddestroy lists.
//
    pub lru_lock: spinlock_t,
//
// @unevictable: Buffer objects which are pinned or swapped and as such
// not on an LRU list.
//
    pub unevictable: list_head,
//
// @dev_mapping: A pointer to the struct address_space for invalidating
// CPU mappings on buffer move. Protected by load/unload sync.
//
    pub dev_mapping: *mut address_space,
//
// @wq: Work queue structure for the delayed delete workqueue.
//
    pub wq: *mut workqueue_struct,
}

extern "C" {
    pub fn ttm_global_swapout(ctx: *mut ttm_operation_ctx, gfp_flags: gfp_t) -> c_int;
}
extern "C" {
    pub fn ttm_device_prepare_hibernation(bdev: *mut ttm_device) -> c_int;
}
extern "C" {
    pub fn ttm_device_fini(bdev: *mut ttm_device);
}
extern "C" {
    pub fn ttm_device_clear_dma_mappings(bdev: *mut ttm_device);
}
