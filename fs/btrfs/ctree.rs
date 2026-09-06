//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/ctree.h
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
// Copyright (C) 2007 Oracle.  All rights reserved.
//

// Read ahead values for struct btrfs_path.reada
//
// Similar to READA_FORWARD but unlike it:
//
// 1) It will trigger readahead even for leaves that are not close to
// each other on disk;
// 2) It also triggers readahead for nodes;
// 3) During a search, even when a node or leaf is already in memory, it
// will still trigger readahead for other nodes and leaves that follow
// it.
//
// This is meant to be used only when we know we are iterating over the
// entire tree or a very large part of it.
//
// btrfs_paths remember the path taken from the root down to the leaf.
// level 0 is always the leaf, and nodes[1...BTRFS_MAX_LEVEL] will point
// to any other levels that are present.
//
// The slots array records the index of the item or block pointer
// used while walking the tree.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_path {
    pub nodes: [*mut extent_buffer; BTRFS_MAX_LEVEL],
    pub slots: [c_int; BTRFS_MAX_LEVEL],
// if there is real range locking, this locks field will change
    pub locks: [u8; BTRFS_MAX_LEVEL],
    pub reada: u8,
    pub lowest_level: u8,
//
// set by btrfs_split_item, tells search_slot to keep all locks
// and to force calls to keep space in the nodes
//
    pub search_for_split:1: bool,
// Keep some upper locks as we walk down.
    pub keep_locks:1: bool,
    pub skip_locking:1: bool,
    pub search_commit_root:1: bool,
    pub need_commit_sem:1: bool,
    pub skip_release_on_error:1: bool,
//
// Indicate that new item (btrfs_search_slot) is extending already
// existing item and ins_len contains only the data size and not item
// header (ie. sizeof(struct btrfs_item) is not included).
//
    pub search_for_extension:1: bool,
// Stop search if any locks need to be taken (for read)
    pub nowait:1: bool,
}

//
// This defines an on-stack path that will be auto released when exiting the scope.
//
// It is compatible with any existing manual btrfs_release_path() calls.
//

//
// The state of btrfs root
//
// btrfs_record_root_in_trans is a multi-step process, and it can race
// with the balancing code.   But the race is very small, and only the
// first time the root is added to each transaction.  So IN_TRANS_SETUP
// is used to tell us when more checks are required
//
// Set if tree blocks of this root can be shared by other roots.
// Only subvolume trees and their reloc trees have this bit set.
// Conflicts with TRACK_DIRTY bit.
//
// This affects two things:
//
// - How balance works
// For shareable roots, we need to use reloc tree and do path
// replacement for balance, and need various pre/post hooks for
// snapshot creation to handle them.
//
// While for non-shareable trees, we just simply do a tree search
// with COW.
//
// - How dirty roots are tracked
// For shareable roots, btrfs_record_root_in_trans() is needed to
// track them, while non-subvolume roots have TRACK_DIRTY bit, they
// don't need to set this manually.
//
// Reloc tree is orphan, only kept here for qgroup delayed subtree scan
//
// Set for the subvolume tree owning the reloc tree.
//
// Mark dead root stored on device whose cleanup needs to be resumed
// The root has a log tree. Used for subvolume roots and the tree root.
// Qgroup flushing is in progress
// We started the orphan cleanup for this root.
// This root has a drop operation that was started previously.
// This reloc root needs to have its buffers lockdep class reset.
//
// Record swapped tree blocks of a subvolume tree for delayed subtree trace
// code. For detail check comment in fs/btrfs/qgroup.c.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_qgroup_swapped_blocks {
    pub lock: spinlock_t,
// RM_EMPTY_ROOT() of above blocks[]
    pub swapped: bool,
    pub blocks: [rb_root; BTRFS_MAX_LEVEL],
}

