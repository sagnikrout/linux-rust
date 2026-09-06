//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/m88ds3103_priv.h
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
// Montage Technology M88DS3103/M88RS6000 demodulator driver
//
// Copyright (C) 2013 Antti Palosaari <crope@iki.fi>
//

pub const M88DS3103_CHIP_ID: c_uint = 0x70;
pub const M88RS6000_CHIP_ID: c_uint = 0x74;
pub const M88DS3103C_CHIP_ID: c_uint = 0x71;
pub const M88DS3103_CHIPTYPE_3103: c_int = 0;
pub const M88DS3103_CHIPTYPE_RS6000: c_int = 1;
pub const M88DS3103_CHIPTYPE_3103B: c_int = 2;
pub const M88DS3103_CHIPTYPE_3103C: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct m88ds3103_dev {
    pub client: *mut i2c_client,
    pub dt_client: *mut i2c_client,
    pub regmap_config: regmap_config,
    pub regmap: *mut regmap,
    pub config: m88ds3103_config,
    pub cfg: *const m88ds3103_config,
    pub fe: dvb_frontend,
    pub delivery_system: fe_delivery_system,
    pub fe_status: fe_status,
    pub /: *mut *mut u32 dvbv3_ber; / for old DVBv3 API read_ber,
    pub /: *mut *mut bool warm; / FW running,
    pub muxc: *mut i2c_mux_core,
// auto detect chip id to do different config
    pub chip_id: u8,
// chip type to differentiate m88rs6000 from m88ds3103b
    pub chiptype: u8,
// main mclk is calculated for M88RS6000 dynamically
    pub mclk: i32,
    pub post_bit_error: u64,
    pub post_bit_count: u64,
    pub dt_addr: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct m88ds3103_reg_val {
    pub reg: u8,
    pub val: u8,
}
