//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/misc.h
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
// Convenience macros to define a pointer with the __free(kfree) and
// __free(kvfree) cleanup attributes and initialized to NULL.
//

//
// Enumerate bits using enum autoincrement. Define the @name as the n-th bit.
//

extern "C" {
    pub fn bvec_phys(_arg: &bv) -> return;
}
//
// Iterate bio using btrfs block size.
//
// This will handle large folio and highmem.
//
// @paddr:	Physical memory address of each iteration
// @bio:	The bio to iterate
// @iter:	The bvec_iter (pointer) to use.
// @blocksize:	The blocksize to iterate.
//
// This requires all folios in the bio to cover at least one block.
//

// Can only be called on a non-cloned bio.
// Initialize a bvec_iter to the size of the specified bio.

//
// This implies a full smp_mb barrier, see comments for
// waitqueue_active why.
//
// Special case for conditional wakeup where the barrier required for
// waitqueue_active is implied by some of the preceding code. Eg. one
// of such atomic operations (atomic_dec_and_return, ...), or a
// unlock/lock sequence, etc.
//
extern "C" {
    pub fn div_u64(percent: *mut *mut num, _arg: 100) -> return;
}
// Copy of is_power_of_two that is 64bit safe
extern "C" {
    pub fn is_power_of_two_u64(_arg: n) -> return;
}
//
// Simple bytenr based rb_tree relate structures
//
// Any structure wants to use bytenr as single search index should have their
// structure start with these members.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_simple_node {
    pub rb_node: rb_node,
    pub bytenr: u64,
}

//
// Search @root from an entry that starts or comes after @bytenr.
//
// @root:	the root to search.
// @bytenr:	bytenr to search from.
//
// Return the rb_node that start at or after @bytenr.  If there is no entry at
// or after @bytner return NULL.
//
extern "C" {
    pub fn rb_find_add(_arg: &simple_node->rb_node, _arg: root, _arg: rb_simple_node_bytenr_cmp) -> return;
}
