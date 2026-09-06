//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nilfs2/btree.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// NILFS B-tree.
//
// Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
//
// Written by Koji Sato.
//

//
// struct nilfs_btree_path - A path on which B-tree operations are executed
// @bp_bh: buffer head of node block
// @bp_sib_bh: buffer head of sibling node block
// @bp_index: index of child node
// @bp_oldreq: ptr end request for old ptr
// @bp_newreq: ptr alloc request for new ptr
// @bp_ctxt: context information for changing the key of a b-tree node block
// @bp_op: rebalance operation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_btree_path {
    pub bp_bh: *mut buffer_head,
    pub bp_sib_bh: *mut buffer_head,
    pub bp_index: c_int,
    pub bp_oldreq: nilfs_bmap_ptr_req,
    pub bp_newreq: nilfs_bmap_ptr_req,
    pub bp_ctxt: nilfs_btnode_chkey_ctxt,
    pub ): *mut *mut int, __u64 , __u64,
}

pub const NILFS_BTREE_ROOT_NCHILDREN_MIN: c_int = 0;

extern "C" {
    pub fn nilfs_btree_init(: *mut nilfs_bmap) -> c_int;
}
extern "C" {
    pub fn nilfs_btree_init_gc(: *mut nilfs_bmap);
}
extern "C" {
    pub fn nilfs_btree_broken_node_block(bh: *mut buffer_head) -> c_int;
}
