//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/iio/temperature/thermocouple.h
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
pub const THERMOCOUPLE_TYPE_B: c_uint = 0x00;
pub const THERMOCOUPLE_TYPE_E: c_uint = 0x01;
pub const THERMOCOUPLE_TYPE_J: c_uint = 0x02;
pub const THERMOCOUPLE_TYPE_K: c_uint = 0x03;
pub const THERMOCOUPLE_TYPE_N: c_uint = 0x04;
pub const THERMOCOUPLE_TYPE_R: c_uint = 0x05;
pub const THERMOCOUPLE_TYPE_S: c_uint = 0x06;
pub const THERMOCOUPLE_TYPE_T: c_uint = 0x07;
