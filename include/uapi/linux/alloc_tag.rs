//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/alloc_tag.h
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
// alloc_tag IOCTL API definition
//
// Copyright (C) 2026 Google, LLC.  All rights reserved.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//

//
// Function, file and module names often have the same prefixes, therefore
// when filtering by these criteria, we compare the last 64 characters to
// minimize the chances of name collisions
//
pub const ALLOCINFO_STR_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct allocinfo_content_id {
    pub id: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct allocinfo_tag {
// Longer names are trimmed
    pub modname: [c_char; ALLOCINFO_STR_SIZE],
    pub function: [c_char; ALLOCINFO_STR_SIZE],
    pub filename: [c_char; ALLOCINFO_STR_SIZE],
    pub lineno: __u64,
}

// The alignment ensures 32-bit compatible interfaces are not broken
#[repr(C)]
#[derive(Copy, Clone)]
pub struct allocinfo_counter {
    pub bytes: __u64,
    pub calls: __u64,
    pub accurate: __u8,
    pub __attribute__((aligned(8))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct allocinfo_tag_data {
    pub tag: allocinfo_tag,
    pub counter: allocinfo_counter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct allocinfo_filter {
    pub /: *mut *mut __u64 mask; / bitmask of the filter fields used,
    pub fields: allocinfo_tag,
    pub min_size: __u64,
    pub max_size: __u64,
// filter criteria only; see allocinfo_counter.accurate for actual accuracy
    pub inaccurate: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct allocinfo_get_at {
// inputs
    pub pos: __u64,
    pub filter: allocinfo_filter,
// output
    pub data: allocinfo_tag_data,
}

pub const _ALLOCINFO_IOC_CONTENT_ID: c_int = 0;
pub const _ALLOCINFO_IOC_GET_AT: c_int = 1;
pub const _ALLOCINFO_IOC_GET_NEXT: c_int = 2;
pub const ALLOCINFO_IOC_BASE: c_uint = 0xA6;

