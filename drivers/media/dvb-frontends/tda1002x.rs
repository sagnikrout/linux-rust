//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/tda1002x.h
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

// Macro flag: #define TDA1002x_H

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tda1002x_config {
// the demodulator's i2c address
    pub demod_address: u8,
    pub invert: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tda10023_output_mode {
    TDA10023_OUTPUT_MODE_PARALLEL_A = 0xe0,
    TDA10023_OUTPUT_MODE_PARALLEL_B = 0xa1,
    TDA10023_OUTPUT_MODE_PARALLEL_C = 0xa0,
    TDA10023_OUTPUT_MODE_SERIAL, /* TODO: not implemented */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tda10023_config {
// the demodulator's i2c address
    pub demod_address: u8,
    pub invert: u8,
// clock settings
    pub /: *mut *mut u32 xtal; / defaults: 28920000,
    pub /: *mut *mut u8 pll_m; / defaults: 8,
    pub /: *mut *mut u8 pll_p; / defaults: 4,
    pub /: *mut *mut u8 pll_n; / defaults: 1,
// MPEG2 TS output mode
    pub output_mode: u8,
// input freq offset + baseband conversion type
    pub deltaf: u16,
}

