//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_ondisk.h
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
// Copyright (c) 2016 Oracle.
// All Rights Reserved.
//

// file structures
// space btrees
// dir/attr trees
// realtime structures
//
// m68k has problems with struct xfs_attr_leaf_name_remote, but we pad
// it to 4 bytes anyway so it's not obviously a problem.  Hence for the
// moment we don't check this structure. This can be re-instated when
// the attr definitions are updated to use c99 VLA definitions.
//
// ondisk dir/attr structures from xfs/122
// log structures
// ondisk log structures from xfs/122
// parent pointer ioctls
//
// The v5 superblock format extended several v4 header structures with
// additional data. While new fields are only accessible on v5
// superblocks, it's important that the v5 structures place original v4
// fields/headers in the correct location on-disk. For example, we must
// be able to find magic values at the same location in certain blocks
// regardless of superblock version.
//
// The following checks ensure that various v5 data structures place the
// subset of v4 metadata associated with the same type of block at the
// start of the on-disk block. If there is no data structure definition
// for certain types of v4 blocks, traverse down to the first field of
// common metadata (e.g., magic value) and make sure it is at offset
// zero.
//
// Make sure the incore inode timestamp range corresponds to hand
// converted values based on the ondisk format specification.
//
// Do the same with the incore quota expiration range.
// superblock field checks we got from xfs/122
//
// ioctl UABI
//
// Due to different padding/alignment requirements across
// different architectures, some structures are ommited from
// the size checks. In addition, structures with architecture
// dependent size fields are also ommited (e.g. __kernel_long_t).
//
