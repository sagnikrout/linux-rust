//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nilfs2/nilfs.h
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
// NILFS local header file.
//
// Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
//
// Written by Koji Sato and Ryusuke Konishi.
//

//
// struct nilfs_inode_info - nilfs inode data in memory
// @i_flags: inode flags
// @i_type: inode type (combination of flags that inidicate usage)
// @i_state: dynamic state flags
// @i_bmap: pointer on i_bmap_data
// @i_bmap_data: raw block mapping
// @i_xattr: <TODO>
// @i_dir_start_lookup: page index of last successful search
// @i_cno: checkpoint number for GC inode
// @i_assoc_inode: associated inode (B-tree node cache holder or back pointer)
// @i_dirty: list for connecting dirty files
// @xattr_sem: semaphore for extended attributes processing
// @i_bh: buffer contains disk inode
// @i_root: root object of the current filesystem tree
// @vfs_inode: VFS inode object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_inode_info {
    pub i_flags: __u32,
    pub i_type: c_uint,
    pub /: *mut *mut unsigned long i_state; / Dynamic state flags,
    pub i_bmap: *mut nilfs_bmap,
    pub i_bmap_data: nilfs_bmap,
    pub /: *mut *mut __u64 i_xattr; / sector_t ???,
    pub i_dir_start_lookup: __u32,
    pub /: *mut *mut __u64 i_cno; / check point number for GC inode,
    pub i_assoc_inode: *mut inode,
    pub /: *mut *mut list_head i_dirty; / List for connecting dirty files,

//
// Extended attributes can be read independently of the main file
// data. Taking i_sem even when reading would cause contention
// between readers of EAs and writers of regular file data, so
// instead we synchronize on xattr_sem when reading or changing
// EAs.
//
    pub xattr_sem: rw_semaphore,

    pub /*: *mut *mut buffer_head i_bh;,
// i_bh contains a new or dirty
// disk inode.
//
    pub i_root: *mut nilfs_root,
    pub vfs_inode: inode,
}

extern "C" {
    pub fn container_of(_arg: inode, nilfs_inode_info: struct, _arg: vfs_inode) -> return;
}
extern "C" {
    pub fn container_of(_arg: bmap, nilfs_inode_info: struct, _arg: i_bmap_data) -> return;
}
//
// Dynamic state flags of NILFS on-memory inode (i_state)
//
// Inode is grabbed by a segment
// constructor
//
// Flags to identify the usage of on-memory inodes (i_type)
//
// commit flags for nilfs_commit_super and nilfs_sync_super
//
// define NILFS_MAX_VOLUME_NAME - maximum number of characters (bytes) in a
// file system volume name
//
// Defined by the size of the volume name field in the on-disk superblocks.
// This volume name does not include the terminating NULL byte if the string
// length matches the field size, so use (NILFS_MAX_VOLUME_NAME + 1) for the
// size of the buffer that requires a NULL byte termination.
//

//
// Macros to check inode numbers
//

//
// struct nilfs_transaction_info: context information for synchronization
// @ti_magic: Magic number
// @ti_save: Backup of journal_info field of task_struct
// @ti_flags: Flags
// @ti_count: Nest level
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_transaction_info {
    pub ti_magic: u32,
    pub ti_save: *mut c_void,
//
// This should never be used.  If it happens,
// one of other filesystems has a bug.
//
    pub ti_flags: c_ushort,
    pub ti_count: c_ushort,
}

// ti_magic
pub const NILFS_TI_MAGIC: c_uint = 0xd9e392fb;
// ti_flags
pub const NILFS_TI_DYNAMIC_ALLOC: c_uint = 0x0001  /* Allocated from slab */;
pub const NILFS_TI_SYNC: c_uint = 0x0002	/*;
// Force to construct segment at the
// end of transaction.
//
pub const NILFS_TI_GC: c_uint = 0x0004	/* GC context */;
pub const NILFS_TI_COMMIT: c_uint = 0x0008	/* Change happened or not */;
pub const NILFS_TI_WRITER: c_uint = 0x0010	/* Constructor context */;
extern "C" {
    pub fn nilfs_transaction_commit(: *mut super_block) -> c_int;
}
extern "C" {
    pub fn nilfs_transaction_abort(: *mut super_block);
}
extern "C" {
    pub fn nilfs_test_transaction_flag(_arg: NILFS_TI_GC) -> return;
}
extern "C" {
    pub fn nilfs_test_transaction_flag(_arg: NILFS_TI_WRITER) -> return;
}
//
// function prototype
//

extern "C" {
    pub fn nilfs_acl_chmod(: *mut inode) -> c_int;
}
extern "C" {
    pub fn nilfs_init_acl(: *mut inode, : *mut inode) -> c_int;
}

