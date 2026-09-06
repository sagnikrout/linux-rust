//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/silabs/wfx/hif_tx_mib.h
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
// Implementation of the host-to-chip MIBs of the hardware API.
//
// Copyright (c) 2017-2020, Silicon Laboratories, Inc.
// Copyright (c) 2010, ST-Ericsson
// Copyright (C) 2010, ST-Ericsson SA
//

extern "C" {
    pub fn wfx_hif_set_output_power(wvif: *mut wfx_vif, val: c_int) -> c_int;
}
extern "C" {
    pub fn wfx_hif_set_rcpi_rssi_threshold(wvif: *mut wfx_vif, rssi_thold: c_int, rssi_hyst: c_int) -> c_int;
}
extern "C" {
    pub fn wfx_hif_set_macaddr(wvif: *mut wfx_vif, mac: *mut u8) -> c_int;
}
extern "C" {
    pub fn wfx_hif_set_rx_filter(wvif: *mut wfx_vif, filter_bssid: bool, fwd_probe_req: bool) -> c_int;
}
extern "C" {
    pub fn wfx_hif_beacon_filter_control(wvif: *mut wfx_vif, enable: c_int, beacon_count: c_int) -> c_int;
}
extern "C" {
    pub fn wfx_hif_set_operational_mode(wdev: *mut wfx_dev, mode: wfx_hif_op_power_mode) -> c_int;
}
extern "C" {
    pub fn wfx_hif_set_mfp(wvif: *mut wfx_vif, capable: bool, required: bool) -> c_int;
}
extern "C" {
    pub fn wfx_hif_set_block_ack_policy(wvif: *mut wfx_vif, tx_tid_policy: u8, rx_tid_policy: u8) -> c_int;
}
extern "C" {
    pub fn wfx_hif_set_tx_rate_retry_policy(wvif: *mut wfx_vif, policy_index: c_int, rates: *mut u8) -> c_int;
}
extern "C" {
    pub fn wfx_hif_keep_alive_period(wvif: *mut wfx_vif, period: c_int) -> c_int;
}
extern "C" {
    pub fn wfx_hif_set_arp_ipv4_filter(wvif: *mut wfx_vif, idx: c_int, addr: *mut __be32) -> c_int;
}
extern "C" {
    pub fn wfx_hif_use_multi_tx_conf(wdev: *mut wfx_dev, enable: bool) -> c_int;
}
extern "C" {
    pub fn wfx_hif_set_uapsd_info(wvif: *mut wfx_vif, val: c_ulong) -> c_int;
}
extern "C" {
    pub fn wfx_hif_erp_use_protection(wvif: *mut wfx_vif, enable: bool) -> c_int;
}
extern "C" {
    pub fn wfx_hif_slot_time(wvif: *mut wfx_vif, val: c_int) -> c_int;
}
extern "C" {
    pub fn wfx_hif_wep_default_key_id(wvif: *mut wfx_vif, val: c_int) -> c_int;
}
extern "C" {
    pub fn wfx_hif_rts_threshold(wvif: *mut wfx_vif, val: c_int) -> c_int;
}
