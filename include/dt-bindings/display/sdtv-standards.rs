//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/display/sdtv-standards.h
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


// SPDX-License-Identifier: GPL-2.0-only OR X11
//
// Copyright 2019 Pengutronix, Marco Felsch <kernel@pengutronix.de>
//
// Attention: Keep the SDTV_STD_* bit definitions in sync with
// include/uapi/linux/videodev2.h V4L2_STD_* bit definitions.
//
// One bit for each standard
pub const SDTV_STD_PAL_B: c_uint = 0x00000001;
pub const SDTV_STD_PAL_B1: c_uint = 0x00000002;
pub const SDTV_STD_PAL_G: c_uint = 0x00000004;
pub const SDTV_STD_PAL_H: c_uint = 0x00000008;
pub const SDTV_STD_PAL_I: c_uint = 0x00000010;
pub const SDTV_STD_PAL_D: c_uint = 0x00000020;
pub const SDTV_STD_PAL_D1: c_uint = 0x00000040;
pub const SDTV_STD_PAL_K: c_uint = 0x00000080;

pub const SDTV_STD_PAL_M: c_uint = 0x00000100;
pub const SDTV_STD_PAL_N: c_uint = 0x00000200;
pub const SDTV_STD_PAL_Nc: c_uint = 0x00000400;
pub const SDTV_STD_PAL_60: c_uint = 0x00000800;
pub const SDTV_STD_NTSC_M: c_uint = 0x00001000	/* BTSC */;
pub const SDTV_STD_NTSC_M_JP: c_uint = 0x00002000	/* EIA-J */;
pub const SDTV_STD_NTSC_443: c_uint = 0x00004000;
pub const SDTV_STD_NTSC_M_KR: c_uint = 0x00008000	/* FM A2 */;

pub const SDTV_STD_SECAM_B: c_uint = 0x00010000;
pub const SDTV_STD_SECAM_D: c_uint = 0x00020000;
pub const SDTV_STD_SECAM_G: c_uint = 0x00040000;
pub const SDTV_STD_SECAM_H: c_uint = 0x00080000;
pub const SDTV_STD_SECAM_K: c_uint = 0x00100000;
pub const SDTV_STD_SECAM_K1: c_uint = 0x00200000;
pub const SDTV_STD_SECAM_L: c_uint = 0x00400000;
pub const SDTV_STD_SECAM_LC: c_uint = 0x00800000;

// Standards for Countries with 60Hz Line frequency

// Standards for Countries with 50Hz Line frequency

