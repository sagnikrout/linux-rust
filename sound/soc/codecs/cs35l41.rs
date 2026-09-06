//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/cs35l41.h
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
//
// cs35l41.h -- CS35L41 ALSA SoC audio driver
//
// Copyright 2017-2021 Cirrus Logic, Inc.
//
// Author: David Rhodes <david.rhodes@cirrus.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l41_private {
    pub /: *mut *mut wm_adsp dsp; / needs to be first member,
    pub codec: *mut snd_soc_codec,
    pub hw_cfg: cs35l41_hw_cfg,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub supplies: [regulator_bulk_data; CS35L41_NUM_SUPPLIES],
    pub irq: c_int,
// GPIO for /RST
    pub reset_gpio: *mut gpio_desc,
}

extern "C" {
    pub fn cs35l41_probe(cs35l41: *mut cs35l41_private, hw_cfg: *const cs35l41_hw_cfg) -> c_int;
}
extern "C" {
    pub fn cs35l41_remove(cs35l41: *mut cs35l41_private);
}
