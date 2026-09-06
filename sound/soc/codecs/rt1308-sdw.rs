//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt1308-sdw.h
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
// rt1308-sdw.h -- RT1308 ALSA SoC audio driver header
//
// Copyright(c) 2019 Realtek Semiconductor Corp.
//
pub const RT1308_SDW_OFFSET: c_uint = 0xc000;
pub const RT1308_SDW_OFFSET_BYTE0: c_uint = 0xc000;
pub const RT1308_SDW_OFFSET_BYTE1: c_uint = 0xc001;
pub const RT1308_SDW_OFFSET_BYTE2: c_uint = 0xc002;
pub const RT1308_SDW_OFFSET_BYTE3: c_uint = 0xc003;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt1308_sdw_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub sdw_slave: *mut sdw_slave,
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub rx_mask: c_int,
    pub slots: c_int,
    pub hw_ver: c_int,
    pub bq_params: *mut c_uchar,
    pub bq_params_cnt: c_uint,
}
