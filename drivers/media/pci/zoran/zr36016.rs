//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/zoran/zr36016.h
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
// Zoran ZR36016 basic configuration functions - header file
//
// Copyright (C) 2001 Wolfgang Scherr <scherr@net4you.at>
//
// data stored for each zoran jpeg codec chip
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zr36016 {
    pub name: [c_char; 32],
    pub num: c_int,
// io datastructure
    pub codec: *mut videocodec,
// coder status
    pub version: __u8,
// actual coder setup
    pub mode: c_int,
    pub xoff: __u16,
    pub yoff: __u16,
    pub width: __u16,
    pub height: __u16,
    pub xdec: __u16,
    pub ydec: __u16,
}

// direct  register addresses
pub const ZR016_GOSTOP: c_uint = 0x00;
pub const ZR016_MODE: c_uint = 0x01;
pub const ZR016_IADDR: c_uint = 0x02;
pub const ZR016_IDATA: c_uint = 0x03;
// indirect  register addresses
pub const ZR016I_SETUP1: c_uint = 0x00;
pub const ZR016I_SETUP2: c_uint = 0x01;
pub const ZR016I_NAX_LO: c_uint = 0x02;
pub const ZR016I_NAX_HI: c_uint = 0x03;
pub const ZR016I_PAX_LO: c_uint = 0x04;
pub const ZR016I_PAX_HI: c_uint = 0x05;
pub const ZR016I_NAY_LO: c_uint = 0x06;
pub const ZR016I_NAY_HI: c_uint = 0x07;
pub const ZR016I_PAY_LO: c_uint = 0x08;
pub const ZR016I_PAY_HI: c_uint = 0x09;
pub const ZR016I_NOL_LO: c_uint = 0x0a;
pub const ZR016I_NOL_HI: c_uint = 0x0b;
// possible values for mode register
pub const ZR016_RGB444_YUV444: c_uint = 0x00;
pub const ZR016_RGB444_YUV422: c_uint = 0x01;
pub const ZR016_RGB444_YUV411: c_uint = 0x02;
pub const ZR016_RGB444_Y400: c_uint = 0x03;
pub const ZR016_RGB444_RGB444: c_uint = 0x04;
pub const ZR016_YUV444_YUV444: c_uint = 0x08;
pub const ZR016_YUV444_YUV422: c_uint = 0x09;
pub const ZR016_YUV444_YUV411: c_uint = 0x0a;
pub const ZR016_YUV444_Y400: c_uint = 0x0b;
pub const ZR016_YUV444_RGB444: c_uint = 0x0c;
pub const ZR016_YUV422_YUV422: c_uint = 0x11;
pub const ZR016_YUV422_YUV411: c_uint = 0x12;
pub const ZR016_YUV422_Y400: c_uint = 0x13;
pub const ZR016_YUV411_YUV411: c_uint = 0x16;
pub const ZR016_YUV411_Y400: c_uint = 0x17;
pub const ZR016_4444_4444: c_uint = 0x19;
pub const ZR016_100_100: c_uint = 0x1b;
pub const ZR016_RGB444: c_uint = 0x00;
pub const ZR016_YUV444: c_uint = 0x20;
pub const ZR016_YUV422: c_uint = 0x40;
pub const ZR016_COMPRESSION: c_uint = 0x80;
pub const ZR016_EXPANSION: c_uint = 0x80;
// possible values for setup 1 register
pub const ZR016_CKRT: c_uint = 0x80;
pub const ZR016_VERT: c_uint = 0x40;
pub const ZR016_HORZ: c_uint = 0x20;
pub const ZR016_HRFL: c_uint = 0x10;
pub const ZR016_DSFL: c_uint = 0x08;
pub const ZR016_SBFL: c_uint = 0x04;
pub const ZR016_RSTR: c_uint = 0x02;
pub const ZR016_CNTI: c_uint = 0x01;
// possible values for setup 2 register
pub const ZR016_SYEN: c_uint = 0x40;
pub const ZR016_CCIR: c_uint = 0x04;
pub const ZR016_SIGN: c_uint = 0x02;
pub const ZR016_YMCS: c_uint = 0x01;
extern "C" {
    pub fn zr36016_init_module() -> c_int;
}
extern "C" {
    pub fn zr36016_cleanup_module();
}
