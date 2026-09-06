//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw89/sar.h
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
// Copyright(c) 2019-2020  Realtek Corporation
//

pub const RTW89_SAR_TXPWR_MAC_MAX: c_int = 63;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_sar_parm {
    pub center_freq: u32,
    pub ntx: rtw89_ntx,
    pub force_path: bool,
    pub path: rtw89_rf_path,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_sar_handler {
    pub descr_sar_source: *const c_char,
    pub txpwr_factor_sar: u8,
    pub cfg): *const *const rtw89_sar_parm sar_parm, s32,
}

extern "C" {
    pub fn rtw89_query_sar(rtwdev: *mut rtw89_dev, sar_parm: *const rtw89_sar_parm) -> i8;
}
extern "C" {
    pub fn rtw89_print_tas(rtwdev: *mut rtw89_dev, buf: *mut c_char, bufsz: usize) -> c_int;
}
extern "C" {
    pub fn rtw89_tas_reset(rtwdev: *mut rtw89_dev, force: bool);
}
extern "C" {
    pub fn rtw89_tas_scan(rtwdev: *mut rtw89_dev, start: bool);
}
extern "C" {
    pub fn rtw89_tas_fw_timer_enable(rtwdev: *mut rtw89_dev, enable: bool);
}
extern "C" {
    pub fn rtw89_sar_init(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_sar_track(rtwdev: *mut rtw89_dev);
}
