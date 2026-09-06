//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/silabs/wfx/wfx.h
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
// Common private data.
//
// Copyright (c) 2017-2020, Silicon Laboratories, Inc.
// Copyright (c) 2010, ST-Ericsson
// Copyright (c) 2006, Michael Wu <flamingice@sourmilk.net>
// Copyright 2004-2006 Jean-Baptiste Note <jbnote@gmail.com>, et al.
//

pub const USEC_PER_TU: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_dev {
    pub pdata: wfx_platform_data,
    pub dev: *mut device,
    pub hw: *mut ieee80211_hw,
    pub vif: [*mut ieee80211_vif; 2],
    pub addresses: [mac_address; 2],
    pub hwbus_ops: *const wfx_hwbus_ops,
    pub hwbus_priv: *mut c_void,
    pub keyset: u8,
    pub firmware_ready: completion,
    pub hw_caps: wfx_hif_ind_startup,
    pub hif: wfx_hif,
    pub cooling_timeout_work: delayed_work,
    pub poll_irq: bool,
    pub chip_frozen: bool,
    pub scan_lock: mutex,
    pub conf_mutex: mutex,
    pub hif_cmd: wfx_hif_cmd,
    pub tx_pending: sk_buff_head,
    pub tx_dequeue: wait_queue_head_t,
    pub tx_lock: core::sync::atomic::AtomicI32,
    pub packet_id: core::sync::atomic::AtomicI32,
    pub key_map: u32,
    pub rx_stats: wfx_hif_rx_stats,
    pub rx_stats_lock: mutex,
    pub tx_power_loop_info: wfx_hif_tx_power_loop_info,
    pub tx_power_loop_info_lock: mutex,
    pub bh_wq: *mut workqueue_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_vif {
    pub wdev: *mut wfx_dev,
    pub channel: *mut ieee80211_channel,
    pub id: c_int,
    pub link_id_map: u32,
    pub after_dtim_tx_allowed: bool,
    pub join_in_progress: bool,
    pub set_pm_mode_complete: completion,
    pub beacon_loss_work: delayed_work,
    pub tx_queue: [wfx_queue; 4],
    pub tx_policy_cache: wfx_tx_policy_cache,
    pub tx_policy_upload_work: work_struct,
    pub update_tim_work: work_struct,
    pub uapsd_mask: c_ulong,
    pub scan_work: work_struct,
    pub scan_complete: completion,
    pub scan_nb_chan_done: c_int,
    pub scan_abort: bool,
    pub scan_req: *mut ieee80211_scan_request,
    pub remain_on_channel_chan: *mut ieee80211_channel,
    pub remain_on_channel_duration: c_int,
    pub remain_on_channel_work: work_struct,
}

extern "C" {
    pub fn container_of()wvif: *mut (void, ieee80211_vif: struct, _arg: drv_priv) -> return;
}
// lo++ = *hi;
// hi-- = swap;
extern "C" {
    pub fn memcmp(_arg: buf, 1: buf +, 1: size -) -> return;
}
