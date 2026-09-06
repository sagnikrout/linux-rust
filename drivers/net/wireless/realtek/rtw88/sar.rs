//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/sar.h
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
// Copyright(c) 2018-2021  Realtek Corporation
//

// NL80211_SAR_TYPE_POWER means unit is in 0.25 dBm,
// where 0.25 = 1/4 = 2^(-2), so make factor 2.
//
pub const RTW_COMMON_SAR_FCT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_sar_arg {
    pub sar_band: u8,
    pub path: u8,
    pub rs: u8,
}

extern "C" {
    pub fn rtw_query_sar(rtwdev: *mut rtw_dev, arg: *const rtw_sar_arg) -> i8;
}
