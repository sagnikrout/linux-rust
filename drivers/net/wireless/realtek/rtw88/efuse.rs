//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/efuse.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright(c) 2018-2019  Realtek Corporation
//
pub const EFUSE_HW_CAP_IGNORE: c_int = 0;
pub const EFUSE_HW_CAP_PTCL_VHT: c_int = 3;
pub const EFUSE_HW_CAP_SUPP_BW80: c_int = 7;
pub const EFUSE_HW_CAP_SUPP_BW40: c_int = 6;
pub const EFUSE_READ_FAIL: c_uint = 0xff;

extern "C" {
    pub fn rtw_parse_efuse_map(rtwdev: *mut rtw_dev) -> c_int;
}
extern "C" {
    pub fn rtw_read8_physical_efuse(rtwdev: *mut rtw_dev, addr: u16, data: *mut u8) -> c_int;
}
