//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/base.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ap_peer {
    PEER_UNKNOWN = 0,
    PEER_RTL = 1,
    PEER_RTL_92SE = 2,
    PEER_BROAD = 3,
    PEER_RAL = 4,
    PEER_ATH = 5,
    PEER_CISCO = 6,
    PEER_MARV = 7,
    PEER_AIRGO = 9,
    PEER_MAX = 10,
}

pub const RTL_DUMMY_OFFSET: c_int = 0;
pub const RTL_DUMMY_UNIT: c_int = 8;

pub const RTL_TX_DESC_SIZE: c_int = 32;

pub const FRAME_OFFSET_FRAME_CONTROL: c_int = 0;
pub const FRAME_OFFSET_DURATION: c_int = 2;
pub const FRAME_OFFSET_ADDRESS1: c_int = 4;
pub const FRAME_OFFSET_ADDRESS2: c_int = 10;
pub const FRAME_OFFSET_ADDRESS3: c_int = 16;
pub const FRAME_OFFSET_SEQUENCE: c_int = 22;
pub const FRAME_OFFSET_ADDRESS4: c_int = 24;
pub const MAX_LISTEN_INTERVAL: c_int = 10;
pub const MAX_RATE_TRIES: c_int = 4;

extern "C" {
    pub fn rtl_init_core(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn rtl_deinit_core(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl_init_rx_config(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl_init_rfkill(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl_deinit_rfkill(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl_watch_dog_timer_callback(t: *mut timer_list);
}
extern "C" {
    pub fn rtl_deinit_deferred_work(hw: *mut ieee80211_hw, ips_wq: bool);
}
extern "C" {
    pub fn rtl_action_proc(hw: *mut ieee80211_hw, skb: *mut sk_buff, is_tx: u8) -> bool;
}
extern "C" {
    pub fn rtl_tx_mgmt_proc(hw: *mut ieee80211_hw, skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn rtl_tx_ackqueue(hw: *mut ieee80211_hw, skb: *mut sk_buff);
}
extern "C" {
    pub fn rtl_is_tx_report_skb(hw: *mut ieee80211_hw, skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn rtl_check_tx_report_acked(hw: *mut ieee80211_hw) -> bool;
}
extern "C" {
    pub fn rtl_wait_tx_report_acked(hw: *mut ieee80211_hw, wait_ms: u32);
}
extern "C" {
    pub fn rtl_beacon_statistic(hw: *mut ieee80211_hw, skb: *mut sk_buff);
}
extern "C" {
    pub fn rtl_collect_scan_list(hw: *mut ieee80211_hw, skb: *mut sk_buff);
}
extern "C" {
    pub fn rtl_scan_list_expire(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl_rx_ampdu_apply(rtlpriv: *mut rtl_priv);
}
extern "C" {
    pub fn rtl_c2hcmd_launcher(hw: *mut ieee80211_hw, exec: c_int);
}
extern "C" {
    pub fn rtl_c2hcmd_enqueue(hw: *mut ieee80211_hw, skb: *mut sk_buff);
}
extern "C" {
    pub fn rtl_recognize_peer(hw: *mut ieee80211_hw, data: *mut u8, len: c_uint);
}
extern "C" {
    pub fn rtl_tid_to_ac(tid: u8) -> u8;
}
extern "C" {
    pub fn rtl_phy_scan_operation_backup(hw: *mut ieee80211_hw, operation: u8);
}
