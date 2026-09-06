//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ext4/fsmap.h
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
// Copyright (C) 2017 Oracle.  All Rights Reserved.
//
// Author: Darrick J. Wong <darrick.wong@oracle.com>
//
// internal fsmap representation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_fsmap {
    pub fmr_list: list_head,
    pub /: *mut *mut dev_t fmr_device; / device id,
    pub /: *mut *mut uint32_t fmr_flags; / mapping flags,
    pub /: *mut *mut uint64_t fmr_physical; / device offset of segment,
    pub /: *mut *mut uint64_t fmr_owner; / owner id,
    pub /: *mut *mut uint64_t fmr_length; / length of segment, blocks,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_fsmap_head {
    pub /: *mut *mut uint32_t fmh_iflags; / control flags,
    pub /: *mut *mut uint32_t fmh_oflags; / output flags,
    pub /: *mut *mut unsigned int fmh_count; / # of entries in array incl. input,
    pub /: *mut *mut unsigned int fmh_entries; / # of entries filled in (output).,
    pub /: *mut *mut ext4_fsmap fmh_keys[2]; / low and high keys,
}

// fsmap to userspace formatter - copy to user & advance pointer
extern "C" {
    pub fn int(: *mut *mut ext4_fsmap_format_t)(struct ext4_fsmap, : *mut c_void) -> typedef;
}
pub const EXT4_QUERY_RANGE_ABORT: c_int = 1;
pub const EXT4_QUERY_RANGE_CONTINUE: c_int = 0;
// fmr_owner special values for FS_IOC_GETFSMAP; some share w/ XFS

