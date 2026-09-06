//! Automatically rewritten from C to Rust
//! Source: net/mac802154/mib.c
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
pub unsafe extern "C" fn mac802154_dev_set_page_channel(dev: *mut net_device, page: u8, chan: u8) {
    void mac802154_dev_set_page_channel(struct net_device *dev, u8 page, u8 chan)
    {
    struct ieee802154_sub_if_data *sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    struct ieee802154_local *local = sdata.local;
    int res;
    ASSERT_RTNL();
    BUG_ON(dev.type != ARPHRD_IEEE802154);
    res = drv_set_channel(local, page, chan);
    if (res) {
    pr_debug("set_channel failed\n");
    } else {
    local.phy.current_channel = chan;
    local.phy.current_page = page;
    }
    }
    int mac802154_get_params(struct net_device *dev,
    struct ieee802154_llsec_params *params)
    {
    struct ieee802154_sub_if_data *sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    int res;
    BUG_ON(dev.type != ARPHRD_IEEE802154);
    mutex_lock(&sdata.sec_mtx);
    res = mac802154_llsec_get_params(&sdata.sec, params);
    mutex_unlock(&sdata.sec_mtx);
    return res;
    }
    int mac802154_set_params(struct net_device *dev,
    const struct ieee802154_llsec_params *params,
    int changed)
    {
    struct ieee802154_sub_if_data *sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    int res;
    BUG_ON(dev.type != ARPHRD_IEEE802154);
    mutex_lock(&sdata.sec_mtx);
    res = mac802154_llsec_set_params(&sdata.sec, params, changed);
    mutex_unlock(&sdata.sec_mtx);
    return res;
    }
    int mac802154_add_key(struct net_device *dev,
    const struct ieee802154_llsec_key_id *id,
    const struct ieee802154_llsec_key *key)
    {
    struct ieee802154_sub_if_data *sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    int res;
    BUG_ON(dev.type != ARPHRD_IEEE802154);
    mutex_lock(&sdata.sec_mtx);
    res = mac802154_llsec_key_add(&sdata.sec, id, key);
    mutex_unlock(&sdata.sec_mtx);
    return res;
    }
    int mac802154_del_key(struct net_device *dev,
    const struct ieee802154_llsec_key_id *id)
    {
    struct ieee802154_sub_if_data *sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    int res;
    BUG_ON(dev.type != ARPHRD_IEEE802154);
    mutex_lock(&sdata.sec_mtx);
    res = mac802154_llsec_key_del(&sdata.sec, id);
    mutex_unlock(&sdata.sec_mtx);
    return res;
    }
    int mac802154_add_dev(struct net_device *dev,
    const struct ieee802154_llsec_device *llsec_dev)
    {
    struct ieee802154_sub_if_data *sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    int res;
    BUG_ON(dev.type != ARPHRD_IEEE802154);
    mutex_lock(&sdata.sec_mtx);
    res = mac802154_llsec_dev_add(&sdata.sec, llsec_dev);
    mutex_unlock(&sdata.sec_mtx);
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn mac802154_del_dev(dev: *mut net_device, dev_addr: __le64) -> c_int {
    int mac802154_del_dev(struct net_device *dev, __le64 dev_addr)
    {
    struct ieee802154_sub_if_data *sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    int res;
    BUG_ON(dev.type != ARPHRD_IEEE802154);
    mutex_lock(&sdata.sec_mtx);
    res = mac802154_llsec_dev_del(&sdata.sec, dev_addr);
    mutex_unlock(&sdata.sec_mtx);
    return res;
    }
    int mac802154_add_devkey(struct net_device *dev,
    __le64 device_addr,
    const struct ieee802154_llsec_device_key *key)
    {
    struct ieee802154_sub_if_data *sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    int res;
    BUG_ON(dev.type != ARPHRD_IEEE802154);
    mutex_lock(&sdata.sec_mtx);
    res = mac802154_llsec_devkey_add(&sdata.sec, device_addr, key);
    mutex_unlock(&sdata.sec_mtx);
    return res;
    }
    int mac802154_del_devkey(struct net_device *dev,
    __le64 device_addr,
    const struct ieee802154_llsec_device_key *key)
    {
    struct ieee802154_sub_if_data *sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    int res;
    BUG_ON(dev.type != ARPHRD_IEEE802154);
    mutex_lock(&sdata.sec_mtx);
    res = mac802154_llsec_devkey_del(&sdata.sec, device_addr, key);
    mutex_unlock(&sdata.sec_mtx);
    return res;
    }
    int mac802154_add_seclevel(struct net_device *dev,
    const struct ieee802154_llsec_seclevel *sl)
    {
    struct ieee802154_sub_if_data *sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    int res;
    BUG_ON(dev.type != ARPHRD_IEEE802154);
    mutex_lock(&sdata.sec_mtx);
    res = mac802154_llsec_seclevel_add(&sdata.sec, sl);
    mutex_unlock(&sdata.sec_mtx);
    return res;
    }
    int mac802154_del_seclevel(struct net_device *dev,
    const struct ieee802154_llsec_seclevel *sl)
    {
    struct ieee802154_sub_if_data *sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    int res;
    BUG_ON(dev.type != ARPHRD_IEEE802154);
    mutex_lock(&sdata.sec_mtx);
    res = mac802154_llsec_seclevel_del(&sdata.sec, sl);
    mutex_unlock(&sdata.sec_mtx);
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn mac802154_lock_table(dev: *mut net_device) {
    void mac802154_lock_table(struct net_device *dev)
    {
    struct ieee802154_sub_if_data *sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    BUG_ON(dev.type != ARPHRD_IEEE802154);
    mutex_lock(&sdata.sec_mtx);
    }
    void mac802154_get_table(struct net_device *dev,
    struct ieee802154_llsec_table **t)
    {
    struct ieee802154_sub_if_data *sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    BUG_ON(dev.type != ARPHRD_IEEE802154);
// t = &sdata->sec.table;
    }
#[no_mangle]
pub unsafe extern "C" fn mac802154_unlock_table(dev: *mut net_device) {
    void mac802154_unlock_table(struct net_device *dev)
    {
    struct ieee802154_sub_if_data *sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    BUG_ON(dev.type != ARPHRD_IEEE802154);
    mutex_unlock(&sdata.sec_mtx);
    }
