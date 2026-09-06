//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/e4000_priv.h
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
// Elonics E4000 silicon tuner driver
//
// Copyright (C) 2012 Antti Palosaari <crope@iki.fi>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e4000_dev {
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub clk: u32,
    pub fe: *mut dvb_frontend,
    pub sd: v4l2_subdev,
    pub active: bool,
    pub f_frequency: c_uint,
    pub f_bandwidth: c_uint,
// Controls
    pub hdl: v4l2_ctrl_handler,
    pub bandwidth_auto: *mut v4l2_ctrl,
    pub bandwidth: *mut v4l2_ctrl,
    pub lna_gain_auto: *mut v4l2_ctrl,
    pub lna_gain: *mut v4l2_ctrl,
    pub mixer_gain_auto: *mut v4l2_ctrl,
    pub mixer_gain: *mut v4l2_ctrl,
    pub if_gain_auto: *mut v4l2_ctrl,
    pub if_gain: *mut v4l2_ctrl,
    pub pll_lock: *mut v4l2_ctrl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e4000_pll {
    pub freq: u32,
    pub div_out_reg: u8,
    pub div_out: u8,
}

// VCO min    VCO max
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e4000_lna_filter {
    pub freq: u32,
    pub val: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e4000_band {
    pub freq: u32,
    pub reg07_val: u8,
    pub reg78_val: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e4000_if_filter {
    pub freq: u32,
    pub reg11_val: u8,
    pub reg12_val: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e4000_if_gain {
    pub reg16_val: u8,
    pub reg17_val: u8,
}
