//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/dvm/agn.h
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
// Copyright (C) 2005-2014, 2021, 2024-2025 Intel Corporation
//

// Macro flag: #define __iwl_agn_h__

// The first 11 queues (0-10) are used otherwise
pub const IWLAGN_FIRST_AMPDU_QUEUE: c_int = 11;
// AUX (TX during scan dwell) queue
pub const IWL_AUX_QUEUE: c_int = 10;
pub const IWL_INVALID_STATION: c_int = 255;
// device operations
pub const TIME_UNIT: c_int = 1024;
//
// DRIVER STATUS FUNCTIONS
//
pub const STATUS_RF_KILL_HW: c_int = 0;
pub const STATUS_CT_KILL: c_int = 1;
pub const STATUS_ALIVE: c_int = 2;
pub const STATUS_READY: c_int = 3;
pub const STATUS_EXIT_PENDING: c_int = 5;
pub const STATUS_STATISTICS: c_int = 6;
pub const STATUS_SCANNING: c_int = 7;
pub const STATUS_SCAN_ABORTING: c_int = 8;
pub const STATUS_SCAN_HW: c_int = 9;
pub const STATUS_FW_ERROR: c_int = 10;
pub const STATUS_CHANNEL_SWITCH_PENDING: c_int = 11;
pub const STATUS_SCAN_COMPLETE: c_int = 12;
pub const STATUS_POWER_PMI: c_int = 13;
extern "C" {
    pub fn iwl_down(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwl_cancel_deferred_work(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwlagn_prepare_restart(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwl_check_for_ct_kill(priv: *mut iwl_priv) -> bool;
}
extern "C" {
    pub fn iwlagn_lift_passive_no_rx(priv: *mut iwl_priv);
}
// MAC80211
extern "C" {
    pub fn iwlagn_mac_unregister(priv: *mut iwl_priv);
}
// commands
extern "C" {
    pub fn iwl_dvm_send_cmd(priv: *mut iwl_priv, cmd: *mut iwl_host_cmd) -> c_int;
}
// RXON
extern "C" {
    pub fn iwlagn_set_pan_params(priv: *mut iwl_priv) -> c_int;
}
extern "C" {
    pub fn iwlagn_commit_rxon(priv: *mut iwl_priv, ctx: *mut iwl_rxon_context) -> c_int;
}
extern "C" {
    pub fn iwlagn_set_rxon_chain(priv: *mut iwl_priv, ctx: *mut iwl_rxon_context);
}
extern "C" {
    pub fn iwlagn_mac_config(hw: *mut ieee80211_hw, radio_idx: c_int, changed: u32) -> c_int;
}
extern "C" {
    pub fn iwl_set_rxon_ht(priv: *mut iwl_priv, ht_conf: *mut iwl_ht_config);
}
// uCode
extern "C" {
    pub fn iwl_send_bt_env(priv: *mut iwl_priv, action: u8, type: u8) -> c_int;
}
extern "C" {
    pub fn iwl_send_prio_tbl(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwl_init_alive_start(priv: *mut iwl_priv) -> c_int;
}
extern "C" {
    pub fn iwl_run_init_ucode(priv: *mut iwl_priv) -> c_int;
}
extern "C" {
    pub fn iwl_send_calib_results(priv: *mut iwl_priv) -> c_int;
}
extern "C" {
    pub fn iwl_calib_free_results(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwlagn_hw_valid_rtc_data_addr(addr: u32) -> c_int;
}
// lib
extern "C" {
    pub fn iwlagn_send_tx_power(priv: *mut iwl_priv) -> c_int;
}
extern "C" {
    pub fn iwlagn_temperature(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwlagn_txfifo_flush(priv: *mut iwl_priv, scd_q_msk: u32) -> c_int;
}
extern "C" {
    pub fn iwlagn_dev_txfifo_flush(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwlagn_send_beacon_cmd(priv: *mut iwl_priv) -> c_int;
}

extern "C" {
    pub fn iwlagn_suspend(priv: *mut iwl_priv, wowlan: *mut cfg80211_wowlan) -> c_int;
}

// rx
extern "C" {
    pub fn iwlagn_hwrate_to_mac80211_idx(rate_n_flags: u32, band: nl80211_band) -> c_int;
}
extern "C" {
    pub fn iwl_setup_rx_handlers(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwl_chswitch_done(priv: *mut iwl_priv, is_success: bool);
}
// tx
extern "C" {
    pub fn iwlagn_rx_reply_tx(priv: *mut iwl_priv, rxb: *mut iwl_rx_cmd_buffer);
}
extern "C" {
    pub fn iwl_toggle_tx_ant(priv: *mut iwl_priv, ant_idx: u8, valid: u8) -> u8;
}
// scan
extern "C" {
    pub fn iwlagn_post_scan(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwl_force_rf_reset(priv: *mut iwl_priv, external: bool) -> c_int;
}
extern "C" {
    pub fn iwl_init_scan_params(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwl_scan_cancel(priv: *mut iwl_priv) -> c_int;
}
extern "C" {
    pub fn iwl_scan_cancel_timeout(priv: *mut iwl_priv, ms: c_ulong);
}
extern "C" {
    pub fn iwl_force_scan_end(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwl_internal_short_hw_scan(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwl_setup_rx_scan_handlers(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwl_setup_scan_deferred_work(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwl_cancel_scan_deferred_work(priv: *mut iwl_priv);
}
// For faster active scanning, scan will move to the next channel if fewer than
// PLCP_QUIET_THRESH packets are heard on this channel within
// ACTIVE_QUIET_TIME after sending probe request.  This shortens the dwell
// time if it's a quiet channel (nothing responded to our probe, and there's
// no other traffic).
// Disable "quiet" feature by setting PLCP_QUIET_THRESH to 0.

// bt coex
extern "C" {
    pub fn iwlagn_send_advance_bt_config(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwlagn_bt_rx_handler_setup(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwlagn_bt_setup_deferred_work(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwlagn_bt_cancel_deferred_work(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwlagn_bt_coex_rssi_monitor(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwlagn_bt_adjust_rssi_monitor(priv: *mut iwl_priv, rssi_ena: bool);
}

// station management

extern "C" {
    pub fn iwl_restore_stations(priv: *mut iwl_priv, ctx: *mut iwl_rxon_context);
}
extern "C" {
    pub fn iwl_dealloc_bcast_stations(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwl_get_free_ucode_key_offset(priv: *mut iwl_priv) -> c_int;
}
extern "C" {
    pub fn iwl_add_sta_callback(priv: *mut iwl_priv, rxb: *mut iwl_rx_cmd_buffer);
}
extern "C" {
    pub fn iwl_sta_tx_modify_enable_tid(priv: *mut iwl_priv, sta_id: c_int, tid: c_int) -> c_int;
}
extern "C" {
    pub fn iwl_sta_modify_sleep_tx_count(priv: *mut iwl_priv, sta_id: c_int, cnt: c_int);
}
extern "C" {
    pub fn iwl_update_bcast_stations(priv: *mut iwl_priv) -> c_int;
}
// rate
extern "C" {
    pub fn cpu_to_le32(_arg: flags|(u32)rate) -> return;
}
extern "C" {
    pub fn iwl_alive_start(priv: *mut iwl_priv) -> c_int;
}

// status checks
// The adapter is 'ready' if READY EXIT_PENDING is not set
extern "C" {
    pub fn test_bit(_arg: STATUS_ALIVE, _arg: &priv->status) -> return;
}
extern "C" {
    pub fn test_bit(_arg: STATUS_RF_KILL_HW, _arg: &priv->status) -> return;
}
extern "C" {
    pub fn test_bit(_arg: STATUS_CT_KILL, _arg: &priv->status) -> return;
}
extern "C" {
    pub fn iwl_is_ready(_arg: priv) -> return;
}
//
// iwl_parse_eeprom_data - parse EEPROM data and return values
//
// @trans: transport we're parsing for, for debug only
// @cfg: device configuration for parsing and overrides
// @eeprom: the EEPROM data
// @eeprom_size: length of the EEPROM data
//
// This function parses all EEPROM values we need and then
// returns a (newly allocated) struct containing all the
// relevant values for driver use. The struct must be freed
// later with iwl_free_nvm_data().
//
// Return: the parsed NVM data
//
extern "C" {
    pub fn iwl_read_eeprom(trans: *mut iwl_trans, eeprom: *mut u8, eeprom_size: *mut usize) -> c_int;
}

extern "C" {
    pub fn iwl_dbgfs_register(priv: *mut iwl_priv, dbgfs_dir: *mut dentry);
}

