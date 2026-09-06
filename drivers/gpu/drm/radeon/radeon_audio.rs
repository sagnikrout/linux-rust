//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/radeon_audio.h
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
// Copyright 2014 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: Slava Grigorev <slava.grigorev@amd.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_audio_basic_funcs {
    pub reg): *mut *mut *mut u32 (endpoint_rreg)(struct radeon_device rdev, u32 offset, u32,
    pub v): u32 offset, u32 reg, u32,
    pub enable_mask): *mut *mut r600_audio_pin pin, u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_audio_funcs {
    pub encoder): *mut *mut void (select_pin)(struct drm_encoder,
    pub rdev): *mut *mut *mut r600_audio_pin (get_pin)(radeon_device,
    pub mode): *mut *mut drm_connector connector, drm_display_mode,
    pub sad_count): *mut *mut cea_sad sads, int,
    pub sad_count): *mut *mut u8 sadb, int,
    pub clock): *mut *mut radeon_crtc crtc, unsigned int,
    pub acr): *const radeon_hdmi_acr,
    pub offset): *mut *mut *mut void (set_vbi_packet)(struct drm_encoder encoder, u32,
    pub bpc): *mut *mut *mut void (set_color_depth)(struct drm_encoder encoder, u32 offset, int,
    pub size): *mut *mut unsigned char buffer, size_t,
    pub offset): *mut *mut *mut void (set_audio_packet)(struct drm_encoder encoder, u32,
    pub mute): *mut *mut *mut void (set_mute)(struct drm_encoder encoder, u32 offset, bool,
    pub mode): *mut drm_display_mode,
    pub mode): *mut *mut *mut void (dpms)(struct drm_encoder encoder, bool,
}

extern "C" {
    pub fn radeon_audio_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_audio_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_audio_dpms(encoder: *mut drm_encoder, mode: c_int);
}
extern "C" {
    pub fn radeon_audio_decode_dfs_div(div: c_uint) -> c_uint;
}
extern "C" {
    pub fn dce3_2_set_audio_packet(encoder: *mut drm_encoder, offset: u32);
}
extern "C" {
    pub fn dce3_2_set_mute(encoder: *mut drm_encoder, offset: u32, mute: bool);
}
