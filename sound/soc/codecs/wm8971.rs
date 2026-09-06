//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8971.h
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
// wm8971.h  --  audio driver for WM8971
//
// Copyright 2005 Lab126, Inc.
//
// Author: Kenneth Kiraly <kiraly@lab126.com>
//
pub const WM8971_LINVOL: c_uint = 0x00;
pub const WM8971_RINVOL: c_uint = 0x01;
pub const WM8971_LOUT1V: c_uint = 0x02;
pub const WM8971_ROUT1V: c_uint = 0x03;
pub const WM8971_ADCDAC: c_uint = 0x05;
pub const WM8971_IFACE: c_uint = 0x07;
pub const WM8971_SRATE: c_uint = 0x08;
pub const WM8971_LDAC: c_uint = 0x0a;
pub const WM8971_RDAC: c_uint = 0x0b;
pub const WM8971_BASS: c_uint = 0x0c;
pub const WM8971_TREBLE: c_uint = 0x0d;
pub const WM8971_RESET: c_uint = 0x0f;
pub const WM8971_ALC1: c_uint = 0x11;
pub const WM8971_ALC2: c_uint = 0x12;
pub const WM8971_ALC3: c_uint = 0x13;
pub const WM8971_NGATE: c_uint = 0x14;
pub const WM8971_LADC: c_uint = 0x15;
pub const WM8971_RADC: c_uint = 0x16;
pub const WM8971_ADCTL1: c_uint = 0x17;
pub const WM8971_ADCTL2: c_uint = 0x18;
pub const WM8971_PWR1: c_uint = 0x19;
pub const WM8971_PWR2: c_uint = 0x1a;
pub const WM8971_ADCTL3: c_uint = 0x1b;
pub const WM8971_ADCIN: c_uint = 0x1f;
pub const WM8971_LADCIN: c_uint = 0x20;
pub const WM8971_RADCIN: c_uint = 0x21;
pub const WM8971_LOUTM1: c_uint = 0x22;
pub const WM8971_LOUTM2: c_uint = 0x23;
pub const WM8971_ROUTM1: c_uint = 0x24;
pub const WM8971_ROUTM2: c_uint = 0x25;
pub const WM8971_MOUTM1: c_uint = 0x26;
pub const WM8971_MOUTM2: c_uint = 0x27;
pub const WM8971_LOUT2V: c_uint = 0x28;
pub const WM8971_ROUT2V: c_uint = 0x29;
pub const WM8971_MOUTV: c_uint = 0x2A;
pub const WM8971_SYSCLK: c_int = 0;
