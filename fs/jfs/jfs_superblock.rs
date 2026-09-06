//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jfs/jfs_superblock.h
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
// Copyright (C) International Business Machines Corp., 2000-2003
//

//
// make the magic number something a human could read
//

//
// aggregate superblock
//
// The name superblock is too close to super_block, so the name has been
// changed to jfs_superblock.  The utilities are still using the old name.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jfs_superblock {
    pub /: *mut *mut char s_magic[4]; / 4: magic number,
    pub /: *mut *mut __le32 s_version; / 4: version number,
    pub blocks: *mut *mut __le64 s_size; / 8: aggregate size in hardware/LVM,
// VFS: number of blocks
//
    pub bytes: *mut *mut __le32 s_bsize; / 4: aggregate block size in,
// VFS: fragment size
//
    pub /: *mut *mut __le16 s_l2bsize; / 2: log2 of s_bsize,
    pub /: *mut *mut __le16 s_l2bfactor; / 2: log2(s_bsize/hardware block size),
    pub /: *mut *mut __le32 s_pbsize; / 4: hardware/LVM block size in bytes,
    pub /: *mut *mut __le16 s_l2pbsize; / 2: log2 of s_pbsize,
    pub /: *mut *mut __le16 pad; / 2: padding necessary for alignment,
    pub /: *mut *mut __le32 s_agsize; / 4: allocation group size in aggr. blocks,
    pub attributes:: *mut *mut __le32 s_flag; / 4: aggregate,
// see jfs_filsys.h
//
    pub state:: *mut *mut __le32 s_state; / 4: mount/unmount/recovery,
// see jfs_filsys.h
//
    pub /: *mut *mut __le32 s_compress; / 4: > 0 if data compression,
    pub secondary: *mut *mut pxd_t s_ait2; / 8: first extent of,
// aggregate inode table
//
    pub secondary: *mut *mut pxd_t s_aim2; / 8: first extent of,
// aggregate inode map
//
    pub /: *mut *mut __le32 s_logdev; / 4: device address of log,
    pub /: *mut *mut __le32 s_logserial; / 4: log serial number at aggregate mount,
    pub /: *mut *mut pxd_t s_logpxd; / 8: inline log extent,
    pub /: *mut *mut pxd_t s_fsckpxd; / 8: inline fsck work space extent,
    pub /: *mut *mut timestruc_t s_time; / 8: time last updated,
    pub for: *mut *mut __le32 s_fsckloglen; / 4: Number of filesystem blocks reserved,
// the fsck service log.
// N.B. These blocks are divided among the
// versions kept.  This is not a per
// version size.
// N.B. These blocks are included in the
// length field of s_fsckpxd.
//
    pub recent: *mut *mut s8 s_fscklog; / 1: which fsck service log is most,
// 0 => no service log data yet
// 1 => the first one
// 2 => the 2nd one
//
    pub name: *mut *mut char s_fpack[11]; / 11: file system volume,
// N.B. This must be 11 bytes to
// conform with the OS/2 BootSector
// requirements
// Only used when s_version is 1
//
// extendfs() parameter under s_state & FM_EXTENDFS
    pub /: *mut *mut __le64 s_xsize; / 8: extendfs s_size,
    pub /: *mut *mut pxd_t s_xfsckpxd; / 8: extendfs fsckpxd,
    pub /: *mut *mut pxd_t s_xlogpxd; / 8: extendfs logpxd,
    pub /: *mut *mut uuid_t s_uuid; / 16: 128-bit uuid for volume,
    pub /: *mut *mut char s_label[16]; / 16: volume label,
    pub /: *mut *mut uuid_t s_loguuid; / 16: 128-bit uuid for log device,
}

extern "C" {
    pub fn readSuper(: *mut super_block, : *mut buffer_head) -> c_int;
}
extern "C" {
    pub fn updateSuper(: *mut super_block, _arg: c_uint) -> c_int;
}
extern "C" {
    pub fn jfs_error(: *mut super_block, : *const c_char, ...);
}
extern "C" {
    pub fn jfs_mount(: *mut super_block) -> c_int;
}
extern "C" {
    pub fn jfs_mount_rw(: *mut super_block, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn jfs_umount(: *mut super_block) -> c_int;
}
extern "C" {
    pub fn jfs_umount_rw(: *mut super_block) -> c_int;
}
extern "C" {
    pub fn jfs_extendfs(: *mut super_block, _arg: i64, _arg: c_int) -> c_int;
}
