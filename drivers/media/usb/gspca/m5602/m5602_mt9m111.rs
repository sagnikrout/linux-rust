//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/gspca/m5602/m5602_mt9m111.h
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
// Driver for the mt9m111 sensor
//
// Copyright (C) 2008 Erik Andrén
// Copyright (C) 2007 Ilyes Gouta. Based on the m5603x Linux Driver Project.
// Copyright (C) 2005 m5603x Linux Driver Project <m5602@x3ng.com.br>
//
// Portions of code to USB interface and ALi driver software,
// Copyright (c) 2006 Willem Duinker
// v4l2 interface modeled after the V4L2 driver
// for SN9C10x PC Camera Controllers
//
// Some defines taken from the mt9m111 sensor driver
// Copyright (C) 2008, Robert Jarzmik <robert.jarzmik@free.fr>
//

//
pub const MT9M111_SC_CHIPVER: c_uint = 0x00;
pub const MT9M111_SC_ROWSTART: c_uint = 0x01;
pub const MT9M111_SC_COLSTART: c_uint = 0x02;
pub const MT9M111_SC_WINDOW_HEIGHT: c_uint = 0x03;
pub const MT9M111_SC_WINDOW_WIDTH: c_uint = 0x04;
pub const MT9M111_SC_HBLANK_CONTEXT_B: c_uint = 0x05;
pub const MT9M111_SC_VBLANK_CONTEXT_B: c_uint = 0x06;
pub const MT9M111_SC_HBLANK_CONTEXT_A: c_uint = 0x07;
pub const MT9M111_SC_VBLANK_CONTEXT_A: c_uint = 0x08;
pub const MT9M111_SC_SHUTTER_WIDTH: c_uint = 0x09;
pub const MT9M111_SC_ROW_SPEED: c_uint = 0x0a;
pub const MT9M111_SC_EXTRA_DELAY: c_uint = 0x0b;
pub const MT9M111_SC_SHUTTER_DELAY: c_uint = 0x0c;
pub const MT9M111_SC_RESET: c_uint = 0x0d;
pub const MT9M111_SC_R_MODE_CONTEXT_B: c_uint = 0x20;
pub const MT9M111_SC_R_MODE_CONTEXT_A: c_uint = 0x21;
pub const MT9M111_SC_FLASH_CONTROL: c_uint = 0x23;
pub const MT9M111_SC_GREEN_1_GAIN: c_uint = 0x2b;
pub const MT9M111_SC_BLUE_GAIN: c_uint = 0x2c;
pub const MT9M111_SC_RED_GAIN: c_uint = 0x2d;
pub const MT9M111_SC_GREEN_2_GAIN: c_uint = 0x2e;
pub const MT9M111_SC_GLOBAL_GAIN: c_uint = 0x2f;
pub const MT9M111_CONTEXT_CONTROL: c_uint = 0xc8;
pub const MT9M111_PAGE_MAP: c_uint = 0xf0;
pub const MT9M111_BYTEWISE_ADDRESS: c_uint = 0xf1;
pub const MT9M111_CP_OPERATING_MODE_CTL: c_uint = 0x06;
pub const MT9M111_CP_LUMA_OFFSET: c_uint = 0x34;
pub const MT9M111_CP_LUMA_CLIP: c_uint = 0x35;
pub const MT9M111_CP_OUTPUT_FORMAT_CTL2_CONTEXT_A: c_uint = 0x3a;
pub const MT9M111_CP_LENS_CORRECTION_1: c_uint = 0x3b;
pub const MT9M111_CP_DEFECT_CORR_CONTEXT_A: c_uint = 0x4c;
pub const MT9M111_CP_DEFECT_CORR_CONTEXT_B: c_uint = 0x4d;
pub const MT9M111_CP_OUTPUT_FORMAT_CTL2_CONTEXT_B: c_uint = 0x9b;
pub const MT9M111_CP_GLOBAL_CLK_CONTROL: c_uint = 0xb3;
pub const MT9M111_CC_AUTO_EXPOSURE_PARAMETER_18: c_uint = 0x65;
pub const MT9M111_CC_AWB_PARAMETER_7: c_uint = 0x28;
pub const MT9M111_SENSOR_CORE: c_uint = 0x00;
pub const MT9M111_COLORPIPE: c_uint = 0x01;
pub const MT9M111_CAMERA_CONTROL: c_uint = 0x02;

pub const INITIAL_MAX_GAIN: c_int = 64;
pub const MT9M111_DEFAULT_GAIN: c_int = 283;
pub const MT9M111_GREEN_GAIN_DEFAULT: c_uint = 0x20;
pub const MT9M111_BLUE_GAIN_DEFAULT: c_uint = 0x20;
pub const MT9M111_RED_GAIN_DEFAULT: c_uint = 0x20;
//
// Kernel module parameters
extern "C" {
    pub fn mt9m111_probe(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn mt9m111_init(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn mt9m111_init_controls(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn mt9m111_start(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn mt9m111_disconnect(sd: *mut sd);
}
