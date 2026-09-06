//! Automatically rewritten from C to Rust
//! Source: net/wireless/of.c
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


// SPDX-License-Identifier: ISC
//
// Copyright (C) 2017 Rafał Miłecki <rafal@milecki.pl>
//

    static bool wiphy_freq_limits_valid_chan(struct wiphy *wiphy,
    struct ieee80211_freq_range *freq_limits,
    unsigned int n_freq_limits,
    struct ieee80211_channel *chan)
    {
    let mut bw: u32 = MHZ_TO_KHZ(20);
    int i;
    for (i = 0; i < n_freq_limits; i++) {
    struct ieee80211_freq_range *limit = &freq_limits[i];
    if (cfg80211_does_bw_fit_range(limit,
    MHZ_TO_KHZ(chan.center_freq),
    bw))
    return true;
    }
    return false;
    }
    static void wiphy_freq_limits_apply(struct wiphy *wiphy,
    struct ieee80211_freq_range *freq_limits,
    unsigned int n_freq_limits)
    {
    enum nl80211_band band;
    int i;
    if (WARN_ON(!n_freq_limits))
    return;
    for (band = 0; band < NUM_NL80211_BANDS; band++) {
    struct ieee80211_supported_band *sband = wiphy.bands[band];
    if (!sband)
    continue;
    for (i = 0; i < sband.n_channels; i++) {
    struct ieee80211_channel *chan = &sband.channels[i];
    if (chan.flags & IEEE80211_CHAN_DISABLED)
    continue;
    if (!wiphy_freq_limits_valid_chan(wiphy, freq_limits,
    n_freq_limits,
    chan)) {
    pr_debug("Disabling freq %d MHz as it's out of OF limits\n",
    chan.center_freq);
    chan.flags |= IEEE80211_CHAN_DISABLED;
    }
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn wiphy_read_of_freq_limits(wiphy: *mut wiphy) {
    void wiphy_read_of_freq_limits(struct wiphy *wiphy)
    {
    struct device *dev = wiphy_dev(wiphy);
    struct device_node *np;
    struct property *prop;
    struct ieee80211_freq_range *freq_limits;
    unsigned int n_freq_limits;
    const __be32 *p;
    int len, i;
    let mut err: c_int = 0;
    if (!dev)
    return;
    np = dev_of_node(dev);
    if (!np)
    return;
    prop = of_find_property(np, "ieee80211-freq-limit", &len);
    if (!prop)
    return;
    if (!len || len % sizeof(u32) || len / sizeof(u32) % 2) {
    dev_err(dev, "ieee80211-freq-limit wrong format");
    return;
    }
    n_freq_limits = len / sizeof(u32) / 2;
    freq_limits = kzalloc_objs(*freq_limits, n_freq_limits);
    if (!freq_limits) {
    err = -ENOMEM;
    goto out_kfree;
    }
    p = core::ptr::null_mut();
    for (i = 0; i < n_freq_limits; i++) {
    struct ieee80211_freq_range *limit = &freq_limits[i];
    p = of_prop_next_u32(prop, p, &limit.start_freq_khz);
    if (!p) {
    err = -EINVAL;
    goto out_kfree;
    }
    p = of_prop_next_u32(prop, p, &limit.end_freq_khz);
    if (!p) {
    err = -EINVAL;
    goto out_kfree;
    }
    if (!limit.start_freq_khz ||
    !limit.end_freq_khz ||
    limit.start_freq_khz >= limit.end_freq_khz) {
    err = -EINVAL;
    goto out_kfree;
    }
    }
    wiphy_freq_limits_apply(wiphy, freq_limits, n_freq_limits);
    out_kfree:
    kfree(freq_limits);
    if (err)
    dev_err(dev, "Failed to get limits: %d\n", err);
    }
    EXPORT_SYMBOL(wiphy_read_of_freq_limits);
