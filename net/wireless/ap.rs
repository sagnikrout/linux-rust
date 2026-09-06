//! Automatically rewritten from C to Rust
//! Source: net/wireless/ap.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Parts of this file are
// Copyright (C) 2022-2023 Intel Corporation
//

    static int ___cfg80211_stop_ap(struct cfg80211_registered_device *rdev,
    struct net_device *dev, unsigned int link_id,
    bool notify)
    {
    struct wireless_dev *wdev = dev.ieee80211_ptr;
    int err;
    lockdep_assert_wiphy(wdev.wiphy);
    if (!rdev.ops.stop_ap)
    return -EOPNOTSUPP;
    if (dev.ieee80211_ptr.iftype != NL80211_IFTYPE_AP &&
    dev.ieee80211_ptr.iftype != NL80211_IFTYPE_P2P_GO)
    return -EOPNOTSUPP;
    if (!wdev.links[link_id].ap.beacon_interval)
    return -ENOENT;
    err = rdev_stop_ap(rdev, dev, link_id);
    if (!err) {
    wdev.conn_owner_nlportid = 0;
    wdev.links[link_id].ap.beacon_interval = 0;
    memset(&wdev.links[link_id].ap.chandef, 0,
    sizeof(wdev.links[link_id].ap.chandef));
    wdev.u.ap.ssid_len = 0;
    rdev_set_qos_map(rdev, dev, core::ptr::null_mut());
    if (notify)
    nl80211_send_ap_stopped(wdev, link_id);
// Should we apply the grace period during beaconing interface
// shutdown also?
//
    cfg80211_sched_dfs_chan_update(rdev);
    }
    schedule_work(&cfg80211_disconnect_work);
    return err;
    }
    int cfg80211_stop_ap(struct cfg80211_registered_device *rdev,
    struct net_device *dev, int link_id,
    bool notify)
    {
    unsigned int link;
    let mut ret: c_int = 0;
    if (link_id >= 0)
    return ___cfg80211_stop_ap(rdev, dev, link_id, notify);
    for_each_valid_link(dev.ieee80211_ptr, link) {
    let mut ret1: c_int = ___cfg80211_stop_ap(rdev, dev, link, notify);
    if (ret1)
    ret = ret1;
// try the next one also if one errored
    }
    return ret;
    }
