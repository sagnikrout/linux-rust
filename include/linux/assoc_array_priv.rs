//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/assoc_array_priv.h
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
// Private definitions for the generic associative array implementation.
//
// See Documentation/core-api/assoc_array.rst for information.
//
// Copyright (C) 2013 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Undefined type representing a pointer with type information in the bottom
// two bits.
//
// An N-way node in the tree.
//
// Each slot contains one of four things:
//
// (1) Nothing (NULL).
//
// (2) A leaf object (pointer types 0).
//
// (3) A next-level node (pointer type 1, subtype 0).
//
// (4) A shortcut (pointer type 1, subtype 1).
//
// The tree is optimised for search-by-ID, but permits reasonable iteration
// also.
//
// The tree is navigated by constructing an index key consisting of an array of
// segments, where each segment is ilog2(ASSOC_ARRAY_FAN_OUT) bits in size.
//
// The segments correspond to levels of the tree (the first segment is used at
// level 0, the second at level 1, etc.).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct assoc_array_node {
    pub back_pointer: *mut assoc_array_ptr,
    pub parent_slot: u8,
    pub slots: [*mut assoc_array_ptr; ASSOC_ARRAY_FAN_OUT],
    pub nr_leaves_on_branch: c_ulong,
}

//
// A shortcut through the index space out to where a collection of nodes/leaves
// with the same IDs live.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct assoc_array_shortcut {
    pub back_pointer: *mut assoc_array_ptr,
    pub parent_slot: c_int,
    pub skip_to_level: c_int,
    pub next_node: *mut assoc_array_ptr,
    pub index_key: [c_ulong; ],
}

//
// Preallocation cache.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct assoc_array_edit {
    pub rcu: rcu_head,
    pub array: *mut assoc_array,
    pub ops: *const assoc_array_ops,
    pub ops_for_excised_subtree: *const assoc_array_ops,
    pub leaf: *mut assoc_array_ptr,
    pub leaf_p: *mut assoc_array_ptr,
    pub dead_leaf: *mut assoc_array_ptr,
    pub new_meta: [*mut assoc_array_ptr; 3],
    pub excised_meta: [*mut assoc_array_ptr; 1],
    pub excised_subtree: *mut assoc_array_ptr,
    pub set_backpointers: [*mut assoc_array_ptr; ASSOC_ARRAY_FAN_OUT],
    pub set_backpointers_to: *mut assoc_array_ptr,
    pub adjust_count_on: *mut assoc_array_node,
    pub adjust_count_by: c_long,
    pub ptr: *mut assoc_array_ptr,
    pub to: *mut assoc_array_ptr,
    pub set: [}; 2],
    pub p: *mut u8,
    pub to: u8,
    pub set_parent_slot: [}; 1],
    pub 1]: u8 segment_cache[ASSOC_ARRAY_FAN_OUT +,
}

//
// Internal tree member pointers are marked in the bottom one or two bits to
// indicate what type they are so that we don't have to look behind every
// pointer to see what it points to.
//
// We provide functions to test type annotations and to create and translate
// the annotated pointers.
//
pub const ASSOC_ARRAY_PTR_TYPE_MASK: c_uint = 0x1UL;
pub const ASSOC_ARRAY_PTR_LEAF_TYPE: c_uint = 0x0UL	/* Points to leaf (or nowhere) */;
pub const ASSOC_ARRAY_PTR_META_TYPE: c_uint = 0x1UL	/* Points to node or shortcut */;
pub const ASSOC_ARRAY_PTR_SUBTYPE_MASK: c_uint = 0x2UL;
pub const ASSOC_ARRAY_PTR_NODE_SUBTYPE: c_uint = 0x0UL;
pub const ASSOC_ARRAY_PTR_SHORTCUT_SUBTYPE: c_uint = 0x2UL;
extern "C" {
    pub fn __assoc_array_x_to_ptr(_arg: p, _arg: ASSOC_ARRAY_PTR_LEAF_TYPE) -> return;
}

