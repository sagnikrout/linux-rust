//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/tea6415c.h
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
// the tea6415c's design is quite brain-dead. although there are
// input pins
pub const TEA6415C_OUTPUT1: c_int = 18;
pub const TEA6415C_OUTPUT2: c_int = 14;
pub const TEA6415C_OUTPUT3: c_int = 16;
pub const TEA6415C_OUTPUT4: c_int = 17;
pub const TEA6415C_OUTPUT5: c_int = 13;
pub const TEA6415C_OUTPUT6: c_int = 15;
// output pins
pub const TEA6415C_INPUT1: c_int = 5;
pub const TEA6415C_INPUT2: c_int = 8;
pub const TEA6415C_INPUT3: c_int = 3;
pub const TEA6415C_INPUT4: c_int = 20;
pub const TEA6415C_INPUT5: c_int = 6;
pub const TEA6415C_INPUT6: c_int = 10;
pub const TEA6415C_INPUT7: c_int = 1;
pub const TEA6415C_INPUT8: c_int = 11;
