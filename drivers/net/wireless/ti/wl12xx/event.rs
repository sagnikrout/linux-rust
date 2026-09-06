//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl12xx/event.h
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
// This file is part of wl12xx
//
// Copyright (C) 2012 Texas Instruments. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_event_mailbox {
    pub events_vector: __le32,
    pub events_mask: __le32,
    pub reserved_1: __le32,
    pub reserved_2: __le32,
    pub number_of_scan_results: u8,
    pub scan_tag: u8,
    pub completed_scan_status: u8,
    pub reserved_3: u8,
    pub soft_gemini_sense_info: u8,
    pub soft_gemini_protective_info: u8,
    pub rssi_snr_trigger_metric: [i8; NUM_OF_RSSI_SNR_TRIGGERS],
    pub change_auto_mode_timeout: u8,
    pub scheduled_scan_status: u8,
    pub reserved4: u8,
// tuned channel (roc)
    pub roc_channel: u8,
    pub hlid_removed_bitmap: __le16,
// bitmap of aged stations (by HLID)
    pub sta_aging_status: __le16,
// bitmap of stations (by HLID) which exceeded max tx retries
    pub sta_tx_retry_exceeded: __le16,
// discovery completed results
    pub discovery_tag: u8,
    pub number_of_preq_results: u8,
    pub number_of_prsp_results: u8,
    pub reserved_5: u8,
// rx ba constraint
    pub /: *mut *mut u8 role_id; / 0xFF means any role.,
    pub rx_ba_allowed: u8,
    pub reserved_6: [u8; 2],
// Channel switch results
    pub channel_switch_role_id: u8,
    pub channel_switch_status: u8,
    pub reserved_7: [u8; 2],
    pub ps_poll_delivery_failure_role_ids: u8,
    pub stopped_role_ids: u8,
    pub started_role_ids: u8,
    pub reserved_8: [u8; 9],
    pub __packed: },
    pub timeout): *mut bool,
    pub wl): *mut int wl12xx_process_mailbox_events(struct wl1271,
