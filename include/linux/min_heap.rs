//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/min_heap.h
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
// The Min Heap API provides utilities for managing min-heaps, a binary tree
// structure where each node's value is less than or equal to its children's
// values, ensuring the smallest element is at the root.
//
// Users should avoid directly calling functions prefixed with __min_heap_*().
// Instead, use the provided macro wrappers.
//
// For further details and examples, refer to Documentation/core-api/min_heap.rst.
//
// Data structure to hold a min-heap.
// @nr: Number of elements currently in the heap.
// @size: Maximum number of elements that can be held in current storage.
// @data: Pointer to the start of array holding the heap elements.
// @preallocated: Start of the static preallocated array holding the heap elements.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _name {
    pub \: size_t nr;,
    pub \: size_t size;,
    pub \: *mut *mut _type data;,
    pub \: _type preallocated[_nr];,

    pub min_heap_char: typedef DEFINE_MIN_HEAP(char, min_heap_char),

//
// struct min_heap_callbacks - Data/functions to customise the min_heap.
// @less: Partial order function for this heap.
// @swp: Swap elements function.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct min_heap_callbacks {
    pub args): *const *const *const *const bool (less)(void lhs, void rhs, void,
    pub args): *mut *mut *mut *mut void (swp)(void lhs, void rhs, void,
}

//
// is_aligned - is this pointer & size okay for word-wide copying?
// @base: pointer to data
// @size: size of each element
// @align: required alignment (typically 4 or 8)
//
// Returns true if elements can be copied using word loads and stores.
// The size must be a multiple of the alignment, and the base address must
// be if we do not have CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS.
//
// For some reason, gcc doesn't know to optimize "if (a & mask || b & mask)"
// to "if ((a | b) & mask)", so we do that by hand.
//

//
// swap_words_32 - swap two elements in 32-bit chunks
// @a: pointer to the first element to swap
// @b: pointer to the second element to swap
// @n: element size (must be a multiple of 4)
//
// Exchange the two objects in memory.  This exploits base+index addressing,
// which basically all CPUs have, to minimize loop overhead computations.
//
// For some reason, on x86 gcc 7.3.0 adds a redundant test of n at the
// bottom of the loop, even though the zero flag is still valid from the
// subtract (since the intervening mov instructions don't alter the flags).
// Gcc 8.1.0 doesn't have that problem.
//
// (u32 *)(a + n) = *(u32 *)(b + n);
// (u32 *)(b + n) = t;
//
// swap_words_64 - swap two elements in 64-bit chunks
// @a: pointer to the first element to swap
// @b: pointer to the second element to swap
// @n: element size (must be a multiple of 8)
//
// Exchange the two objects in memory.  This exploits base+index
// addressing, which basically all CPUs have, to minimize loop overhead
// computations.
//
// We'd like to use 64-bit loads if possible.  If they're not, emulating
// one requires base+index+4 addressing which x86 has but most other
// processors do not.  If CONFIG_64BIT, we definitely have 64-bit loads,
// but it's possible to have 64-bit loads without 64-bit pointers (e.g.
// x32 ABI).  Are there any cases the kernel needs to worry about?
//

// (u64 *)(a + n) = *(u64 *)(b + n);
// (u64 *)(b + n) = t;

// Use two 32-bit transfers to avoid base+index+4 addressing
// (u32 *)(a + n) = *(u32 *)(b + n);
// (u32 *)(b + n) = t;
// (u32 *)(a + n) = *(u32 *)(b + n);
// (u32 *)(b + n) = t;

//
// swap_bytes - swap two elements a byte at a time
// @a: pointer to the first element to swap
// @b: pointer to the second element to swap
// @n: element size
//
// This is the fallback if alignment doesn't allow using larger chunks.
//
// The values are arbitrary as long as they can't be confused with
// a pointer, but small integers make for the smallest compare
// instructions.
//

//
// Selects the appropriate swap function based on the element size.
//
// parent - given the offset of the child, find the offset of the parent.
// @i: the offset of the heap element whose parent is sought.  Non-zero.
// @lsbit: a precomputed 1-bit mask, equal to "size & -size"
// @size: size of each element
//
// In terms of array indexes, the parent of element j = @i/@size is simply
// (j-1)/2.  But when working in byte offsets, we can't use implicit
// truncation of integer divides.
//
// Fortunately, we only need one bit of the quotient, not the full divide.
// @size has a least significant bit.  That bit will be clear if @i is
// an even multiple of @size, and set if it's an odd multiple.
//
// Logically, we're doing "if (i & lsbit) i -= size;", but since the
// branch is unpredictable, it's done with a bit of clever branch-free
// code instead.
//
// Initialize a min-heap.

// Get the minimum element from the heap.

// Check if the heap is full.

// Sift the element at pos down the heap.
// pre-scale counters for performance
// Find the sift-down path all the way to the leaves.
// Special case for the last leaf with no sibling.
// Backtrack to the correct location.
// Shift the element into its correct place.

// Sift up ith element from the heap, O(log2(nr)).
// pre-scale counters for performance

// Floyd's approach to heapification that is O(nr).

// Remove minimum element from the heap, O(log2(nr)).
// Place last element at the root (position 0) and then sift down.

//
// Remove the minimum element and then push the given element. The
// implementation performs 1 sift (O(log2(nr))) and is therefore more
// efficient than a pop followed by a push that does 2.
//

// Push an element on to the heap, O(log2(nr)).
// Place at the end of data.
// Sift child at pos up.

// Remove ith element from the heap, O(log2(nr)).
// Place last element at the root (position 0) and then sift down.

extern "C" {
    pub fn __min_heap_init(heap: *mut min_heap_char, data: *mut c_void, size: usize);
}
extern "C" {
    pub fn __min_heap_full(heap: *mut min_heap_char) -> bool;
}

