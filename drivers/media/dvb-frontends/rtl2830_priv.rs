//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/rtl2830_priv.h
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
// Realtek RTL2830 DVB-T demodulator driver
//
// Copyright (C) 2011 Antti Palosaari <crope@iki.fi>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl2830_dev {
    pub pdata: *mut rtl2830_platform_data,
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub muxc: *mut i2c_mux_core,
    pub fe: dvb_frontend,
    pub sleeping: bool,
    pub filters: c_ulong,
    pub fe_status: fe_status,
    pub /: *mut *mut u64 post_bit_error_prev; / for old DVBv3 read_ber() calculation,
    pub post_bit_error: u64,
    pub post_bit_count: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl2830_reg_val_mask {
    pub reg: u16,
    pub val: u8,
    pub mask: u8,
}
