//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vkms/vkms_composer.h
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
// This enum is related to the positions of the variables inside
// `struct drm_color_lut`, so the order of both needs to be the same.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lut_channel {
    LUT_RED = 0,
    LUT_GREEN,
    LUT_BLUE,
    LUT_RESERVED
}

extern "C" {
    pub fn lerp_u16(a: u16, b: u16, t: i64) -> u16;
}
extern "C" {
    pub fn get_lut_index(lut: *const vkms_color_lut, channel_value: u16) -> i64;
}
extern "C" {
    pub fn apply_3x4_matrix(pixel: *mut pixel_argb_s32, matrix: *const drm_color_ctm_3x4);
}

