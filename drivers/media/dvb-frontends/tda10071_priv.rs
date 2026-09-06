//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/tda10071_priv.h
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
// NXP TDA10071 + Conexant CX24118A DVB-S/S2 demodulator + tuner driver
//
// Copyright (C) 2011 Antti Palosaari <crope@iki.fi>
//

// Macro flag: #define TDA10071_PRIV

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tda10071_dev {
    pub fe: dvb_frontend,
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub cmd_execute_mutex: mutex,
    pub clk: u32,
    pub i2c_wr_max: u16,
    pub ts_mode: u8,
    pub spec_inv: bool,
    pub pll_multiplier: u8,
    pub tuner_i2c_addr: u8,
    pub meas_count: u8,
    pub dvbv3_ber: u32,
    pub fe_status: fe_status,
    pub delivery_system: fe_delivery_system,
    pub /: *mut *mut bool warm; / FW running,
    pub post_bit_error: u64,
    pub block_error: u64,
}

// NBC-QPSK
// 8PSK
// QPSK
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tda10071_reg_val_mask {
    pub reg: u8,
    pub val: u8,
    pub mask: u8,
}

// firmware filename

// firmware commands
pub const CMD_DEMOD_INIT: c_uint = 0x10;
pub const CMD_CHANGE_CHANNEL: c_uint = 0x11;
pub const CMD_MPEG_CONFIG: c_uint = 0x13;
pub const CMD_TUNER_INIT: c_uint = 0x15;
pub const CMD_GET_AGCACC: c_uint = 0x1a;
pub const CMD_LNB_CONFIG: c_uint = 0x20;
pub const CMD_LNB_SEND_DISEQC: c_uint = 0x21;
pub const CMD_LNB_SET_DC_LEVEL: c_uint = 0x22;
pub const CMD_LNB_PCB_CONFIG: c_uint = 0x23;
pub const CMD_LNB_SEND_TONEBURST: c_uint = 0x24;
pub const CMD_LNB_UPDATE_REPLY: c_uint = 0x25;
pub const CMD_GET_FW_VERSION: c_uint = 0x35;
pub const CMD_SET_SLEEP_MODE: c_uint = 0x36;
pub const CMD_BER_CONTROL: c_uint = 0x3e;
pub const CMD_BER_UPDATE_COUNTERS: c_uint = 0x3f;
// firmware command struct
pub const TDA10071_ARGLEN: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tda10071_cmd {
    pub args: [u8; TDA10071_ARGLEN],
    pub len: u8,
}
