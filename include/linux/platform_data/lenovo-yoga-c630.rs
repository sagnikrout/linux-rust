//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/lenovo-yoga-c630.h
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
// Copyright (c) 2022-2024, Linaro Ltd
// Authors:
// Bjorn Andersson
// Dmitry Baryshkov
//

extern "C" {
    pub fn yoga_c630_ec_read8(ec: *mut yoga_c630_ec, addr: u8) -> c_int;
}
extern "C" {
    pub fn yoga_c630_ec_read16(ec: *mut yoga_c630_ec, addr: u8) -> c_int;
}
extern "C" {
    pub fn yoga_c630_ec_register_notify(ec: *mut yoga_c630_ec, nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn yoga_c630_ec_unregister_notify(ec: *mut yoga_c630_ec, nb: *mut notifier_block);
}
pub const YOGA_C630_UCSI_WRITE_SIZE: c_int = 8;
pub const YOGA_C630_UCSI_CCI_SIZE: c_int = 4;
pub const YOGA_C630_UCSI_DATA_SIZE: c_int = 16;

extern "C" {
    pub fn yoga_c630_ec_ucsi_get_version(ec: *mut yoga_c630_ec) -> u16;
}
pub const LENOVO_EC_EVENT_USB: c_uint = 0x20;
pub const LENOVO_EC_EVENT_UCSI: c_uint = 0x21;
pub const LENOVO_EC_EVENT_HPD: c_uint = 0x22;
pub const LENOVO_EC_EVENT_BAT_STATUS: c_uint = 0x24;
pub const LENOVO_EC_EVENT_BAT_INFO: c_uint = 0x25;
pub const LENOVO_EC_EVENT_BAT_ADPT_STATUS: c_uint = 0x37;
