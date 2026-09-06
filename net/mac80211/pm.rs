//! Automatically rewritten from C to Rust
//! Source: net/mac80211/pm.c
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
// Portions
// Copyright (C) 2020-2021, 2023-2024 Intel Corporation
//

#[no_mangle]
unsafe extern "C" fn ieee80211_sched_scan_cancel(local: *mut ieee80211_local) {
    static void ieee80211_sched_scan_cancel(struct ieee80211_local *local)
    {
    if (ieee80211_request_sched_scan_stop(local))
    return;
    cfg80211_sched_scan_stopped_locked(local.hw.wiphy, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn __ieee80211_suspend(hw: *mut ieee80211_hw, wowlan: *mut cfg80211_wowlan) -> c_int {
    int __ieee80211_suspend(struct ieee80211_hw *hw, struct cfg80211_wowlan *wowlan)
    {
    struct ieee80211_local *local = hw_to_local(hw);
    struct ieee80211_sub_if_data *sdata;
    struct sta_info *sta;
    if (!local.open_count)
    goto suspend;
    local.suspending = true;
    mb(); /* make suspending visible before any cancellation */
    ieee80211_scan_cancel(local);
    ieee80211_dfs_cac_cancel(local, core::ptr::null_mut());
    ieee80211_roc_purge(local, core::ptr::null_mut());
    ieee80211_del_virtual_monitor(local);
    if (ieee80211_hw_check(hw, AMPDU_AGGREGATION) &&
    !(wowlan && wowlan.any)) {
    lockdep_assert_wiphy(local.hw.wiphy);
    list_for_each_entry(sta, &local.sta_list, list) {
    set_sta_flag(sta, WLAN_STA_BLOCK_BA);
    ieee80211_sta_tear_down_BA_sessions(
    sta, AGG_STOP_LOCAL_REQUEST);
    }
    }
// keep sched_scan only in case of 'any' trigger
    if (!(wowlan && wowlan.any))
    ieee80211_sched_scan_cancel(local);
    ieee80211_stop_queues_by_reason(hw,
    IEEE80211_MAX_QUEUE_MAP,
    IEEE80211_QUEUE_STOP_REASON_SUSPEND,
    false);
// flush out all packets
    synchronize_net();
    ieee80211_flush_queues(local, core::ptr::null_mut(), true);
    local.quiescing = true;
// make quiescing visible to timers everywhere
    mb();
    flush_workqueue(local.workqueue);
// Don't try to run timers while suspended.
    timer_delete_sync(&local.sta_cleanup);
//
// Note that this particular timer doesn't need to be
// restarted at resume.
//
    wiphy_work_cancel(local.hw.wiphy, &local.dynamic_ps_enable_work);
    timer_delete_sync(&local.dynamic_ps_timer);
    local.wowlan = wowlan;
    if (local.wowlan) {
    int err;
// Drivers don't expect to suspend while some operations like
// authenticating or associating are in progress. It doesn't
// make sense anyway to accept that, since the authentication
// or association would never finish since the driver can't do
// that on its own.
// Thus, clean up in-progress auth/assoc first.
//
    list_for_each_entry(sdata, &local.interfaces, list) {
    if (!ieee80211_sdata_running(sdata))
    continue;
    if (sdata.vif.type != NL80211_IFTYPE_STATION)
    continue;
    ieee80211_mgd_quiesce(sdata);
// If suspended during TX in progress, and wowlan
// is enabled (connection will be active) there
// can be a race where the driver is put out
// of power-save due to TX and during suspend
// dynamic_ps_timer is cancelled and TX packet
// is flushed, leaving the driver in ACTIVE even
// after resuming until dynamic_ps_timer puts
// driver back in DOZE.
//
    if (sdata.u.mgd.associated &&
    sdata.u.mgd.powersave &&
    !(local.hw.conf.flags & IEEE80211_CONF_PS)) {
    local.hw.conf.flags |= IEEE80211_CONF_PS;
    ieee80211_hw_config(local, -1,
    IEEE80211_CONF_CHANGE_PS);
    }
    }
    err = drv_suspend(local, wowlan);
    if (err < 0) {
    local.quiescing = false;
    local.wowlan = false;
    if (ieee80211_hw_check(hw, AMPDU_AGGREGATION)) {
    lockdep_assert_wiphy(local.hw.wiphy);
    list_for_each_entry(sta,
    &local.sta_list, list) {
    clear_sta_flag(sta, WLAN_STA_BLOCK_BA);
    }
    }
    ieee80211_wake_queues_by_reason(hw,
    IEEE80211_MAX_QUEUE_MAP,
    IEEE80211_QUEUE_STOP_REASON_SUSPEND,
    false);
    return err;
    } else if (err > 0) {
    WARN_ON(err != 1);
// cfg80211 will call back into mac80211 to disconnect
// all interfaces, allow that to proceed properly
//
    ieee80211_wake_queues_by_reason(hw,
    IEEE80211_MAX_QUEUE_MAP,
    IEEE80211_QUEUE_STOP_REASON_SUSPEND,
    false);
    return err;
    } else {
    goto suspend;
    }
    }
// remove all interfaces that were created in the driver
    list_for_each_entry(sdata, &local.interfaces, list) {
    if (!ieee80211_sdata_running(sdata))
    continue;
    switch (sdata.vif.type) {
    case NL80211_IFTYPE_AP_VLAN:
    case NL80211_IFTYPE_MONITOR:
    continue;
    case NL80211_IFTYPE_STATION:
    ieee80211_mgd_quiesce(sdata);
    break;
    default:
    break;
    }
    wiphy_delayed_work_flush(local.hw.wiphy,
    &sdata.dec_tailroom_needed_wk);
    drv_remove_interface(local, sdata);
    }
//
// We disconnected on all interfaces before suspend, all channel
// contexts should be released.
//
    WARN_ON(!list_empty(&local.chanctx_list));
// stop hardware - this must stop RX
    ieee80211_stop_device(local, true);
    suspend:
    local.suspended = true;
// need suspended to be visible before quiescing is false
    barrier();
    local.quiescing = false;
    local.suspending = false;
    return 0;
    }
//
// __ieee80211_resume() is a static inline which just calls
// ieee80211_reconfig(), which is also needed for hardware
// hang/firmware failure/etc. recovery.
//
    void ieee80211_report_wowlan_wakeup(struct ieee80211_vif *vif,
    struct cfg80211_wowlan_wakeup *wakeup,
    gfp_t gfp)
    {
    struct ieee80211_sub_if_data *sdata = vif_to_sdata(vif);
    cfg80211_report_wowlan_wakeup(&sdata.wdev, wakeup, gfp);
    }
    EXPORT_SYMBOL(ieee80211_report_wowlan_wakeup);
