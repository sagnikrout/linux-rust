//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/microchip/wilc1000/hif.h
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
//
// Copyright (c) 2012 - 2018 Microchip Technology Inc., and its subsidiaries
// All rights reserved.
//

pub const WILC_MAX_NUM_PROBED_SSID: c_int = 10;
pub const WILC_TX_MIC_KEY_LEN: c_int = 8;
pub const WILC_RX_MIC_KEY_LEN: c_int = 8;
pub const WILC_ADD_STA_LENGTH: c_int = 40;
pub const WILC_NUM_CONCURRENT_IFC: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rf_info {
    pub link_speed: u8,
    pub rssi: i8,
    pub tx_cnt: u32,
    pub rx_cnt: u32,
    pub tx_fail_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_if_state {
    HOST_IF_IDLE			= 0,
    HOST_IF_SCANNING		= 1,
    HOST_IF_CONNECTING		= 2,
    HOST_IF_WAITING_CONN_RESP	= 3,
    HOST_IF_CONNECTED		= 4,
    HOST_IF_P2P_LISTEN		= 5,
    HOST_IF_EXTERNAL_AUTH           = 6,
    HOST_IF_FORCE_32BIT		= 0xFFFFFFFF
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg_param_attr {
    pub flag: u32,
    pub short_retry_limit: u16,
    pub long_retry_limit: u16,
    pub frag_threshold: u16,
    pub rts_threshold: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cfg_param {
    WILC_CFG_PARAM_RETRY_SHORT = BIT(0),
    WILC_CFG_PARAM_RETRY_LONG = BIT(1),
    WILC_CFG_PARAM_FRAG_THRESHOLD = BIT(2),
    WILC_CFG_PARAM_RTS_THRESHOLD = BIT(3)
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scan_event {
    SCAN_EVENT_NETWORK_FOUND	= 0,
    SCAN_EVENT_DONE			= 1,
    SCAN_EVENT_ABORTED		= 2,
    SCAN_EVENT_FORCE_32BIT		= 0xFFFFFFFF
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum conn_event {
    CONN_DISCONN_EVENT_CONN_RESP		= 0,
    CONN_DISCONN_EVENT_DISCONN_NOTIF	= 1,
    CONN_DISCONN_EVENT_FORCE_32BIT		= 0xFFFFFFFF
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_rcvd_net_info {
    pub rssi: i8,
    pub ch: u8,
    pub frame_len: u16,
    pub mgmt: *mut ieee80211_mgmt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_user_scan_req {
    pub priv): *mut wilc_priv,
    pub priv: *mut wilc_priv,
    pub ch_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_conn_info {
    pub bssid: [u8; ETH_ALEN],
    pub security: u8,
    pub auth_type: authtype,
    pub mfp_type: mfptype,
    pub req_ies: *mut u8,
    pub req_ies_len: usize,
    pub resp_ies: *mut u8,
    pub resp_ies_len: u16,
    pub status: u16,
    pub priv): *mut wilc_priv,
    pub priv: *mut wilc_priv,
    pub param: *mut wilc_join_bss_param,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilc_remain_ch {
    pub ch: u16,
    pub cookie): *mut *mut *mut void (expired)(struct wilc_vif vif, u64,
    pub vif: *mut wilc_vif,
    pub cookie: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_if_drv {
    pub usr_scan_req: wilc_user_scan_req,
    pub conn_info: wilc_conn_info,
    pub remain_on_ch: wilc_remain_ch,
    pub p2p_timeout: u64,
    pub hif_state: host_if_state,
    pub assoc_bssid: [u8; ETH_ALEN],
    pub scan_timer: timer_list,
    pub scan_timer_vif: *mut wilc_vif,
    pub connect_timer: timer_list,
    pub connect_timer_vif: *mut wilc_vif,
    pub remain_on_ch_timer: timer_list,
    pub remain_on_ch_timer_vif: *mut wilc_vif,
    pub ifc_up: bool,
    pub assoc_resp: [u8; WILC_MAX_ASSOC_RESP_FRAME_SIZE],
}

extern "C" {
    pub fn wilc_set_pmkid_info(vif: *mut wilc_vif, pmkid: *mut wilc_pmkid_attr) -> c_int;
}
extern "C" {
    pub fn wilc_get_mac_address(vif: *mut wilc_vif, mac_addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn wilc_set_mac_address(vif: *mut wilc_vif, mac_addr: *const u8) -> c_int;
}
extern "C" {
    pub fn wilc_disconnect(vif: *mut wilc_vif) -> c_int;
}
extern "C" {
    pub fn wilc_set_mac_chnl_num(vif: *mut wilc_vif, channel: u8) -> c_int;
}
extern "C" {
    pub fn wilc_get_rssi(vif: *mut wilc_vif, rssi_level: *mut i8) -> c_int;
}
extern "C" {
    pub fn wilc_init(dev: *mut net_device, hif_drv_handler: *mut host_if_drv) -> c_int;
}
extern "C" {
    pub fn wilc_deinit(vif: *mut wilc_vif) -> c_int;
}
extern "C" {
    pub fn wilc_del_beacon(vif: *mut wilc_vif) -> c_int;
}
extern "C" {
    pub fn wilc_del_allstation(vif: *mut wilc_vif, mac_addr[][ETH_ALEN]: u8) -> c_int;
}
extern "C" {
    pub fn wilc_del_station(vif: *mut wilc_vif, mac_addr: *const u8) -> c_int;
}
extern "C" {
    pub fn wilc_set_power_mgmt(vif: *mut wilc_vif, enabled: bool, timeout: u32) -> c_int;
}
extern "C" {
    pub fn wilc_listen_state_expired(vif: *mut wilc_vif, cookie: u64) -> c_int;
}
extern "C" {
    pub fn wilc_frame_register(vif: *mut wilc_vif, frame_type: u16, reg: bool);
}
extern "C" {
    pub fn wilc_get_statistics(vif: *mut wilc_vif, stats: *mut rf_info) -> c_int;
}
extern "C" {
    pub fn wilc_get_vif_idx(vif: *mut wilc_vif) -> c_int;
}
extern "C" {
    pub fn wilc_set_tx_power(vif: *mut wilc_vif, tx_power: u8) -> c_int;
}
extern "C" {
    pub fn wilc_get_tx_power(vif: *mut wilc_vif, tx_power: *mut u8) -> c_int;
}
extern "C" {
    pub fn wilc_set_wowlan_trigger(vif: *mut wilc_vif, enabled: bool);
}
extern "C" {
    pub fn wilc_scan_complete_received(wilc: *mut wilc, buffer: *mut u8, length: u32);
}
extern "C" {
    pub fn wilc_network_info_received(wilc: *mut wilc, buffer: *mut u8, length: u32);
}
extern "C" {
    pub fn wilc_gnrl_async_info_received(wilc: *mut wilc, buffer: *mut u8, length: u32);
}
extern "C" {
    pub fn wilc_set_default_mgmt_key_index(vif: *mut wilc_vif, index: u8) -> c_int;
}
extern "C" {
    pub fn wilc_handle_disconnect(vif: *mut wilc_vif);
}
