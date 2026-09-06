//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/mt2131_priv.h
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
// Driver for Microtune MT2131 "QAM/8VSB single chip tuner"
//
// Copyright (c) 2006 Steven Toth <stoth@linuxtv.org>
//
// Regs
pub const MT2131_PWR: c_uint = 0x07;
pub const MT2131_UPC_1: c_uint = 0x0b;
pub const MT2131_AGC_RL: c_uint = 0x10;
pub const MT2131_MISC_2: c_uint = 0x15;
// frequency values in KHz
pub const MT2131_IF1: c_int = 1220;
pub const MT2131_IF2: c_int = 44000;
pub const MT2131_FREF: c_int = 16000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt2131_priv {
    pub cfg: *mut mt2131_config,
    pub i2c: *mut i2c_adapter,
    pub frequency: u32,
}
