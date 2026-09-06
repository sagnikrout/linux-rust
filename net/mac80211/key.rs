//! Automatically rewritten from C Header to Rust Module
//! Source: net/mac80211/key.h
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
// Copyright 2002-2004, Instant802 Networks, Inc.
// Copyright 2005, Devicescape Software, Inc.
// Copyright (C) 2019, 2022-2023 Intel Corporation
//

pub const NUM_DEFAULT_KEYS: c_int = 4;
pub const NUM_DEFAULT_MGMT_KEYS: c_int = 2;
pub const NUM_DEFAULT_BEACON_KEYS: c_int = 2;

//
// enum ieee80211_internal_key_flags - internal key flags
//
// @KEY_FLAG_UPLOADED_TO_HARDWARE: Indicates that this key is present
// in the hardware for TX crypto hardware acceleration.
// @KEY_FLAG_TAINTED: Key is tainted and packets should be dropped.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_internal_key_flags {
    KEY_FLAG_UPLOADED_TO_HARDWARE	= BIT(0),
    KEY_FLAG_TAINTED		= BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_internal_tkip_state {
    TKIP_STATE_NOT_INIT,
    TKIP_STATE_PHASE1_DONE,
    TKIP_STATE_PHASE1_HW_UPLOADED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tkip_ctx {
    pub /: *mut *mut u16 p1k[5]; / p1k cache,
    pub /: *mut *mut u32 p1k_iv32; / iv32 for which p1k computed,
    pub state: ieee80211_internal_tkip_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tkip_ctx_rx {
    pub ctx: tkip_ctx,
    pub /: *mut *mut u32 iv32; / current iv32,
    pub /: *mut *mut u16 iv16; / current iv16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_key {
    pub local: *mut ieee80211_local,
    pub sdata: *mut ieee80211_sub_if_data,
    pub sta: *mut sta_info,
// for sdata list
    pub list: list_head,
// protected by key mutex
    pub flags: c_uint,
// protects tx context
    pub txlock: spinlock_t,
// last used TSC
    pub tx: tkip_ctx,
// last received RSC
    pub rx: [tkip_ctx_rx; IEEE80211_NUM_TIDS],
// number of mic failures
    pub mic_failures: u32,
    pub tkip: },
//
// Last received packet number. The first
// IEEE80211_NUM_TIDS counters are used with Data
// frames and the last counter is used with Robust
// Management frames.
//
    pub 1][IEEE80211_CCMP_PN_LEN]: u8 rx_pn[IEEE80211_NUM_TIDS +,
    pub tfm: *mut crypto_aead,
    pub /: *mut *mut u32 replays; / dot11RSNAStatsCCMPReplays,
    pub ccmp: },
    pub rx_pn: [u8; IEEE80211_CMAC_PN_LEN],
    pub key: aes_cmac_key,
    pub /: *mut *mut u32 replays; / dot11RSNAStatsCMACReplays,
    pub /: *mut *mut u32 icverrors; / dot11RSNAStatsCMACICVErrors,
    pub aes_cmac: },
    pub rx_pn: [u8; IEEE80211_GMAC_PN_LEN],
    pub tfm: *mut crypto_aead,
    pub /: *mut *mut u32 replays; / dot11RSNAStatsCMACReplays,
    pub /: *mut *mut u32 icverrors; / dot11RSNAStatsCMACICVErrors,
    pub aes_gmac: },
// Last received packet number. The first
// IEEE80211_NUM_TIDS counters are used with Data
// frames and the last counter is used with Robust
// Management frames.
//
    pub 1][IEEE80211_GCMP_PN_LEN]: u8 rx_pn[IEEE80211_NUM_TIDS +,
    pub tfm: *mut crypto_aead,
    pub /: *mut *mut u32 replays; / dot11RSNAStatsGCMPReplays,
    pub gcmp: },
// generic cipher scheme
    pub 1][IEEE80211_MAX_PN_LEN]: u8 rx_pn[IEEE80211_NUM_TIDS +,
    pub gen: },
    pub u: },

    pub stalink: *mut dentry,
    pub dir: *mut dentry,
    pub cnt: c_int,
    pub debugfs: },

    pub color: c_uint,
//
// key config, must be last because it contains key
// material as variable length member
//
    pub conf: ieee80211_key_conf,
}

//
// Insert a key into data structures (sdata, sta if necessary)
// to make it used, free old key. On failure, also free the new key.
//
extern "C" {
    pub fn ieee80211_set_tx_key(key: *mut ieee80211_key) -> c_int;
}
extern "C" {
    pub fn ieee80211_key_free(key: *mut ieee80211_key, delay_tailroom: bool);
}
extern "C" {
    pub fn ieee80211_key_free_unused(key: *mut ieee80211_key);
}
extern "C" {
    pub fn ieee80211_reenable_keys(sdata: *mut ieee80211_sub_if_data);
}
