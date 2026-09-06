//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/regd.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright(c) 2009-2012  Realtek Corporation.
// for kernel 3.14 , both value are changed to IEEE80211_CHAN_NO_IR

#[repr(C)]
#[derive(Copy, Clone)]
pub struct country_code_to_enum_rd {
    pub countrycode: u16,
    pub iso_name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum country_code_type_t {
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

// add new channel plan above this line
    COUNTRY_CODE_MAX
}

extern "C" {
    pub fn rtl_reg_notifier(wiphy: *mut wiphy, request: *mut regulatory_request);
}
