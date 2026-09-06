//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/mac.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (c) 2018-2021 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_generic_iter {
    pub ar: *mut ath12k,
    pub ret: c_int,
}

// number of failed packets (20 packets with 16 sw reties each)

// Use insanely high numbers to make sure that the firmware implementation
// won't start, we have the same functionality already in hostapd. Unit
// is seconds.
//
pub const ATH12K_KEEPALIVE_MIN_IDLE: c_int = 3747;
pub const ATH12K_KEEPALIVE_MAX_IDLE: c_int = 3895;
pub const ATH12K_KEEPALIVE_MAX_UNRESPONSIVE: c_int = 3900;

// FIXME: should these be in ieee80211.h?

pub const ATH12K_CHAN_WIDTH_NUM: c_int = 14;

pub const ATH12K_TX_POWER_MAX_VAL: c_int = 70;
pub const ATH12K_TX_POWER_MIN_VAL: c_int = 0;
pub const ATH12K_DEFAULT_LINK_ID: c_int = 0;
pub const ATH12K_INVALID_LINK_ID: c_int = 255;
// Default link after the IEEE802.11 defined Max link id limit
// for driver usage purpose.
//

pub const ATH12K_NUM_MAX_ACTIVE_LINKS_PER_DEVICE: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_supported_bw {
    ATH12K_BW_20    = 0,
    ATH12K_BW_40    = 1,
    ATH12K_BW_80    = 2,
    ATH12K_BW_160   = 3,
    ATH12K_BW_320   = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_gi {
    ATH12K_RATE_INFO_GI_0_8,
    ATH12K_RATE_INFO_GI_1_6,
    ATH12K_RATE_INFO_GI_3_2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_ltf {
    ATH12K_RATE_INFO_1XLTF,
    ATH12K_RATE_INFO_2XLTF,
    ATH12K_RATE_INFO_4XLTF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_mac_get_any_chanctx_conf_arg {
    pub ar: *mut ath12k,
    pub chanctx_conf: *mut ieee80211_chanctx_conf,
}

//
// struct ath12k_chan_power_info - TPE containing power info per channel chunk
// @chan_cfreq: channel center freq (MHz)
// e.g.
// channel 37/20 MHz,  it is 6135
// channel 37/40 MHz,  it is 6125
// channel 37/80 MHz,  it is 6145
// channel 37/160 MHz, it is 6185
// @tx_power: transmit power (dBm)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_chan_power_info {
    pub chan_cfreq: u16,
    pub tx_power: i8,
}

// ath12k only deals with 320 MHz, so 16 subchannels
pub const ATH12K_NUM_PWR_LEVELS: c_int = 16;
//
// struct ath12k_reg_tpc_power_info - regulatory TPC power info
// @is_psd_power: is PSD power or not
// @eirp_power: Maximum EIRP power (dBm), valid only if power is PSD
// @ap_power_type: type of power (SP/LPI/VLP)
// @num_pwr_levels: number of power levels
// @reg_max: Array of maximum TX power (dBm) per PSD value
// @ap_constraint_power: AP constraint power (dBm)
// @tpe: TPE values processed from TPE IE
// @chan_power_info: power info to send to firmware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_reg_tpc_power_info {
    pub is_psd_power: bool,
    pub eirp_power: u8,
    pub ap_power_type: wmi_reg_6g_ap_type,
    pub num_pwr_levels: u8,
    pub reg_max: [u8; ATH12K_NUM_PWR_LEVELS],
    pub ap_constraint_power: u8,
    pub tpe: [i8; ATH12K_NUM_PWR_LEVELS],
    pub chan_power_info: [ath12k_chan_power_info; ATH12K_NUM_PWR_LEVELS],
}

pub const ATH12K_SCAN_11D_INTERVAL: c_int = 600000;
pub const ATH12K_11D_INVALID_VDEV_ID: c_uint = 0xFFFF;
extern "C" {
    pub fn ath12k_mac_11d_scan_start(ar: *mut ath12k, vdev_id: u32);
}
extern "C" {
    pub fn ath12k_mac_11d_scan_stop(ar: *mut ath12k);
}
extern "C" {
    pub fn ath12k_mac_11d_scan_stop_all(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_mac_destroy(ag: *mut ath12k_hw_group);
}
extern "C" {
    pub fn ath12k_mac_unregister(ag: *mut ath12k_hw_group);
}
extern "C" {
    pub fn ath12k_mac_register(ag: *mut ath12k_hw_group) -> c_int;
}
extern "C" {
    pub fn ath12k_mac_allocate(ag: *mut ath12k_hw_group) -> c_int;
}
extern "C" {
    pub fn __ath12k_mac_scan_finish(ar: *mut ath12k);
}
extern "C" {
    pub fn ath12k_mac_scan_finish(ar: *mut ath12k);
}
extern "C" {
    pub fn ath12k_mac_drain_tx(ar: *mut ath12k);
}
extern "C" {
    pub fn ath12k_mac_peer_cleanup_all(ar: *mut ath12k);
}
extern "C" {
    pub fn ath12k_mac_dp_peer_cleanup(ah: *mut ath12k_hw);
}
extern "C" {
    pub fn ath12k_mac_tx_mgmt_pending_free(buf_id: c_int, skb: *mut c_void, ctx: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ath12k_mac_bw_to_mac80211_bw(bw: ath12k_supported_bw) -> rate_info_bw;
}
extern "C" {
    pub fn ath12k_mac_mac80211_bw_to_ath12k_bw(bw: rate_info_bw) -> ath12k_supported_bw;
}
extern "C" {
    pub fn ath12k_dp_tx_get_encrypt_type(cipher: u32) -> hal_encrypt_type;
}
extern "C" {
    pub fn ath12k_mac_rfkill_enable_radio(ar: *mut ath12k, enable: bool) -> c_int;
}
extern "C" {
    pub fn ath12k_mac_rfkill_config(ar: *mut ath12k) -> c_int;
}
extern "C" {
    pub fn ath12k_mac_wait_tx_complete(ar: *mut ath12k) -> c_int;
}
extern "C" {
    pub fn ath12k_mac_handle_beacon(ar: *mut ath12k, skb: *mut sk_buff);
}
extern "C" {
    pub fn ath12k_mac_get_target_pdev_id(ar: *mut ath12k) -> u8;
}
extern "C" {
    pub fn ath12k_mac_mlo_setup(ag: *mut ath12k_hw_group) -> c_int;
}
extern "C" {
    pub fn ath12k_mac_mlo_ready(ag: *mut ath12k_hw_group) -> c_int;
}
extern "C" {
    pub fn ath12k_mac_mlo_teardown(ag: *mut ath12k_hw_group);
}
extern "C" {
    pub fn ath12k_mac_vdev_stop(arvif: *mut ath12k_link_vif) -> c_int;
}
extern "C" {
    pub fn ath12k_mac_he_convert_tones_to_ru_tones(tones: u16) -> u16;
}
extern "C" {
    pub fn ath12k_mac_eht_ru_tones_to_nl80211_eht_ru_alloc(ru_tones: u16) -> nl80211_eht_ru_alloc;
}
extern "C" {
    pub fn ath12k_mac_eht_gi_to_nl80211_eht_gi(sgi: u8) -> nl80211_eht_gi;
}
extern "C" {
    pub fn ath12k_mac_get_fw_stats(ar: *mut ath12k, param: *mut ath12k_fw_stats_req_params) -> c_int;
}
extern "C" {
    pub fn ath12k_mac_op_start(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn ath12k_mac_op_stop(hw: *mut ieee80211_hw, suspend: bool);
}
extern "C" {
    pub fn ath12k_mac_op_config(hw: *mut ieee80211_hw, radio_idx: c_int, changed: u32) -> c_int;
}
