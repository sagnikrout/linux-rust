//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/fiemap.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// FS_IOC_FIEMAP ioctl infrastructure.
//
// Some portions copyright (C) 2007 Cluster File Systems, Inc
//
// Authors: Mark Fasheh <mfasheh@suse.com>
// Kalpak Shah <kalpak.shah@sun.com>
// Andreas Dilger <adilger@sun.com>
//

//
// struct fiemap_extent - description of one fiemap extent
// @fe_logical: byte offset of the extent in the file
// @fe_physical: byte offset of extent on disk
// @fe_length: length in bytes for this extent
// @fe_flags: FIEMAP_EXTENT_* flags for this extent
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fiemap_extent {
    pub fe_logical: __u64,
    pub fe_physical: __u64,
    pub fe_length: __u64,
// private:
    pub fe_reserved64: [__u64; 2],
// public:
    pub fe_flags: __u32,
// private:
    pub fe_reserved: [__u32; 3],
}

//
// struct fiemap - file extent mappings
// @fm_start: byte offset (inclusive) at which to start mapping (in)
// @fm_length: logical length of mapping which userspace wants (in)
// @fm_flags: FIEMAP_FLAG_* flags for request (in/out)
// @fm_mapped_extents: number of extents that were mapped (out)
// @fm_extent_count: size of fm_extents array (in)
// @fm_extents: array of mapped extents (out)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fiemap {
    pub fm_start: __u64,
    pub fm_length: __u64,
    pub fm_flags: __u32,
    pub fm_mapped_extents: __u32,
    pub fm_extent_count: __u32,
// private:
    pub fm_reserved: __u32,
// public:
    pub fm_extents: [fiemap_extent; ],
}

// flags used in fm_flags:
pub const FIEMAP_FLAG_SYNC: c_uint = 0x00000001 /* sync file data before map */;
pub const FIEMAP_FLAG_XATTR: c_uint = 0x00000002 /* map extended attribute tree */;
pub const FIEMAP_FLAG_CACHE: c_uint = 0x00000004 /* request caching of the extents */;

// flags used in fe_flags:
pub const FIEMAP_EXTENT_LAST: c_uint = 0x00000001 /* Last extent in file. */;
pub const FIEMAP_EXTENT_UNKNOWN: c_uint = 0x00000002 /* Data location unknown. */;
pub const FIEMAP_EXTENT_DELALLOC: c_uint = 0x00000004 /* Location still pending.;
// Sets EXTENT_UNKNOWN.
pub const FIEMAP_EXTENT_ENCODED: c_uint = 0x00000008 /* Data can not be read;
// while fs is unmounted
pub const FIEMAP_EXTENT_DATA_ENCRYPTED: c_uint = 0x00000080 /* Data is encrypted by fs.;
// Sets EXTENT_NO_BYPASS.
pub const FIEMAP_EXTENT_NOT_ALIGNED: c_uint = 0x00000100 /* Extent offsets may not be;
// block aligned.
pub const FIEMAP_EXTENT_DATA_INLINE: c_uint = 0x00000200 /* Data mixed with metadata.;
// Sets EXTENT_NOT_ALIGNED.
pub const FIEMAP_EXTENT_DATA_TAIL: c_uint = 0x00000400 /* Multiple files in block.;
// Sets EXTENT_NOT_ALIGNED.
pub const FIEMAP_EXTENT_UNWRITTEN: c_uint = 0x00000800 /* Space allocated, but;
// no data (i.e. zero).
pub const FIEMAP_EXTENT_MERGED: c_uint = 0x00001000 /* File does not natively;
// support extents. Result
// merged for efficiency.
pub const FIEMAP_EXTENT_SHARED: c_uint = 0x00002000 /* Space shared with other;
// files.
