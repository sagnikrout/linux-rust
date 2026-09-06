//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_errortag.h
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
// Copyright (c) 2000-2002,2005 Silicon Graphics, Inc.
// Copyright (C) 2017 Oracle.
// All Rights Reserved.
//

//
// There are two ways to use this header file.  The first way is to #include it
// bare, which will define all the XFS_ERRTAG_* error injection knobs for use
// with the XFS_TEST_ERROR macro.  The second way is to enclose the #include
// with a #define for an XFS_ERRTAG macro, in which case the header will define
// defined error injection knob.
//
// These are the actual error injection tags.  The numbers should be consecutive
// because arrays are sized based on the maximum.
//
pub const XFS_ERRTAG_NOERROR: c_int = 0;
pub const XFS_ERRTAG_IFLUSH_1: c_int = 1;
pub const XFS_ERRTAG_IFLUSH_2: c_int = 2;
pub const XFS_ERRTAG_IFLUSH_3: c_int = 3;
pub const XFS_ERRTAG_IFLUSH_4: c_int = 4;
pub const XFS_ERRTAG_IFLUSH_5: c_int = 5;
pub const XFS_ERRTAG_IFLUSH_6: c_int = 6;
pub const XFS_ERRTAG_DA_READ_BUF: c_int = 7;
pub const XFS_ERRTAG_BTREE_CHECK_LBLOCK: c_int = 8;
pub const XFS_ERRTAG_BTREE_CHECK_SBLOCK: c_int = 9;
pub const XFS_ERRTAG_ALLOC_READ_AGF: c_int = 10;
pub const XFS_ERRTAG_IALLOC_READ_AGI: c_int = 11;
pub const XFS_ERRTAG_ITOBP_INOTOBP: c_int = 12;
pub const XFS_ERRTAG_IUNLINK: c_int = 13;
pub const XFS_ERRTAG_IUNLINK_REMOVE: c_int = 14;
pub const XFS_ERRTAG_DIR_INO_VALIDATE: c_int = 15;
pub const XFS_ERRTAG_BULKSTAT_READ_CHUNK: c_int = 16;
pub const XFS_ERRTAG_IODONE_IOERR: c_int = 17;
pub const XFS_ERRTAG_STRATREAD_IOERR: c_int = 18;
pub const XFS_ERRTAG_STRATCMPL_IOERR: c_int = 19;
pub const XFS_ERRTAG_DIOWRITE_IOERR: c_int = 20;
pub const XFS_ERRTAG_BMAPIFORMAT: c_int = 21;
pub const XFS_ERRTAG_FREE_EXTENT: c_int = 22;
pub const XFS_ERRTAG_RMAP_FINISH_ONE: c_int = 23;
pub const XFS_ERRTAG_REFCOUNT_CONTINUE_UPDATE: c_int = 24;
pub const XFS_ERRTAG_REFCOUNT_FINISH_ONE: c_int = 25;
pub const XFS_ERRTAG_BMAP_FINISH_ONE: c_int = 26;
pub const XFS_ERRTAG_AG_RESV_CRITICAL: c_int = 27;
//
// Drop-writes support removed because write error handling cannot trash
// pre-existing delalloc extents in any useful way anymore. We retain the
// definition so that we can reject it as an invalid value in
// xfs_errortag_add().
//
pub const XFS_ERRTAG_DROP_WRITES: c_int = 28;
pub const XFS_ERRTAG_LOG_BAD_CRC: c_int = 29;
pub const XFS_ERRTAG_LOG_ITEM_PIN: c_int = 30;
pub const XFS_ERRTAG_BUF_LRU_REF: c_int = 31;
pub const XFS_ERRTAG_FORCE_SCRUB_REPAIR: c_int = 32;
pub const XFS_ERRTAG_FORCE_SUMMARY_RECALC: c_int = 33;
pub const XFS_ERRTAG_IUNLINK_FALLBACK: c_int = 34;
pub const XFS_ERRTAG_BUF_IOERROR: c_int = 35;
pub const XFS_ERRTAG_REDUCE_MAX_IEXTENTS: c_int = 36;
pub const XFS_ERRTAG_BMAP_ALLOC_MINLEN_EXTENT: c_int = 37;
pub const XFS_ERRTAG_AG_RESV_FAIL: c_int = 38;
pub const XFS_ERRTAG_LARP: c_int = 39;
pub const XFS_ERRTAG_DA_LEAF_SPLIT: c_int = 40;
pub const XFS_ERRTAG_ATTR_LEAF_TO_NODE: c_int = 41;
pub const XFS_ERRTAG_WB_DELAY_MS: c_int = 42;
pub const XFS_ERRTAG_WRITE_DELAY_MS: c_int = 43;
pub const XFS_ERRTAG_EXCHMAPS_FINISH_ONE: c_int = 44;
pub const XFS_ERRTAG_METAFILE_RESV_CRITICAL: c_int = 45;
pub const XFS_ERRTAG_FORCE_ZERO_RANGE: c_int = 46;
pub const XFS_ERRTAG_ZONE_RESET: c_int = 47;
pub const XFS_ERRTAG_MAX: c_int = 48;
//
// Random factors for above tags, 1 means always, 2 means 1/2 time, etc.
//
pub const XFS_RANDOM_DEFAULT: c_int = 100;
//
// Table of errror injection knobs.  The parameters to the XFS_ERRTAG macro are:
// 1. The XFS_ERRTAG_ flag but without the prefix;
// 2. The name of the sysfs knob; and
// 3. The default value for the knob.
//

