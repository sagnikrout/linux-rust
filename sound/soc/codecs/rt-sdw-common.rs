//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt-sdw-common.h
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
// rt-sdw-common.h
//
// Copyright(c) 2024 Realtek Semiconductor Corp.
//
// This file defines common functions used with Realtek soundwire codecs.
//
pub const SDCA_NUM_JACK_CODEC: c_uint = 0x01;
pub const SDCA_NUM_MIC_ARRAY: c_uint = 0x02;
pub const SDCA_NUM_HID: c_uint = 0x03;
pub const SDCA_NUM_AMP: c_uint = 0x04;
pub const RT_SDCA_CTL_SELECTED_MODE: c_uint = 0x01;
pub const RT_SDCA_CTL_DETECTED_MODE: c_uint = 0x02;
pub const RT_SDCA_CTL_HIDTX_CURRENT_OWNER: c_uint = 0x10;
pub const RT_SDCA_CTL_HIDTX_MESSAGE_OFFSET: c_uint = 0x12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt_sdca_dmic_kctrl_priv {
    pub reg_base: c_uint,
    pub count: c_uint,
    pub max: c_uint,
    pub invert: c_uint,
}

extern "C" {
    pub fn rt_sdca_btn_type(buffer: *mut c_uchar) -> c_int;
}
extern "C" {
    pub fn rt_sdca_headset_detect(map: *mut regmap, entity_id: c_uint) -> c_int;
}
