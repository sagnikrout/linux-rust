//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/display/drm_dp_dual_mode_helper.h
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
// Copyright © 2016 Intel Corporation
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

//
// Optional for type 1 DVI adaptors
// Mandatory for type 1 HDMI and type 2 adaptors
//
pub const DP_DUAL_MODE_HDMI_ID: c_uint = 0x00 /* 00-0f */;
pub const DP_DUAL_MODE_HDMI_ID_LEN: c_int = 16;
//
// Optional for type 1 adaptors
// Mandatory for type 2 adaptors
//
pub const DP_DUAL_MODE_ADAPTOR_ID: c_uint = 0x10;
pub const DP_DUAL_MODE_REV_MASK: c_uint = 0x07;
pub const DP_DUAL_MODE_REV_TYPE2: c_uint = 0x00;
pub const DP_DUAL_MODE_TYPE_MASK: c_uint = 0xf0;
pub const DP_DUAL_MODE_TYPE_TYPE2: c_uint = 0xa0;
// This field is marked reserved in dual mode spec, used in LSPCON
pub const DP_DUAL_MODE_TYPE_HAS_DPCD: c_uint = 0x08;
pub const DP_DUAL_MODE_IEEE_OUI: c_uint = 0x11 /* 11-13*/;
pub const DP_DUAL_IEEE_OUI_LEN: c_int = 3;
pub const DP_DUAL_DEVICE_ID: c_uint = 0x14 /* 14-19 */;
pub const DP_DUAL_DEVICE_ID_LEN: c_int = 6;
pub const DP_DUAL_MODE_HARDWARE_REV: c_uint = 0x1a;
pub const DP_DUAL_MODE_FIRMWARE_MAJOR_REV: c_uint = 0x1b;
pub const DP_DUAL_MODE_FIRMWARE_MINOR_REV: c_uint = 0x1c;
pub const DP_DUAL_MODE_MAX_TMDS_CLOCK: c_uint = 0x1d;
pub const DP_DUAL_MODE_I2C_SPEED_CAP: c_uint = 0x1e;
pub const DP_DUAL_MODE_TMDS_OEN: c_uint = 0x20;
pub const DP_DUAL_MODE_TMDS_DISABLE: c_uint = 0x01;
pub const DP_DUAL_MODE_HDMI_PIN_CTRL: c_uint = 0x21;
pub const DP_DUAL_MODE_CEC_ENABLE: c_uint = 0x01;
pub const DP_DUAL_MODE_I2C_SPEED_CTRL: c_uint = 0x22;
// LSPCON specific registers, defined by MCA
pub const DP_DUAL_MODE_LSPCON_MODE_CHANGE: c_uint = 0x40;
pub const DP_DUAL_MODE_LSPCON_CURRENT_MODE: c_uint = 0x41;
pub const DP_DUAL_MODE_LSPCON_MODE_PCON: c_uint = 0x1;
//
// enum drm_lspcon_mode
// @DRM_LSPCON_MODE_INVALID: No LSPCON.
// @DRM_LSPCON_MODE_LS: Level shifter mode of LSPCON
// which drives DP++ to HDMI 1.4 conversion.
// @DRM_LSPCON_MODE_PCON: Protocol converter mode of LSPCON
// which drives DP++ to HDMI 2.0 active conversion.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_lspcon_mode {
    DRM_LSPCON_MODE_INVALID,
    DRM_LSPCON_MODE_LS,
    DRM_LSPCON_MODE_PCON,
}

//
// enum drm_dp_dual_mode_type - Type of the DP dual mode adaptor
// @DRM_DP_DUAL_MODE_NONE: No DP dual mode adaptor
// @DRM_DP_DUAL_MODE_UNKNOWN: Could be either none or type 1 DVI adaptor
// @DRM_DP_DUAL_MODE_TYPE1_DVI: Type 1 DVI adaptor
// @DRM_DP_DUAL_MODE_TYPE1_HDMI: Type 1 HDMI adaptor
// @DRM_DP_DUAL_MODE_TYPE2_DVI: Type 2 DVI adaptor
// @DRM_DP_DUAL_MODE_TYPE2_HDMI: Type 2 HDMI adaptor
// @DRM_DP_DUAL_MODE_LSPCON: Level shifter / protocol converter
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_dp_dual_mode_type {
    DRM_DP_DUAL_MODE_NONE,
    DRM_DP_DUAL_MODE_UNKNOWN,
    DRM_DP_DUAL_MODE_TYPE1_DVI,
    DRM_DP_DUAL_MODE_TYPE1_HDMI,
    DRM_DP_DUAL_MODE_TYPE2_DVI,
    DRM_DP_DUAL_MODE_TYPE2_HDMI,
    DRM_DP_DUAL_MODE_LSPCON,
}