// Macro flag: #define NILFS_ATIME_DISABLE
// Flags that should be inherited by new inodes from their parent.

// Mask out flags that are inappropriate for the given type of inode.
// dir.c
extern "C" {
    pub fn nilfs_add_link(: *mut dentry, : *mut inode) -> c_int;
}
extern "C" {
    pub fn nilfs_inode_by_name(dir: *mut inode, qstr: *const qstr, ino: *mut u64) -> c_int;
}
extern "C" {
    pub fn nilfs_make_empty(: *mut inode, : *mut inode) -> c_int;
}
extern "C" {
    pub fn nilfs_delete_entry(: *mut nilfs_dir_entry, : *mut folio) -> c_int;
}
extern "C" {
    pub fn nilfs_empty_dir(: *mut inode) -> c_int;
}
// file.c
extern "C" {
    pub fn nilfs_sync_file(: *mut file, _arg: loff_t, _arg: loff_t, _arg: c_int) -> c_int;
}
// ioctl.c
extern "C" {
    pub fn nilfs_fileattr_get(dentry: *mut dentry, m: *mut file_kattr) -> c_int;
}
extern "C" {
    pub fn nilfs_ioctl(: *mut file, int: unsigned, long: unsigned) -> c_long;
}
extern "C" {
    pub fn nilfs_compat_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}
// inode.c
extern "C" {
    pub fn nilfs_inode_add_blocks(inode: *mut inode, n: c_int);
}
extern "C" {
    pub fn nilfs_inode_sub_blocks(inode: *mut inode, n: c_int);
}
extern "C" {
    pub fn nilfs_get_block(: *mut inode, _arg: sector_t, : *mut buffer_head, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn nilfs_set_inode_flags(: *mut inode);
}
extern "C" {
    pub fn nilfs_read_inode_common(: *mut inode, : *mut nilfs_inode) -> c_int;
}
extern "C" {
    pub fn nilfs_attach_btree_node_cache(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn nilfs_detach_btree_node_cache(inode: *mut inode);
}
extern "C" {
    pub fn nilfs_update_inode(: *mut inode, : *mut buffer_head, _arg: c_int);
}
extern "C" {
    pub fn nilfs_truncate(: *mut inode);
}
extern "C" {
    pub fn nilfs_evict_inode(: *mut inode);
}
extern "C" {
    pub fn nilfs_write_failed(mapping: *mut address_space, to: loff_t);
}
extern "C" {
    pub fn nilfs_load_inode_block(inode: *mut inode, pbh: *mut buffer_head) -> c_int;
}
extern "C" {
    pub fn nilfs_inode_dirty(: *mut inode) -> c_int;
}
extern "C" {
    pub fn nilfs_set_file_dirty(inode: *mut inode, nr_dirty: c_uint) -> c_int;
}
extern "C" {
    pub fn __nilfs_mark_inode_dirty(: *mut inode, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn nilfs_dirty_inode(: *mut inode, flags: c_int);
}
extern "C" {
    pub fn __nilfs_mark_inode_dirty(_arg: inode, _arg: I_DIRTY) -> return;
}
extern "C" {
    pub fn __nilfs_mark_inode_dirty(_arg: inode, _arg: I_DIRTY_SYNC) -> return;
}
// super.c
extern "C" {
    pub fn __nilfs_msg(sb: *mut super_block, fmt: *const c_char, ...);
}

extern "C" {
    pub fn nilfs_commit_super(sb: *mut super_block, flag: c_int) -> c_int;
}
extern "C" {
    pub fn nilfs_cleanup_super(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn nilfs_resize_fs(sb: *mut super_block, newsize: __u64) -> c_int;
}
extern "C" {
    pub fn nilfs_checkpoint_is_mounted(sb: *mut super_block, cno: __u64) -> c_int;
}
// gcinode.c
extern "C" {
    pub fn nilfs_gccache_wait_and_mark_dirty(: *mut buffer_head) -> c_int;
}
extern "C" {
    pub fn nilfs_init_gcinode(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn nilfs_remove_all_gcinodes(nilfs: *mut the_nilfs);
}
// sysfs.c
extern "C" {
    pub fn nilfs_sysfs_init() -> int __init;
}
extern "C" {
    pub fn nilfs_sysfs_exit();
}
extern "C" {
    pub fn nilfs_sysfs_create_device_group(: *mut super_block) -> c_int;
}
extern "C" {
    pub fn nilfs_sysfs_delete_device_group(: *mut the_nilfs);
}
extern "C" {
    pub fn nilfs_sysfs_create_snapshot_group(: *mut nilfs_root) -> c_int;
}
extern "C" {
    pub fn nilfs_sysfs_delete_snapshot_group(: *mut nilfs_root);
}
//
// Inodes and files operations
//
// filesystem type
//
