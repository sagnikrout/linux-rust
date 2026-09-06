//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/btrfs_inode.h
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

//
// Since we search a directory based on f_pos (struct dir_context::pos) we have
// to start at 2 since '.' and '..' have f_pos of 0 and 1 respectively, so
// everybody else has to start at 2 (see btrfs_real_readdir() and dir_emit_dots()).
//
pub const BTRFS_DIR_START_INDEX: c_int = 2;
//
// ordered_data_close is set by truncate when a file that used
// to have good data has been truncated to zero.  When it is set
// the btrfs file release call will add this inode to the
// ordered operations list so that we make sure to flush out any
// new data the application may have written before commit.
//
// Always set under the VFS' inode lock, otherwise it can cause races
// during fsync (we start as a fast fsync and then end up in a full
// fsync racing with ordered extent completion).
//
// Set and used when logging an inode and it serves to signal that an
// inode does not have xattrs, so subsequent fsyncs can avoid searching
// for xattrs to log. This bit must be cleared whenever a xattr is added
// to an inode.
//
// Set when we are in a context where we need to start a transaction and
// have dirty pages with the respective file range locked. This is to
// ensure that when reserving space for the transaction, if we are low
// on available space and need to flush delalloc, we will not flush
// delalloc for this inode, because that could result in a deadlock (on
// the file range, inode's io_tree).
//
// Set when we are working on enabling verity for a file. Computing and
// writing the whole Merkle tree can take a while so we want to prevent
// races where two separate tasks attempt to simultaneously start verity
// on the same file.
//
// Set when this inode is a free space inode.
// Set when there are no capabilities in XATTs for the inode.
//
// Set if an error happened when doing a COW write before submitting a
// bio or during writeback. Used for both buffered writes and direct IO
// writes. This is to signal a fast fsync that it has to wait for
// ordered extents to complete and therefore not log extent maps that
// point to unwritten extents (when an ordered extent completes and it
// has the BTRFS_ORDERED_IOERR flag set, it drops extent maps in its
// range).
//
// Indicate this is a directory that points to a subvolume for which
// there is no root reference item. That's a case like the following:
//
// $ btrfs subvolume create /mnt/parent
// $ btrfs subvolume create /mnt/parent/child
// $ btrfs subvolume snapshot /mnt/parent /mnt/snap
//
// If subvolume "parent" is root 256, subvolume "child" is root 257 and
// snapshot "snap" is root 258, then there's no root reference item (key
// BTRFS_ROOT_REF_KEY in the root tree) for the subvolume "child"
// associated to root 258 (the snapshot) - there's only for the root
// of the "parent" subvolume (root 256). In the chunk root we have a
// (256 BTRFS_ROOT_REF_KEY 257) key but we don't have a
// (258 BTRFS_ROOT_REF_KEY 257) key - the sames goes for backrefs, we
// have a (257 BTRFS_ROOT_BACKREF_KEY 256) but we don't have a
// (257 BTRFS_ROOT_BACKREF_KEY 258) key.
//
// So when opening the "child" dentry from the snapshot's directory,
// we don't find a root ref item and we create a stub inode. This is
// done at new_simple_dir(), called from btrfs_lookup_dentry().
//
// in memory btrfs inode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_inode {
// which subvolume this inode belongs to
    pub root: *mut btrfs_root,
// Cached value of inode property 'compression'.
    pub prop_compress: u8,
//
// Force compression on the file using the defrag ioctl, could be
// different from prop_compress and takes precedence if set.
//
    pub defrag_compress: u8,
    pub defrag_compress_level: i8,
//
// Lock for counters and all fields used to determine if the inode is in
// the log or not (last_trans, last_sub_trans, last_log_commit,
// logged_trans), to access/update delalloc_bytes, new_delalloc_bytes,
// defrag_bytes, disk_i_size, outstanding_extents, csum_bytes and to
// update the VFS' inode number of bytes used.
// Also protects setting struct file::private_data.
//
    pub lock: spinlock_t,
// the extent_tree has caches of all the extent mappings to disk
    pub extent_tree: extent_map_tree,
// the io_tree does range state (DIRTY, LOCKED etc)
    pub io_tree: extent_io_tree,
