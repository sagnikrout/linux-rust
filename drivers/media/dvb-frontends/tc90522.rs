//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/tc90522.h
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
//
// Toshiba TC90522 Demodulator
//
// Copyright (C) 2014 Akihiro Tsukada <tskd08@gmail.com>
//
// The demod has 4 input (2xISDB-T and 2xISDB-S),
// and provides independent sub modules for each input.
// As the sub modules work in parallel and have the separate i2c addr's,
// this driver treats each sub module as one demod device.
//

// I2C device types

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc90522_config {
// [OUT] frontend returned by driver
    pub fe: *mut dvb_frontend,
// [OUT] tuner I2C adapter returned by driver
    pub tuner_i2c: *mut i2c_adapter,
// [IN] use two separate I2C transactions for one tuner read
    pub split_tuner_read_i2c: bool,
}
