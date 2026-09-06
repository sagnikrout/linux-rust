//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/i2c-algo-bit.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// i2c-algo-bit.h: i2c driver algorithms for bit-shift adapters
//
// Copyright (C) 1995-99 Simon G. Vogl
// With some changes from Kyösti Mälkki <kmalkki@cc.hut.fi> and even
// Frodo Looijaard <frodol@dds.nl>
//

// --- Defines for bit-adapters ---------------------------------------
//
// This struct contains the hw-dependent functions of bit-style adapters to
// manipulate the line states, and to init any hw-specific features. This is
// only used if you have more than one hw-type of adapter running.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_algo_bit_data {
    pub /: *mut *mut *mut void data; / private data for lowlevel routines,
    pub state): *mut *mut *mut void (setsda) (void data, int,
    pub state): *mut *mut *mut void (setscl) (void data, int,
    pub data): *mut *mut int (getsda) (void,
    pub data): *mut *mut int (getscl) (void,
    pub ): *mut *mut int (pre_xfer) (struct i2c_adapter,
    pub ): *mut *mut void (post_xfer) (struct i2c_adapter,
// local settings
    pub us,: *mut *mut int udelay; / half clock cycle time in,
    pub /: *mut *mut int timeout; / in jiffies,
    pub /: *mut *mut bool can_do_atomic; / callbacks don't sleep, we can be atomic,
    pub /: *mut *mut bool skip_bit_test; / override bit_test module parameter,
}

extern "C" {
    pub fn i2c_bit_add_bus(: *mut i2c_adapter) -> c_int;
}
extern "C" {
    pub fn i2c_bit_add_numbered_bus(: *mut i2c_adapter) -> c_int;
}
