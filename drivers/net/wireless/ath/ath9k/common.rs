//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/common.h
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


//
// Copyright (c) 2009-2011 Atheros Communications Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

// Common header for Atheros 802.11n base driver cores
pub const WME_BA_BMP_SIZE: c_int = 64;

pub const ATH_RSSI_DUMMY_MARKER: c_int = 127;
pub const ATH_RSSI_LPF_LEN: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_beacon_config {
    pub main_vif: *mut ieee80211_vif,
    pub beacon_interval: c_int,
    pub dtim_period: u16,
    pub bmiss_timeout: u16,
    pub dtim_count: u8,
    pub enable_beacon: u8,
    pub ibss_creator: bool,
    pub nexttbtt: u32,
    pub intval: u32,
}

extern "C" {
    pub fn ath9k_cmn_get_hw_crypto_keytype(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ath9k_cmn_count_streams(chainmask: c_uint, max: c_int) -> c_int;
}
extern "C" {
    pub fn ath9k_cmn_init_crypto(ah: *mut ath_hw);
}
