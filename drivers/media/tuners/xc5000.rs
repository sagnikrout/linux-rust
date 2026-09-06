//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/xc5000.h
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
// Driver for Xceive XC5000 "QAM/8VSB single chip tuner"
//
// Copyright (c) 2007 Steven Toth <stoth@linuxtv.org>
//

pub const XC5000A: c_int = 1;
pub const XC5000C: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xc5000_config {
    pub i2c_address: u8,
    pub if_khz: u32,
    pub radio_input: u8,
    pub xtal_khz: u16,
    pub output_amp: u16,
    pub chip_id: c_int,
}

// xc5000 callback command
pub const XC5000_TUNER_RESET: c_int = 0;
// Possible Radio inputs
pub const XC5000_RADIO_NOT_CONFIGURED: c_int = 0;
pub const XC5000_RADIO_FM1: c_int = 1;
pub const XC5000_RADIO_FM2: c_int = 2;
pub const XC5000_RADIO_FM1_MONO: c_int = 3;
// For each bridge framework, when it attaches either analog or digital,
// it has to store a reference back to its _core equivalent structure,
// so that it can service the hardware by steering gpio's etc.
// Each bridge implementation is different so cast devptr accordingly.
// The xc5000 driver cares not for this value, other than ensuring
// it's passed back to a bridge during tuner_callback().
//

