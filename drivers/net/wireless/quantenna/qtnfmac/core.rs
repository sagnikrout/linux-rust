//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/quantenna/qtnfmac/core.h
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
// Copyright (c) 2015-2016 Quantenna Communications. All rights reserved.

pub const QTNF_MAX_VSIE_LEN: c_int = 255;
pub const QTNF_MAX_INTF: c_int = 8;
pub const QTNF_MAX_EVENT_QUEUE_LEN: c_int = 255;
pub const QTNF_SCAN_TIMEOUT_SEC: c_int = 15;
pub const QTNF_DEF_BSS_PRIORITY: c_int = 0;
pub const QTNF_DEF_WDOG_TIMEOUT: c_int = 5;
pub const QTNF_TX_TIMEOUT_TRSHLD: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtnf_sta_node {
    pub list: list_head,
    pub mac_addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtnf_sta_list {
    pub head: list_head,
    pub size: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtnf_vif {
    pub wdev: wireless_dev,
    pub bssid: [u8; ETH_ALEN],
    pub mac_addr: [u8; ETH_ALEN],
    pub vifid: u8,
    pub bss_priority: u8,
    pub bss_status: u8,
    pub mgmt_frames_bitmask: u16,
    pub netdev: *mut net_device,
    pub mac: *mut qtnf_wmac,
    pub reset_work: work_struct,
    pub high_pri_tx_work: work_struct,
    pub high_pri_tx_queue: sk_buff_head,
    pub sta_list: qtnf_sta_list,
    pub cons_tx_timeout_cnt: c_ulong,
    pub generation: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtnf_mac_info {
    pub bands_cap: u8,
    pub num_tx_chain: u8,
    pub num_rx_chain: u8,
    pub max_ap_assoc_sta: u16,
    pub frag_thr: u32,
    pub rts_thr: u32,
    pub lretry_limit: u8,
    pub sretry_limit: u8,
    pub coverage_class: u8,
    pub radar_detect_widths: u8,
    pub max_scan_ssids: u8,
    pub max_acl_mac_addrs: u16,
    pub ht_cap_mod_mask: ieee80211_ht_cap,
    pub vht_cap_mod_mask: ieee80211_vht_cap,
    pub if_comb: *mut ieee80211_iface_combination,
    pub n_if_comb: usize,
    pub extended_capabilities: *mut u8,
    pub extended_capabilities_mask: *mut u8,
    pub extended_capabilities_len: u8,
    pub wowlan: *mut wiphy_wowlan_support,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtnf_wmac {
    pub macid: u8,
    pub wiphy_registered: u8,
    pub macaddr: [u8; ETH_ALEN],
    pub bus: *mut qtnf_bus,
    pub macinfo: qtnf_mac_info,
    pub iflist: [qtnf_vif; QTNF_MAX_INTF],
    pub scan_req: *mut cfg80211_scan_request,
    pub /: *mut *mut mutex mac_lock; / lock during wmac specific ops,
    pub scan_timeout: delayed_work,
    pub rd: *mut ieee80211_regdomain,
    pub pdev: *mut platform_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtnf_hw_info {
    pub ql_proto_ver: u32,
    pub num_mac: u8,
    pub mac_bitmap: u8,
    pub fw_ver: u32,
    pub total_tx_chain: u8,
    pub total_rx_chain: u8,
    pub fw_version: [c_char; ETHTOOL_FWVERS_LEN],
    pub hw_version: u32,
    pub 1]: u8 hw_capab[QLINK_HW_CAPAB_NUM / BITS_PER_BYTE +,
}

extern "C" {
    pub fn qtnf_mac_iface_comb_free(mac: *mut qtnf_wmac);
}
extern "C" {
    pub fn qtnf_mac_ext_caps_free(mac: *mut qtnf_wmac);
}
extern "C" {
    pub fn qtnf_slave_radar_get() -> bool;
}
extern "C" {
    pub fn qtnf_dfs_offload_get() -> bool;
}
extern "C" {
    pub fn qtnf_main_work_queue(work: *mut work_struct);
}
extern "C" {
    pub fn qtnf_cmd_send_update_phy_params(mac: *mut qtnf_wmac, changed: u32) -> c_int;
}
extern "C" {
    pub fn qtnf_wake_all_queues(ndev: *mut net_device);
}
extern "C" {
    pub fn qtnf_virtual_intf_cleanup(ndev: *mut net_device);
}
extern "C" {
    pub fn qtnf_netdev_updown(ndev: *mut net_device, up: bool);
}
extern "C" {
    pub fn qtnf_scan_done(mac: *mut qtnf_wmac, aborted: bool);
}
extern "C" {
    pub fn qtnf_netdev_is_qtn(ndev: *const net_device) -> bool;
}
