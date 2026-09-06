//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/dlm_plock.h
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
// Copyright (C) 2005-2008 Red Hat, Inc.  All rights reserved.
//
// This copyrighted material is made available to anyone wishing to use,
// modify, copy, or redistribute it subject to the terms and conditions
// of the GNU General Public License v.2.
//

pub const DLM_PLOCK_VERSION_MAJOR: c_int = 1;
pub const DLM_PLOCK_VERSION_MINOR: c_int = 2;
pub const DLM_PLOCK_VERSION_PATCH: c_int = 0;
pub const DLM_PLOCK_FL_CLOSE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_plock_info {
    pub version: [__u32; 3],
    pub optype: __u8,
    pub ex: __u8,
    pub wait: __u8,
    pub flags: __u8,
    pub pid: __u32,
    pub nodeid: __s32,
    pub rv: __s32,
    pub fsid: __u32,
    pub number: __u64,
    pub start: __u64,
    pub end: __u64,
    pub owner: __u64,
}
