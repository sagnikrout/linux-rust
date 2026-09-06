//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mld/link.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2024-2026 Intel Corporation
//

// Macro flag: #define __iwl_mld_link_h__

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mld_link_chan_load_level {
    LINK_CHAN_LOAD_LVL_NONE,
    LINK_CHAN_LOAD_LVL1,
    LINK_CHAN_LOAD_LVL2,
    LINK_CHAN_LOAD_LVL3,
    LINK_CHAN_LOAD_LVL_MAX  = LINK_CHAN_LOAD_LVL3
}

//
// struct iwl_probe_resp_data - data for NoA/CSA updates
// @rcu_head: used for freeing the data on update
// @notif: notification data
// @noa_len: length of NoA attribute, calculated from the notification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_probe_resp_data {
    pub rcu_head: rcu_head,
    pub notif: iwl_probe_resp_data_notif,
    pub noa_len: c_int,
}

//
// struct iwl_mld_link - link configuration parameters
//
// @rcu_head: RCU head for freeing this data.
// @fw_id: the fw id of the link.
// @active: if the link is active or not.
// @avg_signal: The current average signal of beacons [dBm] retrieved from
// firmware per-link periodic stats (STATISTICS_OPER_NOTIF).
// @queue_params: QoS data from mac80211. This is updated with a call to
// drv_conf_tx per each AC, and then notified once with BSS_CHANGED_QOS.
// So we store it here and then send one link cmd for all the ACs.
// @chan_ctx: pointer to the channel context assigned to the link. If a link
// has an assigned channel context it means that it is active.
// @he_ru_2mhz_block: 26-tone RU OFDMA transmissions should be blocked.
// @tx_igtk: FW can only have one IGTK per MAC at a time, whereas mac80211 can
// have two. This tracks the one IGTK that currently exists in FW, for TX
// purposes. The RX IGTKs are tracked per station.
// @bigtks: BIGTKs of the AP. Only valid for STA mode.
// @bcast_sta: station used for broadcast packets. Used in AP, GO and IBSS.
// @mcast_sta: station used for multicast packets. Used in AP, GO and IBSS.
// @mon_sta: station used for TX injection in monitor interface.
// @last_cqm_rssi_event: rssi of the last cqm rssi event
// @average_beacon_energy: average beacon energy for beacons received during
// client connections
// @ap_early_keys: The firmware cannot install keys before bcast/mcast STAs,
// but higher layers work differently, so we store the keys here for
// later installation.
// @silent_deactivation: next deactivation needs to be silent.
// @probe_resp_data: data from FW notification to store NOA related data to be
// inserted into probe response.
// @chan_load_lvl: current channel load level for a link, computed based on
// channel load by others on a link.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_link {
    pub rcu_head: rcu_head,
// Add here fields that need clean up on restart
    pub fw_id: u8,
    pub active: bool,
    pub avg_signal: i8,
    pub queue_params: [ieee80211_tx_queue_params; IEEE80211_NUM_ACS],
    pub chan_ctx: *mut ieee80211_chanctx_conf __rcu,
    pub he_ru_2mhz_block: bool,
    pub tx_igtk: *mut ieee80211_key_conf,
    pub bigtks: [*mut ieee80211_key_conf __rcu; 2],
    pub chan_load_lvl: iwl_mld_link_chan_load_level,
// And here fields that survive a fw restart
    pub bcast_sta: iwl_mld_int_sta,
    pub mcast_sta: iwl_mld_int_sta,
    pub mon_sta: iwl_mld_int_sta,
    pub last_cqm_rssi_event: c_int,
// we can only have 2 GTK + 2 IGTK + 2 BIGTK active at a time
    pub ap_early_keys: [*mut ieee80211_key_conf; 6],
    pub average_beacon_energy: u32,
    pub silent_deactivation: bool,
    pub probe_resp_data: *mut iwl_probe_resp_data __rcu,
}

// Cleanup function for struct iwl_mld_link, will be called in restart
// Convert a percentage from [0,100] to [0,255]

extern "C" {
    pub fn iwl_mld_get_psd_eirp_rssi_adjust(link_conf: *mut ieee80211_bss_conf) -> i8;
}

