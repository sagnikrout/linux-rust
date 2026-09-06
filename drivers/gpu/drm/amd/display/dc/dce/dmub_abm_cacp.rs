//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dce/dmub_abm_cacp.h
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


// Copyright (C) 2022 Advanced Micro Devices, Inc. All rights reserved.

extern "C" {
    pub fn dmub_cacp_init(abm: *mut abm, src: *const c_char, bytes: c_uint, panel_inst: c_uint);
}
extern "C" {
    pub fn dmub_cacp_set_level(abm: *mut abm, cacp_level: c_uint, panel_mask: c_uchar) -> bool;
}
extern "C" {
    pub fn dmub_cacp_set_pause(abm: *mut abm, pause: bool, panel_inst: c_uint, otg_inst: c_uint) -> bool;
}
extern "C" {
    pub fn dmub_cacp_enable_fractional_pwm(abm: *mut abm, panel_mask: u8);
}
