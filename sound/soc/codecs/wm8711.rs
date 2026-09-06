//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8711.h
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
// wm8711.h  --  WM8711 Soc Audio driver
//
// Copyright 2006 Wolfson Microelectronics
//
// Author: Mike Arthur <linux@wolfsonmicro.com>
//
// Based on wm8731.h
//
// WM8711 register space
pub const WM8711_LOUT1V: c_uint = 0x02;
pub const WM8711_ROUT1V: c_uint = 0x03;
pub const WM8711_APANA: c_uint = 0x04;
pub const WM8711_APDIGI: c_uint = 0x05;
pub const WM8711_PWR: c_uint = 0x06;
pub const WM8711_IFACE: c_uint = 0x07;
pub const WM8711_SRATE: c_uint = 0x08;
pub const WM8711_ACTIVE: c_uint = 0x09;
pub const WM8711_RESET: c_uint = 0x0f;
pub const WM8711_CACHEREGNUM: c_int = 8;
pub const WM8711_SYSCLK: c_int = 0;
pub const WM8711_DAI: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8711_setup_data {
    pub i2c_address: c_ushort,
}
