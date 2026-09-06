//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/screen_info.h
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

//
// SCREEN_INFO_MAX_RESOURCES - maximum number of resources per screen_info
//
pub const SCREEN_INFO_MAX_RESOURCES: c_int = 3;
//
// VESA modes typically run on VGA hardware. Set bit 5 signals that this
// is not the case. Drivers can then not make use of VGA resources. See
// Sec 4.4 of the VBE 2.0 spec.
//
// screen_info_video_type() - Decodes the video type from struct screen_info
// @si: an instance of struct screen_info
//
// Returns:
// A VIDEO_TYPE_ constant representing si's type of video display, or 0 otherwise.
//
// check if display output is on
// check for a known VIDEO_TYPE_ constant
// check if text mode has been initialized
// 80x25 text, mono
// EGA/VGA, 16 colors
// the rest...
extern "C" {
    pub fn screen_info_resources(si: *const screen_info, r: *mut resource, num: usize) -> isize;
}
extern "C" {
    pub fn __screen_info_lfb_bits_per_pixel(si: *const screen_info) -> u32;
}
extern "C" {
    pub fn screen_info_pixel_format(si: *const screen_info, f: *mut pixel_format) -> c_int;
}

extern "C" {
    pub fn screen_info_apply_fixups();
}

