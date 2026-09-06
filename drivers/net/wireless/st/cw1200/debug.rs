//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/st/cw1200/debug.h
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
// DebugFS code for ST-Ericsson CW1200 mac80211 driver
//
// Copyright (c) 2011, ST-Ericsson
// Author: Dmitry Tarnyagin <dmitry.tarnyagin@lockless.no>
//

// Macro flag: #define CW1200_DEBUG_H_INCLUDED
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cw1200_debug_priv {
    pub debugfs_phy: *mut dentry,
    pub tx: c_int,
    pub tx_agg: c_int,
    pub rx: c_int,
    pub rx_agg: c_int,
    pub tx_multi: c_int,
    pub tx_multi_frames: c_int,
    pub tx_cache_miss: c_int,
    pub tx_align: c_int,
    pub tx_ttl: c_int,
    pub tx_burst: c_int,
    pub ba_cnt: c_int,
    pub ba_acc: c_int,
    pub ba_cnt_rx: c_int,
    pub ba_acc_rx: c_int,
}

extern "C" {
    pub fn cw1200_debug_init(priv: *mut cw1200_common) -> c_int;
}
extern "C" {
    pub fn cw1200_debug_release(priv: *mut cw1200_common);
}
