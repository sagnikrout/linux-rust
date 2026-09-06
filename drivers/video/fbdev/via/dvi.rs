//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/via/dvi.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 1998-2008 VIA Technologies, Inc. All Rights Reserved.
// Copyright 2001-2008 S3 Graphics, Inc. All Rights Reserved.
//
// Definition TMDS Device ID register
pub const VT1632_DEVICE_ID_REG: c_uint = 0x02;
pub const VT1632_DEVICE_ID: c_uint = 0x92;
pub const GET_DVI_SIZE_BY_SYSTEM_BIOS: c_uint = 0x01;
pub const GET_DVI_SIZE_BY_VGA_BIOS: c_uint = 0x02;
pub const GET_DVI_SZIE_BY_HW_STRAPPING: c_uint = 0x03;
// Definition DVI Panel ID
// Resolution: 640x480,   Channel: single, Dithering: Enable
pub const DVI_PANEL_ID0_640X480: c_uint = 0x00;
// Resolution: 800x600,   Channel: single, Dithering: Enable
pub const DVI_PANEL_ID1_800x600: c_uint = 0x01;
// Resolution: 1024x768,  Channel: single, Dithering: Enable
pub const DVI_PANEL_ID1_1024x768: c_uint = 0x02;
// Resolution: 1280x768,  Channel: single, Dithering: Enable
pub const DVI_PANEL_ID1_1280x768: c_uint = 0x03;
// Resolution: 1280x1024, Channel: dual,   Dithering: Enable
pub const DVI_PANEL_ID1_1280x1024: c_uint = 0x04;
// Resolution: 1400x1050, Channel: dual,   Dithering: Enable
pub const DVI_PANEL_ID1_1400x1050: c_uint = 0x05;
// Resolution: 1600x1200, Channel: dual,   Dithering: Enable
pub const DVI_PANEL_ID1_1600x1200: c_uint = 0x06;
// Define the version of EDID
pub const EDID_VERSION_1: c_int = 1;
pub const EDID_VERSION_2: c_int = 2;
pub const DEV_CONNECT_DVI: c_uint = 0x01;
pub const DEV_CONNECT_HDMI: c_uint = 0x02;
extern "C" {
    pub fn viafb_dvi_sense() -> c_int;
}
extern "C" {
    pub fn viafb_dvi_disable();
}
extern "C" {
    pub fn viafb_dvi_enable();
}
extern "C" {
    pub fn viafb_tmds_trasmitter_identify() -> bool;
}