//
// in ram representation of the tree.  extent_root is used for all allocations
// and for the extent tree extent_root root.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_root {
    pub rb_node: rb_node,
    pub node: *mut extent_buffer,
    pub commit_root: *mut extent_buffer,
    pub log_root: *mut btrfs_root,
    pub reloc_root: *mut btrfs_root,
    pub state: c_ulong,
    pub root_item: btrfs_root_item,
    pub root_key: btrfs_key,
    pub fs_info: *mut btrfs_fs_info,
    pub dirty_log_pages: extent_io_tree,
    pub objectid_mutex: mutex,
    pub accounting_lock: spinlock_t,
    pub block_rsv: *mut btrfs_block_rsv,
    pub log_mutex: mutex,
    pub log_writer_wait: wait_queue_head_t,
    pub log_commit_wait: [wait_queue_head_t; 2],
    pub log_ctxs: [list_head; 2],
// Used only for log trees of subvolumes, not for the log root tree
    pub log_writers: core::sync::atomic::AtomicI32,
    pub log_commit: [bool; 2],
//
// Protected by the 'log_mutex' lock but can be read without holding
// that lock to avoid unnecessary lock contention, in which case it
// should be read using btrfs_get_root_log_transid() except if it's a
// log tree in which case it can be directly accessed. Updates to this
// field should always use btrfs_set_root_log_transid(), except for log
// trees where the field can be updated directly.
//
    pub log_transid: c_int,
// No matter the commit succeeds or not
    pub log_transid_committed: c_int,
//
// Just be updated when the commit succeeds. Use
// btrfs_get_root_last_log_commit() and btrfs_set_root_last_log_commit()
// to access this field.
//
    pub last_log_commit: c_int,
    pub last_trans: u64,
    pub free_objectid: u64,
    pub defrag_progress: btrfs_key,
    pub defrag_max: btrfs_key,
// The dirty list is only used by non-shareable roots
    pub dirty_list: list_head,
    pub root_list: list_head,
// Xarray that keeps track of in-memory inodes.
    pub inodes: xarray,
// Xarray that keeps track of delayed nodes of every inode.
    pub delayed_nodes: xarray,
//
// right now this just gets used so that a root has its own devid
// for stat.  It may be used for more later
//
    pub anon_dev: dev_t,
    pub root_item_lock: spinlock_t,
    pub refs: refcount_t,
    pub delalloc_mutex: mutex,
    pub delalloc_lock: spinlock_t,
//
// all of the inodes that have delalloc bytes.  It is possible for
// this list to be empty even when there is still dirty data=ordered
// extents waiting to finish IO.
//
    pub delalloc_inodes: list_head,
    pub delalloc_root: list_head,
    pub nr_delalloc_inodes: u64,
    pub ordered_extent_mutex: mutex,
//
// this is used by the balancing code to wait for all the pending
// ordered extents
//
    pub ordered_extent_lock: spinlock_t,
//
// all of the data=ordered extents pending writeback
// these can span multiple transactions and basically include
// every dirty data page that isn't from nodatacow
//
    pub ordered_extents: list_head,
    pub ordered_root: list_head,
    pub nr_ordered_extents: u64,
//
// Not empty if this subvolume root has gone through tree block swap
// (relocation)
//
// Will be used by reloc_control::dirty_subvol_roots.
//
    pub reloc_dirty_list: list_head,
//
// Number of currently running SEND ioctls to prevent
// manipulation with the read-only status via SUBVOL_SETFLAGS
//
    pub send_in_progress: c_int,
//
// Number of currently running deduplication operations that have a
// destination inode belonging to this root. Protected by the lock
// root_item_lock.
//
    pub dedupe_in_progress: c_int,
// For exclusion of snapshot creation and nocow writes
    pub snapshot_lock: btrfs_drew_lock,
    pub snapshot_force_cow: core::sync::atomic::AtomicI32,
// For qgroup metadata reserved space
    pub qgroup_meta_rsv_lock: spinlock_t,
    pub qgroup_meta_rsv_pertrans: u64,
    pub qgroup_meta_rsv_prealloc: u64,
    pub qgroup_flush_wait: wait_queue_head_t,
// Number of active swapfiles
    pub nr_swapfiles: core::sync::atomic::AtomicI32,
// Record pairs of swapped blocks for qgroup
    pub swapped_blocks: btrfs_qgroup_swapped_blocks,
