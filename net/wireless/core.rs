//! Automatically rewritten from C Header to Rust Module
//! Source: net/wireless/core.h
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
// Wireless configuration interface internals.
//
// Copyright 2006-2010	Johannes Berg <johannes@sipsolutions.net>
// Copyright (C) 2018-2026 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_scan_request_int {
    pub info: cfg80211_scan_info,
    pub notified: bool,
// must be last - variable members
    pub req: cfg80211_scan_request,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_registered_device {
    pub ops: *const cfg80211_ops,
    pub list: list_head,
// rfkill support
    pub rfkill_ops: rfkill_ops,
    pub rfkill_block: work_struct,
// ISO / IEC 3166 alpha2 for which this device is receiving
// country IEs on, this can help disregard country IEs from APs
// on the same alpha2 quickly. The alpha2 may differ from
// cfg80211_regdomain's alpha2 when an intersection has occurred.
// If the AP is reconfigured this can also be used to tell us if
// the country on the country IE changed.
    pub country_ie_alpha2: [c_char; 2],
//
// the driver requests the regulatory core to set this regulatory
// domain as the wiphy's. Only used for %REGULATORY_WIPHY_SELF_MANAGED
// devices using the regulatory_set_wiphy_regd() API
//
    pub requested_regd: *const ieee80211_regdomain,
// If a Country IE has been received this tells us the environment
// which its telling us its in. This defaults to ENVIRON_ANY
    pub env: environment_cap,
// wiphy index, internal only
    pub wiphy_idx: c_int,
// protected by RTNL
    pub wdev_id: int devlist_generation,,
    pub opencount: c_int,
    pub dev_wait: wait_queue_head_t,
    pub beacon_registrations: list_head,
    pub beacon_registrations_lock: spinlock_t,
// protected by RTNL only
    pub num_running_ifaces: c_int,
    pub num_running_monitor_ifaces: c_int,
    pub cookie_counter: u64,
// BSSes/scanning
    pub bss_lock: spinlock_t,
    pub bss_list: list_head,
    pub bss_tree: rb_root,
    pub bss_generation: u32,
    pub bss_entries: u32,
    pub /: *mut *mut *mut cfg80211_scan_request_int scan_req; / protected by RTNL,
    pub int_scan_req: *mut cfg80211_scan_request_int,
    pub scan_msg: *mut sk_buff,
    pub sched_scan_req_list: list_head,
    pub suspend_at: time64_t,
    pub scan_done_wk: wiphy_work,
    pub cur_cmd_info: *mut genl_info,
    pub conn_work: work_struct,
    pub event_work: work_struct,
    pub dfs_update_channels_wk: delayed_work,
    pub background_radar_wdev: *mut wireless_dev,
    pub background_radar_chandef: cfg80211_chan_def,
    pub background_cac_done_wk: delayed_work,
    pub background_cac_abort_wk: work_struct,
// netlink port which started critical protocol (0 means not started)
    pub crit_proto_nlportid: u32,
    pub coalesce: *mut cfg80211_coalesce,
    pub destroy_work: work_struct,
    pub sched_scan_stop_wk: wiphy_work,
    pub sched_scan_res_wk: work_struct,
    pub radar_chandef: cfg80211_chan_def,
    pub propagate_radar_detect_wk: work_struct,
    pub cac_done_chandef: cfg80211_chan_def,
    pub propagate_cac_done_wk: work_struct,
    pub mgmt_registrations_update_wk: work_struct,
// lock for all wdev lists
    pub mgmt_registrations_lock: spinlock_t,
    pub wiphy_work: work_struct,
    pub wiphy_work_list: list_head,
// protects the list above
    pub wiphy_work_lock: spinlock_t,
    pub suspended: bool,
// must be last because of the way we do wiphy_priv(),
// and it should at least be aligned to NETDEV_ALIGN
    pub __aligned(NETDEV_ALIGN): wiphy wiphy,
}

extern "C" {
    pub fn container_of(_arg: wiphy, cfg80211_registered_device: struct, _arg: wiphy) -> return;
}