//
// Keep track of where the inode has extent items mapped in order to
// make sure the i_size adjustments are accurate. Not required when the
// filesystem is NO_HOLES, the status can't be set while mounted as
// it's a mkfs-time feature.
//
    pub file_extent_tree: *mut extent_io_tree,
// held while logging the inode in tree-log.c
    pub log_mutex: mutex,
//
// Counters to keep track of the number of extent item's we may use due
// to delalloc and such.  outstanding_extents is the number of extent
// items we think we'll end up using, and reserved_extents is the number
// of extent items we've reserved metadata for. Protected by 'lock'.
//
    pub outstanding_extents: unsigned,
// used to order data wrt metadata
    pub ordered_tree_lock: spinlock_t,
    pub ordered_tree: rb_root,
    pub ordered_tree_last: *mut rb_node,
// list of all the delalloc inodes in the FS.  There are times we need
// to write all the delalloc pages to disk, and this list is used
// to walk them all.
//
    pub delalloc_inodes: list_head,
    pub runtime_flags: c_ulong,
// full 64 bit generation number, struct vfs_inode doesn't have a big
// enough field for this.
//
    pub generation: u64,
//
// ID of the transaction handle that last modified this inode.
// Protected by 'lock'.
//
    pub last_trans: u64,
//
// ID of the transaction that last logged this inode.
// Protected by 'lock'.
//
    pub logged_trans: u64,
//
// Log transaction ID when this inode was last modified.
// Protected by 'lock'.
//
    pub last_sub_trans: c_int,
// A local copy of root's last_log_commit. Protected by 'lock'.
    pub last_log_commit: c_int,
//
// Total number of bytes pending delalloc, used by stat to
// calculate the real block usage of the file. This is used
// only for files. Protected by 'lock'.
//
    pub delalloc_bytes: u64,
//
// The lowest possible index of the next dir index key which
// points to an inode that needs to be logged.
// This is used only for directories.
// Use the helpers btrfs_get_first_dir_index_to_log() and
// btrfs_set_first_dir_index_to_log() to access this field.
//
    pub first_dir_index_to_log: u64,
}

//
// Total number of bytes pending delalloc that fall within a file
// range that is either a hole or beyond EOF (and no prealloc extent
// exists in the range). This is always <= delalloc_bytes and this
// is used only for files. Protected by 'lock'.
//
// The offset of the last dir index key that was logged.
// This is used only for directories. Protected by 'log_mutex'.
//
// Total number of bytes pending defrag, used by stat to check whether
// it needs COW. Protected by 'lock'.
// Used by inodes other than the data relocation inode.
//
// Logical address of the block group being relocated.
// Used only by the data relocation inode.
//
// The size of the file stored in the metadata on disk.  data=ordered
// means the in-memory i_size might be larger than the size on disk
// because not all the blocks are written yet. Protected by 'lock'.
//
// If this is a directory then index_cnt is the counter for the
// index number for new files that are created. For an empty
// directory, this must be initialized to BTRFS_DIR_START_INDEX.
//
// If this is not a directory, this is the number of bytes
// outstanding that are going to need csums. This is used in
// ENOSPC accounting. Protected by 'lock'.
//
// Cache the directory index number to speed the dir/file remove
// the fsync log has some corner cases that mean we have to check
// directories to see if any unlinks have been done before
// the directory was logged.  See tree-log.c for all the
// details
//
// The id/generation of the last transaction where this inode
// was either the source or the destination of a clone/dedupe
// operation. Used when logging an inode to know if there are
// shared extents that need special care when logging checksum
// items, to avoid duplicate checksum items in a log (which can
// lead to a corruption where we end up with missing checksum
// ranges after log replay). Protected by the VFS inode lock.
// Used for regular files only.
//
// In case this a root stub inode (BTRFS_INODE_ROOT_STUB flag set),
// the ID of that root.
//
// Backwards incompatible flags, lower half of inode_item::flags
// Read-only compatibility flags, upper half of inode_item::flags
// File creation time.
// Hook into fs_info->delayed_iputs
extern "C" {
    pub fn READ_ONCE(_arg: inode->first_dir_index_to_log) -> return;
}
// Type checked and const-preserving VFS inode -> btrfs inode.

