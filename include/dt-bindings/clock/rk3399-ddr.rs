//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/rk3399-ddr.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR MIT)
//
// DDR3 SDRAM Standard Speed Bins include tCK, tRCD, tRP, tRAS and tRC for
// each corresponding bin.
//
// DDR3-800 (5-5-5)
pub const DDR3_800D: c_int = 0;
// DDR3-800 (6-6-6)
pub const DDR3_800E: c_int = 1;
// DDR3-1066 (6-6-6)
pub const DDR3_1066E: c_int = 2;
// DDR3-1066 (7-7-7)
pub const DDR3_1066F: c_int = 3;
// DDR3-1066 (8-8-8)
pub const DDR3_1066G: c_int = 4;
// DDR3-1333 (7-7-7)
pub const DDR3_1333F: c_int = 5;
// DDR3-1333 (8-8-8)
pub const DDR3_1333G: c_int = 6;
// DDR3-1333 (9-9-9)
pub const DDR3_1333H: c_int = 7;
// DDR3-1333 (10-10-10)
pub const DDR3_1333J: c_int = 8;
// DDR3-1600 (8-8-8)
pub const DDR3_1600G: c_int = 9;
// DDR3-1600 (9-9-9)
pub const DDR3_1600H: c_int = 10;
// DDR3-1600 (10-10-10)
pub const DDR3_1600J: c_int = 11;
// DDR3-1600 (11-11-11)
pub const DDR3_1600K: c_int = 12;
// DDR3-1600 (10-10-10)
pub const DDR3_1866J: c_int = 13;
// DDR3-1866 (11-11-11)
pub const DDR3_1866K: c_int = 14;
// DDR3-1866 (12-12-12)
pub const DDR3_1866L: c_int = 15;
// DDR3-1866 (13-13-13)
pub const DDR3_1866M: c_int = 16;
// DDR3-2133 (11-11-11)
pub const DDR3_2133K: c_int = 17;
// DDR3-2133 (12-12-12)
pub const DDR3_2133L: c_int = 18;
// DDR3-2133 (13-13-13)
pub const DDR3_2133M: c_int = 19;
// DDR3-2133 (14-14-14)
pub const DDR3_2133N: c_int = 20;
// DDR3 ATF default
pub const DDR3_DEFAULT: c_int = 21;
