//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/gpu_buddy.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2021 Intel Corporation
//

//
// GPU_BUDDY_RANGE_ALLOCATION - Allocate within a specific address range
//
// When set, allocation is restricted to the range [start, end) specified
// in gpu_buddy_alloc_blocks(). Without this flag, start/end are ignored
// and allocation can use any free space.
//

//
// GPU_BUDDY_TOPDOWN_ALLOCATION - Allocate from top of address space
//
// Allocate starting from high addresses and working down. Useful for
// separating different allocation types (e.g., kernel vs userspace)
// to reduce fragmentation.
//

//
// GPU_BUDDY_CONTIGUOUS_ALLOCATION - Require physically contiguous blocks
//
// The allocation must be satisfied with a single contiguous block.
// If the requested size cannot be allocated contiguously, the
// allocation fails with -ENOSPC.
//

//
// GPU_BUDDY_CLEAR_ALLOCATION - Prefer pre-cleared (zeroed) memory
//
// Attempt to allocate from the clear tree first. If insufficient clear
// memory is available, falls back to dirty memory. Useful when the
// caller needs zeroed memory and wants to avoid GPU clear operations.
//

//
// GPU_BUDDY_CLEARED - Mark returned blocks as cleared
//
// Used with gpu_buddy_free_list() to indicate that the memory being
// freed has been cleared (zeroed). The blocks will be placed in the
// clear tree for future GPU_BUDDY_CLEAR_ALLOCATION requests.
//

//
// GPU_BUDDY_TRIM_DISABLE - Disable automatic block trimming
//
// By default, if an allocation is smaller than the allocated block,
// excess memory is trimmed and returned to the free pool. This flag
// disables trimming, keeping the full power-of-two block size.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpu_buddy_free_tree {
    GPU_BUDDY_CLEAR_TREE = 0,
    GPU_BUDDY_DIRTY_TREE,
    GPU_BUDDY_MAX_FREE_TREES,
}

//
// struct gpu_buddy_block - Block within a buddy allocator
//
// Each block in the buddy allocator is represented by this structure.
// Blocks are organized in a binary tree where each parent block can be
// split into two children (left and right buddies). The allocator manages
// blocks at various orders (power-of-2 sizes) from chunk_size up to the
// largest contiguous region.
//
// @private: Private data owned by the allocator user (e.g., driver-specific data)
// @link: List node for user ownership while block is allocated
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_buddy_block {
// private:
//
// Header bit layout:
// - Bits 63:12: block offset within the address space
// - Bits 11:10: state (ALLOCATED, FREE, or SPLIT)
// - Bit 9: clear bit (1 if memory is zeroed)
// - Bits 8:6: reserved
// - Bits 5:0: order (log2 of size relative to chunk_size)
//

// Free to be used, if needed in the future

    pub header: u64,
    pub left: *mut gpu_buddy_block,
    pub right: *mut gpu_buddy_block,
    pub parent: *mut gpu_buddy_block,
// public:
    pub /: *mut *mut *mut void private; / owned by creator,
//
// While the block is allocated by the user through gpu_buddy_alloc*,
// the user has ownership of the link, for example to maintain within
// a list, if so desired. As soon as the block is freed with
// gpu_buddy_free* ownership is given back to the mm.
//
// private:
    pub rb: rb_node,
// public:
    pub link: list_head,
}

// private:
// Order-zero must be at least SZ_4K

//
// struct gpu_buddy - GPU binary buddy allocator
//
// The buddy allocator provides efficient power-of-two memory allocation
// with fast allocation and free operations. It is commonly used for GPU
// memory management where allocations can be split into power-of-two
// block sizes.
//
// Locking should be handled by the user; a simple mutex around
// gpu_buddy_alloc_blocks() and gpu_buddy_free_block()/gpu_buddy_free_list()
// should suffice.
//
// @n_roots: Number of root blocks in the roots array.
// @max_order: Maximum block order (log2 of largest block size / chunk_size).
// @chunk_size: Minimum allocation granularity in bytes. Must be at least SZ_4K.
// @size: Total size of the address space managed by this allocator in bytes.
// @avail: Total free space currently available for allocation in bytes.
// @clear_avail: Free space available in the clear tree (zeroed memory) in bytes.
// This is a subset of @avail.
// @lock_dep_map: Annotates gpu_buddy API with a driver provided lock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_buddy {
// private:
//
// Array of red-black trees for free block management.
// Indexed as free_trees[clear/dirty][order] where:
// - Index 0 (GPU_BUDDY_CLEAR_TREE): blocks with zeroed content
// - Index 1 (GPU_BUDDY_DIRTY_TREE): blocks with unknown content
// Each tree holds free blocks of the corresponding order.
//
    pub free_trees: *mut rb_root,
//
// Array of root blocks representing the top-level blocks of the
// binary tree(s). Multiple roots exist when the total size is not
// a power of two, with each root being the largest power-of-two
// that fits in the remaining space.
//
    pub roots: *mut gpu_buddy_block,
//
// Per-order free block scoreboard: free_scoreboard[order] holds the
// number of blocks of that order currently in the free state.
// Incremented in mark_free(), decremented wherever rbtree_remove() is
// called on a free block.
//
    pub free_scoreboard: *mut u64,
//
// Per-order used block scoreboard: used_scoreboard[order] holds the
// number of blocks of that order currently in the allocated state.
// Incremented in mark_allocated(), decremented in mark_free() (guarded
// by gpu_buddy_block_is_allocated()) and in __gpu_buddy_free() when an
// allocated block is consumed directly during buddy coalescing.
//
    pub used_scoreboard: *mut u64,
// public:
    pub n_roots: c_uint,
    pub max_order: c_uint,
    pub chunk_size: u64,
    pub size: u64,
    pub avail: u64,
    pub clear_avail: u64,

    pub lock_dep_map: *mut lockdep_map,

}

//
// gpu_buddy_driver_set_lock() - Set the lock protecting accesses to GPU BUDDY
// @mm: Pointer to GPU buddy structure.
// @lock: the lock used to protect the gpu buddy. The locking primitive
// must contain a dep_map field.
//
// Call this to annotate gpu_buddy APIs which access/modify gpu_buddy manager
//

//
// gpu_buddy_driver_lock_held() - Assert GPU BUDDY manager lock is held
// @mm: Pointer to the GPU BUDDY structure.
//
// Ensure driver lock is held.
//

extern "C" {
    pub fn gpu_buddy_init(mm: *mut gpu_buddy, size: u64, chunk_size: u64) -> c_int;
}
extern "C" {
    pub fn gpu_buddy_fini(mm: *mut gpu_buddy);
}
extern "C" {
    pub fn gpu_buddy_reset_clear(mm: *mut gpu_buddy, is_clear: bool);
}
extern "C" {
    pub fn gpu_buddy_free_block(mm: *mut gpu_buddy, block: *mut gpu_buddy_block);
}
extern "C" {
    pub fn gpu_buddy_print(mm: *mut gpu_buddy);
}
