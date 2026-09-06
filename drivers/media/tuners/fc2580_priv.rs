//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/fc2580_priv.h
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
// FCI FC2580 silicon tuner driver
//
// Copyright (C) 2012 Antti Palosaari <crope@iki.fi>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc2580_reg_val {
    pub reg: u8,
    pub val: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc2580_pll {
    pub freq: u32,
    pub div_out: u8,
    pub band: u8,
}

// VCO min    VCO max
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc2580_if_filter {
    pub freq: u32,
    pub r36_val: u8,
    pub r39_val: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc2580_freq_regs {
    pub freq: u32,
    pub r25_val: u8,
    pub r27_val: u8,
    pub r28_val: u8,
    pub r29_val: u8,
    pub r2b_val: u8,
    pub r2c_val: u8,
    pub r2d_val: u8,
    pub r30_val: u8,
    pub r44_val: u8,
    pub r50_val: u8,
    pub r53_val: u8,
    pub r5f_val: u8,
    pub r61_val: u8,
    pub r62_val: u8,
    pub r63_val: u8,
    pub r67_val: u8,
    pub r68_val: u8,
    pub r69_val: u8,
    pub r6a_val: u8,
    pub r6b_val: u8,
    pub r6c_val: u8,
    pub r6d_val: u8,
    pub r6e_val: u8,
    pub r6f_val: u8,
}

// XXX: 0xff is used for don't-care!
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc2580_dev {
    pub clk: u32,
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub subdev: v4l2_subdev,
    pub active: bool,
    pub f_frequency: c_uint,
    pub f_bandwidth: c_uint,
// Controls
    pub hdl: v4l2_ctrl_handler,
    pub bandwidth_auto: *mut v4l2_ctrl,
    pub bandwidth: *mut v4l2_ctrl,
}
