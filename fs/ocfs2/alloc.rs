//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/alloc.h
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
//
// alloc.h
//
// Function prototypes
//
// Copyright (C) 2002, 2004 Oracle.  All rights reserved.
//
// For xattr tree leaf, we limit the leaf byte size to be 64K.
//
pub const OCFS2_MAX_XATTR_TREE_LEAF_SIZE: c_int = 65536;
//
// ocfs2_extent_tree and ocfs2_extent_tree_operations are used to abstract
// the b-tree operations in ocfs2. Now all the b-tree operations are not
// limited to ocfs2_dinode only. Any data which need to allocate clusters
// to store can use b-tree. And it only needs to implement its ocfs2_extent_tree
// and operation.
//
// ocfs2_extent_tree becomes the first-class object for extent tree
// manipulation.  Callers of the alloc.c code need to fill it via one of
// the ocfs2_init_*_extent_tree() operations below.
//
// ocfs2_extent_tree contains info for the root of the b-tree, it must have a
// root ocfs2_extent_list and a root_bh so that they can be used in the b-tree
// functions.  It needs the ocfs2_caching_info structure associated with
// I/O on the tree.  With metadata ecc, we now call different journal_access
// functions for each type of metadata, so it must have the
// root_journal_access function.
// ocfs2_extent_tree_operations abstract the normal operations we do for
// the root of extent b-tree.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_extent_tree {
    pub et_ops: *const ocfs2_extent_tree_operations,
    pub et_root_bh: *mut buffer_head,
    pub et_root_el: *mut ocfs2_extent_list,
    pub et_ci: *mut ocfs2_caching_info,
    pub et_root_journal_access: ocfs2_journal_access_func,
    pub et_object: *mut c_void,
    pub et_max_leaf_clusters: c_uint,
    pub et_dealloc: *mut ocfs2_cached_dealloc_ctxt,
}

//
// ocfs2_init_*_extent_tree() will fill an ocfs2_extent_tree from the
// specified object buffer.
//
// Read an extent block into *bh.  If *bh is NULL, a bh will be
// allocated.  This is a cached read.  The extent block will be validated
// with ocfs2_validate_extent_block().
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocfs2_alloc_restarted {
    RESTART_NONE = 0,
    RESTART_TRANS,
    RESTART_META
}

extern "C" {
    pub fn ocfs2_num_free_extents(et: *mut ocfs2_extent_tree) -> c_int;
}
//
// how many new metadata chunks would an allocation need at maximum?
//
// Please note that the caller must make sure that root_el is the root
// of extent tree. So for an inode, it should be &fe->id2.i_list. Otherwise
// the result may be wrong.
//
// Rather than do all the work of determining how much we need
// (involves a ton of reads and locks), just ask for the
// maximal limit.  That's a tree depth shift.  So, one block for
// level of the tree (current l_tree_depth), one block for the
// new tree_depth==0 extent_block, and one block at the new
// top-of-the tree.
//
extern "C" {
    pub fn ocfs2_dinode_new_extent_list(inode: *mut inode, di: *mut ocfs2_dinode);
}
extern "C" {
    pub fn ocfs2_set_inode_data_inline(inode: *mut inode, di: *mut ocfs2_dinode);
}
extern "C" {
    pub fn ocfs2_truncate_log_init(osb: *mut ocfs2_super) -> c_int;
}
extern "C" {
    pub fn ocfs2_truncate_log_shutdown(osb: *mut ocfs2_super);
}
extern "C" {
    pub fn ocfs2_flush_truncate_log(osb: *mut ocfs2_super) -> c_int;
}
extern "C" {
    pub fn ocfs2_truncate_log_needs_flush(osb: *mut ocfs2_super) -> c_int;
}
extern "C" {
    pub fn __ocfs2_flush_truncate_log(osb: *mut ocfs2_super) -> c_int;
}
//
// Process local structure which describes the block unlinks done
// during an operation. This is populated via
// ocfs2_cache_block_dealloc().
//
// ocfs2_run_deallocs() should be called after the potentially
// de-allocating routines. No journal handles should be open, and most
// locks should have been dropped.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_cached_dealloc_ctxt {
    pub c_first_suballocator: *mut ocfs2_per_slot_free_list,
    pub c_global_allocator: *mut ocfs2_cached_block_free,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_truncate_context {
    pub tc_dealloc: ocfs2_cached_dealloc_ctxt,
    pub /: *mut *mut int tc_ext_alloc_locked; / is it cluster locked?,
// these get destroyed once it's passed to ocfs2_commit_truncate.
    pub tc_last_eb_bh: *mut buffer_head,
}

extern "C" {
    pub fn ocfs2_search_extent_list(el: *mut ocfs2_extent_list, v_cluster: u32) -> c_int;
}
extern "C" {
    pub fn ocfs2_trim_fs(sb: *mut super_block, range: *mut fstrim_range) -> c_int;
}
//
// Helper function to look at the # of clusters in an extent record.
//
// Cluster count in extent records is slightly different
// between interior nodes and leaf nodes. This is to support
// unwritten extents which need a flags field in leaf node
// records, thus shrinking the available space for a clusters
// field.
//
extern "C" {
    pub fn le32_to_cpu(_arg: rec->e_int_clusters) -> return;
}
extern "C" {
    pub fn le16_to_cpu(_arg: rec->e_leaf_clusters) -> return;
}
//
// This is only valid for leaf nodes, which are the only ones that can
// have empty extents anyway.
//
// Structures which describe a path through a btree, and functions to
// manipulate them.
//
// The idea here is to be as generic as possible with the tree
// manipulation code.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_path_item {
    pub bh: *mut buffer_head,
    pub el: *mut ocfs2_extent_list,
}

pub const OCFS2_MAX_PATH_DEPTH: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_path {
    pub p_tree_depth: c_int,
    pub p_root_access: ocfs2_journal_access_func,
    pub p_node: [ocfs2_path_item; OCFS2_MAX_PATH_DEPTH],
}

// Macro flag: #define path_root_access(_path)((_path)->p_root_access)

extern "C" {
    pub fn ocfs2_reinit_path(path: *mut ocfs2_path, keep_root: c_int);
}
extern "C" {
    pub fn ocfs2_free_path(path: *mut ocfs2_path);
}
