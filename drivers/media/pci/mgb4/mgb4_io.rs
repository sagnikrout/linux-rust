//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/mgb4/mgb4_io.h
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
// Copyright (C) 2021-2022 Digiteq Automotive
// author: Martin Tuma <martin.tuma@digiteqautomotive.com>
//

// Register access error indication
pub const MGB4_ERR_NO_REG: c_uint = 0xFFFFFFFE;
// Frame buffer addresses greater than 0xFFFFFFFA indicate HW errors
pub const MGB4_ERR_QUEUE_TIMEOUT: c_uint = 0xFFFFFFFD;
pub const MGB4_ERR_QUEUE_EMPTY: c_uint = 0xFFFFFFFC;
pub const MGB4_ERR_QUEUE_FULL: c_uint = 0xFFFFFFFB;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgb4_frame_buffer {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
}

extern "C" {
    pub fn container_of(_arg: vbuf, mgb4_frame_buffer: struct, _arg: vb) -> return;
}

