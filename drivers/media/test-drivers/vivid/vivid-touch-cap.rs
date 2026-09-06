//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/vivid/vivid-touch-cap.h
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
// vivid-touch-cap.h - touch support functions.
//
pub const VIVID_TCH_HEIGHT: c_int = 12;
pub const VIVID_TCH_WIDTH: c_int = 21;
pub const VIVID_MIN_PRESSURE: c_int = 180;
pub const VIVID_PRESSURE_LIMIT: c_int = 40;
pub const TCH_SEQ_COUNT: c_int = 16;
pub const TCH_PATTERN_COUNT: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vivid_tch_test {
    SINGLE_TAP,
    DOUBLE_TAP,
    TRIPLE_TAP,
    MOVE_LEFT_TO_RIGHT,
    ZOOM_IN,
    ZOOM_OUT,
    PALM_PRESS,
    MULTIPLE_PRESS,
    TEST_CASE_MAX
}

extern "C" {
    pub fn vivid_enum_fmt_tch(file: *mut file, priv: *mut c_void, f: *mut v4l2_fmtdesc) -> c_int;
}
extern "C" {
    pub fn vivid_g_fmt_tch(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vivid_g_fmt_tch_mplane(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vivid_enum_input_tch(file: *mut file, priv: *mut c_void, inp: *mut v4l2_input) -> c_int;
}
extern "C" {
    pub fn vivid_g_input_tch(file: *mut file, priv: *mut c_void, i: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn vivid_s_input_tch(file: *mut file, priv: *mut c_void, i: c_uint) -> c_int;
}
extern "C" {
    pub fn vivid_fillbuff_tch(dev: *mut vivid_dev, buf: *mut vivid_buffer);
}
extern "C" {
    pub fn vivid_set_touch(dev: *mut vivid_dev, i: c_uint) -> c_int;
}
