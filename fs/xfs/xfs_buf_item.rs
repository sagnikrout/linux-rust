//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_buf_item.h
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
// Copyright (c) 2000-2001,2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
// kernel only definitions
// buf log item flags

//
// This is the in core log item structure used to track information
// needed to log buffers.  It tracks how many times the lock has been
// locked, and which 128 byte chunks of the buffer are dirty.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_buf_log_item {
    pub /: *mut *mut xfs_log_item bli_item; / common item structure,
    pub /: *mut *mut *mut xfs_buf bli_buf; / real buffer pointer,
    pub /: *mut *mut unsigned int bli_flags; / misc flags,
    pub /: *mut *mut unsigned int bli_recur; / lock recursion count,
    pub /: *mut *mut atomic_t bli_refcount; / cnt of tp refs,
    pub /: *mut *mut int bli_format_count; / count of headers,
    pub /: *mut *mut *mut xfs_buf_log_format bli_formats; / array of in-log header ptrs,
    pub /: *mut *mut xfs_buf_log_format __bli_format; / embedded in-log header,
}

extern "C" {
    pub fn xfs_buf_item_init(: *mut xfs_buf, : *mut xfs_mount) -> c_int;
}
extern "C" {
    pub fn xfs_buf_item_done(bp: *mut xfs_buf);
}
extern "C" {
    pub fn xfs_buf_item_put(bip: *mut xfs_buf_log_item);
}
extern "C" {
    pub fn xfs_buf_item_log(: *mut xfs_buf_log_item, _arg: c_uint, _arg: c_uint);
}
extern "C" {
    pub fn xfs_buf_item_dirty_format(: *mut xfs_buf_log_item) -> bool;
}
extern "C" {
    pub fn xfs_buf_inode_iodone(: *mut xfs_buf);
}

extern "C" {
    pub fn xfs_buf_dquot_iodone(: *mut xfs_buf);
}

extern "C" {
    pub fn xfs_buf_iodone(: *mut xfs_buf);
}
extern "C" {
    pub fn xfs_buf_log_check_iovec(iovec: *mut kvec) -> bool;
}
