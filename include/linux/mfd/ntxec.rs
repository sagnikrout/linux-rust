//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/ntxec.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2020 Jonathan Neuschäfer
//
// Register access and version information for the Netronix embedded
// controller.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntxec {
    pub dev: *mut device,
    pub regmap: *mut regmap,
}

//
// Some registers, such as the battery status register (0x41), are in
// big-endian, but others only have eight significant bits, which are in the
// first byte transmitted over I2C (the MSB of the big-endian value).
// This convenience function converts an 8-bit value to 16-bit for use in the
// second kind of register.
//
// Known firmware versions
pub const NTXEC_VERSION_KOBO_AURA: c_uint = 0xd726	/* found in Kobo Aura */;
pub const NTXEC_VERSION_TOLINO_SHINE2: c_uint = 0xf110 /* found in Tolino Shine 2 HD */;
pub const NTXEC_VERSION_TOLINO_VISION: c_uint = 0xe135 /* found in Tolino Vision, contains RTC, ADC, PWM, home pad */;
