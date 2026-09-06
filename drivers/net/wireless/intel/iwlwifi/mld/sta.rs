//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mld/sta.h
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

// Macro flag: #define __iwl_mld_sta_h__

//
// struct iwl_mld_rxq_dup_data - Duplication detection data, per STA & Rx queue
// @last_seq: last sequence per tid.
// @last_sub_frame_idx: the index of the last subframe in an A-MSDU. This value
// will be zero if the packet is not part of an A-MSDU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_rxq_dup_data {
    pub 1]: __le16 last_seq[IWL_MAX_TID_COUNT +,
    pub 1]: u8 last_sub_frame_idx[IWL_MAX_TID_COUNT +,
    pub ____cacheline_aligned_in_smp: },
//
// struct iwl_mld_link_sta - link-level station
//
// This represents the link-level sta - the driver level equivalent to the
// ieee80211_link_sta
//
// @rx_igtk: FW can only have one IGTK for RX at a time, whereas mac80211 will
// have two. This tracks the one IGTK that currently exists in FW, to
// remove it there when a new one is installed.
// @last_rate_n_flags: rate_n_flags from the last &iwl_tlc_update_notif
// @signal_avg: the signal average coming from the firmware
// @in_fw: whether the link STA is uploaded to the FW (false during restart)
// @rcu_head: RCU head for freeing this object
// @fw_id: the FW id of this link sta.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_link_sta {
// Add here fields that need clean up on restart
    pub rx_igtk: *mut ieee80211_key_conf,
    pub last_rate_n_flags: u32,
    pub in_fw: bool,
    pub signal_avg: i8,
// And here fields that survive a fw restart
    pub rcu_head: rcu_head,
    pub fw_id: u32,
}

pub const IWL_NUM_DEFAULT_KEYS: c_int = 4;
// struct iwl_mld_ptk_pn - Holds Packet Number (PN) per TID.
// @rcu_head: RCU head for freeing this data.
// @pn: Array storing PN for each TID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_ptk_pn {
    pub rcu_head: rcu_head,
    pub pn: [u8; IWL_MAX_TID_COUNT][IEEE80211_CCMP_PN_LEN],
    pub q: [} ____cacheline_aligned_in_smp; ],
}

//
// struct iwl_mld_per_link_mpdu_counter - per-link TX/RX MPDU counters
//
// @tx: Number of TX MPDUs.
// @rx: Number of RX MPDUs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_per_link_mpdu_counter {
    pub tx: u32,
    pub rx: u32,
}

//
// struct iwl_mld_per_q_mpdu_counter - per-queue MPDU counter
//
// @lock: Needed to protect the counters when modified from statistics.
// @per_link: per-link counters.
// @window_start_time: timestamp of the counting-window start
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_per_q_mpdu_counter {
    pub lock: spinlock_t,
    pub per_link: [iwl_mld_per_link_mpdu_counter; IWL_FW_MAX_LINKS],
    pub window_start_time: c_ulong,
    pub ____cacheline_aligned_in_smp: },
//
// struct iwl_mld_sta - representation of a station in the driver.
//
// This represent the MLD-level sta, and will not be added to the FW.
// Embedded in ieee80211_sta.
//
// @vif: pointer the vif object.
// @sta_state: station state according to enum %ieee80211_sta_state
// @sta_type: type of this station. See &enum iwl_fw_sta_type
// @mld: a pointer to the iwl_mld object
// @dup_data: per queue duplicate packet detection data
// @data_tx_ant: stores the last TX antenna index; used for setting
// TX rate_n_flags for injected data frames (toggles on every TX failure).
// @tid_to_baid: a simple map of TID to Block-Ack fw id
// @deflink: This holds the default link STA information, for non MLO STA all
// link specific STA information is accessed through @deflink or through
// link[0] which points to address of @deflink. For MLO Link STA
// the first added link STA will point to deflink.
// @link: reference to Link Sta entries. For Non MLO STA, except 1st link,
// i.e link[0] all links would be assigned to NULL by default and
// would access link information via @deflink or link[0]. For MLO
// STA, first link STA being added will point its link pointer to
// @deflink address and remaining would be allocated and the address
// would be assigned to link[link_id] where link_id is the id assigned
// by the AP.
// @ptk_pn: Array of pointers to PTK PN data, used to track the Packet Number
// per key index and per queue (TID).
// @mpdu_counters: RX/TX MPDUs counters for each queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_sta {
// Add here fields that need clean up on restart
    pub sta_state: ieee80211_sta_state,
    pub sta_type: iwl_fw_sta_type,
// And here fields that survive a fw restart
    pub mld: *mut iwl_mld,
    pub vif: *mut ieee80211_vif,
    pub dup_data: *mut iwl_mld_rxq_dup_data,
    pub tid_to_baid: [u8; IWL_MAX_TID_COUNT],
    pub data_tx_ant: u8,
    pub deflink: iwl_mld_link_sta,
    pub link: [*mut iwl_mld_link_sta __rcu; IEEE80211_MLD_MAX_NUM_LINKS],
    pub ptk_pn: [*mut iwl_mld_ptk_pn __rcu; IWL_NUM_DEFAULT_KEYS],
    pub mpdu_counters: *mut iwl_mld_per_q_mpdu_counter,
}

// not an MLD STA; only has the deflink with ID zero
// Should not happen as link removal should always succeed
extern "C" {
    pub fn iwl_mld_link_sta_dereference_check(_arg: mld_sta, _arg: link_sta->link_id) -> return;
}
extern "C" {
    pub fn iwl_mld_remove_sta(mld: *mut iwl_mld, sta: *mut ieee80211_sta);
}
extern "C" {
    pub fn iwl_mld_fw_sta_id_mask(mld: *mut iwl_mld, sta: *mut ieee80211_sta) -> u32;
}
extern "C" {
    pub fn iwl_mld_flush_sta_txqs(mld: *mut iwl_mld, sta: *mut ieee80211_sta);
}
extern "C" {
    pub fn iwl_mld_count_mpdu_tx(link_sta: *mut ieee80211_link_sta, count: u32);
}
//
// struct iwl_mld_int_sta - representation of an internal station
// (a station that exist in FW and in driver, but not in mac80211)
//
// @sta_id: the index of the station in the fw
// @queue_id: the if of the queue used by the station
// @sta_type: station type. One of &iwl_fw_sta_type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_int_sta {
    pub sta_id: u8,
    pub queue_id: u32,
    pub sta_type: iwl_fw_sta_type,
}
