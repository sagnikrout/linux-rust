//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/gspca/m5602/m5602_po1030.h
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
// Driver for the po1030 sensor.
//
// Copyright (c) 2008 Erik Andrén
// Copyright (c) 2007 Ilyes Gouta. Based on the m5603x Linux Driver Project.
// Copyright (c) 2005 m5603x Linux Driver Project <m5602@x3ng.com.br>
//
// Portions of code to USB interface and ALi driver software,
// Copyright (c) 2006 Willem Duinker
// v4l2 interface modeled after the V4L2 driver
// for SN9C10x PC Camera Controllers
//
// Register defines taken from Pascal Stangs Procyon Armlib
//

//
pub const PO1030_DEVID_H: c_uint = 0x00;
pub const PO1030_DEVID_L: c_uint = 0x01;
pub const PO1030_FRAMEWIDTH_H: c_uint = 0x04;
pub const PO1030_FRAMEWIDTH_L: c_uint = 0x05;
pub const PO1030_FRAMEHEIGHT_H: c_uint = 0x06;
pub const PO1030_FRAMEHEIGHT_L: c_uint = 0x07;
pub const PO1030_WINDOWX_H: c_uint = 0x08;
pub const PO1030_WINDOWX_L: c_uint = 0x09;
pub const PO1030_WINDOWY_H: c_uint = 0x0a;
pub const PO1030_WINDOWY_L: c_uint = 0x0b;
pub const PO1030_WINDOWWIDTH_H: c_uint = 0x0c;
pub const PO1030_WINDOWWIDTH_L: c_uint = 0x0d;
pub const PO1030_WINDOWHEIGHT_H: c_uint = 0x0e;
pub const PO1030_WINDOWHEIGHT_L: c_uint = 0x0f;
pub const PO1030_GLOBALIBIAS: c_uint = 0x12;
pub const PO1030_PIXELIBIAS: c_uint = 0x13;
pub const PO1030_GLOBALGAIN: c_uint = 0x15;
pub const PO1030_RED_GAIN: c_uint = 0x16;
pub const PO1030_GREEN_1_GAIN: c_uint = 0x17;
pub const PO1030_BLUE_GAIN: c_uint = 0x18;
pub const PO1030_GREEN_2_GAIN: c_uint = 0x19;
pub const PO1030_INTEGLINES_H: c_uint = 0x1a;
pub const PO1030_INTEGLINES_M: c_uint = 0x1b;
pub const PO1030_INTEGLINES_L: c_uint = 0x1c;
pub const PO1030_CONTROL1: c_uint = 0x1d;
pub const PO1030_CONTROL2: c_uint = 0x1e;
pub const PO1030_CONTROL3: c_uint = 0x1f;
pub const PO1030_CONTROL4: c_uint = 0x20;
pub const PO1030_PERIOD50_H: c_uint = 0x23;
pub const PO1030_PERIOD50_L: c_uint = 0x24;
pub const PO1030_PERIOD60_H: c_uint = 0x25;
pub const PO1030_PERIOD60_L: c_uint = 0x26;
pub const PO1030_REGCLK167: c_uint = 0x27;
pub const PO1030_FLICKER_DELTA50: c_uint = 0x28;
pub const PO1030_FLICKERDELTA60: c_uint = 0x29;
pub const PO1030_ADCOFFSET: c_uint = 0x2c;
// Gamma Correction Coeffs
pub const PO1030_GC0: c_uint = 0x2d;
pub const PO1030_GC1: c_uint = 0x2e;
pub const PO1030_GC2: c_uint = 0x2f;
pub const PO1030_GC3: c_uint = 0x30;
pub const PO1030_GC4: c_uint = 0x31;
pub const PO1030_GC5: c_uint = 0x32;
pub const PO1030_GC6: c_uint = 0x33;
pub const PO1030_GC7: c_uint = 0x34;
// Color Transform Matrix
pub const PO1030_CT0: c_uint = 0x35;
pub const PO1030_CT1: c_uint = 0x36;
pub const PO1030_CT2: c_uint = 0x37;
pub const PO1030_CT3: c_uint = 0x38;
pub const PO1030_CT4: c_uint = 0x39;
pub const PO1030_CT5: c_uint = 0x3a;
pub const PO1030_CT6: c_uint = 0x3b;
pub const PO1030_CT7: c_uint = 0x3c;
pub const PO1030_CT8: c_uint = 0x3d;
pub const PO1030_AUTOCTRL1: c_uint = 0x3e;
pub const PO1030_AUTOCTRL2: c_uint = 0x3f;
pub const PO1030_YTARGET: c_uint = 0x40;
pub const PO1030_GLOBALGAINMIN: c_uint = 0x41;
pub const PO1030_GLOBALGAINMAX: c_uint = 0x42;
pub const PO1030_AWB_RED_TUNING: c_uint = 0x47;
pub const PO1030_AWB_BLUE_TUNING: c_uint = 0x48;
// Output format control
pub const PO1030_OUTFORMCTRL1: c_uint = 0x5a;
pub const PO1030_OUTFORMCTRL2: c_uint = 0x5b;
pub const PO1030_OUTFORMCTRL3: c_uint = 0x5c;
pub const PO1030_OUTFORMCTRL4: c_uint = 0x5d;
pub const PO1030_OUTFORMCTRL5: c_uint = 0x5e;
pub const PO1030_EDGE_ENH_OFF: c_uint = 0x5f;
pub const PO1030_EGA: c_uint = 0x60;
pub const PO1030_Cb_U_GAIN: c_uint = 0x63;
pub const PO1030_Cr_V_GAIN: c_uint = 0x64;
pub const PO1030_YCONTRAST: c_uint = 0x74;
pub const PO1030_YSATURATION: c_uint = 0x75;

pub const PO1030_RAW_RGB_BAYER: c_uint = 0x4;

//
pub const PO1030_GLOBAL_GAIN_DEFAULT: c_uint = 0x12;
pub const PO1030_EXPOSURE_DEFAULT: c_uint = 0x0085;
pub const PO1030_BLUE_GAIN_DEFAULT: c_uint = 0x36;
pub const PO1030_RED_GAIN_DEFAULT: c_uint = 0x36;
pub const PO1030_GREEN_GAIN_DEFAULT: c_uint = 0x40;
//
// Kernel module parameters
extern "C" {
    pub fn po1030_probe(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn po1030_init(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn po1030_init_controls(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn po1030_start(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn po1030_disconnect(sd: *mut sd);
}