// Used only by log trees, when logging csum items
    pub log_csum_range: extent_io_tree,
// Used in simple quotas, track root during relocation.
    pub relocation_src_root: u64,

    pub alloc_bytenr: u64,

    pub leak_list: list_head,

}

// Byte-swap the constant at compile time, root_item::flags is LE
extern "C" {
    pub fn READ_ONCE(_arg: root->log_transid) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: root->last_log_commit) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: root->last_trans) -> return;
}
//
// Return the generation this root started with.
//
// Every normal root that is created with root->root_key.offset set to it's
// originating generation.  If it is a snapshot it is the generation when the
// snapshot was created.
//
// However for TREE_RELOC roots root_key.offset is the objectid of the owning
// tree root.  Thankfully we copy the root item of the owning tree root, which
// has it's last_snapshot set to what we would have root_key.offset set to, so
// return that if this is a TREE_RELOC root.
//
extern "C" {
    pub fn btrfs_root_last_snapshot(_arg: &root->root_item) -> return;
}
//
// Structure that conveys information about an extent that is going to replace
// all the extents in a file range.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_replace_extent_info {
    pub disk_offset: u64,
    pub disk_len: u64,
    pub data_offset: u64,
    pub data_len: u64,
    pub file_offset: u64,
// Pointer to a file extent item of type regular or prealloc.
    pub extent_buf: *mut c_char,
//
// Set to true when attempting to replace a file range with a new extent
// described by this structure, set to false when attempting to clone an
// existing extent into a file range.
//
    pub is_new_extent: bool,
// Indicate if we should update the inode's mtime and ctime.
    pub update_times: bool,
// Meaningful only if is_new_extent is true.
    pub qgroup_reserved: c_int,
//
// Meaningful only if is_new_extent is true.
// Used to track how many extent items we have already inserted in a
// subvolume tree that refer to the extent described by this structure,
// so that we know when to create a new delayed ref or update an existing
// one.
//
    pub insertions: c_int,
}

// Arguments for btrfs_drop_extents()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_drop_extents_args {
// Input parameters
//
// If NULL, btrfs_drop_extents() will allocate and free its own path.
// If 'replace_extent' is true, this must not be NULL. Also the path
// is always released except if 'replace_extent' is true and
// btrfs_drop_extents() sets 'extent_inserted' to true, in which case
// the path is kept locked.
//
    pub path: *mut btrfs_path,
// Start offset of the range to drop extents from
    pub start: u64,
// End (exclusive, last byte + 1) of the range to drop extents from
    pub end: u64,
// If true drop all the extent maps in the range
    pub drop_cache: bool,
//
// If true it means we want to insert a new extent after dropping all
// the extents in the range. If this is true, the 'extent_item_size'
// parameter must be set as well and the 'extent_inserted' field will
// be set to true by btrfs_drop_extents() if it could insert the new
// extent.
// Note: when this is set to true the path must not be NULL.
//
    pub replace_extent: bool,
//
// Used if 'replace_extent' is true. Size of the file extent item to
// insert after dropping all existing extents in the range
//
    pub extent_item_size: u32,
// Output parameters
//
// Set to the minimum between the input parameter 'end' and the end
// (exclusive, last byte + 1) of the last dropped extent. This is always
// set even if btrfs_drop_extents() returns an error.
//
    pub drop_end: u64,
//
// The number of allocated bytes found in the range. This can be smaller
// than the range's length when there are holes in the range.
//
    pub bytes_found: u64,
//
// Only set if 'replace_extent' is true. Set to true if we were able
// to insert a replacement extent after dropping all extents in the
// range, otherwise set to false by btrfs_drop_extents().
// Also, if btrfs_drop_extents() has set this to true it means it
// returned with the path locked, otherwise if it has set this to
// false it has returned with the path released.
//
    pub extent_inserted: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_file_private {
    pub filldir_buf: *mut c_void,
    pub last_index: u64,
    pub llseek_cached_state: *mut extent_state,
// Task that allocated this structure.
    pub owner_task: *mut task_struct,
}

