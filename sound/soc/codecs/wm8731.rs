//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8731.h
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
// wm8731.h  --  WM8731 Soc Audio driver
//
// Copyright 2005 Openedhand Ltd.
//
// Author: Richard Purdie <richard@openedhand.com>
//
// Based on wm8753.h
//

// WM8731 register space
pub const WM8731_LINVOL: c_uint = 0x00;
pub const WM8731_RINVOL: c_uint = 0x01;
pub const WM8731_LOUT1V: c_uint = 0x02;
pub const WM8731_ROUT1V: c_uint = 0x03;
pub const WM8731_APANA: c_uint = 0x04;
pub const WM8731_APDIGI: c_uint = 0x05;
pub const WM8731_PWR: c_uint = 0x06;
pub const WM8731_IFACE: c_uint = 0x07;
pub const WM8731_SRATE: c_uint = 0x08;
pub const WM8731_ACTIVE: c_uint = 0x09;
pub const WM8731_RESET: c_uint = 0x0f;
pub const WM8731_CACHEREGNUM: c_int = 10;
pub const WM8731_SYSCLK_MCLK: c_int = 0;
pub const WM8731_SYSCLK_XTAL: c_int = 1;
pub const WM8731_DAI: c_int = 0;
pub const WM8731_NUM_SUPPLIES: c_int = 4;
// codec private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8731_priv {
    pub regmap: *mut regmap,
    pub mclk: *mut clk,
    pub supplies: [regulator_bulk_data; WM8731_NUM_SUPPLIES],
    pub constraints: *const snd_pcm_hw_constraint_list,
    pub sysclk: c_uint,
    pub sysclk_type: c_int,
    pub playback_fs: c_int,
    pub deemph: bool,
    pub lock: mutex,
}

extern "C" {
    pub fn wm8731_init(dev: *mut device, wm8731: *mut wm8731_priv) -> c_int;
}
