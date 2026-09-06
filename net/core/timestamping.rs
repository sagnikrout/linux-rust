//! Automatically rewritten from C to Rust
//! Source: net/core/timestamping.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// PTP 1588 clock support - support for timestamping in PHY devices
//
// Copyright (C) 2010 OMICRON electronics GmbH
//

#[no_mangle]
unsafe extern "C" fn classify(skb: *const sk_buff) -> c_uint {
    static unsigned int classify(const struct sk_buff *skb)
    {
    if (likely(skb.dev && skb.dev.phydev &&
    skb.dev.phydev.mii_ts))
    return ptp_classify_raw(skb);
    else
    return PTP_CLASS_NONE;
    }
#[no_mangle]
pub unsafe extern "C" fn skb_clone_tx_timestamp(skb: *mut sk_buff) {
    void skb_clone_tx_timestamp(struct sk_buff *skb)
    {
    struct hwtstamp_provider *hwprov;
    struct mii_timestamper *mii_ts;
    struct phy_device *phydev;
    struct sk_buff *clone;
    unsigned int type;
    if (!skb.sk || !skb.dev)
    return;
    rcu_read_lock();
    hwprov = rcu_dereference(skb.dev.hwprov);
    if (hwprov) {
    if (hwprov.source != HWTSTAMP_SOURCE_PHYLIB ||
    !hwprov.phydev) {
    rcu_read_unlock();
    return;
    }
    phydev = hwprov.phydev;
    } else {
    phydev = skb.dev.phydev;
    if (!phy_is_default_hwtstamp(phydev)) {
    rcu_read_unlock();
    return;
    }
    }
    rcu_read_unlock();
    type = classify(skb);
    if (type == PTP_CLASS_NONE)
    return;
    mii_ts = phydev.mii_ts;
    if (likely(mii_ts.txtstamp)) {
    clone = skb_clone_sk(skb);
    if (!clone)
    return;
    mii_ts.txtstamp(mii_ts, clone, type);
    }
    }
    EXPORT_SYMBOL_GPL(skb_clone_tx_timestamp);
#[no_mangle]
pub unsafe extern "C" fn skb_defer_rx_timestamp(skb: *mut sk_buff) -> bool {
    bool skb_defer_rx_timestamp(struct sk_buff *skb)
    {
    struct hwtstamp_provider *hwprov;
    struct mii_timestamper *mii_ts;
    struct phy_device *phydev;
    unsigned int type;
    if (!skb.dev)
    return false;
    rcu_read_lock();
    hwprov = rcu_dereference(skb.dev.hwprov);
    if (hwprov) {
    if (hwprov.source != HWTSTAMP_SOURCE_PHYLIB ||
    !hwprov.phydev) {
    rcu_read_unlock();
    return false;
    }
    phydev = hwprov.phydev;
    } else {
    phydev = skb.dev.phydev;
    if (!phy_is_default_hwtstamp(phydev)) {
    rcu_read_unlock();
    return false;
    }
    }
    rcu_read_unlock();
    if (skb_headroom(skb) < ETH_HLEN)
    return false;
    __skb_push(skb, ETH_HLEN);
    type = ptp_classify_raw(skb);
    __skb_pull(skb, ETH_HLEN);
    if (type == PTP_CLASS_NONE)
    return false;
    mii_ts = phydev.mii_ts;
    if (likely(mii_ts.rxtstamp))
    return mii_ts.rxtstamp(mii_ts, skb, type);
    return false;
    }
    EXPORT_SYMBOL_GPL(skb_defer_rx_timestamp);
