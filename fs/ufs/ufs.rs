//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ufs/ufs.h
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
pub const _UFS_UFS_H: c_int = 1;

pub const UFS_MAX_GROUP_LOADED: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_sb_info {
    pub s_uspi: *mut *mut ufs_sb_private_info,
    pub s_csp: *mut *mut ufs_csum,
    pub s_bytesex: unsigned,
    pub s_flags: unsigned,
    pub s_ucg: *mut *mut *mut buffer_head,
    pub s_ucpi: [*mut *mut ufs_cg_private_info; UFS_MAX_GROUP_LOADED],
    pub s_cgno: [unsigned; UFS_MAX_GROUP_LOADED],
    pub s_cg_loaded: c_ushort,
    pub s_flavour: unsigned,
    pub s_on_err: unsigned,
    pub sb: *mut super_block,
    pub /: *mut *mut int work_queued; / non-zero if the delayed work is queued,
    pub /: *mut *mut delayed_work sync_work; / FS sync delayed work,
    pub /: *mut *mut spinlock_t work_lock; / protects sync_work and work_queued,
    pub s_lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_inode_info {
    pub i_data: [__fs32; 15],
    pub 15]: *mut *mut *mut __u8 i_symlink[2  4,
    pub u2_i_data: [__fs64; 15],
    pub i_u1: },
    pub i_flags: __u32,
    pub i_shadow: __u32,
    pub i_unused1: __u32,
    pub i_unused2: __u32,
    pub i_oeftflag: __u32,
    pub i_osync: __u16,
    pub i_lastfrag: __u64,
    pub meta_lock: seqlock_t,
    pub truncate_mutex: mutex,
    pub i_dir_start_lookup: __u32,
    pub vfs_inode: inode,
}

// mount options
pub const UFS_MOUNT_ONERROR_PANIC: c_uint = 0x00000001;
pub const UFS_MOUNT_ONERROR_LOCK: c_uint = 0x00000002;
pub const UFS_MOUNT_ONERROR_UMOUNT: c_uint = 0x00000004;
pub const UFS_MOUNT_ONERROR_REPAIR: c_uint = 0x00000008;
pub const UFS_MOUNT_UFSTYPE_OLD: c_uint = 0x00000010;
pub const UFS_MOUNT_UFSTYPE_44BSD: c_uint = 0x00000020;
pub const UFS_MOUNT_UFSTYPE_SUN: c_uint = 0x00000040;
pub const UFS_MOUNT_UFSTYPE_NEXTSTEP: c_uint = 0x00000080;
pub const UFS_MOUNT_UFSTYPE_NEXTSTEP_CD: c_uint = 0x00000100;
pub const UFS_MOUNT_UFSTYPE_OPENSTEP: c_uint = 0x00000200;
pub const UFS_MOUNT_UFSTYPE_SUNx86: c_uint = 0x00000400;
pub const UFS_MOUNT_UFSTYPE_HP: c_uint = 0x00000800;
pub const UFS_MOUNT_UFSTYPE_UFS2: c_uint = 0x00001000;
pub const UFS_MOUNT_UFSTYPE_SUNOS: c_uint = 0x00002000;
//
// Debug code
//

// balloc.c
extern "C" {
    pub fn ufs_free_fragments(: *mut inode, fragment: u64, count: unsigned);
}
extern "C" {
    pub fn ufs_free_blocks(: *mut inode, fragment: u64, count: unsigned);
}
// cylinder.c
extern "C" {
    pub fn ufs_load_cylinder(: *mut super_block, _arg: unsigned) -> *mut ufs_cg_private_info;
}
extern "C" {
    pub fn ufs_put_cylinder(: *mut super_block, _arg: unsigned);
}
// dir.c
extern "C" {
    pub fn ufs_add_link(: *mut dentry, : *mut inode) -> c_int;
}
extern "C" {
    pub fn ufs_inode_by_name(: *mut inode, : *const qstr) -> ino_t;
}
extern "C" {
    pub fn ufs_make_empty(: *mut inode, : *mut inode) -> c_int;
}
extern "C" {
    pub fn ufs_delete_entry(: *mut inode, : *mut ufs_dir_entry, : *mut folio) -> c_int;
}
extern "C" {
    pub fn ufs_empty_dir(: *mut inode) -> c_int;
}
// file.c
// ialloc.c
extern "C" {
    pub fn ufs_free_inode(inode: *mut inode);
}
extern "C" {
    pub fn ufs_new_inode(: *mut inode, _arg: umode_t) -> *mut inode;
}
// inode.c
extern "C" {
    pub fn ufs_write_inode(: *mut inode, : *mut writeback_control) -> c_int;
}
extern "C" {
    pub fn ufs_sync_inode(: *mut inode) -> c_int;
}
extern "C" {
    pub fn ufs_evict_inode(: *mut inode);
}
// namei.c
// super.c
extern "C" {
    pub fn ufs_warning(: *mut super_block, : *const c_char, : *const c_char, ...);
}
extern "C" {
    pub fn ufs_error(: *mut super_block, : *const c_char, : *const c_char, ...);
}
extern "C" {
    pub fn ufs_panic(: *mut super_block, : *const c_char, : *const c_char, ...);
}
extern "C" {
    pub fn ufs_mark_sb_dirty(sb: *mut super_block);
}
extern "C" {
    pub fn container_of(_arg: inode, ufs_inode_info: struct, _arg: vfs_inode) -> return;
}
//
// Give cylinder group number for a file system block.
// Give cylinder group block number for a file system block.
//
// #define	ufs_dtog(d)	((d) / uspi->s_fpg)
// #define	ufs_dtogd(d)	((d) % uspi->s_fpg)
extern "C" {
    pub fn do_div(_arg: b, _arg: uspi->s_fpg) -> return;
}
