//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/regd.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_chplan_id {
    RTW_CHPLAN_ETSI1_NULL = 0x21,
    RTW_CHPLAN_WORLD_ETSI1 = 0x26,
    RTW_CHPLAN_MKK1_MKK1 = 0x27,
    RTW_CHPLAN_IC1_IC2 = 0x2B,
    RTW_CHPLAN_WORLD_CHILE1 = 0x2D,
    RTW_CHPLAN_WORLD_FCC3 = 0x30,
    RTW_CHPLAN_WORLD_FCC5 = 0x32,
    RTW_CHPLAN_FCC1_FCC7 = 0x34,
    RTW_CHPLAN_WORLD_ETSI2 = 0x35,
    RTW_CHPLAN_WORLD_ETSI3 = 0x36,
    RTW_CHPLAN_ETSI1_ETSI12 = 0x3D,
    RTW_CHPLAN_KCC1_KCC2 = 0x3E,
    RTW_CHPLAN_ETSI1_ETSI4 = 0x42,
    RTW_CHPLAN_FCC1_NCC3 = 0x44,
    RTW_CHPLAN_WORLD_ACMA1 = 0x45,
    RTW_CHPLAN_WORLD_ETSI6 = 0x47,
    RTW_CHPLAN_WORLD_ETSI7 = 0x48,
    RTW_CHPLAN_WORLD_ETSI8 = 0x49,
    RTW_CHPLAN_KCC1_KCC3 = 0x4B,
    RTW_CHPLAN_WORLD_ETSI10 = 0x51,
    RTW_CHPLAN_WORLD_ETSI14 = 0x59,
    RTW_CHPLAN_FCC2_FCC7 = 0x61,
    RTW_CHPLAN_FCC2_FCC1 = 0x62,
    RTW_CHPLAN_WORLD_ETSI15 = 0x63,
    RTW_CHPLAN_WORLD_FCC7 = 0x73,
    RTW_CHPLAN_FCC2_FCC17 = 0x74,
    RTW_CHPLAN_WORLD_ETSI20 = 0x75,
    RTW_CHPLAN_FCC2_FCC11 = 0x76,
    RTW_CHPLAN_REALTEK_DEFINE = 0x7f,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct country_code_to_enum_rd {
    pub countrycode: u16,
    pub iso_name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum country_code_type {
    COUNTRY_CODE_FCC = 0,
    COUNTRY_CODE_IC = 1,
    COUNTRY_CODE_ETSI = 2,
    COUNTRY_CODE_SPAIN = 3,
    COUNTRY_CODE_FRANCE = 4,
    COUNTRY_CODE_MKK = 5,
    COUNTRY_CODE_MKK1 = 6,
    COUNTRY_CODE_ISRAEL = 7,
    COUNTRY_CODE_TELEC = 8,
    COUNTRY_CODE_MIC = 9,
    COUNTRY_CODE_GLOBAL_DOMAIN = 10,
    COUNTRY_CODE_WORLD_WIDE_13 = 11,
    COUNTRY_CODE_TELEC_NETGEAR = 12,
    COUNTRY_CODE_WORLD_WIDE_13_5G_ALL = 13,

// new channel plan above this
    COUNTRY_CODE_MAX
}

extern "C" {
    pub fn rtw_regd_init(rtwdev: *mut rtw_dev) -> c_int;
}
extern "C" {
    pub fn rtw_regd_hint(rtwdev: *mut rtw_dev) -> c_int;
}
extern "C" {
    pub fn rtw_regd_get(rtwdev: *mut rtw_dev) -> u8;
}
extern "C" {
    pub fn rtw_regd_has_alt(regd: u8, regd_alt: *mut u8) -> bool;
}
extern "C" {
    pub fn rtw_regd_srrc(rtwdev: *mut rtw_dev) -> bool;
}
