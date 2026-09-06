//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/meson/axg-tdm.h
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
// Copyright (c) 2018 Baylibre SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>
//

pub const AXG_TDM_NUM_LANES: c_int = 4;
pub const AXG_TDM_CHANNEL_MAX: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct axg_tdm_iface {
    pub sclk: *mut clk,
    pub lrclk: *mut clk,
    pub mclk: *mut clk,
    pub mclk_rate: c_ulong,
// format is common to all the DAIs of the iface
    pub fmt: c_uint,
    pub slots: c_uint,
    pub slot_width: c_uint,
// For component wide symmetry
    pub rate: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct axg_tdm_stream {
    pub iface: *mut axg_tdm_iface,
    pub formatter_list: list_head,
    pub lock: mutex,
    pub channels: c_uint,
    pub width: c_uint,
    pub physical_width: c_uint,
    pub mask: *mut u32,
    pub ready: bool,
// For continuous clock tracking
    pub clk_enabled: bool,
}

extern "C" {
    pub fn axg_tdm_stream_free(ts: *mut axg_tdm_stream);
}
extern "C" {
    pub fn axg_tdm_stream_start(ts: *mut axg_tdm_stream) -> c_int;
}
extern "C" {
    pub fn axg_tdm_stream_stop(ts: *mut axg_tdm_stream);
}
extern "C" {
    pub fn axg_tdm_stream_start(_arg: ts) -> return;
}
