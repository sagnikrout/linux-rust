//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/rsi/rsi_ps.h
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
// Copyright (c) 2017 Redpine Signals Inc.
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
pub const PS_CONFIRM_INDEX: c_int = 12;
pub const RSI_DEF_DS_WAKEUP_PERIOD: c_int = 200;
pub const RSI_DEF_LISTEN_INTERVAL: c_int = 200;
pub const RSI_SLEEP_TYPE_LP: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ps_state {
    PS_NONE = 0,
    PS_ENABLE_REQ_SENT = 1,
    PS_DISABLE_REQ_SENT = 2,
    PS_ENABLED = 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps_sleep_params {
    pub enable: u8,
    pub sleep_type: u8,
    pub connected_sleep: u8,
    pub reserved1: u8,
    pub num_bcns_per_lis_int: __le16,
    pub wakeup_type: __le16,
    pub sleep_duration: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_ps_info {
    pub enabled: u8,
    pub sleep_type: u8,
    pub tx_threshold: u8,
    pub rx_threshold: u8,
    pub tx_hysterisis: u8,
    pub rx_hysterisis: u8,
    pub monitor_interval: u16,
    pub listen_interval: u32,
    pub num_bcns_per_lis_int: u16,
    pub dtim_interval_duration: u32,
    pub num_dtims_per_sleep: u16,
    pub deep_sleep_wakeup_period: u32,
    pub __packed: },
    pub state): *mut *mut char str_psstate(enum ps_state,
    pub vif): *mut *mut void rsi_enable_ps(struct rsi_hw adapter, struct ieee80211_vif,
    pub vif): *mut *mut void rsi_disable_ps(struct rsi_hw adapter, struct ieee80211_vif,
    pub msg): *mut *mut int rsi_handle_ps_confirm(struct rsi_hw adapter, u8,
    pub hw): *mut void rsi_default_ps_params(struct rsi_hw,
    pub vif): *mut *mut void rsi_conf_uapsd(struct rsi_hw adapter, struct ieee80211_vif,
