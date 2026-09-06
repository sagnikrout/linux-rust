//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/morsemicro/mm81x/mac.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2017-2026 Morse Micro
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_queue_params {
    pub uapsd: u8,
    pub aci: u8,
    pub aifs: u8,
    pub cw_min: u16,
    pub cw_max: u16,
    pub txop: u32,
}

extern "C" {
    pub fn mm81x_generate_cssid(_arg: vif->cfg.ssid, _arg: vif->cfg.ssid_len) -> return;
}
//
// Build a little-endian word from the last four octets of a MAC address;
// the first two octets are dropped.
//
extern "C" {
    pub fn rcu_dereference(_arg: mors->vifs[vif_id]) -> return;
}
extern "C" {
    pub fn mm81x_mac_register(mors: *mut mm81x) -> c_int;
}
extern "C" {
    pub fn mm81x_mac_free(mors: *mut mm81x);
}
extern "C" {
    pub fn mm81x_mac_unregister(mors: *mut mm81x);
}
extern "C" {
    pub fn mm81x_mac_event_recv(mors: *mut mm81x, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn mm81x_mac_beacon_irq_handle(mors: *mut mm81x, status: u32);
}
extern "C" {
    pub fn mm81x_hw_scan_h_get_cmd_size(params: *mut mm81x_hw_scan_params) -> usize;
}
extern "C" {
    pub fn mm81x_tx_h_check_aggr(pubsta: *mut ieee80211_sta, skb: *mut sk_buff);
}
