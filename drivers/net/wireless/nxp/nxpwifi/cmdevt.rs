//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/nxp/nxpwifi/cmdevt.h
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
// nxpwifi: commands and events
//
// Copyright 2011-2024 NXP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_cmd_entry {
    pub cmd_no: u16,
    pub cmd_type): u16 cmd_action, u32,
    pub data_buf): *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_evt_entry {
    pub event_cause: u32,
    pub priv): *mut *mut int (event_handler)(struct nxpwifi_private,
}

extern "C" {
    pub fn nxpwifi_sta_init_cmd(priv: *mut nxpwifi_private, first_sta: u8, init: bool) -> c_int;
}
extern "C" {
    pub fn nxpwifi_set_sys_config_invalid_data(config: *mut nxpwifi_uap_bss_param);
}
extern "C" {
    pub fn nxpwifi_process_event(adapter: *mut nxpwifi_adapter) -> c_int;
}
extern "C" {
    pub fn nxpwifi_process_sta_event(priv: *mut nxpwifi_private) -> c_int;
}
extern "C" {
    pub fn nxpwifi_process_uap_event(priv: *mut nxpwifi_private) -> c_int;
}
extern "C" {
    pub fn nxpwifi_mgmt_frame_reg(priv: *mut nxpwifi_private, mask: u32) -> c_int;
}
extern "C" {
    pub fn nxpwifi_set_rts(priv: *mut nxpwifi_private, rts_thr: u32) -> c_int;
}
extern "C" {
    pub fn nxpwifi_set_frag(priv: *mut nxpwifi_private, frag_thr: u32) -> c_int;
}
extern "C" {
    pub fn nxpwifi_set_bss_mode(priv: *mut nxpwifi_private) -> c_int;
}
extern "C" {
    pub fn nxpwifi_apply_regdomain(priv: *mut nxpwifi_private) -> c_int;
}
extern "C" {
    pub fn nxpwifi_get_tx_pwr(priv: *mut nxpwifi_private) -> c_int;
}
extern "C" {
    pub fn nxpwifi_get_rssi_info(priv: *mut nxpwifi_private) -> c_int;
}
extern "C" {
    pub fn nxpwifi_get_802_11_snmp_mib(priv: *mut nxpwifi_private, oid: u16, value: *mut c_void) -> c_int;
}
extern "C" {
    pub fn nxpwifi_set_rf_antenna(priv: *mut nxpwifi_private, antcfg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn nxpwifi_get_rf_antenna(priv: *mut nxpwifi_private, tx_ant: *mut u32, rx_ant: *mut u32) -> c_int;
}
extern "C" {
    pub fn nxpwifi_ap_stop_bss(priv: *mut nxpwifi_private) -> c_int;
}
extern "C" {
    pub fn nxpwifi_ap_sys_reset(priv: *mut nxpwifi_private) -> c_int;
}
extern "C" {
    pub fn nxpwifi_cfg80211_deinit_p2p(priv: *mut nxpwifi_private) -> c_int;
}
extern "C" {
    pub fn nxpwifi_ap_get_sta_list(priv: *mut nxpwifi_private) -> c_int;
}
extern "C" {
    pub fn nxpwifi_set_tx_rate(priv: *mut nxpwifi_private, bitmap_rates: *mut c_void) -> c_int;
}
extern "C" {
    pub fn nxpwifi_uap_sta_deauth(priv: *mut nxpwifi_private, mac: *mut u8) -> c_int;
}
extern "C" {
    pub fn nxpwifi_bg_scan_config(priv: *mut nxpwifi_private, bg_scan_cfg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn nxpwifi_mef_cfg(priv: *mut nxpwifi_private, mef_cfg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn nxpwifi_coalesce_cfg(priv: *mut nxpwifi_private, coalesce_cfg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn nxpwifi_add_new_station(priv: *mut nxpwifi_private, add_sta: *mut c_void) -> c_int;
}
extern "C" {
    pub fn nxpwifi_hostcmd(priv: *mut nxpwifi_private, hostcmd: *mut nxpwifi_ds_misc_cmd) -> c_int;
}
extern "C" {
    pub fn nxpwifi_chan_report_request(priv: *mut nxpwifi_private, radar_params: *mut c_void) -> c_int;
}
