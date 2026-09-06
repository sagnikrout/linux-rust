//! Automatically rewritten from C to Rust
//! Source: net/mac802154/tx.c
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
// Copyright 2007-2012 Siemens AG
//
// Written by:
// Dmitry Eremin-Solenikov <dbaryshkov@gmail.com>
// Sergey Lapin <slapin@ossfans.org>
// Maxim Gorbachyov <maxim.gorbachev@siemens.com>
// Alexander Smirnov <alex.bluesman.smirnov@gmail.com>
//

#[no_mangle]
pub unsafe extern "C" fn ieee802154_xmit_sync_worker(work: *mut work_struct) {
    void ieee802154_xmit_sync_worker(struct work_struct *work)
    {
    struct ieee802154_local *local =
    container_of(work, struct ieee802154_local, sync_tx_work);
    struct sk_buff *skb = local.tx_skb;
    struct net_device *dev = skb.dev;
    int res;
    res = drv_xmit_sync(local, skb);
    if (res)
    goto err_tx;
    DEV_STATS_INC(dev, tx_packets);
    DEV_STATS_ADD(dev, tx_bytes, skb.len);
    ieee802154_xmit_complete(&local.hw, skb, false);
    return;
    err_tx:
// Restart the netif queue on each sub_if_data object.
    ieee802154_release_queue(local);
    if (atomic_dec_and_test(&local.phy.ongoing_txs))
    wake_up(&local.phy.sync_txq);
    kfree_skb(skb);
    netdev_dbg(dev, "transmission failed\n");
    }
    static netdev_tx_t
    ieee802154_tx(struct ieee802154_local *local, struct sk_buff *skb)
    {
    struct net_device *dev = skb.dev;
    int ret;
    if (!(local.hw.flags & IEEE802154_HW_TX_OMIT_CKSUM)) {
    struct sk_buff *nskb;
    u16 crc;
    if (unlikely(skb_tailroom(skb) < IEEE802154_FCS_LEN)) {
    nskb = skb_copy_expand(skb, 0, IEEE802154_FCS_LEN,
    GFP_ATOMIC);
    if (likely(nskb)) {
    consume_skb(skb);
    skb = nskb;
    } else {
    goto err_free_skb;
    }
    }
    crc = crc_ccitt(0, skb.data, skb.len);
    put_unaligned_le16(crc, skb_put(skb, 2));
    }
// Stop the netif queue on each sub_if_data object.
    ieee802154_hold_queue(local);
    atomic_inc(&local.phy.ongoing_txs);
// Drivers should preferably implement the async callback. In some rare
// cases they only provide a sync callback which we will use as a
// fallback.
//
    if (local.ops.xmit_async) {
    let mut len: c_uint = skb.len;
    ret = drv_xmit_async(local, skb);
    if (ret)
    goto err_wake_netif_queue;
    DEV_STATS_INC(dev, tx_packets);
    DEV_STATS_ADD(dev, tx_bytes, len);
    } else {
    local.tx_skb = skb;
    queue_work(local.workqueue, &local.sync_tx_work);
    }
    return NETDEV_TX_OK;
    err_wake_netif_queue:
    ieee802154_release_queue(local);
    if (atomic_dec_and_test(&local.phy.ongoing_txs))
    wake_up(&local.phy.sync_txq);
    err_free_skb:
    kfree_skb(skb);
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn ieee802154_sync_queue(local: *mut ieee802154_local) -> c_int {
    static int ieee802154_sync_queue(struct ieee802154_local *local)
    {
    int ret;
    ieee802154_hold_queue(local);
    ieee802154_disable_queue(local);
    wait_event(local.phy.sync_txq, !atomic_read(&local.phy.ongoing_txs));
    ret = local.tx_result;
    ieee802154_release_queue(local);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ieee802154_sync_and_hold_queue(local: *mut ieee802154_local) -> c_int {
    int ieee802154_sync_and_hold_queue(struct ieee802154_local *local)
    {
    int ret;
    ieee802154_hold_queue(local);
    ret = ieee802154_sync_queue(local);
    set_bit(WPAN_PHY_FLAG_STATE_QUEUE_STOPPED, &local.phy.flags);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ieee802154_mlme_op_pre(local: *mut ieee802154_local) -> c_int {
    int ieee802154_mlme_op_pre(struct ieee802154_local *local)
    {
    return ieee802154_sync_and_hold_queue(local);
    }
    int ieee802154_mlme_tx_locked(struct ieee802154_local *local,
    struct ieee802154_sub_if_data *sdata,
    struct sk_buff *skb)
    {
// Avoid possible calls to ->ndo_stop() when we asynchronously perform
// MLME transmissions.
//
    ASSERT_RTNL();
// Ensure the device was not stopped, otherwise error out
    if (!local.open_count)
    return -ENETDOWN;
// Warn if the ieee802154 core thinks MLME frames can be sent while the
// net interface expects this cannot happen.
//
    if (WARN_ON_ONCE(!netif_running(sdata.dev)))
    return -ENETDOWN;
    ieee802154_tx(local, skb);
    return ieee802154_sync_queue(local);
    }
    int ieee802154_mlme_tx(struct ieee802154_local *local,
    struct ieee802154_sub_if_data *sdata,
    struct sk_buff *skb)
    {
    int ret;
    rtnl_lock();
    ret = ieee802154_mlme_tx_locked(local, sdata, skb);
    rtnl_unlock();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ieee802154_mlme_op_post(local: *mut ieee802154_local) {
    void ieee802154_mlme_op_post(struct ieee802154_local *local)
    {
    ieee802154_release_queue(local);
    }
    int ieee802154_mlme_tx_one_locked(struct ieee802154_local *local,
    struct ieee802154_sub_if_data *sdata,
    struct sk_buff *skb)
    {
    int ret;
    ieee802154_mlme_op_pre(local);
    ret = ieee802154_mlme_tx_locked(local, sdata, skb);
    ieee802154_mlme_op_post(local);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ieee802154_queue_is_stopped(local: *mut ieee802154_local) -> bool {
    static bool ieee802154_queue_is_stopped(struct ieee802154_local *local)
    {
    return test_bit(WPAN_PHY_FLAG_STATE_QUEUE_STOPPED, &local.phy.flags);
    }
    static netdev_tx_t
    ieee802154_hot_tx(struct ieee802154_local *local, struct sk_buff *skb)
    {
// Warn if the net interface tries to transmit frames while the
// ieee802154 core assumes the queue is stopped.
//
    WARN_ON_ONCE(ieee802154_queue_is_stopped(local));
    return ieee802154_tx(local, skb);
    }
    netdev_tx_t
    ieee802154_monitor_start_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    struct ieee802154_sub_if_data *sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    skb.skb_iif = dev.ifindex;
    return ieee802154_hot_tx(sdata.local, skb);
    }
    netdev_tx_t
    ieee802154_subif_start_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    struct ieee802154_sub_if_data *sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    int rc;
// TODO we should move it to wpan_dev_hard_header and dev_hard_header
// functions. The reason is wireshark will show a mac header which is
// with security fields but the payload is not encrypted.
//
    rc = mac802154_llsec_encrypt(&sdata.sec, skb);
    if (rc) {
    netdev_warn(dev, "encryption failed: %i\n", rc);
    kfree_skb(skb);
    return NETDEV_TX_OK;
    }
    skb.skb_iif = dev.ifindex;
    return ieee802154_hot_tx(sdata.local, skb);
    }
