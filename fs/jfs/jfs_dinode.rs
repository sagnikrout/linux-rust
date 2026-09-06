//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jfs/jfs_dinode.h
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
// Copyright (C) International Business Machines Corp., 2000-2001
//
// jfs_dinode.h: on-disk inode manager
//
pub const INODESLOTSIZE: c_int = 128;
pub const L2INODESLOTSIZE: c_int = 7;

//
// on-disk inode : 512 bytes
//
// note: align 64-bit fields on 8-byte boundary.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dinode {
//
// I. base area (128 bytes)
// ------------------------
//
// define generic/POSIX attributes
//
    pub /: *mut *mut __le32 di_inostamp; / 4: stamp to show inode belongs to fileset,
    pub /: *mut *mut __le32 di_fileset; / 4: fileset number,
    pub /: *mut *mut __le32 di_number; / 4: inode number, aka file serial number,
    pub /: *mut *mut __le32 di_gen; / 4: inode generation number,
    pub /: *mut *mut pxd_t di_ixpxd; / 8: inode extent descriptor,
    pub /: *mut *mut __le64 di_size; / 8: size,
    pub /: *mut *mut __le64 di_nblocks; / 8: number of blocks allocated,
    pub /: *mut *mut __le32 di_nlink; / 4: number of links to the object,
    pub /: *mut *mut __le32 di_uid; / 4: user id of owner,
    pub /: *mut *mut __le32 di_gid; / 4: group id of owner,
    pub /: *mut *mut __le32 di_mode; / 4: attribute, format and permission,
    pub /: *mut *mut timestruc_t di_atime; / 8: time last data accessed,
    pub /: *mut *mut timestruc_t di_ctime; / 8: time last status changed,
    pub /: *mut *mut timestruc_t di_mtime; / 8: time last data modified,
    pub /: *mut *mut timestruc_t di_otime; / 8: time created,
    pub /: *mut *mut dxd_t di_acl; / 16: acl descriptor,
    pub /: *mut *mut dxd_t di_ea; / 16: ea descriptor,
    pub /: *mut *mut __le32 di_next_index; / 4: Next available dir_table index,
    pub /: *mut *mut __le32 di_acltype; / 4: Type of ACL,
//
// Extension Areas.
//
// Historically, the inode was partitioned into 4 128-byte areas,
// the last 3 being defined as unions which could have multiple
// uses.  The first 96 bytes had been completely unused until
// an index table was added to the directory.  It is now more
// useful to describe the last 3/4 of the inode as a single
// union.  We would probably be better off redesigning the
// entire structure from scratch, but we don't want to break
// commonality with OS/2's JFS at this time.
//
// This table contains the information needed to
// find a directory entry from a 32-bit index.
// If the index is small enough, the table is inline,
// otherwise, an x-tree root overlays this table
//
    pub /: *mut *mut dir_table_slot _table[12]; / 96: inline,
    pub /: *mut *mut dtroot_t _dtroot; / 288: dtree root,
    pub /: *mut *mut } _dir; / (384),

    pub /: *mut *mut u8 _data[96]; / 96: unused,
    pub /: *mut *mut *mut void _imap; / 4: unused,
    pub /: *mut *mut __le32 _gengen; / 4: generator,
    pub _imap: },
    pub /: *mut *mut } _u1; / 96:,

    pub _xtroot: xtroot_t,
    pub /: *mut *mut u8 unused[16]; / 16:,
    pub /: *mut *mut dxd_t _dxd; / 16:,
//
// The fast symlink area
// is expected to overflow
// into _inlineea when
// needed (which will clear
// INLINEEA).
//
    pub /: *mut *mut __le32 _rdev; / 4:,
    pub _fastsymlink: [u8; 128],
    pub _u: },
    pub _inlineea: [u8; 128],
}

// extended mode bits (on-disk inode di_mode)
pub const IFJOURNAL: c_uint = 0x00010000	/* journalled file */;
pub const ISPARSE: c_uint = 0x00020000	/* sparse file enabled */;
pub const INLINEEA: c_uint = 0x00040000	/* inline EA area free */;
pub const ISWAPFILE: c_uint = 0x00800000	/* file open for pager swap space */;
// more extended mode bits: attributes for OS/2
pub const IREADONLY: c_uint = 0x02000000	/* no write access to file */;
pub const IHIDDEN: c_uint = 0x04000000	/* hidden file */;
pub const ISYSTEM: c_uint = 0x08000000	/* system file */;
pub const IDIRECTORY: c_uint = 0x20000000	/* directory (shadow of real bit) */;
pub const IARCHIVE: c_uint = 0x40000000	/* file archive bit */;
pub const INEWNAME: c_uint = 0x80000000	/* non-8.3 filename format */;
pub const IRASH: c_uint = 0x4E000000	/* mask for changeable attributes */;

// extended attributes for Linux
pub const JFS_NOATIME_FL: c_uint = 0x00080000 /* do not update atime */;
pub const JFS_DIRSYNC_FL: c_uint = 0x00100000 /* dirsync behaviour */;
pub const JFS_SYNC_FL: c_uint = 0x00200000 /* Synchronous updates */;
pub const JFS_SECRM_FL: c_uint = 0x00400000 /* Secure deletion */;
pub const JFS_UNRM_FL: c_uint = 0x00800000 /* allow for undelete */;
pub const JFS_APPEND_FL: c_uint = 0x01000000 /* writes to file may only append */;
pub const JFS_IMMUTABLE_FL: c_uint = 0x02000000 /* Immutable file */;
pub const JFS_FL_USER_VISIBLE: c_uint = 0x03F80000;
pub const JFS_FL_USER_MODIFIABLE: c_uint = 0x03F80000;
pub const JFS_FL_INHERIT: c_uint = 0x03C80000;
