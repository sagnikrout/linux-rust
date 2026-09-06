//! Automatically rewritten from C to Rust
//! Source: net/mac80211/uhr.c
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
// UHR handling
//
// Copyright(c) 2025-2026 Intel Corporation
//

    void
    ieee80211_uhr_cap_ie_to_sta_uhr_cap(struct ieee80211_sub_if_data *sdata,
    struct ieee80211_supported_band *sband,
    const struct ieee80211_uhr_cap *uhr_cap,
    u8 uhr_cap_len,
    struct link_sta_info *link_sta)
    {
    struct ieee80211_sta_uhr_cap *sta_uhr_cap = &link_sta.pub.uhr_cap;
    memset(sta_uhr_cap, 0, sizeof(*sta_uhr_cap));
    if (!ieee80211_get_uhr_iftype_cap_vif(sband, &sdata.vif))
    return;
    sta_uhr_cap.has_uhr = true;
    sta_uhr_cap.mac = uhr_cap.mac;
    sta_uhr_cap.phy = uhr_cap.phy;
    }