extern "C" {
    pub fn BTRFS_LEAF_DATA_SIZE(btrfs_item: info) - sizeof(struct) -> return;
}
extern "C" {
    pub fn BTRFS_LEAF_DATA_SIZE(btrfs_key_ptr: info) / sizeof(struct) -> return;
}
extern "C" {
    pub fn BTRFS_MAX_ITEM_SIZE(btrfs_dir_item: info) - sizeof(struct) -> return;
}
extern "C" {
    pub fn btrfs_ctree_init() -> int __init;
}
extern "C" {
    pub fn btrfs_ctree_exit() -> void __cold;
}
extern "C" {
    pub fn btrfs_comp_cpu_keys(k1: *const btrfs_key, k2: *const btrfs_key) -> int __pure;
}

//
// Compare two keys, on little-endian the disk order is same as CPU order and
// we can avoid the conversion.
//
extern "C" {
    pub fn btrfs_comp_cpu_keys(_arg: k1, _arg: k2) -> return;
}

// Compare two keys in a memcmp fashion.
extern "C" {
    pub fn btrfs_comp_cpu_keys(_arg: &k1, _arg: k2) -> return;
}

extern "C" {
    pub fn btrfs_release_path(p: *mut btrfs_path);
}
extern "C" {
    pub fn btrfs_free_path(p: *mut btrfs_path);
}
extern "C" {
    pub fn btrfs_del_items(_arg: trans, _arg: root, _arg: path, _arg: path->slots[0], _arg: 1) -> return;
}
//
// Describes a batch of items to insert in a btree. This is used by
// btrfs_insert_empty_items().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_item_batch {
//
// Pointer to an array containing the keys of the items to insert (in
// sorted order).
//
    pub keys: *const btrfs_key,
// Pointer to an array containing the data size for each item to insert.
    pub data_sizes: *const u32,
//
// The sum of data sizes for all items. The caller can compute this while
// setting up the data_sizes array, so it ends up being more efficient
// than having btrfs_insert_empty_items() or setup_item_for_insert()
// doing it, as it would avoid an extra loop over a potentially large
// array, and in the case of setup_item_for_insert(), we would be doing
// it while holding a write lock on a leaf and often on upper level nodes
// too, unnecessarily increasing the size of a critical section.
//
    pub total_data_size: u32,
// Size of the keys and data_sizes arrays (number of items in the batch).
    pub nr: c_int,
}

extern "C" {
    pub fn btrfs_insert_empty_items(_arg: trans, _arg: root, _arg: path, _arg: &batch) -> return;
}
//
// Search in @root for a given @key, and store the slot found in @found_key.
//
// @root:	The root node of the tree.
// @key:	The key we are looking for.
// @found_key:	Will hold the found item.
// @path:	Holds the current slot/leaf.
// @iter_ret:	Contains the value returned from btrfs_search_slot or
// btrfs_get_next_valid_item, whichever was executed last.
//
// The @iter_ret is an output variable that will contain the return value of
// btrfs_search_slot, if it encountered an error, or the value returned from
// btrfs_get_next_valid_item otherwise. That return value can be 0, if a valid
// slot was found, 1 if there were no more leaves, and <0 if there was an error.
//
// It's recommended to use a separate variable for iter_ret and then use it to
// set the function return value so there's no confusion of the 0/1/errno
// values stemming from btrfs_search_slot.
//

extern "C" {
    pub fn btrfs_next_old_item(root: *mut btrfs_root, path: *mut btrfs_path, time_seq: u64) -> c_int;
}
//
// Search the tree again to find a leaf with greater keys.
//
// Returns 0 if it found something or 1 if there are no greater leaves.
// Returns < 0 on error.
//
extern "C" {
    pub fn btrfs_next_old_leaf(_arg: root, _arg: path, _arg: 0) -> return;
}
extern "C" {
    pub fn btrfs_next_old_item(_arg: root, _arg: p, _arg: 0) -> return;
}
extern "C" {
    pub fn btrfs_leaf_free_space(leaf: *const extent_buffer) -> c_int;
}
