//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/backref.h
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
// Copyright (C) 2011 STRATO.  All rights reserved.
//

//
// Used by implementations of iterate_extent_inodes_t (see definition below) to
// signal that backref iteration can stop immediately and no error happened.
// The value must be non-negative and must not be 0, 1 (which is a common return
// value from things like btrfs_search_slot() and used internally in the backref
// walking code) and different from BACKREF_FOUND_SHARED and
// BACKREF_FOUND_NOT_SHARED
//
pub const BTRFS_ITERATE_EXTENT_INODES_STOP: c_int = 5;
//
// Should return 0 if no errors happened and iteration of backrefs should
// continue. Can return BTRFS_ITERATE_EXTENT_INODES_STOP or any other non-zero
// value to immediately stop iteration and possibly signal an error back to
// the caller.
//
// Context and arguments for backref walking functions. Some of the fields are
// to be filled by the caller of such functions while other are filled by the
// functions themselves, as described below.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_backref_walk_ctx {
//
// The address of the extent for which we are doing backref walking.
// Can be either a data extent or a metadata extent.
//
// Must always be set by the top level caller.
//
    pub bytenr: u64,
//
// Offset relative to the target extent. This is only used for data
// extents, and it's meaningful because we can have file extent items
// that point only to a section of a data extent ("bookend" extents),
// and we want to filter out any that don't point to a section of the
// data extent containing the given offset.
//
// Must always be set by the top level caller.
//
    pub extent_item_pos: u64,
//
// If true and bytenr corresponds to a data extent, then references from
// all file extent items that point to the data extent are considered,
// @extent_item_pos is ignored.
//
    pub ignore_extent_item_pos: bool,
//
// If true and bytenr corresponds to a data extent, then the inode list
// (each member describing inode number, file offset and root) is not
// added to each reference added to the @refs ulist.
//
    pub skip_inode_ref_list: bool,
// A valid transaction handle or NULL.
    pub trans: *mut btrfs_trans_handle,
//
// The file system's info object, can not be NULL.
//
// Must always be set by the top level caller.
//
    pub fs_info: *mut btrfs_fs_info,
//
// Time sequence acquired from btrfs_get_tree_mod_seq(), in case the
// caller joined the tree mod log to get a consistent view of b+trees
// while we do backref walking, or BTRFS_SEQ_LAST.
// When using BTRFS_SEQ_LAST, delayed refs are not checked and it uses
// commit roots when searching b+trees - this is a special case for
// qgroups used during a transaction commit.
//
    pub time_seq: u64,
//
// Used to collect the bytenr of metadata extents that point to the
// target extent.
//
    pub refs: *mut ulist,
//
// List used to collect the IDs of the roots from which the target
// extent is accessible. Can be NULL in case the caller does not care
// about collecting root IDs.
//
    pub roots: *mut ulist,
//
// Used by iterate_extent_inodes() and the main backref walk code
// (find_parent_nodes()). Lookup and store functions for an optional
// cache which maps the logical address (bytenr) of leaves to an array
// of root IDs.
//
    pub root_count_ret): *const *const *const u64 root_ids_ret, int,
    pub user_ctx): *mut c_void,
//
// If this is not NULL, then the backref walking code will call this
// for each indirect data extent reference as soon as it finds one,
// before collecting all the remaining backrefs and before resolving
// indirect backrefs. This allows for the caller to terminate backref
// walking as soon as it finds one backref that matches some specific
// criteria. The @cache_lookup and @cache_store callbacks should not
// be NULL in order to use this callback.
//
    pub indirect_ref_iterator: *mut iterate_extent_inodes_t,
//
// If this is not NULL, then the backref walking code will call this for
// each extent item it's meant to process before it actually starts
// processing it. If this returns anything other than 0, then it stops
// the backref walking code immediately.
//
    pub user_ctx): *const *const extent_buffer leaf, void,
