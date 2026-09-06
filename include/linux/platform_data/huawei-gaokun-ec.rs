//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/huawei-gaokun-ec.h
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
// Huawei Matebook E Go Embedded Controller
//
// Copyright (C) 2024-2025 Pengyu Luo <mitltlatltl@gmail.com>
//
pub const GAOKUN_UCSI_CCI_SIZE: c_int = 4;
pub const GAOKUN_UCSI_MSGI_SIZE: c_int = 16;

// --------------------------------------------------------------------------

// --------------------------------------------------------------------------
// Common API
extern "C" {
    pub fn gaokun_ec_write(ec: *mut gaokun_ec, req: *const u8) -> c_int;
}
extern "C" {
    pub fn gaokun_ec_read_byte(ec: *mut gaokun_ec, req: *const u8, byte: *mut u8) -> c_int;
}
// --------------------------------------------------------------------------
// API for PSY
extern "C" {
    pub fn gaokun_ec_psy_multi_read(_arg: ec, _arg: reg, _arg: *mut sizeof(byte), _arg: byte) -> return;
}
extern "C" {
    pub fn gaokun_ec_psy_multi_read(_arg: ec, _arg: reg, _arg: *mut sizeof(word), )word: *mut (u8) -> return;
}
extern "C" {
    pub fn gaokun_ec_psy_get_smart_charge_enable(ec: *mut gaokun_ec, on: *mut bool) -> c_int;
}
extern "C" {
    pub fn gaokun_ec_psy_set_smart_charge_enable(ec: *mut gaokun_ec, on: bool) -> c_int;
}
// --------------------------------------------------------------------------
// API for UCSI
extern "C" {
    pub fn gaokun_ec_ucsi_read(ec: *mut gaokun_ec, resp[GAOKUN_UCSI_READ_SIZE]: u8) -> c_int;
}
extern "C" {
    pub fn gaokun_ec_ucsi_get_reg(ec: *mut gaokun_ec, ureg: *mut gaokun_ucsi_reg) -> c_int;
}
extern "C" {
    pub fn gaokun_ec_ucsi_pan_ack(ec: *mut gaokun_ec, port_id: c_int) -> c_int;
}
