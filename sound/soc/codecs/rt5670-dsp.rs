//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt5670-dsp.h
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
// rt5670-dsp.h  --  RT5670 ALSA SoC DSP driver
//
// Copyright 2014 Realtek Microelectronics
// Author: Bard Liao <bardliao@realtek.com>
//
pub const RT5670_DSP_CTRL1: c_uint = 0xe0;
pub const RT5670_DSP_CTRL2: c_uint = 0xe1;
pub const RT5670_DSP_CTRL3: c_uint = 0xe2;
pub const RT5670_DSP_CTRL4: c_uint = 0xe3;
pub const RT5670_DSP_CTRL5: c_uint = 0xe4;
// DSP Control 1 (0xe0)

pub const RT5670_DSP_CLK_SFT: c_int = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt5670_dsp_param {
    pub cmd_fmt: u16,
    pub addr: u16,
    pub data: u16,
    pub cmd: u8,
}
