//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/iio/addac/adi,ad74413r.h
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
pub const CH_FUNC_HIGH_IMPEDANCE: c_uint = 0x0;
pub const CH_FUNC_VOLTAGE_OUTPUT: c_uint = 0x1;
pub const CH_FUNC_CURRENT_OUTPUT: c_uint = 0x2;
pub const CH_FUNC_VOLTAGE_INPUT: c_uint = 0x3;
pub const CH_FUNC_CURRENT_INPUT_EXT_POWER: c_uint = 0x4;
pub const CH_FUNC_CURRENT_INPUT_LOOP_POWER: c_uint = 0x5;
pub const CH_FUNC_RESISTANCE_INPUT: c_uint = 0x6;
pub const CH_FUNC_DIGITAL_INPUT_LOGIC: c_uint = 0x7;
pub const CH_FUNC_DIGITAL_INPUT_LOOP_POWER: c_uint = 0x8;
pub const CH_FUNC_CURRENT_INPUT_EXT_POWER_HART: c_uint = 0x9;
pub const CH_FUNC_CURRENT_INPUT_LOOP_POWER_HART: c_uint = 0xA;