// This is constructed like this so it can be used in if/else

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bss_source_type {
    BSS_SOURCE_DIRECT = 0,
    BSS_SOURCE_MBSSID,
    BSS_SOURCE_STA_PROFILE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_internal_bss {
    pub list: list_head,
    pub hidden_list: list_head,
    pub rbn: rb_node,
    pub ts: c_ulong,
    pub refcount: c_ulong,
    pub hold: core::sync::atomic::AtomicI32,
// time at the start of the reception of the first octet of the
// timestamp field of the last beacon/probe received for this BSS.
// The time is the TSF of the BSS specified by %parent_bssid.
//
    pub parent_tsf: u64,
// the BSS according to which %parent_tsf is set. This is set to
// the BSS that the interface that requested the scan was connected to
// when the beacon/probe was received.
//
    pub __aligned(2): u8 parent_bssid[ETH_ALEN],
    pub bss_source: bss_source_type,
// must be last because of priv member
    pub pub: cfg80211_bss,
}

extern "C" {
    pub fn container_of(_arg: pub, cfg80211_internal_bss: struct, _arg: pub) -> return;
}
extern "C" {
    pub fn get_wiphy_idx(wiphy: *mut wiphy) -> c_int;
}
extern "C" {
    pub fn cfg80211_init_wdev(wdev: *mut wireless_dev);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cfg80211_event_type {
    EVENT_CONNECT_RESULT,
    EVENT_ROAMED,
    EVENT_DISCONNECTED,
    EVENT_IBSS_JOINED,
    EVENT_STOPPED,
    EVENT_PORT_AUTHORIZED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_event {
    pub list: list_head,
    pub type: cfg80211_event_type,
    pub cr: cfg80211_connect_resp_params,
    pub rm: cfg80211_roam_info,
    pub ie: *const u8,
    pub ie_len: usize,
    pub reason: u16,
    pub locally_generated: bool,
    pub dc: },
    pub bssid: [u8; ETH_ALEN],
    pub channel: *mut ieee80211_channel,
    pub ij: },
    pub peer_addr: [u8; ETH_ALEN],
    pub td_bitmap: *const u8,
    pub td_bitmap_len: u8,
    pub pa: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_cached_keys {
    pub params: [key_params; 4],
    pub data: [u8; 4][WLAN_KEY_LEN_WEP104],
    pub def: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_beacon_registration {
    pub list: list_head,
    pub nlportid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_cqm_config {
    pub rcu_head: rcu_head,
    pub rssi_hyst: u32,
    pub last_rssi_event_value: i32,
    pub last_rssi_event_type: nl80211_cqm_rssi_threshold_event,
    pub use_range_api: bool,
    pub n_rssi_thresholds: c_int,
    pub __counted_by(n_rssi_thresholds): s32 rssi_thresholds[],
}

extern "C" {
    pub fn cfg80211_destroy_ifaces(rdev: *mut cfg80211_registered_device);
}
// free object
extern "C" {
    pub fn cfg80211_dev_free(rdev: *mut cfg80211_registered_device);
}
extern "C" {
    pub fn ieee80211_set_bitrate_flags(wiphy: *mut wiphy);
}
extern "C" {
    pub fn cfg80211_bss_expire(rdev: *mut cfg80211_registered_device);
}
// IBSS
extern "C" {
    pub fn cfg80211_clear_ibss(dev: *mut net_device, nowext: bool);
}
// mesh
// OCB
// AP
// MLME
extern "C" {
    pub fn cfg80211_mgmt_registrations_update_wk(wk: *mut work_struct);
}
extern "C" {
    pub fn cfg80211_mlme_unregister_socket(wdev: *mut wireless_dev, nlpid: u32);
}
extern "C" {
    pub fn cfg80211_mlme_purge_registrations(wdev: *mut wireless_dev);
}
// SME events
extern "C" {
    pub fn cfg80211_autodisconnect_wk(wiphy: *mut wiphy, work: *mut wiphy_work);
}
// SME implementation
extern "C" {
    pub fn cfg80211_conn_work(work: *mut work_struct);
}
extern "C" {
    pub fn cfg80211_sme_scan_done(dev: *mut net_device);
}
extern "C" {
    pub fn cfg80211_sme_rx_assoc_resp(wdev: *mut wireless_dev, status: u16) -> bool;
}
extern "C" {
    pub fn cfg80211_sme_rx_auth(wdev: *mut wireless_dev, buf: *const u8, len: usize);
}
extern "C" {
    pub fn cfg80211_sme_disassoc(wdev: *mut wireless_dev);
}
extern "C" {
    pub fn cfg80211_sme_deauth(wdev: *mut wireless_dev);
}
extern "C" {
    pub fn cfg80211_sme_auth_timeout(wdev: *mut wireless_dev);
}
extern "C" {
    pub fn cfg80211_sme_assoc_timeout(wdev: *mut wireless_dev);
}
extern "C" {
    pub fn cfg80211_sme_abandon_assoc(wdev: *mut wireless_dev);
}
// internal helpers
extern "C" {
    pub fn cfg80211_supported_cipher_suite(wiphy: *mut wiphy, cipher: u32) -> bool;
}
extern "C" {
    pub fn __cfg80211_scan_done(wiphy: *mut wiphy, wk: *mut wiphy_work);
}
extern "C" {
    pub fn cfg80211_sched_scan_results_wk(work: *mut work_struct);
}
extern "C" {
    pub fn cfg80211_upload_connect_keys(wdev: *mut wireless_dev);
}
extern "C" {
    pub fn cfg80211_process_rdev_events(rdev: *mut cfg80211_registered_device);
}
extern "C" {
    pub fn cfg80211_process_wdev_events(wdev: *mut wireless_dev);
}
extern "C" {
    pub fn cfg80211_scan(rdev: *mut cfg80211_registered_device) -> c_int;
}

extern "C" {
    pub fn cfg80211_dfs_channels_update_work(work: *mut work_struct);
}
extern "C" {
    pub fn cfg80211_sched_dfs_chan_update(rdev: *mut cfg80211_registered_device);
}
extern "C" {
    pub fn cfg80211_stop_radar_detection(wdev: *mut wireless_dev);
}
extern "C" {
    pub fn cfg80211_stop_background_radar_detection(wdev: *mut wireless_dev);
}
extern "C" {
    pub fn cfg80211_background_cac_done_wk(work: *mut work_struct);
}
extern "C" {
    pub fn cfg80211_background_cac_abort_wk(work: *mut work_struct);
}
extern "C" {
    pub fn cfg80211_beaconing_iface_active(wdev: *mut wireless_dev) -> bool;
}
extern "C" {
    pub fn jiffies_to_msecs(start: end -) -> return;
}
extern "C" {
    pub fn jiffies_to_msecs(1: end + (ULONG_MAX - start) +) -> return;
}

//
// Trick to enable using it as a condition,
// and also not give a warning when it's
// not used that way.
//

extern "C" {
    pub fn cfg80211_release_pmsr(wdev: *mut wireless_dev, portid: u32);
}
extern "C" {
    pub fn cfg80211_pmsr_wdev_down(wdev: *mut wireless_dev);
}
extern "C" {
    pub fn cfg80211_pmsr_free_wk(wiphy: *mut wiphy, work: *mut wiphy_work);
}
extern "C" {
    pub fn cfg80211_remove_link(wdev: *mut wireless_dev, link_id: c_uint);
}
extern "C" {
    pub fn cfg80211_remove_links(wdev: *mut wireless_dev);
}
extern "C" {
    pub fn cfg80211_wdev_release_link_bsses(wdev: *mut wireless_dev, link_mask: u16);
}
//
// struct cfg80211_colocated_ap - colocated AP information
//
// @list: linked list to all colocated APs
// @bssid: BSSID of the reported AP
// @ssid: SSID of the reported AP
// @ssid_len: length of the ssid
// @center_freq: frequency the reported AP is on
// @unsolicited_probe: the reported AP is part of an ESS, where all the APs
// that operate in the same channel as the reported AP and that might be
// detected by a STA receiving this frame, are transmitting unsolicited
// Probe Response frames every 20 TUs
// @oct_recommended: OCT is recommended to exchange MMPDUs with the reported AP
// @same_ssid: the reported AP has the same SSID as the reporting AP
// @multi_bss: the reported AP is part of a multiple BSSID set
// @transmitted_bssid: the reported AP is the transmitting BSSID
// @colocated_ess: all the APs that share the same ESS as the reported AP are
// colocated and can be discovered via legacy bands.
// @short_ssid_valid: short_ssid is valid and can be used
// @short_ssid: the short SSID for this SSID
// @psd_20: The 20MHz PSD EIRP of the primary 20MHz channel for the reported AP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg80211_colocated_ap {
    pub list: list_head,
    pub bssid: [u8; ETH_ALEN],
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub ssid_len: usize,
    pub short_ssid: u32,
    pub center_freq: u32,
    pub psd_20: i8,
}

// Macro flag: #define VISIBLE_IF_CFG80211_KUNIT
extern "C" {
    pub fn cfg80211_free_coloc_ap_list(coloc_ap_list: *mut list_head);
}

// Macro flag: #define EXPORT_SYMBOL_IF_CFG80211_KUNIT(sym)

