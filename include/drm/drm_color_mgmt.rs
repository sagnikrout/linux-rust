//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_color_mgmt.h
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


//
// Copyright (c) 2016 Intel Corporation
//
// Permission to use, copy, modify, distribute, and sell this software and its
// documentation for any purpose is hereby granted without fee, provided that
// the above copyright notice appear in all copies and that both that copyright
// notice and this permission notice appear in supporting documentation, and
// that the name of the copyright holders not be used in advertising or
// publicity pertaining to distribution of the software without specific,
// written prior permission.  The copyright holders make no representations
// about the suitability of this software for any purpose.  It is provided "as
// is" without express or implied warranty.
//
// THE COPYRIGHT HOLDERS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
// INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
// EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
// CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
// DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
// TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
// OF THIS SOFTWARE.
//

//
// drm_color_lut_extract - clamp and round LUT entries
// @user_input: input value
// @bit_precision: number of bits the hw LUT supports
//
// Extract a degamma/gamma LUT value provided by user (in the form of
// &drm_color_lut entries) and round it to the precision supported by the
// hardware, following OpenGL int<->float conversion rules
// (see eg. OpenGL 4.6 specification - 2.3.5 Fixed-Point Data Conversions).
//
// drm_color_lut32_extract - clamp and round LUT entries
// @user_input: input value
// @bit_precision: number of bits the hw LUT supports
//
// Extract U0.bit_precision from a U0.32 LUT value.
//
extern "C" {
    pub fn drm_color_ctm_s31_32_to_qm_n(user_input: u64, m: u32, n: u32) -> u64;
}
//
// drm_color_lut_size - calculate the number of entries in the LUT
// @blob: blob containing the LUT
//
// Returns:
// The number of entries in the color LUT stored in @blob.
//
// drm_color_lut32_size - calculate the number of entries in the extended LUT
// @blob: blob containing the LUT
//
// Returns:
// The number of entries in the color LUT stored in @blob.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_color_encoding {
    DRM_COLOR_YCBCR_BT601,
    DRM_COLOR_YCBCR_BT709,
    DRM_COLOR_YCBCR_BT2020,
    DRM_COLOR_ENCODING_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_color_range {
    DRM_COLOR_YCBCR_LIMITED_RANGE,
    DRM_COLOR_YCBCR_FULL_RANGE,
    DRM_COLOR_RANGE_MAX,
}

//
// enum drm_color_lut_tests - hw-specific LUT tests to perform
//
// The drm_color_lut_check() function takes a bitmask of the values here to
// determine which tests to apply to a userspace-provided LUT.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_color_lut_tests {
//
// @DRM_COLOR_LUT_EQUAL_CHANNELS:
//
// Checks whether the entries of a LUT all have equal values for the
// red, green, and blue channels.  Intended for hardware that only
// accepts a single value per LUT entry and assumes that value applies
// to all three color components.
//
    DRM_COLOR_LUT_EQUAL_CHANNELS = BIT(0),

//
// @DRM_COLOR_LUT_NON_DECREASING:
//
// Checks whether the entries of a LUT are always flat or increasing
// (never decreasing).
//
    DRM_COLOR_LUT_NON_DECREASING = BIT(1),
}

extern "C" {
    pub fn drm_color_lut_check(lut: *const drm_property_blob, tests: u32) -> c_int;
}
//
// Gamma-LUT programming
//
extern "C" {
    pub fn void(: *mut *mut drm_crtc_set_lut_func)(struct drm_crtc, int: unsigned, _arg: u16, _arg: u16, _arg: u16) -> typedef;
}
extern "C" {
    pub fn drm_crtc_fill_gamma_888(crtc: *mut drm_crtc, set_gamma: drm_crtc_set_lut_func);
}
extern "C" {
    pub fn drm_crtc_fill_gamma_565(crtc: *mut drm_crtc, set_gamma: drm_crtc_set_lut_func);
}
extern "C" {
    pub fn drm_crtc_fill_gamma_555(crtc: *mut drm_crtc, set_gamma: drm_crtc_set_lut_func);
}
//
// Color-LUT programming
//
extern "C" {
    pub fn drm_crtc_fill_palette_332(crtc: *mut drm_crtc, set_palette: drm_crtc_set_lut_func);
}
extern "C" {
    pub fn drm_crtc_fill_palette_8(crtc: *mut drm_crtc, set_palette: drm_crtc_set_lut_func);
}
extern "C" {
    pub fn drm_color_lut32_check(lut: *const drm_property_blob, tests: u32) -> c_int;
}
