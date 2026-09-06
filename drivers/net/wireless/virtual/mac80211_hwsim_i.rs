//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/virtual/mac80211_hwsim_i.h
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
// mac80211_hwsim - software simulator of 802.11 radio(s) for mac80211
// Copyright (c) 2008, Jouni Malinen <j@w1.fi>
// Copyright (c) 2011, Javier Lopez <jlopex@gmail.com>
// Copyright (c) 2016 - 2017 Intel Deutschland GmbH
// Copyright (C) 2018 - 2026 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwsim_sta_nan_sched {
// Later members are protected by this lock
    pub lock: spinlock_t,
    pub committed_dw: u16,
    pub map_id: u8,
    pub chans: [cfg80211_chan_def; CFG80211_NAN_SCHED_NUM_TIME_SLOTS],
    pub maps: [}; CFG80211_NAN_MAX_PEER_MAPS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwsim_sta_priv {
    pub magic: u32,
    pub last_link: c_uint,
    pub active_links_rx: u16,
// NAN peer schedule - must be accessed under nan_sched.lock
    pub nan_sched: hwsim_sta_nan_sched,
}

pub const HWSIM_STA_MAGIC: c_uint = 0x6d537749;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac80211_hwsim_link_data {
    pub link_id: u32,
    pub /: *mut *mut u64 beacon_int / beacon interval in us,
    pub beacon_timer: hrtimer,
}

pub const HWSIM_NUM_CHANNELS_2GHZ: c_int = 14;
pub const HWSIM_NUM_CHANNELS_5GHZ: c_int = 40;
pub const HWSIM_NUM_CHANNELS_6GHZ: c_int = 59;
pub const HWSIM_NUM_S1G_CHANNELS_US: c_int = 51;
pub const HWSIM_NUM_RATES: c_int = 12;
pub const HWSIM_NUM_CIPHERS: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac80211_hwsim_data {
    pub list: list_head,
    pub rht: rhash_head,
    pub hw: *mut ieee80211_hw,
    pub dev: *mut device,
    pub bands: [ieee80211_supported_band; NUM_NL80211_BANDS],
    pub channels_2ghz: [ieee80211_channel; HWSIM_NUM_CHANNELS_2GHZ],
    pub channels_5ghz: [ieee80211_channel; HWSIM_NUM_CHANNELS_5GHZ],
    pub channels_6ghz: [ieee80211_channel; HWSIM_NUM_CHANNELS_6GHZ],
    pub channels_s1g: [ieee80211_channel; HWSIM_NUM_S1G_CHANNELS_US],
    pub rates: [ieee80211_rate; HWSIM_NUM_RATES],
    pub if_combination: ieee80211_iface_combination,
    pub if_limits: [ieee80211_iface_limit; 5],
    pub n_if_limits: c_int,
// Storage space for channels, etc.
    pub phy_data: *mut mac80211_hwsim_phy_data,
    pub if_combination_radio: ieee80211_iface_combination,
    pub radio_range: [wiphy_radio_freq_range; NUM_NL80211_BANDS],
    pub radio: [wiphy_radio; NUM_NL80211_BANDS],
    pub ciphers: [u32; HWSIM_NUM_CIPHERS],
    pub addresses: [mac_address; 3],
    pub idx: int channels,,
    pub use_chanctx: bool,
    pub destroy_on_close: bool,
    pub portid: u32,
    pub alpha2: [c_char; 2],
    pub regd: *const ieee80211_regdomain,
    pub tmp_chan: *mut ieee80211_channel,
    pub roc_chan: *mut ieee80211_channel,
    pub roc_duration: u32,
    pub roc_start: delayed_work,
    pub roc_done: delayed_work,
    pub hw_scan: delayed_work,
    pub hw_scan_request: *mut cfg80211_scan_request,
    pub hw_scan_vif: *mut ieee80211_vif,
    pub scan_chan_idx: c_int,
    pub scan_addr: [u8; ETH_ALEN],
    pub channel: *mut ieee80211_channel,
    pub end: unsigned long next_start, start,,
    pub channel: *mut ieee80211_channel,
    pub bw: nl80211_chan_width,
    pub rx_filter: c_uint,
    pub scanning: bool started, idle,,
    pub mutex: mutex,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ps_mode {
    PS_DISABLED, PS_ENABLED, PS_AUTO_POLL, PS_MANUAL_POLL
    } ps;
    bool ps_poll_pending;
    struct dentry *debugfs;
    struct cfg80211_chan_def radar_background_chandef;

    atomic_t pending_cookie;
    struct sk_buff_head pending;	/* packets pending */
//
// Only radios in the same group can communicate together (the
// channel has to match too). Each bit represents a group. A
// radio can be in more than one group.
//
    u64 group;

// group shared by radios created in the same netns
    int netgroup;
// wmediumd portid responsible for netgroup of this radio
    u32 wmediumd;

// difference between this hw's clock and the real clock, in usecs
    spinlock_t tsf_offset_lock;
    s64 tsf_offset;

// Stats
    u64 tx_pkts;
    u64 rx_pkts;
    u64 tx_bytes;
    u64 rx_bytes;
    u64 tx_dropped;
    u64 tx_failed;

// RSSI in rx status of the receiver
    int rx_rssi;

// only used when pmsr capability is supplied
    struct cfg80211_pmsr_capabilities pmsr_capa;
    struct cfg80211_pmsr_request *pmsr_request;
    struct wireless_dev *pmsr_request_wdev;

    struct mac80211_hwsim_link_data link_data[IEEE80211_MLD_MAX_NUM_LINKS];

    struct mac80211_hwsim_nan_data nan;
}

    pub hwsim_radio_lock: extern spinlock_t,
    pub hwsim_radios: extern struct list_head,
    pub tsf): u64,
    pub ts): ktime_t,
    pub vif): *mut ieee80211_vif,
    pub chan): *mut ieee80211_channel,
    pub txq): *mut ieee80211_txq,
