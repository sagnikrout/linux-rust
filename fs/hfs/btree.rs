//! Automatically rewritten from C Header to Rust Module
//! Source: fs/hfs/btree.h
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
// linux/fs/hfs/btree.h
//
// Copyright (C) 2001
// Brad Boyer (flar@allandria.com)
// (C) 2003 Ardis Technologies <roman@ardistech.com>
//

extern "C" {
    pub fn int(: *const *const btree_keycmp)(btree_key, : *const btree_key) -> typedef;
}
pub const NODE_HASH_SIZE: c_int = 256;
// B-tree mutex nested subclasses
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfs_btree_mutex_classes {
    CATALOG_BTREE_MUTEX,
    EXTENTS_BTREE_MUTEX,
    ATTR_BTREE_MUTEX,
}

// A HFS BTree held in memory
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_btree {
    pub sb: *mut super_block,
    pub inode: *mut inode,
    pub keycmp: btree_keycmp,
    pub cnid: u32,
    pub root: u32,
    pub leaf_count: u32,
    pub leaf_head: u32,
    pub leaf_tail: u32,
    pub node_count: u32,
    pub free_nodes: u32,
    pub attributes: u32,
    pub node_size: c_uint,
    pub node_size_shift: c_uint,
    pub max_key_len: c_uint,
    pub depth: c_uint,
// unsigned int map1_size, map_size;
    pub tree_lock: mutex,
    pub pages_per_bnode: c_uint,
    pub hash_lock: spinlock_t,
    pub node_hash: [*mut hfs_bnode; NODE_HASH_SIZE],
    pub node_hash_cnt: c_int,
}

// A HFS BTree node in memory
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_bnode {
    pub tree: *mut hfs_btree,
    pub prev: u32,
    pub this: u32,
    pub next: u32,
    pub parent: u32,
    pub num_recs: u16,
    pub type: u8,
    pub height: u8,
    pub next_hash: *mut hfs_bnode,
    pub flags: c_ulong,
    pub lock_wq: wait_queue_head_t,
    pub refcnt: core::sync::atomic::AtomicI32,
    pub page_offset: c_uint,
    pub page: [*mut page; ],
}

pub const HFS_BNODE_ERROR: c_int = 0;
pub const HFS_BNODE_NEW: c_int = 1;
pub const HFS_BNODE_DELETED: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_find_data {
    pub key: *mut btree_key,
    pub search_key: *mut btree_key,
    pub tree: *mut hfs_btree,
    pub bnode: *mut hfs_bnode,
    pub record: c_int,
    pub keylength: int keyoffset,,
    pub entrylength: int entryoffset,,
}

// btree.c
extern "C" {
    pub fn hfs_btree_close(tree: *mut hfs_btree);
}
extern "C" {
    pub fn hfs_btree_write(tree: *mut hfs_btree);
}
extern "C" {
    pub fn hfs_bmap_reserve(tree: *mut hfs_btree, rsvd_nodes: u32) -> c_int;
}
extern "C" {
    pub fn hfs_bmap_free(node: *mut hfs_bnode);
}
// bnode.c
extern "C" {
    pub fn hfs_bnode_read(node: *mut hfs_bnode, buf: *mut c_void, off: u32, len: u32);
}
extern "C" {
    pub fn hfs_bnode_read_u16(node: *mut hfs_bnode, off: u32) -> u16;
}
extern "C" {
    pub fn hfs_bnode_read_u8(node: *mut hfs_bnode, off: u32) -> u8;
}
extern "C" {
    pub fn hfs_bnode_read_key(node: *mut hfs_bnode, key: *mut c_void, off: u32);
}
extern "C" {
    pub fn hfs_bnode_write(node: *mut hfs_bnode, buf: *mut c_void, off: u32, len: u32);
}
extern "C" {
    pub fn hfs_bnode_write_u16(node: *mut hfs_bnode, off: u32, data: u16);
}
extern "C" {
    pub fn hfs_bnode_write_u8(node: *mut hfs_bnode, off: u32, data: u8);
}
extern "C" {
    pub fn hfs_bnode_clear(node: *mut hfs_bnode, off: u32, len: u32);
}
extern "C" {
    pub fn hfs_bnode_move(node: *mut hfs_bnode, dst: u32, src: u32, len: u32);
}
extern "C" {
    pub fn hfs_bnode_dump(node: *mut hfs_bnode);
}
extern "C" {
    pub fn hfs_bnode_unlink(node: *mut hfs_bnode);
}
extern "C" {
    pub fn hfs_bnode_unhash(node: *mut hfs_bnode);
}
extern "C" {
    pub fn hfs_bnode_free(node: *mut hfs_bnode);
}
extern "C" {
    pub fn hfs_bnode_get(node: *mut hfs_bnode);
}
extern "C" {
    pub fn hfs_bnode_put(node: *mut hfs_bnode);
}
// brec.c
extern "C" {
    pub fn hfs_brec_lenoff(node: *mut hfs_bnode, rec: u16, off: *mut u16) -> u16;
}
extern "C" {
    pub fn hfs_brec_keylen(node: *mut hfs_bnode, rec: u16) -> u16;
}
extern "C" {
    pub fn hfs_brec_insert(fd: *mut hfs_find_data, entry: *mut c_void, entry_len: u32) -> c_int;
}
extern "C" {
    pub fn hfs_brec_remove(fd: *mut hfs_find_data) -> c_int;
}
// bfind.c
extern "C" {
    pub fn hfs_find_init(tree: *mut hfs_btree, fd: *mut hfs_find_data) -> c_int;
}
extern "C" {
    pub fn hfs_find_exit(fd: *mut hfs_find_data);
}
extern "C" {
    pub fn __hfs_brec_find(bnode: *mut hfs_bnode, fd: *mut hfs_find_data) -> c_int;
}
extern "C" {
    pub fn hfs_brec_find(fd: *mut hfs_find_data) -> c_int;
}
extern "C" {
    pub fn hfs_brec_read(fd: *mut hfs_find_data, rec: *mut c_void, rec_len: u32) -> c_int;
}
extern "C" {
    pub fn hfs_brec_goto(fd: *mut hfs_find_data, cnt: c_int) -> c_int;
}
