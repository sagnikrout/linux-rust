//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/gspca/m5602/m5602_ov9650.h
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
// Driver for the ov9650 sensor
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

//
pub const OV9650_GAIN: c_uint = 0x00;
pub const OV9650_BLUE: c_uint = 0x01;
pub const OV9650_RED: c_uint = 0x02;
pub const OV9650_VREF: c_uint = 0x03;
pub const OV9650_COM1: c_uint = 0x04;
pub const OV9650_BAVE: c_uint = 0x05;
pub const OV9650_GEAVE: c_uint = 0x06;
pub const OV9650_RSVD7: c_uint = 0x07;
pub const OV9650_COM2: c_uint = 0x09;
pub const OV9650_PID: c_uint = 0x0a;
pub const OV9650_VER: c_uint = 0x0b;
pub const OV9650_COM3: c_uint = 0x0c;
pub const OV9650_COM4: c_uint = 0x0d;
pub const OV9650_COM5: c_uint = 0x0e;
pub const OV9650_COM6: c_uint = 0x0f;
pub const OV9650_AECH: c_uint = 0x10;
pub const OV9650_CLKRC: c_uint = 0x11;
pub const OV9650_COM7: c_uint = 0x12;
pub const OV9650_COM8: c_uint = 0x13;
pub const OV9650_COM9: c_uint = 0x14;
pub const OV9650_COM10: c_uint = 0x15;
pub const OV9650_RSVD16: c_uint = 0x16;
pub const OV9650_HSTART: c_uint = 0x17;
pub const OV9650_HSTOP: c_uint = 0x18;
pub const OV9650_VSTRT: c_uint = 0x19;
pub const OV9650_VSTOP: c_uint = 0x1a;
pub const OV9650_PSHFT: c_uint = 0x1b;
pub const OV9650_MVFP: c_uint = 0x1e;
pub const OV9650_AEW: c_uint = 0x24;
pub const OV9650_AEB: c_uint = 0x25;
pub const OV9650_VPT: c_uint = 0x26;
pub const OV9650_BBIAS: c_uint = 0x27;
pub const OV9650_GbBIAS: c_uint = 0x28;
pub const OV9650_Gr_COM: c_uint = 0x29;
pub const OV9650_RBIAS: c_uint = 0x2c;
pub const OV9650_HREF: c_uint = 0x32;
pub const OV9650_CHLF: c_uint = 0x33;
pub const OV9650_ARBLM: c_uint = 0x34;
pub const OV9650_RSVD35: c_uint = 0x35;
pub const OV9650_RSVD36: c_uint = 0x36;
pub const OV9650_ADC: c_uint = 0x37;
pub const OV9650_ACOM38: c_uint = 0x38;
pub const OV9650_OFON: c_uint = 0x39;
pub const OV9650_TSLB: c_uint = 0x3a;
pub const OV9650_COM12: c_uint = 0x3c;
pub const OV9650_COM13: c_uint = 0x3d;
pub const OV9650_COM15: c_uint = 0x40;
pub const OV9650_COM16: c_uint = 0x41;
pub const OV9650_LCC1: c_uint = 0x62;
pub const OV9650_LCC2: c_uint = 0x63;
pub const OV9650_LCC3: c_uint = 0x64;
pub const OV9650_LCC4: c_uint = 0x65;
pub const OV9650_LCC5: c_uint = 0x66;
pub const OV9650_HV: c_uint = 0x69;
pub const OV9650_DBLV: c_uint = 0x6b;
pub const OV9650_COM21: c_uint = 0x8b;
pub const OV9650_COM22: c_uint = 0x8c;
pub const OV9650_COM24: c_uint = 0x8e;
pub const OV9650_DBLC1: c_uint = 0x8f;
pub const OV9650_RSVD94: c_uint = 0x94;
pub const OV9650_RSVD95: c_uint = 0x95;
pub const OV9650_RSVD96: c_uint = 0x96;
pub const OV9650_LCCFB: c_uint = 0x9d;
pub const OV9650_LCCFR: c_uint = 0x9e;
pub const OV9650_AECHM: c_uint = 0xa1;
pub const OV9650_COM26: c_uint = 0xa5;
pub const OV9650_ACOMA8: c_uint = 0xa8;
pub const OV9650_ACOMA9: c_uint = 0xa9;

pub const OV9650_LEFT_OFFSET: c_uint = 0x62;
pub const GAIN_DEFAULT: c_uint = 0x14;
pub const RED_GAIN_DEFAULT: c_uint = 0x70;
pub const BLUE_GAIN_DEFAULT: c_uint = 0x20;
pub const EXPOSURE_DEFAULT: c_uint = 0x1ff;
//
// Kernel module parameters
extern "C" {
    pub fn ov9650_probe(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn ov9650_init(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn ov9650_init_controls(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn ov9650_start(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn ov9650_stop(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn ov9650_disconnect(sd: *mut sd);
}
