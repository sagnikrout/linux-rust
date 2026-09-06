//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/amba/sp810.h
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
// ARM PrimeXsys System Controller SP810 header file
//
// Copyright (C) 2009 ST Microelectronics
// Viresh Kumar <vireshk@kernel.org>
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

// sysctl registers offset
pub const SCCTRL: c_uint = 0x000;
pub const SCSYSSTAT: c_uint = 0x004;
pub const SCIMCTRL: c_uint = 0x008;
pub const SCIMSTAT: c_uint = 0x00C;
pub const SCXTALCTRL: c_uint = 0x010;
pub const SCPLLCTRL: c_uint = 0x014;
pub const SCPLLFCTRL: c_uint = 0x018;
pub const SCPERCTRL0: c_uint = 0x01C;
pub const SCPERCTRL1: c_uint = 0x020;
pub const SCPEREN: c_uint = 0x024;
pub const SCPERDIS: c_uint = 0x028;
pub const SCPERCLKEN: c_uint = 0x02C;
pub const SCPERSTAT: c_uint = 0x030;
pub const SCSYSID0: c_uint = 0xEE0;
pub const SCSYSID1: c_uint = 0xEE4;
pub const SCSYSID2: c_uint = 0xEE8;
pub const SCSYSID3: c_uint = 0xEEC;
pub const SCITCR: c_uint = 0xF00;
pub const SCITIR0: c_uint = 0xF04;
pub const SCITIR1: c_uint = 0xF08;
pub const SCITOR: c_uint = 0xF0C;
pub const SCCNTCTRL: c_uint = 0xF10;
pub const SCCNTDATA: c_uint = 0xF14;
pub const SCCNTSTEP: c_uint = 0xF18;
pub const SCPERIPHID0: c_uint = 0xFE0;
pub const SCPERIPHID1: c_uint = 0xFE4;
pub const SCPERIPHID2: c_uint = 0xFE8;
pub const SCPERIPHID3: c_uint = 0xFEC;
pub const SCPCELLID0: c_uint = 0xFF0;
pub const SCPCELLID1: c_uint = 0xFF4;
pub const SCPCELLID2: c_uint = 0xFF8;
pub const SCPCELLID3: c_uint = 0xFFC;

// switch to slow mode
// writing any value to SCSYSSTAT reg will reset system
