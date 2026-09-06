//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wlcore/wlcore_i.h
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
// This file is part of wl1271
//
// Copyright (C) 1998-2009 Texas Instruments. All rights reserved.
// Copyright (C) 2008-2009 Nokia Corporation
//
// Contact: Luciano Coelho <luciano.coelho@nokia.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilink_family_data {
    pub name: *const c_char,
    pub /: *const *const *const char nvs_name; / wl12xx nvs file,
    pub /: *const *const *const char cfg_name; / wl18xx cfg file,
}

pub const WL1271_TX_SQN_POST_RECOVERY_PADDING: c_uint = 0xff;
// Use smaller padding for GEM, as some  APs have issues when it's too big
pub const WL1271_TX_SQN_POST_RECOVERY_PADDING_GEM: c_uint = 0x20;
pub const WL1271_CIPHER_SUITE_GEM: c_uint = 0x00147201;
pub const WL1271_BUSY_WORD_CNT: c_int = 1;

pub const WL1271_ELP_HW_STATE_ASLEEP: c_int = 0;
pub const WL1271_ELP_HW_STATE_IRQ: c_int = 1;
pub const WL1271_DEFAULT_BEACON_INT: c_int = 100;
pub const WL1271_DEFAULT_DTIM_PERIOD: c_int = 1;
pub const WL12XX_MAX_ROLES: c_int = 4;
pub const WL12XX_INVALID_ROLE_ID: c_uint = 0xff;
pub const WL12XX_INVALID_LINK_ID: c_uint = 0xff;
//
// max number of links allowed by all HWs.
// this is NOT the actual max links supported by the current hw.
//
pub const WLCORE_MAX_LINKS: c_int = 16;
// the driver supports the 2.4Ghz and 5Ghz bands
pub const WLCORE_NUM_BANDS: c_int = 2;
pub const WL12XX_MAX_RATE_POLICIES: c_int = 16;
pub const WLCORE_MAX_KLV_TEMPLATES: c_int = 4;
// Defined by FW as 0. Will not be freed or allocated.
pub const WL12XX_SYSTEM_HLID: c_int = 0;
//
// When in AP-mode, we allow (at least) this number of packets
// to be transmitted to FW for a STA in PS-mode. Only when packets are
// present in the FW buffers it will wake the sleeping STA. We want to put
// enough packets for the driver to transmit all of its buffered data before
// the STA goes to sleep again. But we don't want to take too much memory
// as it might hurt the throughput of active STAs.
//
pub const WL1271_PS_STA_MAX_PACKETS: c_int = 2;
pub const WL1271_AP_BSS_INDEX: c_int = 0;
pub const WL1271_AP_DEF_BEACON_EXP: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wlcore_state {
    WLCORE_STATE_OFF,
    WLCORE_STATE_RESTARTING,
    WLCORE_STATE_ON,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl12xx_fw_type {
    WL12XX_FW_TYPE_NONE,
    WL12XX_FW_TYPE_NORMAL,
    WL12XX_FW_TYPE_MULTI,
    WL12XX_FW_TYPE_PLT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_chip {
    pub id: u32,
    pub fw_ver_str: [c_char; ETHTOOL_FWVERS_LEN],
    pub fw_ver: [c_uint; NUM_FW_VER],
    pub phy_fw_ver_str: [c_char; ETHTOOL_FWVERS_LEN],
}

pub const NUM_TX_QUEUES: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl_fw_status {
    pub intr: u32,
    pub fw_rx_counter: u8,
    pub drv_rx_counter: u8,
    pub tx_results_counter: u8,
    pub rx_pkt_descs: *mut __le32,
    pub fw_localtime: u32,
//
// A bitmap (where each bit represents a single HLID)
// to indicate if the station is in PS mode.
//
    pub link_ps_bitmap: u32,
//
// A bitmap (where each bit represents a single HLID) to indicate
// if the station is in Fast mode
//
    pub link_fast_bitmap: u32,
// Cumulative counter of total released mem blocks since FW-reset
    pub total_released_blks: u32,
// Size (in Memory Blocks) of TX pool
    pub tx_total: u32,
//
// Cumulative counter of released packets per AC
// (length of the array is NUM_TX_QUEUES)
//
    pub tx_released_pkts: *mut u8,
//
// Cumulative counter of freed packets per HLID
// (length of the array is wl->num_links)
//
    pub tx_lnk_free_pkts: *mut u8,
// PN16 of last TKIP/AES seq-num per HLID
    pub tx_lnk_sec_pn16: *mut __le16,
// Cumulative counter of released Voice memory blocks
    pub tx_voice_released_blks: u8,
// Tx rate of the last transmitted packet
    pub tx_last_rate: u8,
// Tx rate or Tx rate estimate pre calculated by fw in mbps
    pub tx_last_rate_mbps: u8,
// hlid for which the rates were reported
    pub hlid: u8,
    pub counters: },
    pub log_start_addr: u32,
// Private status to be used by the lower drivers
    pub priv: *mut c_void,
}

pub const WL1271_MAX_CHANNELS: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_scan {
    pub req: *mut cfg80211_scan_request,
    pub scanned_ch: [c_ulong; BITS_TO_LONGS(WL1271_MAX_CHANNELS)],
    pub failed: bool,
    pub state: u8,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN+1],
    pub ssid_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_if_operations {
    pub fixed): size_t len, bool,
    pub fixed): size_t len, bool,
    pub child): *mut *mut void (reset)(struct device,
    pub child): *mut *mut void (init)(struct device,
    pub enable): *mut *mut *mut int (power)(struct device child, bool,
    pub blksz): *mut *mut *mut void (set_block_size) (struct device child, unsigned int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlcore_platdev_data {
    pub if_ops: *mut wl1271_if_operations,
    pub family: *const wilink_family_data,
    pub /: *mut *mut bool ref_clock_xtal; / specify whether the clock is XTAL or not,
    pub /: *mut *mut u32 ref_clock_freq; / in Hertz,
    pub /: *mut *mut u32 tcxo_clock_freq; / in Hertz, tcxo is always XTAL,
    pub pwr_in_suspend: bool,
}

pub const MAX_NUM_KEYS: c_int = 14;
pub const MAX_KEY_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_ap_key {
    pub id: u8,
    pub key_type: u8,
    pub key_size: u8,
    pub key: [u8; MAX_KEY_SIZE],
    pub hlid: u8,
    pub tx_seq_32: u32,
    pub tx_seq_16: u16,
    pub is_pairwise: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl12xx_flags {
    WL1271_FLAG_GPIO_POWER,
    WL1271_FLAG_TX_QUEUE_STOPPED,
    WL1271_FLAG_TX_PENDING,
    WL1271_FLAG_IN_ELP,
    WL1271_FLAG_IRQ_RUNNING,
    WL1271_FLAG_FW_TX_BUSY,
    WL1271_FLAG_DUMMY_PACKET_PENDING,
    WL1271_FLAG_SUSPENDED,
    WL1271_FLAG_PENDING_WORK,
    WL1271_FLAG_SOFT_GEMINI,
    WL1271_FLAG_RECOVERY_IN_PROGRESS,
    WL1271_FLAG_VIF_CHANGE_IN_PROGRESS,
    WL1271_FLAG_INTENDED_FW_RECOVERY,
    WL1271_FLAG_IO_FAILED,
    WL1271_FLAG_REINIT_TX_WDOG,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl12xx_vif_flags {
    WLVIF_FLAG_INITIALIZED,
    WLVIF_FLAG_STA_ASSOCIATED,
    WLVIF_FLAG_STA_AUTHORIZED,
    WLVIF_FLAG_IBSS_JOINED,
    WLVIF_FLAG_AP_STARTED,
    WLVIF_FLAG_IN_PS,
    WLVIF_FLAG_STA_STATE_SENT,
    WLVIF_FLAG_RX_STREAMING_STARTED,
    WLVIF_FLAG_PSPOLL_FAILURE,
    WLVIF_FLAG_CS_PROGRESS,
    WLVIF_FLAG_AP_PROBE_RESP_SET,
    WLVIF_FLAG_IN_USE,
    WLVIF_FLAG_ACTIVE,
    WLVIF_FLAG_BEACON_DISABLED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_link {
// AP-mode - TX queue per AC in link
    pub tx_queue: [sk_buff_head; NUM_TX_QUEUES],
// accounting for allocated / freed packets in FW
    pub allocated_pkts: u8,
    pub prev_freed_pkts: u8,
    pub prev_sec_pn16: u16,
    pub addr: [u8; ETH_ALEN],
// bitmap of TIDs where RX BA sessions are active for this link
    pub ba_bitmap: u8,
// the last fw rate index we used for this link
    pub fw_rate_idx: u8,
// the last fw rate [Mbps] we used for this link
    pub fw_rate_mbps: u8,
// The wlvif this link belongs to. Might be null for global links
    pub wlvif: *mut wl12xx_vif,
//
// total freed FW packets on the link - used for tracking the
// AES/TKIP PN across recoveries. Re-initialized each time
// from the wl1271_station structure.
//
    pub total_freed_pkts: u64,
}

pub const WL1271_MAX_RX_FILTERS: c_int = 5;
pub const WL1271_RX_FILTER_MAX_FIELDS: c_int = 8;
pub const WL1271_RX_FILTER_ETH_HEADER_SIZE: c_int = 14;
pub const WL1271_RX_FILTER_MAX_FIELDS_SIZE: c_int = 95;

pub const WL1271_RX_FILTER_FLAG_IP_HEADER: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rx_filter_action {
    FILTER_DROP = 0,
    FILTER_SIGNAL = 1,
    FILTER_FW_HANDLE = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum plt_mode {
    PLT_OFF = 0,
    PLT_ON = 1,
    PLT_FEM_DETECT = 2,
    PLT_CHIP_AWAKE = 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_rx_filter_field {
    pub offset: __le16,
    pub len: u8,
    pub flags: u8,
    pub pattern: *mut u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_rx_filter {
    pub action: u8,
    pub num_fields: c_int,
    pub fields: [wl12xx_rx_filter_field; WL1271_RX_FILTER_MAX_FIELDS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_station {
    pub hlid: u8,
    pub fw_added: bool,
    pub in_connection: bool,
//
// total freed FW packets on the link to the STA - used for tracking the
// AES/TKIP PN across recoveries. Re-initialized each time from the
// wl1271_station structure.
// Used in both AP and STA mode.
//
    pub total_freed_pkts: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_vif {
    pub wl: *mut wl1271,
    pub list: list_head,
    pub flags: c_ulong,
    pub bss_type: u8,
    pub /: *mut *mut u8 p2p; / we are using p2p role,
    pub role_id: u8,
// sta/ibss specific
    pub dev_role_id: u8,
    pub dev_hlid: u8,
    pub hlid: u8,
    pub basic_rate_idx: u8,
    pub ap_rate_idx: u8,
    pub p2p_rate_idx: u8,
    pub klv_template_id: u8,
    pub qos: bool,
// channel type we started the STA role with
    pub role_chan_type: nl80211_channel_type,
    pub sta: },
    pub global_hlid: u8,
    pub bcast_hlid: u8,
// HLIDs bitmap of associated stations
// recoreded keys - set here before AP startup
    pub recorded_keys: [*mut wl1271_ap_key; MAX_NUM_KEYS],
    pub mgmt_rate_idx: u8,
    pub bcast_rate_idx: u8,
    pub ucast_rate_idx: [u8; CONF_TX_MAX_AC_COUNT],
    pub ap: },
}

// the hlid of the last transmitted skb
// counters of packets per AC, across all links in the vif
// The current band
//
// currently configured rate set:
// bits  0-15 - 802.11abg rates
// bits 16-23 - 802.11n   MCS index mask
// support only 1 stream, thus only 8 bits for the MCS rates (0-7).
//
// probe-req template for the current AP
// Beaconing interval (needed for ad-hoc)
// Default key (for WEP)
// Our association ID
// retry counter for PSM entries
// in dBm
// save the current encryption type for auto-arp config
// RX BA constraint value
// Rx Streaming
// number of in connection stations
//
// This vif's queues are mapped to mac80211 HW queues as:
// VO - hw_queue_base
// VI - hw_queue_base + 1
// BE - hw_queue_base + 2
// BK - hw_queue_base + 3
//
// do we have a pending auth reply? (and ROC)
// time when we sent the pending auth reply
// work for canceling ROC after pending auth reply
// update rate conrol
//
// total freed FW packets on the link.
// For STA this holds the PN of the link to the AP.
// For AP this holds the PN of the broadcast link.
//
// This struct must be last!
// data that has to be saved acrossed reconfigs (e.g. recovery)
// should be declared in this struct.
//
extern "C" {
    pub fn container_of()wlvif: *mut (void, ieee80211_vif: struct, _arg: drv_priv) -> return;
}

extern "C" {
    pub fn wl1271_plt_start(wl: *mut wl1271, plt_mode: plt_mode) -> c_int;
}
extern "C" {
    pub fn wl1271_plt_stop(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wl1271_recalc_rx_streaming(wl: *mut wl1271, wlvif: *mut wl12xx_vif) -> c_int;
}
extern "C" {
    pub fn wl12xx_queue_recovery_work(wl: *mut wl1271);
}
extern "C" {
    pub fn wl12xx_copy_fwlog(wl: *mut wl1271, memblock: *mut u8, maxlen: usize) -> usize;
}
extern "C" {
    pub fn wl1271_rx_filter_free(filter: *mut wl12xx_rx_filter);
}
extern "C" {
    pub fn wl1271_rx_filter_get_fields_size(filter: *mut wl12xx_rx_filter) -> c_int;
}

pub const WL1271_DEFAULT_POWER_LEVEL: c_int = 0;
pub const WL1271_TX_QUEUE_LOW_WATERMARK: c_int = 32;
pub const WL1271_TX_QUEUE_HIGH_WATERMARK: c_int = 256;
pub const WL1271_DEFERRED_QUEUE_LIMIT: c_int = 64;
// WL1271 needs a 200ms sleep after power on, and a 20ms sleep before power

// Macros to handle wl1271.sta_rate_set
pub const HW_BG_RATES_MASK: c_uint = 0xffff;
pub const HW_HT_RATES_OFFSET: c_int = 16;
pub const HW_MIMO_RATES_OFFSET: c_int = 24;
