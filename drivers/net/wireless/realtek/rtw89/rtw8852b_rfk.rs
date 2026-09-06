//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw89/rtw8852b_rfk.h
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
// Copyright(c) 2019-2022  Realtek Corporation
//

extern "C" {
    pub fn rtw8852b_rck(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw8852b_dack(rtwdev: *mut rtw89_dev, chanctx_idx: rtw89_chanctx_idx);
}
extern "C" {
    pub fn rtw8852b_dpk_init(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw8852b_dpk_track(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw8852b_mcc_get_ch_info(rtwdev: *mut rtw89_dev, phy_idx: rtw89_phy_idx);
}
