//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/erspan.h
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
// ERSPAN Tunnel Metadata
//
// Copyright (c) 2018 VMware
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2
// as published by the Free Software Foundation.
//
// Userspace API for metadata mode ERSPAN tunnel
//

// ERSPAN version 2 metadata header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erspan_md2 {
    pub timestamp: __be32,
    pub /: *mut *mut __be16 sgt; / security group tag,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erspan_metadata {
    pub version: c_int,
    pub II)*/: *mut *mut __be32 index; / Version 1 (type,
    pub /: *mut *mut erspan_md2 md2; / Version 2 (type III),
    pub u: },
}
