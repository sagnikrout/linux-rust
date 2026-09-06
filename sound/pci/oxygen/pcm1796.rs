//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/oxygen/pcm1796.h
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


// SPDX-License-Identifier: GPL-2.0

// Macro flag: #define PCM1796_H_INCLUDED
// register 16
pub const PCM1796_ATL_MASK: c_uint = 0xff;
// register 17
pub const PCM1796_ATR_MASK: c_uint = 0xff;
// register 18
pub const PCM1796_MUTE: c_uint = 0x01;
pub const PCM1796_DME: c_uint = 0x02;
pub const PCM1796_DMF_MASK: c_uint = 0x0c;
pub const PCM1796_DMF_48: c_uint = 0x04;
pub const PCM1796_DMF_441: c_uint = 0x08;
pub const PCM1796_DMF_32: c_uint = 0x0c;
pub const PCM1796_FMT_MASK: c_uint = 0x70;
pub const PCM1796_FMT_16_RJUST: c_uint = 0x00;
pub const PCM1796_FMT_20_RJUST: c_uint = 0x10;
pub const PCM1796_FMT_24_RJUST: c_uint = 0x20;
pub const PCM1796_FMT_24_LJUST: c_uint = 0x30;
pub const PCM1796_FMT_16_I2S: c_uint = 0x40;
pub const PCM1796_FMT_24_I2S: c_uint = 0x50;
pub const PCM1796_ATLD: c_uint = 0x80;
// register 19
pub const PCM1796_INZD: c_uint = 0x01;
pub const PCM1796_FLT_MASK: c_uint = 0x02;
pub const PCM1796_FLT_SHARP: c_uint = 0x00;
pub const PCM1796_FLT_SLOW: c_uint = 0x02;
pub const PCM1796_DFMS: c_uint = 0x04;
pub const PCM1796_OPE: c_uint = 0x10;
pub const PCM1796_ATS_MASK: c_uint = 0x60;
pub const PCM1796_ATS_1: c_uint = 0x00;
pub const PCM1796_ATS_2: c_uint = 0x20;
pub const PCM1796_ATS_4: c_uint = 0x40;
pub const PCM1796_ATS_8: c_uint = 0x60;
pub const PCM1796_REV: c_uint = 0x80;
// register 20
pub const PCM1796_OS_MASK: c_uint = 0x03;
pub const PCM1796_OS_64: c_uint = 0x00;
pub const PCM1796_OS_32: c_uint = 0x01;
pub const PCM1796_OS_128: c_uint = 0x02;
pub const PCM1796_CHSL_MASK: c_uint = 0x04;
pub const PCM1796_CHSL_LEFT: c_uint = 0x00;
pub const PCM1796_CHSL_RIGHT: c_uint = 0x04;
pub const PCM1796_MONO: c_uint = 0x08;
pub const PCM1796_DFTH: c_uint = 0x10;
pub const PCM1796_DSD: c_uint = 0x20;
pub const PCM1796_SRST: c_uint = 0x40;
// register 21
pub const PCM1796_PCMZ: c_uint = 0x01;
pub const PCM1796_DZ_MASK: c_uint = 0x06;
// register 22
pub const PCM1796_ZFGL: c_uint = 0x01;
pub const PCM1796_ZFGR: c_uint = 0x02;
// register 23
pub const PCM1796_ID_MASK: c_uint = 0x1f;
