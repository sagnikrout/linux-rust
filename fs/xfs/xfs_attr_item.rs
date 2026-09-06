//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_attr_item.h
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
// Copyright (C) 2022 Oracle.  All Rights Reserved.
// Author: Allison Henderson <allison.henderson@oracle.com>
//
// kernel only ATTRI/ATTRD definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_attri_log_nameval {
    pub name: kvec,
    pub /: *mut *mut kvec new_name; / PPTR_REPLACE only,
    pub value: kvec,
    pub /: *mut *mut kvec new_value; / PPTR_REPLACE only,
    pub refcount: refcount_t,
// name and value follow the end of this struct
}

//
// This is the "attr intention" log item.  It is used to log the fact that some
// extended attribute operations need to be processed.  An operation is
// currently either a set or remove.  Set or remove operations are described by
// the xfs_attr_intent which may be logged to this intent.
//
// During a normal attr operation, name and value point to the name and value
// fields of the caller's xfs_da_args structure.  During a recovery, the name
// and value buffers are copied from the log, and stored in a trailing buffer
// attached to the xfs_attr_intent until they are committed.  They are freed
// when the xfs_attr_intent itself is freed when the work is done.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_attri_log_item {
    pub attri_item: xfs_log_item,
    pub attri_refcount: core::sync::atomic::AtomicI32,
    pub attri_nameval: *mut xfs_attri_log_nameval,
    pub attri_format: xfs_attri_log_format,
}

//
// This is the "attr done" log item.  It is used to log the fact that some attrs
// earlier mentioned in an attri item have been freed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_attrd_log_item {
    pub attrd_item: xfs_log_item,
    pub attrd_attrip: *mut xfs_attri_log_item,
    pub attrd_format: xfs_attrd_log_format,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_attr_defer_op {
    XFS_ATTR_DEFER_SET,
    XFS_ATTR_DEFER_REMOVE,
    XFS_ATTR_DEFER_REPLACE,
}

extern "C" {
    pub fn xfs_attr_defer_add(args: *mut xfs_da_args, op: xfs_attr_defer_op);
}
