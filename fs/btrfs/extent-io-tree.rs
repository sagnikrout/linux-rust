//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/extent-io-tree.h
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

// Bits for the extent state
//
// Must be cleared only during ordered extent completion or on error
// paths if we did not manage to submit bios and create the ordered
// extents for the range.  Should not be cleared during page release
// and page invalidation (if there is an ordered extent in flight),
// that is left for the ordered extent completion.
//
// Mark that a range is being locked for finishing an ordered extent.
// Used together with EXTENT_LOCKED.
//
// When an ordered extent successfully completes for a region marked as
// a new delalloc range, use this flag when clearing a new delalloc
// range to indicate that the VFS' inode number of bytes should be
// incremented and the inode's new delalloc bytes decremented, in an
// atomic way to prevent races with stat(2).
//
// Set during truncate when we're clearing an entire range and we just
// want the extent states to go away.
//
// This must be last.
//
// Bit not representing a state but a request for NOWAIT semantics,
// e.g. when allocating memory, and must be masked out from the other
// bits.
//

//
// Redefined bits above which are used only in the device allocation tree,
// shouldn't be using EXTENT_LOCKED / EXTENT_BOUNDARY / EXTENT_CLEAR_META_RESV
// / EXTENT_CLEAR_DATA_RESV because they have special meaning to the bit
// manipulation functions
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct extent_io_tree {
    pub state: rb_root,
//
// The fs_info is needed for trace points, a tree attached to an inode
// needs the inode.
//
// owner == IO_TREE_INODE_IO - then inode is valid and fs_info can be
// accessed as inode->root->fs_info
//
    pub fs_info: *mut btrfs_fs_info,
    pub inode: *mut btrfs_inode,
}

// Who owns this io tree, should be one of IO_TREE_*
#[repr(C)]
#[derive(Copy, Clone)]
pub struct extent_state {
    pub start: u64,
    pub /: *mut *mut u64 end; / inclusive,
    pub rb_node: rb_node,
// ADD NEW ELEMENTS AFTER THIS
    pub wq: wait_queue_head_t,
    pub refs: refcount_t,
    pub state: u32,

    pub leak_list: list_head,

}

extern "C" {
    pub fn btrfs_extent_io_tree_release(tree: *mut extent_io_tree);
}
extern "C" {
    pub fn btrfs_lock_extent_bits(_arg: tree, _arg: start, _arg: end, _arg: EXTENT_LOCKED, _arg: cached) -> return;
}
extern "C" {
    pub fn btrfs_try_lock_extent_bits(_arg: tree, _arg: start, _arg: end, _arg: EXTENT_LOCKED, _arg: cached) -> return;
}
extern "C" {
    pub fn btrfs_extent_state_init_cachep() -> int __init;
}
extern "C" {
    pub fn btrfs_extent_state_free_cachep() -> void __cold;
}
extern "C" {
    pub fn btrfs_free_extent_state(state: *mut extent_state);
}
extern "C" {
    pub fn btrfs_test_range_bit_exists(tree: *mut extent_io_tree, start: u64, end: u64, bit: u32) -> bool;
}
extern "C" {
    pub fn btrfs_clear_extent_bit_changeset(_arg: tree, _arg: start, _arg: end, _arg: bits, _arg: cached, _arg: NULL) -> return;
}
extern "C" {
    pub fn btrfs_lock_extent_bits(_arg: tree, _arg: start, _arg: end, _arg: EXTENT_DIO_LOCKED, _arg: cached) -> return;
}
extern "C" {
    pub fn btrfs_try_lock_extent_bits(_arg: tree, _arg: start, _arg: end, _arg: EXTENT_DIO_LOCKED, _arg: cached) -> return;
}
