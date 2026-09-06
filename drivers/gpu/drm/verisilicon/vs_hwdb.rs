//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/verisilicon/vs_hwdb.h
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
// Copyright (C) 2025 Icenowy Zheng <uwu@icenowy.me>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vs_formats {
    pub array: *const u32,
    pub num: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vs_chip_identity {
    pub model: u32,
    pub revision: u32,
    pub customer_id: u32,
    pub display_count: u32,
//
// The hardware only supports square cursor planes, so this field
// is both the maximum width and height in pixels.
//
    pub max_cursor_size: i32,
    pub formats: *const vs_formats,
}
