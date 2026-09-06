//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/gspca/m5602/m5602_ov7660.h
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
// Driver for the ov7660 sensor
//
// Copyright (C) 2009 Erik Andrén
// Copyright (C) 2007 Ilyes Gouta. Based on the m5603x Linux Driver Project.
// Copyright (C) 2005 m5603x Linux Driver Project <m5602@x3ng.com.br>
//
// Portions of code to USB interface and ALi driver software,
// Copyright (c) 2006 Willem Duinker
// v4l2 interface modeled after the V4L2 driver
// for SN9C10x PC Camera Controllers
//

pub const OV7660_GAIN: c_uint = 0x00;
pub const OV7660_BLUE_GAIN: c_uint = 0x01;
pub const OV7660_RED_GAIN: c_uint = 0x02;
pub const OV7660_VREF: c_uint = 0x03;
pub const OV7660_COM1: c_uint = 0x04;
pub const OV7660_BAVE: c_uint = 0x05;
pub const OV7660_GEAVE: c_uint = 0x06;
pub const OV7660_AECHH: c_uint = 0x07;
pub const OV7660_RAVE: c_uint = 0x08;
pub const OV7660_COM2: c_uint = 0x09;
pub const OV7660_PID: c_uint = 0x0a;
pub const OV7660_VER: c_uint = 0x0b;
pub const OV7660_COM3: c_uint = 0x0c;
pub const OV7660_COM4: c_uint = 0x0d;
pub const OV7660_COM5: c_uint = 0x0e;
pub const OV7660_COM6: c_uint = 0x0f;
pub const OV7660_AECH: c_uint = 0x10;
pub const OV7660_CLKRC: c_uint = 0x11;
pub const OV7660_COM7: c_uint = 0x12;
pub const OV7660_COM8: c_uint = 0x13;
pub const OV7660_COM9: c_uint = 0x14;
pub const OV7660_COM10: c_uint = 0x15;
pub const OV7660_RSVD16: c_uint = 0x16;
pub const OV7660_HSTART: c_uint = 0x17;
pub const OV7660_HSTOP: c_uint = 0x18;
pub const OV7660_VSTART: c_uint = 0x19;
pub const OV7660_VSTOP: c_uint = 0x1a;
pub const OV7660_PSHFT: c_uint = 0x1b;
pub const OV7660_MIDH: c_uint = 0x1c;
pub const OV7660_MIDL: c_uint = 0x1d;
pub const OV7660_MVFP: c_uint = 0x1e;
pub const OV7660_LAEC: c_uint = 0x1f;
pub const OV7660_BOS: c_uint = 0x20;
pub const OV7660_GBOS: c_uint = 0x21;
pub const OV7660_GROS: c_uint = 0x22;
pub const OV7660_ROS: c_uint = 0x23;
pub const OV7660_AEW: c_uint = 0x24;
pub const OV7660_AEB: c_uint = 0x25;
pub const OV7660_VPT: c_uint = 0x26;
pub const OV7660_BBIAS: c_uint = 0x27;
pub const OV7660_GbBIAS: c_uint = 0x28;
pub const OV7660_RSVD29: c_uint = 0x29;
pub const OV7660_RBIAS: c_uint = 0x2c;
pub const OV7660_HREF: c_uint = 0x32;
pub const OV7660_ADC: c_uint = 0x37;
pub const OV7660_OFON: c_uint = 0x39;
pub const OV7660_TSLB: c_uint = 0x3a;
pub const OV7660_COM12: c_uint = 0x3c;
pub const OV7660_COM13: c_uint = 0x3d;
pub const OV7660_LCC1: c_uint = 0x62;
pub const OV7660_LCC2: c_uint = 0x63;
pub const OV7660_LCC3: c_uint = 0x64;
pub const OV7660_LCC4: c_uint = 0x65;
pub const OV7660_LCC5: c_uint = 0x66;
pub const OV7660_HV: c_uint = 0x69;
pub const OV7660_RSVDA1: c_uint = 0xa1;
pub const OV7660_DEFAULT_GAIN: c_uint = 0x0e;
pub const OV7660_DEFAULT_RED_GAIN: c_uint = 0x80;
pub const OV7660_DEFAULT_BLUE_GAIN: c_uint = 0x80;
pub const OV7660_DEFAULT_SATURATION: c_uint = 0x00;
pub const OV7660_DEFAULT_EXPOSURE: c_uint = 0x20;
// Kernel module parameters
extern "C" {
    pub fn ov7660_probe(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn ov7660_init(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn ov7660_init_controls(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn ov7660_start(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn ov7660_stop(sd: *mut sd) -> c_int;
}
extern "C" {
    pub fn ov7660_disconnect(sd: *mut sd);
}
