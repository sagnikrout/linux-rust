//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/si2168_priv.h
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
// Silicon Labs Si2168 DVB-T/T2/C demodulator driver
//
// Copyright (C) 2014 Antti Palosaari <crope@iki.fi>
//

// state struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si2168_dev {
    pub i2c_mutex: mutex,
    pub muxc: *mut i2c_mux_core,
    pub fe: dvb_frontend,
    pub delivery_system: fe_delivery_system,
    pub fe_status: fe_status,

    pub chip_id: c_uint,
    pub version: c_uint,
    pub firmware_name: *const c_char,
    pub ts_mode: u8,
    pub active:1: c_uint,
    pub warm:1: c_uint,
    pub initialized:1: c_uint,
    pub ts_clock_inv:1: c_uint,
    pub ts_clock_gapped:1: c_uint,
    pub spectral_inversion:1: c_uint,
}

// firmware command struct
pub const SI2168_ARGLEN: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si2168_cmd {
    pub args: [u8; SI2168_ARGLEN],
    pub wlen: unsigned,
    pub rlen: unsigned,
}
