//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/libarena/include/libarena/buddy.h
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


// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum buddy_consts {
//
// Minimum allocation is 1 << BUDDY_MIN_ALLOC_SHIFT.
// Larger sizes increase internal fragmentation, but smaller
// sizes increase the space overhead of the block metadata.
//
    BUDDY_MIN_ALLOC_SHIFT	= 4,
    BUDDY_MIN_ALLOC_BYTES	= 1 << BUDDY_MIN_ALLOC_SHIFT,

//
// How many orders the buddy allocator can serve. Minimum block
// size is 1 << BUDDY_MIN_ALLOC_SHIFT, maximum block size is
// 1 << (BUDDY_MIN_ALLOC_SHIFT + BUDDY_CHUNK_NUM_ORDERS - 1):
// Each block has size 1 << BUDDY_MIN_ALLOC_SHIFT, and the
// allocation orders are in [0, BUDDY_CHUNK_NUM_ORDERS).
// We keep two blocks of the maximum size to retain the
// property in the code that all blocks have a buddy.
// Higher values increase the maximum allocation size,
// but also the size of the metadata for each block.
//
    BUDDY_CHUNK_NUM_ORDERS	= 1 << 4,
    BUDDY_CHUNK_BYTES	= BUDDY_MIN_ALLOC_BYTES << (BUDDY_CHUNK_NUM_ORDERS),

// Offset of the buddy header within a free block, see buddy.bpf.c for details
    BUDDY_HEADER_OFF	= 8,

// The maximum number of blocks a chunk may have to track.
    BUDDY_CHUNK_ITEMS	= 1 << (BUDDY_CHUNK_NUM_ORDERS),
    BUDDY_CHUNK_OFFSET_MASK	= BUDDY_CHUNK_BYTES - 1,

//
// Alignment for chunk allocations based on bpf_arena_alloc_pages.
// The arena allocation kfunc does not have an alignment argument,
// but that is required for all block calculations in the chunk to
// work.
//
    BUDDY_VADDR_OFFSET	= BUDDY_CHUNK_BYTES,

// Total arena virtual address space the allocator can consume.
    BUDDY_VADDR_SIZE	= BUDDY_CHUNK_BYTES << 10
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct buddy_header {
    pub /: *mut *mut u32 prev_index; / "Pointer" to the previous available allocation of the same size.,
    pub /: *mut *mut u32 next_index; / Same for the next allocation.,
}

//
// We bring memory into the allocator 1 MiB at a time.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct buddy_chunk {
// The order of the current allocation for a item. 4 bits per order.
    pub 2]: u8 orders[BUDDY_CHUNK_ITEMS /,
//
// Bit to denote whether chunk is allocated. Size of the allocated/free
// chunk found from the orders array.
//
    pub 8]: u8 allocated[BUDDY_CHUNK_ITEMS /,
// Freelists for O(1) allocation.
    pub freelists: [u64; BUDDY_CHUNK_NUM_ORDERS],
    pub next: *mut buddy_chunk __arena,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct buddy {
    pub /: *mut *mut *mut buddy_chunk __arena first_chunk; / Pointer to the chunk linked list.,
    pub /: *mut *mut arena_spinlock_t lock; / Allocator lock,
    pub /: *mut *mut u64 vaddr; / Allocation into reserved vaddr,
}

extern "C" {
    pub fn buddy_init(buddy: *mut buddy __arena) -> c_int;
}
extern "C" {
    pub fn buddy_destroy(buddy: *mut buddy __arena) -> c_int;
}
extern "C" {
    pub fn buddy_free(buddy: *mut buddy __arena, free: *mut void __arena) -> c_int;
}
