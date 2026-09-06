//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/adau7118.h
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
// register map
pub const ADAU7118_REG_VENDOR_ID: c_uint = 0x00;
pub const ADAU7118_REG_DEVICE_ID1: c_uint = 0x01;
pub const ADAU7118_REG_DEVICE_ID2: c_uint = 0x02;
pub const ADAU7118_REG_REVISION_ID: c_uint = 0x03;
pub const ADAU7118_REG_ENABLES: c_uint = 0x04;
pub const ADAU7118_REG_DEC_RATIO_CLK_MAP: c_uint = 0x05;
pub const ADAU7118_REG_HPF_CONTROL: c_uint = 0x06;
pub const ADAU7118_REG_SPT_CTRL1: c_uint = 0x07;
pub const ADAU7118_REG_SPT_CTRL2: c_uint = 0x08;

pub const ADAU7118_REG_DRIVE_STRENGTH: c_uint = 0x11;
pub const ADAU7118_REG_RESET: c_uint = 0x12;
extern "C" {
    pub fn adau7118_probe(dev: *mut device, map: *mut regmap, hw_mode: bool) -> c_int;
}
