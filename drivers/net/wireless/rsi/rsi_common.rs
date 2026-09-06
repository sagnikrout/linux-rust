//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/rsi/rsi_common.h
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
// Copyright (c) 2014 Redpine Signals Inc.
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

pub const EVENT_WAIT_FOREVER: c_int = 0;

pub const QUEUE_NOT_FULL: c_int = 1;
pub const QUEUE_FULL: c_int = 0;
extern "C" {
    pub fn rsi_mac80211_detach(hw: *mut rsi_hw);
}
extern "C" {
    pub fn rsi_mac80211_rfkill_exit(hw: *mut rsi_hw);
}
extern "C" {
    pub fn rsi_get_connected_channel(vif: *mut ieee80211_vif) -> u16;
}
extern "C" {
    pub fn rsi_91x_deinit(adapter: *mut rsi_hw);
}
extern "C" {
    pub fn rsi_read_pkt(common: *mut rsi_common, rx_pkt: *mut u8, rcv_pkt_len: i32) -> c_int;
}

extern "C" {
    pub fn rsi_config_wowlan(adapter: *mut rsi_hw, wowlan: *mut cfg80211_wowlan) -> c_int;
}

extern "C" {
    pub fn rsi_roc_timeout(t: *mut timer_list);
}
