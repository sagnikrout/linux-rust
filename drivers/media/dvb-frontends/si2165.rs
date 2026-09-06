//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/si2165.h
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
// Driver for Silicon Labs SI2165 DVB-C/-T Demodulator
//
// Copyright (C) 2013-2017 Matthias Schwarzott <zzam@gentoo.org>
//
// References:
// https://www.silabs.com/Support%20Documents/TechnicalDocs/Si2165-short.pdf
//

// I2C addresses
// possible values: 0x64,0x65,0x66,0x67
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si2165_platform_data {
//
// frontend
// returned by driver
//
    pub fe: *mut dvb_frontend,
// external clock or XTAL
    pub chip_mode: u8,
// frequency of external clock or xtal in Hz
// possible values: 4000000, 16000000, 20000000, 240000000, 27000000
//
    pub ref_freq_hz: u32,
// invert the spectrum
    pub inversion: bool,
}
