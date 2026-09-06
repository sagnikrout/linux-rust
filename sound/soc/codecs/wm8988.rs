//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8988.h
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
// Copyright 2005 Openedhand Ltd.
//
// Author: Richard Purdie <richard@openedhand.com>
//
// Based on WM8753.h
//
// WM8988 register space
pub const WM8988_LINVOL: c_uint = 0x00;
pub const WM8988_RINVOL: c_uint = 0x01;
pub const WM8988_LOUT1V: c_uint = 0x02;
pub const WM8988_ROUT1V: c_uint = 0x03;
pub const WM8988_ADCDAC: c_uint = 0x05;
pub const WM8988_IFACE: c_uint = 0x07;
pub const WM8988_SRATE: c_uint = 0x08;
pub const WM8988_LDAC: c_uint = 0x0a;
pub const WM8988_RDAC: c_uint = 0x0b;
pub const WM8988_BASS: c_uint = 0x0c;
pub const WM8988_TREBLE: c_uint = 0x0d;
pub const WM8988_RESET: c_uint = 0x0f;
pub const WM8988_3D: c_uint = 0x10;
pub const WM8988_ALC1: c_uint = 0x11;
pub const WM8988_ALC2: c_uint = 0x12;
pub const WM8988_ALC3: c_uint = 0x13;
pub const WM8988_NGATE: c_uint = 0x14;
pub const WM8988_LADC: c_uint = 0x15;
pub const WM8988_RADC: c_uint = 0x16;
pub const WM8988_ADCTL1: c_uint = 0x17;
pub const WM8988_ADCTL2: c_uint = 0x18;
pub const WM8988_PWR1: c_uint = 0x19;
pub const WM8988_PWR2: c_uint = 0x1a;
pub const WM8988_ADCTL3: c_uint = 0x1b;
pub const WM8988_ADCIN: c_uint = 0x1f;
pub const WM8988_LADCIN: c_uint = 0x20;
pub const WM8988_RADCIN: c_uint = 0x21;
pub const WM8988_LOUTM1: c_uint = 0x22;
pub const WM8988_LOUTM2: c_uint = 0x23;
pub const WM8988_ROUTM1: c_uint = 0x24;
pub const WM8988_ROUTM2: c_uint = 0x25;
pub const WM8988_LOUT2V: c_uint = 0x28;
pub const WM8988_ROUT2V: c_uint = 0x29;
pub const WM8988_LPPB: c_uint = 0x43;
pub const WM8988_NUM_REG: c_uint = 0x44;
pub const WM8988_SYSCLK: c_int = 0;
