//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/tua9001.h
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
// Infineon TUA9001 silicon tuner driver
//
// Copyright (C) 2009 Antti Palosaari <crope@iki.fi>
//

//
// I2C address
// 0x60,
//
// struct tua9001_platform_data - Platform data for the tua9001 driver
// @dvb_frontend: DVB frontend.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tua9001_platform_data {
    pub dvb_frontend: *mut dvb_frontend,
}

//
// TUA9001 I/O PINs:
//
// CEN - chip enable
// 0 = chip disabled (chip off)
// 1 = chip enabled (chip on)
//
// RESETN - chip reset
// 0 = reset disabled (chip reset off)
// 1 = reset enabled (chip reset on)
//
// RXEN - RX enable
// 0 = RX disabled (chip idle)
// 1 = RX enabled (chip tuned)
//
pub const TUA9001_CMD_CEN: c_int = 0;
pub const TUA9001_CMD_RESETN: c_int = 1;
pub const TUA9001_CMD_RXEN: c_int = 2;
