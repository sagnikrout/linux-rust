//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/sis/sis_main.h
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
// SiS 300/305/540/630(S)/730(S),
// SiS 315[E|PRO]/550/[M]65x/[M]66x[F|M|G]X/[M]74x[GX]/330/[M]76x[GX],
// XGI V3XT/V5/V8, Z7
// frame buffer driver for Linux kernels >=2.4.14 and >=2.6.3
//
// Copyright (C) 2001-2005 Thomas Winischhofer, Vienna, Austria.
//

// Fbcon stuff

// Boot-time parameters

// List of supported chips

// The memory heap is now handled card-wise, by using
//
pub const MD_SIS300: c_int = 1;
pub const MD_SIS315: c_int = 2;
// Mode table
// 0*/	{"none",         {0xff,0xff}, 0x0000, 0x0000,    0,    0,  0, 0,   0,  0, MD_SIS300|MD_SIS315},
pub const MODE_FSTN_8: c_int = 9;
pub const MODE_FSTN_16: c_int = 10;
// 10*/	{"320x240x16",   {0x5b,0x5b}, 0x0135, 0x0000,  320,  240, 16, 1,  40, 15,           MD_SIS315},  /* FSTN
// 20*/	{"640x400x16",   {0x5d,0x5d}, 0x0000, 0x0000,  640,  400, 16, 1,  80, 25, MD_SIS300|MD_SIS315},
// 30*/	{"720x480x32",   {0x35,0x35}, 0x0000, 0x0000,  720,  480, 32, 1,  90, 30, MD_SIS300|MD_SIS315},
// 40*/	{"800x480x16",   {0x7a,0x7a}, 0x0000, 0x0000,  800,  480, 16, 1, 100, 30, MD_SIS300|MD_SIS315},

pub const DEFAULT_MODE_848: c_int = 48;
// 50*/	{"848x480x32",   {0x3e,0x3e}, 0x0000, 0x0000,  848,  480, 32, 2, 106, 30, MD_SIS300|MD_SIS315},
pub const DEFAULT_MODE_856: c_int = 52;
// 60*/	{"960x600x16",   {0x21,0x21}, 0x0000, 0x0000,  960,  600, 16, 1, 120, 37,           MD_SIS315},
// 70*/	{"1024x600x32",  {0x22,0x22}, 0x0000, 0x0000, 1024,  600, 32, 1, 128, 37, MD_SIS300          },
// 80*/	{"1152x864x16",  {0x2a,0x2a}, 0x0000, 0x0000, 1152,  864, 16, 1, 144, 54, MD_SIS300|MD_SIS315},
// 90*/	{"1280x768x32",  {0x5b,0x25}, 0x0000, 0x0000, 1280,  768, 32, 1, 160, 48, MD_SIS300|MD_SIS315},
// 100*/	{"1280x960x16",  {0x7d,0x7d}, 0x0000, 0x0000, 1280,  960, 16, 1, 160, 60, MD_SIS300|MD_SIS315},
// 110*/	{"1360x768x32",  {0x4e,0x4e}, 0x0000, 0x0000, 1360,  768, 32, 1, 170, 48, MD_SIS300|MD_SIS315},
pub const DEFAULT_MODE_1360: c_int = 112;
// 120*/	{"1600x1200x16", {0x3d,0x3d}, 0x0131, 0x011e, 1600, 1200, 16, 1, 200, 75, MD_SIS300|MD_SIS315},
// 130*/	{"1920x1080x32", {0x73,0x73}, 0x0000, 0x0000, 1920, 1080, 32, 1, 240, 67,           MD_SIS315},
pub const SIS_LCD_NUMBER: c_int = 18;
// CR36 evaluation
pub const FL_550_DSTN: c_uint = 0x01;
pub const FL_550_FSTN: c_uint = 0x02;
pub const FL_300: c_uint = 0x04;
pub const FL_315: c_uint = 0x08;
// TV standard

