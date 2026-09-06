//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rc.h
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
pub const B_MODE_MAX_RIX: c_int = 3;
pub const G_MODE_MAX_RIX: c_int = 11;
pub const A_MODE_MAX_RIX: c_int = 7;
// in mac80211 mcs0-mcs15 is idx0-idx15
pub const N_MODE_MCS7_RIX: c_int = 7;
pub const N_MODE_MCS15_RIX: c_int = 15;
pub const AC_MODE_MCS7_RIX: c_int = 7;
pub const AC_MODE_MCS8_RIX: c_int = 8;
pub const AC_MODE_MCS9_RIX: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_rate_priv {
    pub ht_cap: u8,
}

extern "C" {
    pub fn rtl_rate_control_register() -> c_int;
}
extern "C" {
    pub fn rtl_rate_control_unregister();
}
