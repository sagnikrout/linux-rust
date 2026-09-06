//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/stv6110.h
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
// stv6110.h
//
// Driver for ST STV6110 satellite tuner IC.
//
// Copyright (C) 2009 NetUP Inc.
// Copyright (C) 2009 Igor M. Liplianin <liplianin@netup.ru>
//

// registers
pub const RSTV6110_CTRL1: c_int = 0;
pub const RSTV6110_CTRL2: c_int = 1;
pub const RSTV6110_TUNING1: c_int = 2;
pub const RSTV6110_TUNING2: c_int = 3;
pub const RSTV6110_CTRL3: c_int = 4;
pub const RSTV6110_STAT1: c_int = 5;
pub const RSTV6110_STAT2: c_int = 6;
pub const RSTV6110_STAT3: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv6110_config {
    pub i2c_address: u8,
    pub mclk: u32,
    pub gain: u8,
    pub /: *mut *mut u8 clk_div; / divisor value for the output clock,
}