//
// If this is not NULL, then the backref walking code will call this for
// each extent data ref it finds (BTRFS_EXTENT_DATA_REF_KEY keys) before
// processing that data ref. If this callback return false, then it will
// ignore this data ref and it will never resolve the indirect data ref,
// saving time searching for leaves in a fs tree with file extent items
// matching the data ref.
//
    pub user_ctx): *mut *mut bool (skip_data_ref)(u64 root, u64 ino, u64 offset, void,
// Context object to pass to the callbacks defined above.
    pub user_ctx: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode_fs_paths {
    pub btrfs_path: *mut btrfs_path,
    pub fs_root: *mut btrfs_root,
    pub fspath: *mut btrfs_data_container,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_backref_shared_cache_entry {
    pub bytenr: u64,
    pub gen: u64,
    pub is_shared: bool,
}

pub const BTRFS_BACKREF_CTX_PREV_EXTENTS_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_backref_share_check_ctx {
// Ulists used during backref walking.
    pub refs: ulist,
//
// The current leaf the caller of btrfs_is_data_extent_shared() is at.
// Typically the caller (at the moment only fiemap) tries to determine
// the sharedness of data extents point by file extent items from entire
// leaves.
//
    pub curr_leaf_bytenr: u64,
//
// The previous leaf the caller was at in the previous call to
// btrfs_is_data_extent_shared(). This may be the same as the current
// leaf. On the first call it must be 0.
//
    pub prev_leaf_bytenr: u64,
//
// A path from a root to a leaf that has a file extent item pointing to
// a given data extent should never exceed the maximum b+tree height.
//
    pub path_cache_entries: [btrfs_backref_shared_cache_entry; BTRFS_MAX_LEVEL],
    pub use_path_cache: bool,
//
// Cache the sharedness result for the last few extents we have found,
// but only for extents for which we have multiple file extent items
// that point to them.
// It's very common to have several file extent items that point to the
// same extent (bytenr) but with different offsets and lengths. This
// typically happens for COW writes, partial writes into prealloc
// extents, NOCOW writes after snapshotting a root, hole punching or
// reflinking within the same file (less common perhaps).
// So keep a small cache with the lookup results for the extent pointed
// by the last few file extent items. This cache is checked, with a
// linear scan, whenever btrfs_is_data_extent_shared() is called, so
// it must be small so that it does not negatively affect performance in
// case we don't have multiple file extent items that point to the same
// data extent.
//
    pub bytenr: u64,
    pub is_shared: bool,
    pub prev_extents_cache: [}; BTRFS_BACKREF_CTX_PREV_EXTENTS_SIZE],
//
// The slot in the prev_extents_cache array that will be used for
// storing the sharedness result of a new data extent.
//
    pub prev_extents_cache_slot: c_int,
}

extern "C" {
    pub fn btrfs_free_backref_share_ctx(ctx: *mut btrfs_backref_share_check_ctx);
}
extern "C" {
    pub fn paths_from_inode(inum: u64, ipath: *mut inode_fs_paths) -> c_int;
}
extern "C" {
    pub fn btrfs_find_all_leafs(ctx: *mut btrfs_backref_walk_ctx) -> c_int;
}
extern "C" {
    pub fn btrfs_prelim_ref_init() -> int __init;
}
extern "C" {
    pub fn btrfs_prelim_ref_exit() -> void __cold;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prelim_ref {
    pub rbnode: rb_node,
    pub root_id: u64,
    pub key_for_search: btrfs_key,
    pub level: u8,
    pub count: c_int,
    pub inode_list: *mut extent_inode_elem,
    pub parent: u64,
    pub wanted_disk_byte: u64,
}

//
// Iterate backrefs of one extent.
//
// Now it only supports iteration of tree block in commit root.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_backref_iter {
    pub bytenr: u64,
    pub path: *mut btrfs_path,
    pub cur_key: btrfs_key,
    pub item_ptr: u32,
    pub cur_ptr: u32,
    pub end_ptr: u32,
}

//
// For metadata with EXTENT_ITEM key (non-skinny) case, the first inline data
// is btrfs_tree_block_info, without a btrfs_extent_inline_ref header.
//
// This helper determines if that's the case.
//
extern "C" {
    pub fn btrfs_backref_iter_init(iter: *mut btrfs_backref_iter) -> c_int;
}
extern "C" {
    pub fn btrfs_backref_iter_start(fs_info: *mut btrfs_fs_info, iter: *mut btrfs_backref_iter, bytenr: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_backref_iter_next(fs_info: *mut btrfs_fs_info, iter: *mut btrfs_backref_iter) -> c_int;
}
//
// Backref cache related structures
//
// The whole objective of backref_cache is to build a bi-directional map
// of tree blocks (represented by backref_node) and all their parents.
//
// Represent a tree block in the backref cache
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_backref_node {
// Use rb_simple_node for search/insert
    pub rb_node: rb_node,
    pub bytenr: u64,
}

//
// This is a sanity check, whenever we COW a block we will update
// new_bytenr with it's current location, and we will check this in
// various places to validate that the cache makes sense, it shouldn't
// be used for anything else.
//
// Objectid of tree block owner, can be not uptodate
// Link to pending, changed or detached list
// List of upper level edges, which link this node to its parents
// List of lower level edges, which link this node to its children
// NULL if this node is not tree root
// Extent buffer got by COWing the block
// Level of the tree block
// Is the extent buffer locked
// Has the block been processed
// Have backrefs of this block been checked
//
// 1 if corresponding block has been COWed but some upper level block
// pointers may not point to the new location
//
// 1 if the backref node isn't connected to any other backref node
//
// For generic purpose backref cache, where we only care if it's a reloc
// root, doesn't care the source subvolid.
//
pub const LOWER: c_int = 0;
pub const UPPER: c_int = 1;
//
// Represent an edge connecting upper and lower backref nodes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_backref_edge {
//
// list[LOWER] is linked to btrfs_backref_node::upper of lower level
// node, and list[UPPER] is linked to btrfs_backref_node::lower of
// upper level node.
//
// Also, build_backref_tree() uses list[UPPER] for pending edges, before
// linking list[UPPER] to its upper level nodes.
//
    pub list: [list_head; 2],
// Two related nodes
    pub node: [*mut btrfs_backref_node; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_backref_cache {
// Red black tree of all backref nodes in the cache
    pub rb_root: rb_root,
// For passing backref nodes to btrfs_reloc_cow_block
    pub path: [*mut btrfs_backref_node; BTRFS_MAX_LEVEL],
//
// List of blocks that have been COWed but some block pointers in upper
// level blocks may not reflect the new location
//
    pub pending: [list_head; BTRFS_MAX_LEVEL],
    pub last_trans: u64,
    pub nr_nodes: c_int,
    pub nr_edges: c_int,
// List of unchecked backref edges during backref cache build
    pub pending_edge: list_head,
// List of useless backref nodes during backref cache build
    pub useless_node: list_head,
    pub fs_info: *mut btrfs_fs_info,
//
// Whether this cache is for relocation
//
// Relocation backref cache require more info for reloc root compared
// to generic backref cache.
//
    pub is_reloc: bool,
}

extern "C" {
    pub fn btrfs_backref_unlock_node_buffer(node: *mut btrfs_backref_node);
}
extern "C" {
    pub fn btrfs_backref_drop_node_buffer(node: *mut btrfs_backref_node);
}
extern "C" {
    pub fn btrfs_backref_release_cache(cache: *mut btrfs_backref_cache);
}
