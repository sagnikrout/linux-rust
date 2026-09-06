//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/integrity.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2009 IBM Corporation
// Author: Mimi Zohar <zohar@us.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum integrity_status {
    INTEGRITY_PASS = 0,
    INTEGRITY_PASS_IMMUTABLE,
    INTEGRITY_FAIL,
    INTEGRITY_FAIL_IMMUTABLE,
    INTEGRITY_NOLABEL,
    INTEGRITY_NOXATTRS,
    INTEGRITY_UNKNOWN,
}

extern "C" {
    pub fn integrity_load_keys() -> void __init;
}

// An inode's attributes for detection of changes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct integrity_inode_attributes {
    pub /: *mut *mut u64 version; / track inode changes,
    pub ino: c_ulong,
    pub dev: dev_t,
}

//
// On stacked filesystems the i_version alone is not enough to detect file data
// or metadata change. Additional metadata is required.
//
// On stacked filesystems detect whether the inode or its content has changed.
//
