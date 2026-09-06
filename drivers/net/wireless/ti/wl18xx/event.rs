//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl18xx/event.h
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
// This file is part of wl18xx
//
// Copyright (C) 2012 Texas Instruments. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl18xx_radar_types {
    RADAR_TYPE_NONE,
    RADAR_TYPE_REGULAR,
    RADAR_TYPE_CHIRP
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_event_mailbox {
    pub events_vector: __le32,
    pub number_of_scan_results: u8,
    pub number_of_sched_scan_results: u8,
    pub channel_switch_role_id_bitmap: __le16,
    pub rssi_snr_trigger_metric: [i8; NUM_OF_RSSI_SNR_TRIGGERS],
// bitmap of removed links
    pub hlid_removed_bitmap: __le32,
// rx ba constraint
    pub /: *mut *mut __le16 rx_ba_role_id_bitmap; / 0xfff means any role.,
    pub rx_ba_allowed_bitmap: __le16,
// bitmap of roc completed (by role id)
    pub roc_completed_bitmap: __le16,
// bitmap of stations (by role id) with bss loss
    pub bss_loss_bitmap: __le16,
// bitmap of stations (by HLID) which exceeded max tx retries
    pub tx_retry_exceeded_bitmap: __le16,
// time sync high msb
    pub time_sync_tsf_high_msb: __le16,
// bitmap of inactive stations (by HLID)
    pub inactive_sta_bitmap: __le16,
// time sync high lsb
    pub time_sync_tsf_high_lsb: __le16,
// rx BA win size indicated by RX_BA_WIN_SIZE_CHANGE_EVENT_ID
    pub rx_ba_role_id: u8,
    pub rx_ba_link_id: u8,
    pub rx_ba_win_size: u8,
    pub padding: u8,
// smart config
    pub sc_ssid_len: u8,
    pub sc_pwd_len: u8,
    pub sc_token_len: u8,
    pub padding1: u8,
    pub sc_ssid: [u8; 32],
    pub sc_pwd: [u8; 64],
    pub sc_token: [u8; 32],
// smart config sync channel
    pub sc_sync_channel: u8,
    pub sc_sync_band: u8,
// time sync low msb
    pub time_sync_tsf_low_msb: __le16,
// radar detect
    pub radar_channel: u8,
    pub radar_type: u8,
// time sync low lsb
    pub time_sync_tsf_low_lsb: __le16,
    pub __packed: },
    pub timeout): *mut bool,
    pub wl): *mut int wl18xx_process_mailbox_events(struct wl1271,
