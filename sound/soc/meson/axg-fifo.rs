//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/meson/axg-fifo.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (c) 2018 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>
//
pub const AXG_FIFO_CH_MAX: c_int = 128;

pub const AXG_FIFO_BURST: c_int = 8;

pub const FIFO_CTRL0: c_uint = 0x00;

pub const CTRL0_SEL_SHIFT: c_int = 0;
pub const FIFO_CTRL1: c_uint = 0x04;

pub const STATUS2_SEL_DDR_READ: c_int = 0;

pub const FIFO_START_ADDR: c_uint = 0x08;
pub const FIFO_FINISH_ADDR: c_uint = 0x0c;
pub const FIFO_INT_ADDR: c_uint = 0x10;
pub const FIFO_STATUS1: c_uint = 0x14;

pub const FIFO_STATUS2: c_uint = 0x18;
pub const FIFO_INIT_ADDR: c_uint = 0x24;
pub const FIFO_CTRL2: c_uint = 0x28;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct axg_fifo {
    pub map: *mut regmap,
    pub pclk: *mut clk,
    pub arb: *mut reset_control,
    pub field_threshold: *mut regmap_field,
    pub depth: c_uint,
    pub irq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct axg_fifo_match_data {
    pub component_drv: *const snd_soc_component_driver,
    pub dai_drv: *mut snd_soc_dai_driver,
    pub field_threshold: reg_field,
}

extern "C" {
    pub fn axg_fifo_pcm_new(rtd: *mut snd_soc_pcm_runtime, type: c_uint) -> c_int;
}
extern "C" {
    pub fn axg_fifo_probe(pdev: *mut platform_device) -> c_int;
}
