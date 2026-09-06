//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/ak4535.h
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
// ak4535.h  --  AK4535 Soc Audio driver
//
// Copyright 2005 Openedhand Ltd.
//
// Author: Richard Purdie <richard@openedhand.com>
//
// Based on wm8753.h
//
// AK4535 register space
pub const AK4535_PM1: c_uint = 0x0;
pub const AK4535_PM2: c_uint = 0x1;
pub const AK4535_SIG1: c_uint = 0x2;
pub const AK4535_SIG2: c_uint = 0x3;
pub const AK4535_MODE1: c_uint = 0x4;
pub const AK4535_MODE2: c_uint = 0x5;
pub const AK4535_DAC: c_uint = 0x6;
pub const AK4535_MIC: c_uint = 0x7;
pub const AK4535_TIMER: c_uint = 0x8;
pub const AK4535_ALC1: c_uint = 0x9;
pub const AK4535_ALC2: c_uint = 0xa;
pub const AK4535_PGA: c_uint = 0xb;
pub const AK4535_LATT: c_uint = 0xc;
pub const AK4535_RATT: c_uint = 0xd;
pub const AK4535_VOL: c_uint = 0xe;
pub const AK4535_STATUS: c_uint = 0xf;
