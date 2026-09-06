//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/pfuze100.h
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
// Copyright (C) 2011-2013 Freescale Semiconductor, Inc. All Rights Reserved.
//
pub const PFUZE100_SW1AB: c_int = 0;
pub const PFUZE100_SW1C: c_int = 1;
pub const PFUZE100_SW2: c_int = 2;
pub const PFUZE100_SW3A: c_int = 3;
pub const PFUZE100_SW3B: c_int = 4;
pub const PFUZE100_SW4: c_int = 5;
pub const PFUZE100_SWBST: c_int = 6;
pub const PFUZE100_VSNVS: c_int = 7;
pub const PFUZE100_VREFDDR: c_int = 8;
pub const PFUZE100_VGEN1: c_int = 9;
pub const PFUZE100_VGEN2: c_int = 10;
pub const PFUZE100_VGEN3: c_int = 11;
pub const PFUZE100_VGEN4: c_int = 12;
pub const PFUZE100_VGEN5: c_int = 13;
pub const PFUZE100_VGEN6: c_int = 14;
pub const PFUZE100_COIN: c_int = 15;
pub const PFUZE100_MAX_REGULATOR: c_int = 16;
pub const PFUZE200_SW1AB: c_int = 0;
pub const PFUZE200_SW2: c_int = 1;
pub const PFUZE200_SW3A: c_int = 2;
pub const PFUZE200_SW3B: c_int = 3;
pub const PFUZE200_SWBST: c_int = 4;
pub const PFUZE200_VSNVS: c_int = 5;
pub const PFUZE200_VREFDDR: c_int = 6;
pub const PFUZE200_VGEN1: c_int = 7;
pub const PFUZE200_VGEN2: c_int = 8;
pub const PFUZE200_VGEN3: c_int = 9;
pub const PFUZE200_VGEN4: c_int = 10;
pub const PFUZE200_VGEN5: c_int = 11;
pub const PFUZE200_VGEN6: c_int = 12;
pub const PFUZE200_COIN: c_int = 13;
pub const PFUZE3000_SW1A: c_int = 0;
pub const PFUZE3000_SW1B: c_int = 1;
pub const PFUZE3000_SW2: c_int = 2;
pub const PFUZE3000_SW3: c_int = 3;
pub const PFUZE3000_SWBST: c_int = 4;
pub const PFUZE3000_VSNVS: c_int = 5;
pub const PFUZE3000_VREFDDR: c_int = 6;
pub const PFUZE3000_VLDO1: c_int = 7;
pub const PFUZE3000_VLDO2: c_int = 8;
pub const PFUZE3000_VCCSD: c_int = 9;
pub const PFUZE3000_V33: c_int = 10;
pub const PFUZE3000_VLDO3: c_int = 11;
pub const PFUZE3000_VLDO4: c_int = 12;
pub const PFUZE3001_SW1: c_int = 0;
pub const PFUZE3001_SW2: c_int = 1;
pub const PFUZE3001_SW3: c_int = 2;
pub const PFUZE3001_VSNVS: c_int = 3;
pub const PFUZE3001_VLDO1: c_int = 4;
pub const PFUZE3001_VLDO2: c_int = 5;
pub const PFUZE3001_VCCSD: c_int = 6;
pub const PFUZE3001_V33: c_int = 7;
pub const PFUZE3001_VLDO3: c_int = 8;
pub const PFUZE3001_VLDO4: c_int = 9;
