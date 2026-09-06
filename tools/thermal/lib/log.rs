//! Automatically rewritten from C Header to Rust Module
//! Source: tools/thermal/lib/log.h
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


// SPDX-License-Identifier: LGPL-2.1+
// Copyright (C) 2022, Linaro Ltd - Daniel Lezcano <daniel.lezcano@linaro.org>

pub const TO_SYSLOG: c_uint = 0x1;
pub const TO_STDOUT: c_uint = 0x2;
pub const TO_STDERR: c_uint = 0x4;
extern "C" {
    pub fn logit(level: c_int, format: *const c_char, ...);
}

extern "C" {
    pub fn log_init(level: c_int, ident: *const c_char, options: c_int) -> c_int;
}
extern "C" {
    pub fn log_str2level(lvl: *const c_char) -> c_int;
}
extern "C" {
    pub fn log_exit();
}
