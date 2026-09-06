//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/offload.h
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
// Copyright (C) 2012-2014 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016-2017 Intel Deutschland GmbH
// Copyright (C) 2021-2025 Intel Corporation
//

// Macro flag: #define __iwl_fw_api_offload_h__
//
// enum iwl_prot_offload_subcmd_ids - protocol offload commands
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_prot_offload_subcmd_ids {
//
// @WOWLAN_WAKE_PKT_NOTIFICATION: Notification in &struct iwl_wowlan_wake_pkt_notif
//
    WOWLAN_WAKE_PKT_NOTIFICATION = 0xFC,

//
// @WOWLAN_INFO_NOTIFICATION: Notification in
// &struct iwl_wowlan_info_notif_v1, iwl_wowlan_info_notif_v3,
// &struct iwl_wowlan_info_notif_v5 or &struct iwl_wowlan_info_notif
//
    WOWLAN_INFO_NOTIFICATION = 0xFD,

//
// @D3_END_NOTIFICATION: End D3 state notification
//
    D3_END_NOTIFICATION = 0xFE,

//
// @STORED_BEACON_NTF: &struct iwl_stored_beacon_notif_v2 or
// &struct iwl_stored_beacon_notif
//
    STORED_BEACON_NTF = 0xFF,
}

pub const MAX_STORED_BEACON_SIZE: c_int = 600;
//
// struct iwl_stored_beacon_notif_common - Stored beacon notif common fields
//
// @system_time: system time on air rise
// @tsf: TSF on air rise
// @beacon_timestamp: beacon on air rise
// @band: band, matches &RX_RES_PHY_FLAGS_BAND_24 definition
// @channel: channel this beacon was received on
// @rates: rate in ucode internal format
// @byte_count: frame's byte count
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_stored_beacon_notif_common {
    pub system_time: __le32,
    pub tsf: __le64,
    pub beacon_timestamp: __le32,
    pub band: __le16,
    pub channel: __le16,
    pub rates: __le32,
    pub byte_count: __le32,
    pub __packed: },
//
// struct iwl_stored_beacon_notif_v2 - Stored beacon notification
//
// @common: fields common for all versions
// @data: beacon data, length in @byte_count
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_stored_beacon_notif_v2 {
    pub common: iwl_stored_beacon_notif_common,
    pub data: [u8; MAX_STORED_BEACON_SIZE],
    pub /: *mut *mut } __packed; / WOWLAN_STROED_BEACON_INFO_S_VER_2,
//
// struct iwl_stored_beacon_notif - Stored beacon notification
//
// @common: fields common for all versions
// @sta_id: station for which the beacon was received
// @reserved: reserved for alignment
// @data: beacon data, length in @byte_count
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_stored_beacon_notif {
    pub common: iwl_stored_beacon_notif_common,
    pub sta_id: u8,
    pub reserved: [u8; 3],
    pub data: [u8; MAX_STORED_BEACON_SIZE],
    pub /: *mut *mut } __packed; / WOWLAN_STROED_BEACON_INFO_S_VER_3, _VER_4,
