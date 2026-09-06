//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/generic-radix-tree.h
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
// DOC: Generic radix trees/sparse arrays
//
// Very simple and minimalistic, supporting arbitrary size entries up to
// GENRADIX_NODE_SIZE.
//
// A genradix is defined with the type it will store, like so:
//
// static GENRADIX(struct foo) foo_genradix;
//
// The main operations are:
//
// - genradix_init(radix) - initialize an empty genradix
//
// - genradix_free(radix) - free all memory owned by the genradix and
// reinitialize it
//
// - genradix_ptr(radix, idx) - gets a pointer to the entry at idx, returning
// NULL if that entry does not exist
//
// - genradix_ptr_alloc(radix, idx, gfp) - gets a pointer to an entry,
// allocating it if necessary
//
// - genradix_for_each(radix, iter, p) - iterate over each entry in a genradix
//
// The radix tree allocates one page of entries at a time, so entries may exist
// that were never explicitly allocated - they will be initialized to all
// zeroes.
//
// Internally, a genradix is just a radix tree of pages, and indexing works in
// terms of byte offsets. The wrappers in this header file use sizeof on the
// type the radix contains to calculate a byte offset from the index - see
// __idx_to_offset.
//

pub const GENRADIX_NODE_SHIFT: c_int = 9;

// depth that's needed for a genradix that can address up to ULONG_MAX:

//
// Returns size (of data, in bytes) that a tree of a given depth holds:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __genradix {
    pub root: *mut genradix_root,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct genradix_node {
// Interior node:
    pub children: [*mut genradix_node; GENRADIX_ARY],
// Leaf:
    pub data: [u8; GENRADIX_NODE_SIZE],
}

extern "C" {
    pub fn kzalloc(_arg: GENRADIX_NODE_SIZE, _arg: gfp_mask) -> return;
}
//
// NOTE: currently, sizeof(_type) must not be larger than GENRADIX_NODE_SIZE:
//

//
// We use a 0 size array to stash the type we're storing without taking any
// space at runtime - then the various accessor macros can use typeof() to get
// to it for casts/sizeof - we also force the alignment so that storing a type
// with a ridiculous alignment doesn't blow up the alignment or size of the
// genradix.
//

//
// genradix_init - initialize a genradix
// @_radix:	genradix to initialize
//
// Does not fail
//

// (_radix) = (typeof(*_radix)) __GENRADIX_INITIALIZER;	\
extern "C" {
    pub fn __genradix_free(: *mut __genradix);
}
//
// genradix_free: free all memory owned by a genradix
// @_radix: the genradix to free
//
// After freeing, @_radix will be reinitialized and empty
//

//
// genradix_ptr - get a pointer to a genradix entry
// @_radix:	genradix to access
// @_idx:	index to fetch
//
// Returns a pointer to entry at @_idx, or NULL if that entry does not exist.
//

//
// genradix_ptr_alloc - get a pointer to a genradix entry, allocating it
// if necessary
// @_radix:	genradix to access
// @_idx:	index to fetch
// @_gfp:	gfp mask
//
// Returns a pointer to entry at @_idx, or NULL on allocation failure
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct genradix_iter {
    pub offset: usize,
    pub pos: usize,
}

//
// genradix_iter_init - initialize a genradix_iter
// @_radix:	genradix that will be iterated over
// @_idx:	index to start iterating from
//

//
// genradix_iter_peek - get first entry at or above iterator's current
// position
// @_iter:	a genradix_iter
// @_radix:	genradix being iterated over
//
// If no more entries exist at or above @_iter's current position, returns NULL
//

//
// genradix_iter_peek_prev - get first entry at or below iterator's current
// position
// @_iter:	a genradix_iter
// @_radix:	genradix being iterated over
//
// If no more entries exist at or below @_iter's current position, returns NULL
//

//
// genradix_for_each - iterate over entry in a genradix
// @_radix:	genradix to iterate over
// @_iter:	a genradix_iter to track current position
// @_p:		pointer to genradix entry type
//
// On every iteration, @_p will point to the current entry, and @_iter.pos
// will be the current entry's index.
//

//
// genradix_for_each_reverse - iterate over entry in a genradix, reverse order
// @_radix:	genradix to iterate over
// @_iter:	a genradix_iter to track current position
// @_p:		pointer to genradix entry type
//
// On every iteration, @_p will point to the current entry, and @_iter.pos
// will be the current entry's index.
//

extern "C" {
    pub fn __genradix_prealloc(: *mut __genradix, _arg: usize, _arg: gfp_t) -> c_int;
}
//
// genradix_prealloc - preallocate entries in a generic radix tree
// @_radix:	genradix to preallocate
// @_nr:	number of entries to preallocate
// @_gfp:	gfp mask
//
// Returns 0 on success, -ENOMEM on failure
//

