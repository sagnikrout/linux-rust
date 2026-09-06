//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw89/util.h
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

// call this function with wiphy mutex is held

// Before adding rtwvif to list, we need to check if it already exist, beacase
// in some case such as SER L2 happen during WoWLAN flow, calling reconfig
// twice cause the list to be added twice.
//
// The result of negative dividend and positive divisor is undefined, but it
// should be one case of round-down or round-up. So, make it round-down if the
// result is round-up.
// Note: the maximum value of divisor is 0x7FFF_FFFF, because we cast it to
// signed value to make compiler to use signed divide instruction.
//
// remainder = i_remainder;
extern "C" {
    pub fn s32_div_u32_round_down(2: dividend + divisor /, _arg: divisor, _arg: NULL) -> return;
}
// pn = u64_encode_bits(hdr[0], RTW89_KEY_PN_0) |
extern "C" {
    pub fn rtw89_linear_to_db_quarter(val: u64) -> i32;
}
extern "C" {
    pub fn rtw89_linear_to_db(val: u64) -> i32;
}
extern "C" {
    pub fn rtw89_db_quarter_to_linear(db: i32) -> u64;
}
extern "C" {
    pub fn rtw89_db_to_linear(db: i32) -> u64;
}
extern "C" {
    pub fn rtw89_might_trailing_ellipsis(buf: *mut c_char, size: usize, used: isize);
}
