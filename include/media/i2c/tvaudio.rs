//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/i2c/tvaudio.h
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
// i2c bus addresses for the chips supported by tvaudio.c
//
pub const I2C_ADDR_TDA8425: c_uint = 0x82;
pub const I2C_ADDR_TDA9840: c_uint = 0x84;
pub const I2C_ADDR_TDA9874: c_uint = 0xb0 /* also used by 9875 */;
pub const I2C_ADDR_TDA9875: c_uint = 0xb0;
pub const I2C_ADDR_TDA8425: c_uint = 0x82;
pub const I2C_ADDR_TDA9840: c_uint = 0x84 /* also used by TA8874Z */;
pub const I2C_ADDR_TDA985x_L: c_uint = 0xb4 /* also used by 9873 */;
pub const I2C_ADDR_TDA985x_H: c_uint = 0xb6;
pub const I2C_ADDR_TDA9874: c_uint = 0xb0 /* also used by 9875 */;
pub const I2C_ADDR_TEA6300: c_uint = 0x80 /* also used by 6320 */;
pub const I2C_ADDR_TEA6420: c_uint = 0x98;
pub const I2C_ADDR_PIC16C54: c_uint = 0x96 /* PV951 */;
// The tvaudio module accepts the following inputs:
pub const TVAUDIO_INPUT_TUNER: c_int = 0;
pub const TVAUDIO_INPUT_RADIO: c_int = 1;
pub const TVAUDIO_INPUT_EXTERN: c_int = 2;
pub const TVAUDIO_INPUT_INTERN: c_int = 3;
