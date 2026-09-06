//! Automatically rewritten from C Header to Rust Module
//! Source: fs/quota/quotaio_v2.h
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
// Definitions of structures for vfsv0 quota format
//

//
// Definitions of magics and versions of current quota files
//

// First generic header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v2_disk_dqheader {
    pub /: *mut *mut __le32 dqh_magic; / Magic number identifying file,
    pub /: *mut *mut __le32 dqh_version; / File version,
}

//
// The following structure defines the format of the disk quota file
// (as it appears on disk) - the file is a radix tree whose leaves point
// to blocks of these structures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v2r0_disk_dqblk {
    pub /: *mut *mut __le32 dqb_id; / id this quota applies to,
    pub /: *mut *mut __le32 dqb_ihardlimit; / absolute limit on allocated inodes,
    pub /: *mut *mut __le32 dqb_isoftlimit; / preferred inode limit,
    pub /: *mut *mut __le32 dqb_curinodes; / current # allocated inodes,
    pub /: *mut *mut __le32 dqb_bhardlimit; / absolute limit on disk space (in QUOTABLOCK_SIZE),
    pub /: *mut *mut __le32 dqb_bsoftlimit; / preferred limit on disk space (in QUOTABLOCK_SIZE),
    pub /: *mut *mut __le64 dqb_curspace; / current space occupied (in bytes),
    pub /: *mut *mut __le64 dqb_btime; / time limit for excessive disk use,
    pub /: *mut *mut __le64 dqb_itime; / time limit for excessive inode use,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v2r1_disk_dqblk {
    pub /: *mut *mut __le32 dqb_id; / id this quota applies to,
    pub dqb_pad: __le32,
    pub /: *mut *mut __le64 dqb_ihardlimit; / absolute limit on allocated inodes,
    pub /: *mut *mut __le64 dqb_isoftlimit; / preferred inode limit,
    pub /: *mut *mut __le64 dqb_curinodes; / current # allocated inodes,
    pub /: *mut *mut __le64 dqb_bhardlimit; / absolute limit on disk space (in QUOTABLOCK_SIZE),
    pub /: *mut *mut __le64 dqb_bsoftlimit; / preferred limit on disk space (in QUOTABLOCK_SIZE),
    pub /: *mut *mut __le64 dqb_curspace; / current space occupied (in bytes),
    pub /: *mut *mut __le64 dqb_btime; / time limit for excessive disk use,
    pub /: *mut *mut __le64 dqb_itime; / time limit for excessive inode use,
}

// Header with type and version specific information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v2_disk_dqinfo {
    pub /: *mut *mut __le32 dqi_bgrace; / Time before block soft limit becomes hard limit,
    pub /: *mut *mut __le32 dqi_igrace; / Time before inode soft limit becomes hard limit,
    pub /: *mut *mut *mut __le32 dqi_flags; / Flags for quotafile (DQF_),
    pub /: *mut *mut __le32 dqi_blocks; / Number of blocks in file,
    pub /: *mut *mut __le32 dqi_free_blk; / Number of first free block in the list,
    pub /: *mut *mut __le32 dqi_free_entry; / Number of block with at least one free entry,
}

