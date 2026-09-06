//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/xc4000.h
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
// Driver for Xceive XC4000 "QAM/8VSB single chip tuner"
//
// Copyright (c) 2007 Steven Toth <stoth@linuxtv.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xc4000_config {
    pub i2c_address: u8,
// if non-zero, power management is enabled by default
    pub default_pm: u8,
// value to be written to XREG_AMPLITUDE in DVB-T mode (0: no write)
    pub dvb_amplitude: u8,
// if non-zero, register 0x0E is set to filter analog TV video output
    pub set_smoothedcvbs: u8,
// IF for DVB-T
    pub if_khz: u32,
}

// xc4000 callback command
pub const XC4000_TUNER_RESET: c_int = 0;
// For each bridge framework, when it attaches either analog or digital,
// it has to store a reference back to its _core equivalent structure,
// so that it can service the hardware by steering gpio's etc.
// Each bridge implementation is different so cast devptr accordingly.
// The xc4000 driver cares not for this value, other than ensuring
// it's passed back to a bridge during tuner_callback().
//

