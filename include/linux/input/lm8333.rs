//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/input/lm8333.h
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
// public include for LM8333 keypad driver - same license as driver
// Copyright (C) 2012 Wolfram Sang, Pengutronix <kernel@pengutronix.de>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm8333_platform_data {
// Keymap data
    pub matrix_data: *const matrix_keymap_data,
// Active timeout before enter HALT mode in microseconds
    pub active_time: unsigned,
// Debounce interval in microseconds
    pub debounce_time: unsigned,
}

extern "C" {
    pub fn lm8333_read8(lm8333: *mut lm8333, cmd: u8) -> c_int;
}
extern "C" {
    pub fn lm8333_write8(lm8333: *mut lm8333, cmd: u8, val: u8) -> c_int;
}
extern "C" {
    pub fn lm8333_read_block(lm8333: *mut lm8333, cmd: u8, len: u8, buf: *mut u8) -> c_int;
}
