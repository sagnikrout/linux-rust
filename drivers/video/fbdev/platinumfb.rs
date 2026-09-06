//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/platinumfb.h
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
// linux/drivers/video/platinumfb-hw.c -- Frame buffer device for the
// Platinum on-board video in PowerMac 7200s (and some clones based
// on the same motherboard.)
//
// Created 09 Feb 1998 by Jon Howell <jonh@cs.dartmouth.edu>
//
// Copyright (C) 1998 Jon Howell
//
// based on drivers/macintosh/platinum.c: Console support
// for PowerMac "platinum" display adaptor.
// Copyright (C) 1996 Paul Mackerras and Mark Abene.
//
// based on skeletonfb.c:
// Created 28 Dec 1997 by Geert Uytterhoeven
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//
// Structure of the registers for the DACula colormap device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmap_regs {
    pub addr: c_uchar,
    pub pad1: [c_char; 15],
    pub d1: c_uchar,
    pub pad2: [c_char; 15],
    pub d2: c_uchar,
    pub pad3: [c_char; 15],
    pub lut: c_uchar,
    pub pad4: [c_char; 15],
}

//
// Structure of the registers for the "platinum" display adaptor".
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct preg {
    pub /: *mut *mut unsigned r; / notice this is 32 bits.,
    pub pad: [c_char; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct platinum_regs {
    pub reg: [preg; 128],
}

//
// Register initialization tables for the platinum display.
//
// It seems that there are two different types of platinum display
// out there.  Older ones use the values in clocksel[1], for which
// the formula for the clock frequency seems to be
// F = 14.3MHz * c0 / (c1 & 0x1f) / (1 << (c1 >> 5))
// Newer ones use the values in clocksel[0], for which the formula
// seems to be
// F = 15MHz * c0 / ((c1 & 0x1f) + 2) / (1 << (c1 >> 5))
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct platinum_regvals {
    pub fb_offset: c_int,
    pub pitch: [c_int; 3],
    pub regs: [unsigned; 26],
    pub offset: [c_uchar; 3],
    pub mode: [c_uchar; 3],
    pub dacula_ctrl: [c_uchar; 3],
    pub clock_params: [c_uchar; 2][2],
}

pub const DIV2: c_uint = 0x20;
pub const DIV4: c_uint = 0x40;
pub const DIV8: c_uint = 0x60;
pub const DIV16: c_uint = 0x80;
// 1280x1024, 75Hz (20)
// 1280x960, 75Hz (19)
// 1152x870, 75Hz (18)
// 1024x768, 75Hz (17)
// 1024x768, 75Hz (16)
// 1024x768, 70Hz (15)
// 1024x768, 60Hz (14)
// 832x624, 75Hz (13)
// and we use 3344 instead of 3360 to fit in 2Mb
//
// 800x600, 75Hz (12)
// 800x600, 72Hz (11)
// 800x600, 60Hz (10)
// 800x600, 56Hz (9) --unsupported? copy of mode 10 for now...
// 768x576, 50Hz Interlaced-PAL (8)
// 640x870, 75Hz Portrait (7)
// 640x480, 67Hz (6)
// 640x480, 60Hz (5)
// 640x480, 60Hz Interlaced-NTSC (4)
// 640x480, 50Hz Interlaced-PAL (3)
// 512x384, 60Hz (2)
// 512x384, 60Hz Interlaced-NTSC (1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmode_attr {
    pub hres: c_int,
    pub vres: c_int,
    pub vfreq: c_int,
    pub interlaced: c_int,
}
