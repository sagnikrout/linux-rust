//! Automatically rewritten from C to Rust
//! Source: net/wireless/ibss.c
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
// Some IBSS support code for cfg80211.
//
// Copyright 2009	Johannes Berg <johannes@sipsolutions.net>
// Copyright (C) 2020-2026 Intel Corporation
//

    void __cfg80211_ibss_joined(struct net_device *dev, const u8 *bssid,
    struct ieee80211_channel *channel)
    {
    struct wireless_dev *wdev = dev.ieee80211_ptr;
    struct cfg80211_bss *bss;

    union iwreq_data wrqu;

    if (WARN_ON(wdev.iftype != NL80211_IFTYPE_ADHOC))
    return;
    if (!wdev.u.ibss.ssid_len)
    return;
    bss = cfg80211_get_bss(wdev.wiphy, channel, bssid, core::ptr::null_mut(), 0,
    IEEE80211_BSS_TYPE_IBSS, IEEE80211_PRIVACY_ANY);
    if (WARN_ON(!bss))
    return;
    if (wdev.u.ibss.current_bss) {
    cfg80211_unhold_bss(wdev.u.ibss.current_bss);
    cfg80211_put_bss(wdev.wiphy, &wdev.u.ibss.current_bss.pub);
    }
    cfg80211_hold_bss(bss_from_pub(bss));
    wdev.u.ibss.current_bss = bss_from_pub(bss);
    cfg80211_upload_connect_keys(wdev);
    nl80211_send_ibss_bssid(wiphy_to_rdev(wdev.wiphy), dev, bssid,
    GFP_KERNEL);

    memset(&wrqu, 0, sizeof(wrqu));
    memcpy(wrqu.ap_addr.sa_data, bssid, ETH_ALEN);
    wireless_send_event(dev, SIOCGIWAP, &wrqu, core::ptr::null_mut());

    }
    void cfg80211_ibss_joined(struct net_device *dev, const u8 *bssid,
    struct ieee80211_channel *channel, gfp_t gfp)
    {
    struct wireless_dev *wdev = dev.ieee80211_ptr;
    struct cfg80211_registered_device *rdev = wiphy_to_rdev(wdev.wiphy);
    struct cfg80211_event *ev;
    unsigned long flags;
    trace_cfg80211_ibss_joined(dev, bssid, channel);
    if (WARN_ON(!channel))
    return;
    ev = kzalloc_obj(*ev, gfp);
    if (!ev)
    return;
    ev.type = EVENT_IBSS_JOINED;
    memcpy(ev.ij.bssid, bssid, ETH_ALEN);
    ev.ij.channel = channel;
    spin_lock_irqsave(&wdev.event_lock, flags);
    list_add_tail(&ev.list, &wdev.event_list);
    spin_unlock_irqrestore(&wdev.event_lock, flags);
    queue_work(cfg80211_wq, &rdev.event_work);
    }
    EXPORT_SYMBOL(cfg80211_ibss_joined);
    int __cfg80211_join_ibss(struct cfg80211_registered_device *rdev,
    struct net_device *dev,
    struct cfg80211_ibss_params *params,
    struct cfg80211_cached_keys *connkeys)
    {
    struct wireless_dev *wdev = dev.ieee80211_ptr;
    int err;
    lockdep_assert_held(&rdev.wiphy.mtx);
    if (wdev.links[0].cac_started)
    return -EBUSY;
    if (wdev.u.ibss.ssid_len)
    return -EALREADY;
    if (!params.basic_rates) {
//
// If no rates were explicitly configured,
// use the mandatory rate set for 11b or
// 11a for maximum compatibility.
//
    struct ieee80211_supported_band *sband;
    enum nl80211_band band;
    u32 flag;
    int j;
    band = params.chandef.chan.band;
    if (band == NL80211_BAND_5GHZ ||
    band == NL80211_BAND_6GHZ)
    flag = IEEE80211_RATE_MANDATORY_A;
    else
    flag = IEEE80211_RATE_MANDATORY_B;
    sband = rdev.wiphy.bands[band];
    for (j = 0; j < sband.n_bitrates; j++) {
    if (sband.bitrates[j].flags & flag)
    params.basic_rates |= BIT(j);
    }
    }
    if (WARN_ON(connkeys && connkeys.def < 0))
    return -EINVAL;
    if (WARN_ON(wdev.connect_keys))
    kfree_sensitive(wdev.connect_keys);
    wdev.connect_keys = connkeys;
    wdev.u.ibss.chandef = params.chandef;
    if (connkeys) {
    params.wep_keys = connkeys.params;
    params.wep_tx_key = connkeys.def;
    }

    wdev.wext.ibss.chandef = params.chandef;

    err = rdev_join_ibss(rdev, dev, params);
    if (err) {
    wdev.connect_keys = core::ptr::null_mut();
    return err;
    }
    memcpy(wdev.u.ibss.ssid, params.ssid, params.ssid_len);
    wdev.u.ibss.ssid_len = params.ssid_len;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cfg80211_clear_ibss(dev: *mut net_device, nowext: bool) {
    void cfg80211_clear_ibss(struct net_device *dev, bool nowext)
    {
    struct wireless_dev *wdev = dev.ieee80211_ptr;
    struct cfg80211_registered_device *rdev = wiphy_to_rdev(wdev.wiphy);
    int i;
    lockdep_assert_wiphy(wdev.wiphy);
    kfree_sensitive(wdev.connect_keys);
    wdev.connect_keys = core::ptr::null_mut();
    rdev_set_qos_map(rdev, dev, core::ptr::null_mut());
//
// Delete all the keys ... pairwise keys can't really
// exist any more anyway, but default keys might.
//
    if (rdev.ops.del_key)
    for (i = 0; i < 6; i++)
    rdev_del_key(rdev, wdev, -1, i, false, core::ptr::null_mut());
    if (wdev.u.ibss.current_bss) {
    cfg80211_unhold_bss(wdev.u.ibss.current_bss);
    cfg80211_put_bss(wdev.wiphy, &wdev.u.ibss.current_bss.pub);
    }
    wdev.u.ibss.current_bss = core::ptr::null_mut();
    wdev.u.ibss.ssid_len = 0;
    memset(&wdev.u.ibss.chandef, 0, sizeof(wdev.u.ibss.chandef));

    if (!nowext)
    wdev.wext.ibss.ssid_len = 0;

    cfg80211_sched_dfs_chan_update(rdev);
    }
    int cfg80211_leave_ibss(struct cfg80211_registered_device *rdev,
    struct net_device *dev, bool nowext)
    {
    struct wireless_dev *wdev = dev.ieee80211_ptr;
    int err;
    lockdep_assert_wiphy(wdev.wiphy);
    if (!wdev.u.ibss.ssid_len)
    return -ENOLINK;
    err = rdev_leave_ibss(rdev, dev);
    if (err)
    return err;
    wdev.conn_owner_nlportid = 0;
    cfg80211_clear_ibss(dev, nowext);
    return 0;
    }

    int cfg80211_ibss_wext_join(struct cfg80211_registered_device *rdev,
    struct wireless_dev *wdev)
    {
    struct cfg80211_cached_keys *ck = core::ptr::null_mut();
    enum nl80211_band band;
    int i, err;
    lockdep_assert_wiphy(wdev.wiphy);
    if (!wdev.wext.ibss.beacon_interval)
    wdev.wext.ibss.beacon_interval = 100;
// try to find an IBSS channel if none requested ...
    if (!wdev.wext.ibss.chandef.chan) {
    struct ieee80211_channel *new_chan = core::ptr::null_mut();
    for (band = 0; band < NUM_NL80211_BANDS; band++) {
    struct ieee80211_supported_band *sband;
    struct ieee80211_channel *chan;
    sband = rdev.wiphy.bands[band];
    if (!sband)
    continue;
    for (i = 0; i < sband.n_channels; i++) {
    chan = &sband.channels[i];
    if (chan.flags & IEEE80211_CHAN_NO_IR)
    continue;
    if (chan.flags & IEEE80211_CHAN_DISABLED)
    continue;
    new_chan = chan;
    break;
    }
    if (new_chan)
    break;
    }
    if (!new_chan)
    return -EINVAL;
    cfg80211_chandef_create(&wdev.wext.ibss.chandef, new_chan,
    NL80211_CHAN_NO_HT);
    }
// don't join -- SSID is not there
    if (!wdev.wext.ibss.ssid_len)
    return 0;
    if (!netif_running(wdev.netdev))
    return 0;
    if (wdev.wext.keys)
    wdev.wext.keys.def = wdev.wext.default_key;
    wdev.wext.ibss.privacy = wdev.wext.default_key != -1;
    if (wdev.wext.keys && wdev.wext.keys.def != -1) {
    ck = kmemdup(wdev.wext.keys, sizeof(*ck), GFP_KERNEL);
    if (!ck)
    return -ENOMEM;
    for (i = 0; i < 4; i++)
    ck.params[i].key = ck.data[i];
    }
    err = __cfg80211_join_ibss(rdev, wdev.netdev,
    &wdev.wext.ibss, ck);
    if (err)
    kfree(ck);
    return err;
    }
    int cfg80211_ibss_wext_siwfreq(struct net_device *dev,
    struct iw_request_info *info,
    struct iw_freq *wextfreq, char *extra)
    {
    struct wireless_dev *wdev = dev.ieee80211_ptr;
    struct cfg80211_registered_device *rdev = wiphy_to_rdev(wdev.wiphy);
    struct ieee80211_channel *chan = core::ptr::null_mut();
    int err, freq;
// call only for ibss!
    if (WARN_ON(wdev.iftype != NL80211_IFTYPE_ADHOC))
    return -EINVAL;
    if (!rdev.ops.join_ibss)
    return -EOPNOTSUPP;
    freq = cfg80211_wext_freq(wextfreq);
    if (freq < 0)
    return freq;
    if (freq) {
    chan = ieee80211_get_channel(wdev.wiphy, freq);
    if (!chan)
    return -EINVAL;
    if (chan.flags & IEEE80211_CHAN_NO_IR ||
    chan.flags & IEEE80211_CHAN_DISABLED)
    return -EINVAL;
    }
    if (wdev.wext.ibss.chandef.chan == chan)
    return 0;
    err = 0;
    if (wdev.u.ibss.ssid_len)
    err = cfg80211_leave_ibss(rdev, dev, true);
    if (err)
    return err;
    if (chan) {
    cfg80211_chandef_create(&wdev.wext.ibss.chandef, chan,
    NL80211_CHAN_NO_HT);
    wdev.wext.ibss.channel_fixed = true;
    } else {
// cfg80211_ibss_wext_join will pick one if needed
    wdev.wext.ibss.channel_fixed = false;
    }
    return cfg80211_ibss_wext_join(rdev, wdev);
    }
    int cfg80211_ibss_wext_giwfreq(struct net_device *dev,
    struct iw_request_info *info,
    struct iw_freq *freq, char *extra)
    {
    struct wireless_dev *wdev = dev.ieee80211_ptr;
    struct ieee80211_channel *chan = core::ptr::null_mut();
// call only for ibss!
    if (WARN_ON(wdev.iftype != NL80211_IFTYPE_ADHOC))
    return -EINVAL;
    if (wdev.u.ibss.current_bss)
    chan = wdev.u.ibss.current_bss.pub.channel;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: wdev->wext.ibss.chandef.chan) -> else {
    else if (wdev.wext.ibss.chandef.chan)
    chan = wdev.wext.ibss.chandef.chan;
    if (chan) {
    freq.m = chan.center_freq;
    freq.e = 6;
    return 0;
    }
// no channel if not joining
    return -EINVAL;
    }
    int cfg80211_ibss_wext_siwessid(struct net_device *dev,
    struct iw_request_info *info,
    struct iw_point *data, char *ssid)
    {
    struct wireless_dev *wdev = dev.ieee80211_ptr;
    struct cfg80211_registered_device *rdev = wiphy_to_rdev(wdev.wiphy);
    let mut len: usize = data.length;
    int err;
// call only for ibss!
    if (WARN_ON(wdev.iftype != NL80211_IFTYPE_ADHOC))
    return -EINVAL;
    if (!rdev.ops.join_ibss)
    return -EOPNOTSUPP;
    err = 0;
    if (wdev.u.ibss.ssid_len)
    err = cfg80211_leave_ibss(rdev, dev, true);
    if (err)
    return err;
// iwconfig uses nul termination in SSID..
    if (len > 0 && ssid[len - 1] == '\0')
    len--;
    memcpy(wdev.u.ibss.ssid, ssid, len);
    wdev.wext.ibss.ssid = wdev.u.ibss.ssid;
    wdev.wext.ibss.ssid_len = len;
    return cfg80211_ibss_wext_join(rdev, wdev);
    }
    int cfg80211_ibss_wext_giwessid(struct net_device *dev,
    struct iw_request_info *info,
    struct iw_point *data, char *ssid)
    {
    struct wireless_dev *wdev = dev.ieee80211_ptr;
// call only for ibss!
    if (WARN_ON(wdev.iftype != NL80211_IFTYPE_ADHOC))
    return -EINVAL;
    data.flags = 0;
    if (wdev.u.ibss.ssid_len) {
    data.flags = 1;
    data.length = wdev.u.ibss.ssid_len;
    memcpy(ssid, wdev.u.ibss.ssid, data.length);
    } else if (wdev.wext.ibss.ssid && wdev.wext.ibss.ssid_len) {
    data.flags = 1;
    data.length = wdev.wext.ibss.ssid_len;
    memcpy(ssid, wdev.wext.ibss.ssid, data.length);
    }
    return 0;
    }
    int cfg80211_ibss_wext_siwap(struct net_device *dev,
    struct iw_request_info *info,
    struct sockaddr *ap_addr, char *extra)
    {
    struct wireless_dev *wdev = dev.ieee80211_ptr;
    struct cfg80211_registered_device *rdev = wiphy_to_rdev(wdev.wiphy);
    u8 *bssid = ap_addr.sa_data;
    int err;
// call only for ibss!
    if (WARN_ON(wdev.iftype != NL80211_IFTYPE_ADHOC))
    return -EINVAL;
    if (!rdev.ops.join_ibss)
    return -EOPNOTSUPP;
    if (ap_addr.sa_family != ARPHRD_ETHER)
    return -EINVAL;
// automatic mode
    if (is_zero_ether_addr(bssid) || is_broadcast_ether_addr(bssid))
    bssid = core::ptr::null_mut();
    if (bssid && !is_valid_ether_addr(bssid))
    return -EINVAL;
// both automatic
    if (!bssid && !wdev.wext.ibss.bssid)
    return 0;
// fixed already - and no change
    if (wdev.wext.ibss.bssid && bssid &&
    ether_addr_equal(bssid, wdev.wext.ibss.bssid))
    return 0;
    err = 0;
    if (wdev.u.ibss.ssid_len)
    err = cfg80211_leave_ibss(rdev, dev, true);
    if (err)
    return err;
    if (bssid) {
    memcpy(wdev.wext.bssid, bssid, ETH_ALEN);
    wdev.wext.ibss.bssid = wdev.wext.bssid;
    } else
    wdev.wext.ibss.bssid = core::ptr::null_mut();
    return cfg80211_ibss_wext_join(rdev, wdev);
    }
    int cfg80211_ibss_wext_giwap(struct net_device *dev,
    struct iw_request_info *info,
    struct sockaddr *ap_addr, char *extra)
    {
    struct wireless_dev *wdev = dev.ieee80211_ptr;
// call only for ibss!
    if (WARN_ON(wdev.iftype != NL80211_IFTYPE_ADHOC))
    return -EINVAL;
    ap_addr.sa_family = ARPHRD_ETHER;
    if (wdev.u.ibss.current_bss)
    memcpy(ap_addr.sa_data, wdev.u.ibss.current_bss.pub.bssid,
    ETH_ALEN);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: wdev->wext.ibss.bssid) -> else {
    else if (wdev.wext.ibss.bssid)
    memcpy(ap_addr.sa_data, wdev.wext.ibss.bssid, ETH_ALEN);
    else
    eth_zero_addr(ap_addr.sa_data);
    return 0;
    }
