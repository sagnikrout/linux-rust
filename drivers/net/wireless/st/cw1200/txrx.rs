//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/st/cw1200/txrx.h
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
// Datapath interface for ST-Ericsson CW1200 mac80211 drivers
//
// Copyright (c) 2010, ST-Ericsson
// Author: Dmitry Tarnyagin <dmitry.tarnyagin@lockless.no>
//

// extern */ struct ieee80211_hw;
// extern */ struct sk_buff;
// extern */ struct wsm_tx;
// extern */ struct wsm_rx;
// extern */ struct wsm_tx_confirm;
// extern */ struct cw1200_txpriv;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_policy {
    pub tbl: [__le32; 3],
    pub raw: [u8; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_policy_cache_entry {
    pub policy: tx_policy,
    pub link: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_policy_cache {
    pub cache: [tx_policy_cache_entry; TX_POLICY_CACHE_SIZE],
    pub used: list_head,
    pub free: list_head,
    pub /: *mut *mut spinlock_t lock; / Protect policy cache,
}

// ********************************************************************
// TX policy cache
// Intention of TX policy cache is an overcomplicated WSM API.
// Device does not accept per-PDU tx retry sequence.
// It uses "tx retry policy id" instead, so driver code has to sync
// linux tx retry sequences with a retry policy table in the device.
//
extern "C" {
    pub fn tx_policy_init(priv: *mut cw1200_common);
}
extern "C" {
    pub fn tx_policy_upload_work(work: *mut work_struct);
}
extern "C" {
    pub fn tx_policy_clean(priv: *mut cw1200_common);
}
// ********************************************************************
// TX implementation
// ********************************************************************
// WSM callbacks
// ********************************************************************
// Timeout
extern "C" {
    pub fn cw1200_tx_timeout(work: *mut work_struct);
}
// ********************************************************************
// Security
extern "C" {
    pub fn cw1200_alloc_key(priv: *mut cw1200_common) -> c_int;
}
extern "C" {
    pub fn cw1200_free_key(priv: *mut cw1200_common, idx: c_int);
}
extern "C" {
    pub fn cw1200_free_keys(priv: *mut cw1200_common);
}
extern "C" {
    pub fn cw1200_upload_keys(priv: *mut cw1200_common) -> c_int;
}
// ********************************************************************
// Workaround for WFD test case 6.1.10
extern "C" {
    pub fn cw1200_link_id_reset(work: *mut work_struct);
}

extern "C" {
    pub fn cw1200_find_link_id(priv: *mut cw1200_common, mac: *const u8) -> c_int;
}
extern "C" {
    pub fn cw1200_alloc_link_id(priv: *mut cw1200_common, mac: *const u8) -> c_int;
}
extern "C" {
    pub fn cw1200_link_id_work(work: *mut work_struct);
}
extern "C" {
    pub fn cw1200_link_id_gc_work(work: *mut work_struct);
}
