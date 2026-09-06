//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/cxd2820r_priv.h
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
// Sony CXD2820R demodulator driver
//
// Copyright (C) 2010 Antti Palosaari <crope@iki.fi>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_val_mask {
    pub reg: u32,
    pub val: u8,
    pub mask: u8,
}

pub const CXD2820R_CLK: c_int = 41000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2820r_priv {
    pub client: [*mut i2c_client; 2],
    pub regmap: [*mut regmap; 2],
    pub i2c: *mut i2c_adapter,
    pub fe: dvb_frontend,
    pub ts_mode: u8,
    pub ts_clk_inv: bool,
    pub if_agc_polarity: bool,
    pub spec_inv: bool,
    pub post_bit_error_prev_dvbv3: u64,
    pub post_bit_error: u64,
    pub ber_running: bool,
pub const GPIO_COUNT: c_int = 3;
    pub gpio: [u8; GPIO_COUNT],
    pub gpio_chip: gpio_chip,

    pub delivery_system: fe_delivery_system,
    pub /: *mut *mut bool last_tune_failed; / for switch between T and T2 tune,
}

// cxd2820r_core.c
extern "C" {
    pub fn cxd2820r_gpio(fe: *mut dvb_frontend, gpio: *mut u8) -> c_int;
}
extern "C" {
    pub fn cxd2820r_wr_reg(priv: *mut cxd2820r_priv, reg: u32, val: u8) -> c_int;
}
extern "C" {
    pub fn cxd2820r_rd_reg(priv: *mut cxd2820r_priv, reg: u32, val: *mut u8) -> c_int;
}
// cxd2820r_c.c
extern "C" {
    pub fn cxd2820r_set_frontend_c(fe: *mut dvb_frontend) -> c_int;
}
extern "C" {
    pub fn cxd2820r_read_status_c(fe: *mut dvb_frontend, status: *mut fe_status) -> c_int;
}
extern "C" {
    pub fn cxd2820r_init_c(fe: *mut dvb_frontend) -> c_int;
}
extern "C" {
    pub fn cxd2820r_sleep_c(fe: *mut dvb_frontend) -> c_int;
}
// cxd2820r_t.c
extern "C" {
    pub fn cxd2820r_set_frontend_t(fe: *mut dvb_frontend) -> c_int;
}
extern "C" {
    pub fn cxd2820r_read_status_t(fe: *mut dvb_frontend, status: *mut fe_status) -> c_int;
}
extern "C" {
    pub fn cxd2820r_init_t(fe: *mut dvb_frontend) -> c_int;
}
extern "C" {
    pub fn cxd2820r_sleep_t(fe: *mut dvb_frontend) -> c_int;
}
// cxd2820r_t2.c
extern "C" {
    pub fn cxd2820r_set_frontend_t2(fe: *mut dvb_frontend) -> c_int;
}
extern "C" {
    pub fn cxd2820r_read_status_t2(fe: *mut dvb_frontend, status: *mut fe_status) -> c_int;
}
extern "C" {
    pub fn cxd2820r_init_t2(fe: *mut dvb_frontend) -> c_int;
}
extern "C" {
    pub fn cxd2820r_sleep_t2(fe: *mut dvb_frontend) -> c_int;
}
