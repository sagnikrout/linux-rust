//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/st/cw1200/cw1200.h
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
// Common private data for ST-Ericsson CW1200 drivers
//
// Copyright (c) 2010, ST-Ericsson
// Author: Dmitry Tarnyagin <dmitry.tarnyagin@lockless.no>
//
// Based on the mac80211 Prism54 code, which is
// Copyright (c) 2006, Michael Wu <flamingice@sourmilk.net>
//
// Based on the islsm (softmac prism54) driver, which is:
// Copyright 2004-2006 Jean-Baptiste Note <jbnote@gmail.com>, et al.
//

// Forward declarations

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cw1200_ht_info {
    pub ht_cap: ieee80211_sta_ht_cap,
    pub channel_type: nl80211_channel_type,
    pub operation_mode: u16,
}

// Please keep order
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cw1200_join_status {
    CW1200_JOIN_STATUS_PASSIVE = 0,
    CW1200_JOIN_STATUS_MONITOR,
    CW1200_JOIN_STATUS_JOINING,
    CW1200_JOIN_STATUS_PRE_STA,
    CW1200_JOIN_STATUS_STA,
    CW1200_JOIN_STATUS_IBSS,
    CW1200_JOIN_STATUS_AP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cw1200_link_status {
    CW1200_LINK_OFF,
    CW1200_LINK_RESERVE,
    CW1200_LINK_SOFT,
    CW1200_LINK_HARD,
    CW1200_LINK_RESET,
    CW1200_LINK_RESET_REMAP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cw1200_link_entry {
    pub timestamp: c_ulong,
    pub status: cw1200_link_status,
    pub prev_status: cw1200_link_status,
    pub mac: [u8; ETH_ALEN],
    pub buffered: [u8; CW1200_MAX_TID],
    pub rx_queue: sk_buff_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cw1200_common {
// interfaces to the rest of the stack
    pub hw: *mut ieee80211_hw,
    pub vif: *mut ieee80211_vif,
    pub pdev: *mut device,
// Statistics
    pub stats: ieee80211_low_level_stats,
// Our macaddr
    pub mac_addr: [u8; ETH_ALEN],
// Hardware interface
    pub hwbus_ops: *const hwbus_ops,
    pub hwbus_priv: *mut hwbus_priv,
// Hardware information
    pub hw_type: },
    pub hw_revision: },
    pub hw_refclk: c_int,
    pub hw_have_5ghz: bool,
    pub sdd: *const firmware,
    pub sdd_path: *mut c_char,
    pub debug: *mut cw1200_debug_priv,
    pub workqueue: *mut workqueue_struct,
    pub conf_mutex: mutex,
    pub tx_queue: [cw1200_queue; 4],
    pub tx_queue_stats: cw1200_queue_stats,
    pub tx_burst_idx: c_int,
// firmware/hardware info
    pub tx_hdr_len: c_uint,
// Radio data
    pub output_power: c_int,
// BBP/MAC state
    pub rates: *mut ieee80211_rate,
    pub mcs_rates: *mut ieee80211_rate,
    pub channel: *mut ieee80211_channel,
    pub edca: wsm_edca_params,
    pub tx_queue_params: wsm_tx_queue_params,
    pub association_mode: wsm_mib_association_mode,
    pub bss_params: wsm_set_bss_params,
    pub ht_info: cw1200_ht_info,
    pub powersave_mode: wsm_set_pm,
    pub firmware_ps_mode: wsm_set_pm,
    pub cqm_rssi_thold: c_int,
    pub cqm_rssi_hyst: unsigned,
    pub cqm_use_rssi: bool,
    pub cqm_beacon_loss_count: c_int,
    pub channel_switch_in_progress: c_int,
    pub channel_switch_done: wait_queue_head_t,
    pub long_frame_max_tx_count: u8,
    pub short_frame_max_tx_count: u8,
    pub mode: c_int,
    pub enable_beacon: bool,
    pub beacon_int: c_int,
    pub listening: bool,
    pub rx_filter: wsm_rx_filter,
    pub multicast_filter: wsm_mib_multicast_filter,
    pub has_multicast_subscription: bool,
    pub disable_beacon_filter: bool,
    pub update_filtering_work: work_struct,
    pub set_beacon_wakeup_period_work: work_struct,
    pub ba_rx_tid_mask: u8,
    pub ba_tx_tid_mask: u8,
    pub pm_state: cw1200_pm_state,
    pub p2p_ps_modeinfo: wsm_p2p_ps_modeinfo,
    pub uapsd_info: wsm_uapsd_info,
    pub setbssparams_done: bool,
    pub bt_present: bool,
    pub conf_listen_interval: u8,
    pub listen_interval: u32,
    pub erp_info: u32,
    pub rts_threshold: u32,
// BH
    pub bh_rx: core::sync::atomic::AtomicI32,
    pub bh_tx: core::sync::atomic::AtomicI32,
    pub bh_term: core::sync::atomic::AtomicI32,
    pub bh_suspend: core::sync::atomic::AtomicI32,
    pub bh_workqueue: *mut workqueue_struct,
    pub bh_work: work_struct,
    pub bh_error: c_int,
    pub bh_wq: wait_queue_head_t,
    pub bh_evt_wq: wait_queue_head_t,
    pub buf_id_tx: u8,
    pub buf_id_rx: u8,
    pub wsm_rx_seq: u8,
    pub wsm_tx_seq: u8,
    pub hw_bufs_used: c_int,
    pub powersave_enabled: bool,
    pub device_can_sleep: bool,
// Scan status
    pub scan: cw1200_scan,
// Keep cw1200 awake (WUP = 1) 1 second after each scan to avoid
// FW issue with sleeping/waking up.
//
    pub recent_scan: core::sync::atomic::AtomicI32,
    pub clear_recent_scan_work: delayed_work,
// WSM
    pub wsm_caps: wsm_startup_ind,
    pub wsm_cmd_mux: mutex,
    pub wsm_cmd_buf: wsm_buf,
    pub wsm_cmd: wsm_cmd,
    pub wsm_cmd_wq: wait_queue_head_t,
    pub wsm_startup_done: wait_queue_head_t,
    pub firmware_ready: c_int,
    pub tx_lock: core::sync::atomic::AtomicI32,
// WSM debug
    pub wsm_enable_wsm_dumps: c_int,
// WSM Join
    pub join_status: cw1200_join_status,
    pub pending_frame_id: u32,
    pub join_pending: bool,
    pub join_timeout: delayed_work,
    pub unjoin_work: work_struct,
    pub join_complete_work: work_struct,
    pub join_complete_status: c_int,
    pub join_dtim_period: c_int,
    pub delayed_unjoin: bool,
// TX/RX and security
    pub wep_default_key_id: i8,
    pub wep_key_work: work_struct,
    pub key_map: u32,
    pub 1]: wsm_add_key keys[WSM_KEY_MAX_INDEX +,
// AP powersave
    pub link_id_map: u32,
    pub link_id_db: [cw1200_link_entry; CW1200_MAX_STA_IN_AP_MODE],
    pub link_id_work: work_struct,
    pub link_id_gc_work: delayed_work,
    pub sta_asleep_mask: u32,
    pub pspoll_mask: u32,
    pub aid0_bit_set: bool,
    pub /: *mut *mut spinlock_t ps_state_lock; / Protect power save state,
    pub buffered_multicasts: bool,
    pub tx_multicast: bool,
    pub set_tim_work: work_struct,
    pub set_cts_work: work_struct,
    pub multicast_start_work: work_struct,
    pub multicast_stop_work: work_struct,
    pub mcast_timeout: timer_list,
// WSM events and CQM implementation
    pub /: *mut *mut spinlock_t event_queue_lock; / Protect event queue,
    pub event_queue: list_head,
    pub event_handler: work_struct,
    pub bss_loss_work: delayed_work,
    pub /: *mut *mut spinlock_t bss_loss_lock; / Protect BSS loss state,
    pub bss_loss_state: c_int,
    pub bss_loss_confirm_id: u32,
    pub delayed_link_loss: c_int,
    pub bss_params_work: work_struct,
// TX rate policy cache
    pub tx_policy_cache: tx_policy_cache,
    pub tx_policy_upload_work: work_struct,
// legacy PS mode switch in suspend
    pub ps_mode_switch_in_progress: c_int,
    pub ps_mode_switch_done: wait_queue_head_t,
// Workaround for WFD testcase 6.1.10
    pub linkid_reset_work: work_struct,
    pub action_frame_sa: [u8; ETH_ALEN],
    pub action_linkid: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cw1200_sta_priv {
    pub link_id: c_int,
}

// interfaces for the drivers
extern "C" {
    pub fn cw1200_core_release(self: *mut cw1200_common);
}

