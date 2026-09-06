//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/fsmap.h
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
// FS_IOC_GETFSMAP ioctl infrastructure.
//
// Copyright (C) 2017 Oracle.  All Rights Reserved.
//
// Author: Darrick J. Wong <darrick.wong@oracle.com>
//

//
// Structure for FS_IOC_GETFSMAP.
//
// The memory layout for this call are the scalar values defined in
// struct fsmap_head, followed by two struct fsmap that describe
// the lower and upper bound of mappings to return, followed by an
// array of struct fsmap mappings.
//
// fmh_iflags control the output of the call, whereas fmh_oflags report
// on the overall record output.  fmh_count should be set to the
// length of the fmh_recs array, and fmh_entries will be set to the
// number of entries filled out during each call.  If fmh_count is
// zero, the number of reverse mappings will be returned in
// fmh_entries, though no mappings will be returned.  fmh_reserved
// must be set to zero.
//
// The two elements in the fmh_keys array are used to constrain the
// output.  The first element in the array should represent the
// lowest disk mapping ("low key") that the user wants to learn
// about.  If this value is all zeroes, the filesystem will return
// the first entry it knows about.  For a subsequent call, the
// contents of fsmap_head.fmh_recs[fsmap_head.fmh_count - 1] should be
// copied into fmh_keys[0] to have the kernel start where it left off.
//
// The second element in the fmh_keys array should represent the
// highest disk mapping ("high key") that the user wants to learn
// about.  If this value is all ones, the filesystem will not stop
// until it runs out of mapping to return or runs out of space in
// fmh_recs.
//
// fmr_device can be either a 32-bit cookie representing a device, or
// a 32-bit dev_t if the FMH_OF_DEV_T flag is set.  fmr_physical,
// fmr_offset, and fmr_length are expressed in units of bytes.
// fmr_owner is either an inode number, or a special value if
// FMR_OF_SPECIAL_OWNER is set in fmr_flags.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsmap {
    pub /: *mut *mut __u32 fmr_device; / device id,
    pub /: *mut *mut __u32 fmr_flags; / mapping flags,
    pub /: *mut *mut __u64 fmr_physical; / device offset of segment,
    pub /: *mut *mut __u64 fmr_owner; / owner id,
    pub /: *mut *mut __u64 fmr_offset; / file offset of segment,
    pub /: *mut *mut __u64 fmr_length; / length of segment,
    pub /: *mut *mut __u64 fmr_reserved[3]; / must be zero,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsmap_head {
    pub /: *mut *mut __u32 fmh_iflags; / control flags,
    pub /: *mut *mut __u32 fmh_oflags; / output flags,
    pub /: *mut *mut __u32 fmh_count; / # of entries in array incl. input,
    pub /: *mut *mut __u32 fmh_entries; / # of entries filled in (output).,
    pub /: *mut *mut __u64 fmh_reserved[6]; / must be zero,
    pub /: *mut *mut fsmap fmh_keys[2]; / low and high keys for the mapping search,
    pub /: *mut *mut fsmap fmh_recs[]; / returned records,
}

// Size of an fsmap_head with room for nr records.
extern "C" {
    pub fn sizeof(fsmap: *mut *mut fsmap_head) + nr  sizeof(struct) -> return;
}
// Start the next fsmap query at the end of the current query results.
// fmh_iflags values - set by FS_IOC_GETFSMAP caller in the header.
// no flags defined yet
pub const FMH_IF_VALID: c_int = 0;
// fmh_oflags values - returned in the header segment only.
pub const FMH_OF_DEV_T: c_uint = 0x1	/* fmr_device values will be dev_t */;
// fmr_flags values - returned for each non-header segment
pub const FMR_OF_PREALLOC: c_uint = 0x1	/* segment = unwritten pre-allocation */;
pub const FMR_OF_ATTR_FORK: c_uint = 0x2	/* segment = attribute fork */;
pub const FMR_OF_EXTENT_MAP: c_uint = 0x4	/* segment = extent map */;
pub const FMR_OF_SHARED: c_uint = 0x8	/* segment = shared with another file */;
pub const FMR_OF_SPECIAL_OWNER: c_uint = 0x10	/* owner is a special value */;
pub const FMR_OF_LAST: c_uint = 0x20	/* segment is the last in the dataset */;
// Each FS gets to define its own special owner codes.

