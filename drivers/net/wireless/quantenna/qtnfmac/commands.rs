//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/quantenna/qtnfmac/commands.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2016 Quantenna Communications. All rights reserved.

extern "C" {
    pub fn qtnf_cmd_send_init_fw(bus: *mut qtnf_bus) -> c_int;
}
extern "C" {
    pub fn qtnf_cmd_send_deinit_fw(bus: *mut qtnf_bus);
}
extern "C" {
    pub fn qtnf_cmd_get_hw_info(bus: *mut qtnf_bus) -> c_int;
}
extern "C" {
    pub fn qtnf_cmd_get_mac_info(mac: *mut qtnf_wmac) -> c_int;
}
extern "C" {
    pub fn qtnf_cmd_send_del_intf(vif: *mut qtnf_vif) -> c_int;
}
extern "C" {
    pub fn qtnf_cmd_send_regulatory_config(mac: *mut qtnf_wmac, alpha2: *const c_char) -> c_int;
}
extern "C" {
    pub fn qtnf_cmd_send_stop_ap(vif: *mut qtnf_vif) -> c_int;
}
extern "C" {
    pub fn qtnf_cmd_send_register_mgmt(vif: *mut qtnf_vif, frame_type: u16, reg: bool) -> c_int;
}
extern "C" {
    pub fn qtnf_cmd_send_set_default_mgmt_key(vif: *mut qtnf_vif, key_index: u8) -> c_int;
}
extern "C" {
    pub fn qtnf_cmd_send_scan(mac: *mut qtnf_wmac) -> c_int;
}
extern "C" {
    pub fn qtnf_cmd_get_channel(vif: *mut qtnf_vif, chdef: *mut cfg80211_chan_def) -> c_int;
}
extern "C" {
    pub fn qtnf_cmd_send_pm_set(vif: *const qtnf_vif, pm_mode: u8, timeout: c_int) -> c_int;
}
extern "C" {
    pub fn qtnf_cmd_get_tx_power(vif: *const qtnf_vif, dbm: *mut c_int) -> c_int;
}
extern "C" {
    pub fn qtnf_cmd_netdev_changeupper(vif: *const qtnf_vif, br_domain: c_int) -> c_int;
}