extern "C" {
    pub fn test_bit(_arg: BTRFS_INODE_FREE_SPACE_INODE, _arg: &inode->runtime_flags) -> return;
}
//
// Called every time after doing a buffered, direct IO or memory mapped write.
//
// This is to ensure that if we write to a file that was previously fsynced in
// the current transaction, then try to fsync it again in the same transaction,
// we will know that there were changes in the file and that it needs to be
// logged.
//
// Should be called while holding the inode's VFS lock in exclusive mode, or
// while holding the inode's mmap lock (struct btrfs_inode::i_mmap_lock) in
// either shared or exclusive mode, or in a context where no one else can access
// the inode concurrently (during inode creation or when loading an inode from
// disk).
//
// The inode may have been part of a reflink operation in the last
// transaction that modified it, and then a fsync has reset the
// last_reflink_trans to avoid subsequent fsyncs in the same
// transaction to do unnecessary work. So update last_reflink_trans
// to the last_trans value (we have to be pessimistic and assume a
// reflink happened).
//
// The ->last_trans is protected by the inode's spinlock and we can
// have a concurrent ordered extent completion update it. Also set
// last_reflink_trans to ->last_trans only if the former is less than
// the later, because we can be called in a context where
// last_reflink_trans was set to the current transaction generation
// while ->last_trans was not yet updated in the current transaction,
// and therefore has a lower value.
//
// Check if the inode has flags compatible with compression
//
// Immediately trigger a crash if the inode is not locked.
// Trigger a splat in dmesg if this task is not holding the lock.
// Metadata inode should not reach here.
extern "C" {
    pub fn btrfs_del_delalloc_inode(inode: *mut btrfs_inode);
}
extern "C" {
    pub fn btrfs_set_inode_index(dir: *mut btrfs_inode, index: *mut u64) -> c_int;
}
extern "C" {
    pub fn btrfs_delete_subvolume(dir: *mut btrfs_inode, dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn btrfs_truncate_block(inode: *mut btrfs_inode, offset: u64, start: u64, end: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_start_delalloc_snapshot(root: *mut btrfs_root, in_reclaim_context: bool) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_new_inode_args {
// Input
    pub dir: *mut inode,
    pub dentry: *mut dentry,
    pub inode: *mut inode,
    pub orphan: bool,
    pub subvol: bool,
// Output from btrfs_new_inode_prepare(), input to btrfs_create_new_inode().
    pub default_acl: *mut posix_acl,
    pub acl: *mut posix_acl,
    pub fname: fscrypt_name,
}

extern "C" {
    pub fn btrfs_new_inode_args_destroy(args: *mut btrfs_new_inode_args);
}
extern "C" {
    pub fn btrfs_evict_inode(inode: *mut inode);
}
extern "C" {
    pub fn btrfs_destroy_inode(inode: *mut inode);
}
extern "C" {
    pub fn btrfs_free_inode(inode: *mut inode);
}
extern "C" {
    pub fn btrfs_drop_inode(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn btrfs_init_cachep() -> int __init;
}
extern "C" {
    pub fn btrfs_destroy_cachep() -> void __cold;
}
extern "C" {
    pub fn btrfs_orphan_add(trans: *mut btrfs_trans_handle, inode: *mut btrfs_inode) -> c_int;
}
extern "C" {
    pub fn btrfs_orphan_cleanup(root: *mut btrfs_root) -> c_int;
}
extern "C" {
    pub fn btrfs_cont_expand(inode: *mut btrfs_inode, oldsize: loff_t, size: loff_t) -> c_int;
}
extern "C" {
    pub fn btrfs_add_delayed_iput(inode: *mut btrfs_inode);
}
extern "C" {
    pub fn btrfs_run_delayed_iputs(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_wait_on_delayed_iputs(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_queue_writepage_fixup(inode: *mut btrfs_inode, folio: *mut folio);
}
// Inode locking type flags, by default the exclusive lock is taken.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_ilock_type {
    ENUM_BIT(BTRFS_ILOCK_SHARED),
    ENUM_BIT(BTRFS_ILOCK_TRY),
    ENUM_BIT(BTRFS_ILOCK_MMAP),
}

extern "C" {
    pub fn btrfs_inode_lock(inode: *mut btrfs_inode, ilock_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn btrfs_inode_unlock(inode: *mut btrfs_inode, ilock_flags: c_uint);
}
extern "C" {
    pub fn btrfs_assert_inode_range_clean(inode: *mut btrfs_inode, start: u64, end: u64);
}
