//! Automatically rewritten from C to Rust
//! Source: net/mac802154/util.c
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
// Authors:
// Alexander Aring <aar@pengutronix.de>
//
// Based on: net/mac80211/util.c
//

// privid for wpan_phys to determine whether they belong to us or not
    let mut mac802154_wpan_phy_privid: *const void const = &mac802154_wpan_phy_privid;
//
// ieee802154_wake_queue - wake ieee802154 queue
// @hw: main hardware object
//
// Tranceivers usually have either one transmit framebuffer or one framebuffer
// for both transmitting and receiving. Hence, the core currently only handles
// one frame at a time for each phy, which means we had to stop the queue to
// avoid new skb to come during the transmission. The queue then needs to be
// woken up after the operation.
//
#[no_mangle]
unsafe extern "C" fn ieee802154_wake_queue(hw: *mut ieee802154_hw) {
    static void ieee802154_wake_queue(struct ieee802154_hw *hw)
    {
    struct ieee802154_local *local = hw_to_local(hw);
    struct ieee802154_sub_if_data *sdata;
    rcu_read_lock();
    clear_bit(WPAN_PHY_FLAG_STATE_QUEUE_STOPPED, &local.phy.flags);
    list_for_each_entry_rcu(sdata, &local.interfaces, list) {
    if (!sdata.dev)
    continue;
    netif_wake_queue(sdata.dev);
    }
    rcu_read_unlock();
    }
//
// ieee802154_stop_queue - stop ieee802154 queue
// @hw: main hardware object
//
// Tranceivers usually have either one transmit framebuffer or one framebuffer
// for both transmitting and receiving. Hence, the core currently only handles
// one frame at a time for each phy, which means we need to tell upper layers to
// stop giving us new skbs while we are busy with the transmitted one. The queue
// must then be stopped before transmitting.
//
#[no_mangle]
unsafe extern "C" fn ieee802154_stop_queue(hw: *mut ieee802154_hw) {
    static void ieee802154_stop_queue(struct ieee802154_hw *hw)
    {
    struct ieee802154_local *local = hw_to_local(hw);
    struct ieee802154_sub_if_data *sdata;
    rcu_read_lock();
    list_for_each_entry_rcu(sdata, &local.interfaces, list) {
    if (!sdata.dev)
    continue;
    netif_stop_queue(sdata.dev);
    }
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn ieee802154_hold_queue(local: *mut ieee802154_local) {
    void ieee802154_hold_queue(struct ieee802154_local *local)
    {
    unsigned long flags;
    spin_lock_irqsave(&local.phy.queue_lock, flags);
    if (!atomic_fetch_inc(&local.phy.hold_txs))
    ieee802154_stop_queue(&local.hw);
    spin_unlock_irqrestore(&local.phy.queue_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn ieee802154_release_queue(local: *mut ieee802154_local) {
    void ieee802154_release_queue(struct ieee802154_local *local)
    {
    unsigned long flags;
    spin_lock_irqsave(&local.phy.queue_lock, flags);
    if (atomic_dec_and_test(&local.phy.hold_txs))
    ieee802154_wake_queue(&local.hw);
    spin_unlock_irqrestore(&local.phy.queue_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn ieee802154_disable_queue(local: *mut ieee802154_local) {
    void ieee802154_disable_queue(struct ieee802154_local *local)
    {
    struct ieee802154_sub_if_data *sdata;
    rcu_read_lock();
    list_for_each_entry_rcu(sdata, &local.interfaces, list) {
    if (!sdata.dev)
    continue;
    netif_tx_disable(sdata.dev);
    }
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn ieee802154_xmit_ifs_timer(timer: *mut hrtimer) -> enum hrtimer_restart {
    enum hrtimer_restart ieee802154_xmit_ifs_timer(struct hrtimer *timer)
    {
    struct ieee802154_local *local =
    container_of(timer, struct ieee802154_local, ifs_timer);
    ieee802154_release_queue(local);
    return HRTIMER_NORESTART;
    }
    void ieee802154_xmit_complete(struct ieee802154_hw *hw, struct sk_buff *skb,
    bool ifs_handling)
    {
    struct ieee802154_local *local = hw_to_local(hw);
    local.tx_result = IEEE802154_SUCCESS;
    if (ifs_handling) {
    u8 max_sifs_size;
// If transceiver sets CRC on his own we need to use lifs
// threshold len above 16 otherwise 18, because it's not
// part of skb->len.
//
    if (hw.flags & IEEE802154_HW_TX_OMIT_CKSUM)
    max_sifs_size = IEEE802154_MAX_SIFS_FRAME_SIZE -
    IEEE802154_FCS_LEN;
    else
    max_sifs_size = IEEE802154_MAX_SIFS_FRAME_SIZE;
    if (skb.len > max_sifs_size)
    hrtimer_start(&local.ifs_timer,
    hw.phy.lifs_period * NSEC_PER_USEC,
    HRTIMER_MODE_REL);
    else
    hrtimer_start(&local.ifs_timer,
    hw.phy.sifs_period * NSEC_PER_USEC,
    HRTIMER_MODE_REL);
    } else {
    ieee802154_release_queue(local);
    }
    dev_consume_skb_any(skb);
    if (atomic_dec_and_test(&hw.phy.ongoing_txs))
    wake_up(&hw.phy.sync_txq);
    }
    EXPORT_SYMBOL(ieee802154_xmit_complete);
    void ieee802154_xmit_error(struct ieee802154_hw *hw, struct sk_buff *skb,
    int reason)
    {
    struct ieee802154_local *local = hw_to_local(hw);
    local.tx_result = reason;
    ieee802154_release_queue(local);
    dev_kfree_skb_any(skb);
    if (atomic_dec_and_test(&hw.phy.ongoing_txs))
    wake_up(&hw.phy.sync_txq);
    }
    EXPORT_SYMBOL(ieee802154_xmit_error);
#[no_mangle]
pub unsafe extern "C" fn ieee802154_xmit_hw_error(hw: *mut ieee802154_hw, skb: *mut sk_buff) {
    void ieee802154_xmit_hw_error(struct ieee802154_hw *hw, struct sk_buff *skb)
    {
    ieee802154_xmit_error(hw, skb, IEEE802154_SYSTEM_ERROR);
    }
    EXPORT_SYMBOL(ieee802154_xmit_hw_error);
#[no_mangle]
pub unsafe extern "C" fn ieee802154_stop_device(local: *mut ieee802154_local) {
    void ieee802154_stop_device(struct ieee802154_local *local)
    {
    flush_workqueue(local.workqueue);
    hrtimer_cancel(&local.ifs_timer);
    drv_stop(local);
    }
