//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rl6347a.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// rl6347a.h - RL6347A class device shared support
//
// Copyright 2015 Realtek Semiconductor Corp.
//
// Author: Oder Chiou <oder_chiou@realtek.com>
//

pub const RL6347A_VENDOR_REGISTERS: c_uint = 0x20;
// Macro flag: #define RL6347A_COEF_INDEX\
// Macro flag: #define RL6347A_PROC_COEF\
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rl6347a_priv {
    pub index_cache: *mut reg_default,
    pub index_cache_size: c_int,
}

extern "C" {
    pub fn rl6347a_hw_write(context: *mut c_void, reg: c_uint, value: c_uint) -> c_int;
}
extern "C" {
    pub fn rl6347a_hw_read(context: *mut c_void, reg: c_uint, value: *mut c_uint) -> c_int;
}
