//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/persistent-data/dm-btree-internal.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2011 Red Hat, Inc.
//
// This file is released under the GPL.
//

// ----------------------------------------------------------------
//
// We'll need 2 accessor functions for n->csum and n->blocknr
// to support dm-btree-spine.c in that case.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum node_flags {
    INTERNAL_NODE = 1,
    LEAF_NODE = 1 << 1
}

//
// Every btree node begins with this structure.  Make sure it's a multiple
// of 8-bytes in size, otherwise the 64bit keys will be mis-aligned.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_header {
    pub csum: __le32,
    pub flags: __le32,
    pub /: *mut *mut __le64 blocknr; / Block this node is supposed to live in.,
    pub nr_entries: __le32,
    pub max_entries: __le32,
    pub value_size: __le32,
    pub padding: __le32,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btree_node {
    pub header: node_header,
    pub keys: [__le64; ],
    pub __aligned(8): } __packed,
//
// Locks a block using the btree node validator.
//
    pub result): *mut dm_block,
    pub vt): *mut dm_btree_value_type,
    pub result): *mut *mut int new_block(struct dm_btree_info info, struct dm_block,
    pub b): *mut *mut void unlock_block(struct dm_btree_info info, struct dm_block,
//
// Spines keep track of the rolling locks.  There are 2 variants, read-only
// and one that uses shadowing.  These are separate structs to allow the
// type checker to spot misuse, for example accidentally calling read_lock
// on a shadow spine.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ro_spine {
    pub info: *mut dm_btree_info,
    pub count: c_int,
    pub nodes: [*mut dm_block; 2],
}

extern "C" {
    pub fn init_ro_spine(s: *mut ro_spine, info: *mut dm_btree_info);
}
extern "C" {
    pub fn exit_ro_spine(s: *mut ro_spine);
}
extern "C" {
    pub fn ro_step(s: *mut ro_spine, new_child: dm_block_t) -> c_int;
}
extern "C" {
    pub fn ro_pop(s: *mut ro_spine);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shadow_spine {
    pub info: *mut dm_btree_info,
    pub count: c_int,
    pub nodes: [*mut dm_block; 2],
    pub root: dm_block_t,
}

extern "C" {
    pub fn init_shadow_spine(s: *mut shadow_spine, info: *mut dm_btree_info);
}
extern "C" {
    pub fn exit_shadow_spine(s: *mut shadow_spine);
}
//
// The spine must have at least one entry before calling this.
//
// The spine must have at least two entries before calling this.
//
extern "C" {
    pub fn shadow_has_parent(s: *mut shadow_spine) -> c_int;
}
extern "C" {
    pub fn shadow_root(s: *mut shadow_spine) -> dm_block_t;
}
//
// Some inlines.
//
extern "C" {
    pub fn value_base(index: *mut *mut n) + (value_size) -> return;
}
//
// Assumes the values are suitably-aligned and converts to core format.
//
extern "C" {
    pub fn le64_to_cpu(_arg: values_le[index]) -> return;
}
//
// Searching for a key within a single node.
//
extern "C" {
    pub fn lower_bound(n: *mut btree_node, key: u64) -> c_int;
}
//
// Value type for upper levels of multi-level btrees.
//
// This returns a shadowed btree leaf that you may modify.  In practise
// this means overwrites only, since an insert could cause a node to
// be split.  Useful if you need access to the old value to calculate the
// new one.
//
// This only works with single level btrees.  The given key must be present in
// the tree, otherwise -EINVAL will be returned.
//
