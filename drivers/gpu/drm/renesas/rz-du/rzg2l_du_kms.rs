//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/renesas/rz-du/rzg2l_du_kms.h
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
// RZ/G2L Display Unit Mode Setting
//
// Copyright (C) 2023 Renesas Electronics Corporation
//
// Based on rcar_du_kms.h
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzg2l_du_format_info {
    pub fourcc: u32,
    pub v4l2: u32,
    pub planes: c_uint,
    pub hsub: c_uint,
}

extern "C" {
    pub fn rzg2l_du_modeset_init(rcdu: *mut rzg2l_du_device) -> c_int;
}
