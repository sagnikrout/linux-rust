//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/media/si4713.h
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
// include/linux/platform_data/media/si4713.h
//
// Board related data definitions for Si4713 i2c device driver.
//
// Copyright (c) 2009 Nokia Corporation
// Contact: Eduardo Valentin <eduardo.valentin@nokia.com>
//
// This file is licensed under the terms of the GNU General Public License
// version 2. This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//
// The SI4713 I2C sensor chip has a fixed slave address of 0xc6 or 0x22.
pub const SI4713_I2C_ADDR_BUSEN_HIGH: c_uint = 0x63;
pub const SI4713_I2C_ADDR_BUSEN_LOW: c_uint = 0x11;
//
// Platform dependent definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si4713_platform_data {
    pub is_platform_device: bool,
}

//
// Structure to query for Received Noise Level (RNL).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si4713_rnl {
    pub /: *mut *mut __u32 index; / modulator index,
    pub /: *mut *mut __u32 frequency; / frequency to perform rnl measurement,
    pub /: *mut *mut __s32 rnl; / result of measurement in dBuV,
    pub /: *mut *mut __u32 reserved[4]; / drivers and apps must init this to 0,
}

//
// This is the ioctl number to query for rnl. Users must pass a
// struct si4713_rnl pointer specifying desired frequency in 'frequency' field
// following driver capabilities (i.e V4L2_TUNER_CAP_LOW).
// Driver must return measured value in the same structure, filling 'rnl' field.
//

