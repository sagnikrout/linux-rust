//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/silabs/wfx/data_tx.h
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
// Data transmitting implementation.
//
// Copyright (c) 2017-2020, Silicon Laboratories, Inc.
// Copyright (c) 2010, ST-Ericsson
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_tx_policy {
    pub link: list_head,
    pub usage_count: c_int,
    pub rates: [u8; 12],
    pub uploaded: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_tx_policy_cache {
    pub cache: [wfx_tx_policy; HIF_TX_RETRY_POLICY_MAX],
// FIXME: use a trees and drop hash from tx_policy
    pub used: list_head,
    pub free: list_head,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_tx_priv {
    pub xmit_timestamp: ktime_t,
    pub icv_size: c_uchar,
    pub vif_id: c_uchar,
}

extern "C" {
    pub fn wfx_tx_policy_init(wvif: *mut wfx_vif);
}
extern "C" {
    pub fn wfx_tx_policy_upload_work(work: *mut work_struct);
}
extern "C" {
    pub fn wfx_tx(hw: *mut ieee80211_hw, control: *mut ieee80211_tx_control, skb: *mut sk_buff);
}
extern "C" {
    pub fn wfx_tx_confirm_cb(wdev: *mut wfx_dev, arg: *const wfx_hif_cnf_tx);
}
extern "C" {
    pub fn wfx_flush(hw: *mut ieee80211_hw, vif: *mut ieee80211_vif, queues: u32, drop: bool);
}
