//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/sunxi/sun8i-rotate/sun8i-formats.h
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
// Copyright (C) 2020 Jernej Skrabec <jernej.skrabec@siol.net>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rotate_format {
    pub fourcc: u32,
    pub hw_format: u32,
    pub planes: c_int,
    pub bpp: [c_int; 3],
    pub hsub: c_int,
    pub vsub: c_int,
    pub flags: c_uint,
}

extern "C" {
    pub fn rotate_enum_fmt(f: *mut v4l2_fmtdesc, dst: bool) -> c_int;
}
