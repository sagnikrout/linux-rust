//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/mediatek/mt76/mt76x0/main.c
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
// Copyright (C) 2014 Felix Fietkau <nbd@openwrt.org>
// Copyright (C) 2015 Jakub Kicinski <kubakici@wp.pl>
// Copyright (C) 2018 Stanislaw Gruszka <stf_xl@wp.pl>
//

#[no_mangle]
pub unsafe extern "C" fn mt76x0_set_channel(mphy: *mut mt76_phy) -> c_int {
    int mt76x0_set_channel(struct mt76_phy *mphy)
    {
    struct mt76x02_dev *dev = container_of(mphy.dev, struct mt76x02_dev, mt76);
    mt76x02_pre_tbtt_enable(dev, false);
    if (mt76_is_mmio(&dev.mt76))
    tasklet_disable(&dev.dfs_pd.dfs_tasklet);
    mt76x0_phy_set_channel(dev, &mphy.chandef);
    mt76x02_mac_cc_reset(dev);
    mt76x02_edcca_init(dev);
    if (mt76_is_mmio(&dev.mt76)) {
    mt76x02_dfs_init_params(dev);
    tasklet_enable(&dev.dfs_pd.dfs_tasklet);
    }
    mt76x02_pre_tbtt_enable(dev, true);
    return 0;
    }
    EXPORT_SYMBOL_GPL(mt76x0_set_channel);
    int mt76x0_set_sar_specs(struct ieee80211_hw *hw,
    const struct cfg80211_sar_specs *sar)
    {
    let mut err: c_int = -EINVAL, power = hw.conf.power_level * 2;
    struct mt76x02_dev *dev = hw.priv;
    struct mt76_phy *mphy = &dev.mphy;
    mutex_lock(&dev.mt76.mutex);
    if (!cfg80211_chandef_valid(&mphy.chandef))
    goto out;
    err = mt76_init_sar_power(hw, sar);
    if (err)
    goto out;
    dev.txpower_conf = mt76_get_sar_power(mphy, mphy.chandef.chan,
    power);
    if (test_bit(MT76_STATE_RUNNING, &mphy.state))
    mt76x0_phy_set_txpower(dev);
    out:
    mutex_unlock(&dev.mt76.mutex);
    return err;
    }
    EXPORT_SYMBOL_GPL(mt76x0_set_sar_specs);
#[no_mangle]
pub unsafe extern "C" fn mt76x0_config(hw: *mut ieee80211_hw, radio_idx: c_int, changed: u32) -> c_int {
    int mt76x0_config(struct ieee80211_hw *hw, int radio_idx, u32 changed)
    {
    struct mt76x02_dev *dev = hw.priv;
    if (changed & IEEE80211_CONF_CHANGE_CHANNEL)
    mt76_update_channel(&dev.mphy);
    mutex_lock(&dev.mt76.mutex);
    if (changed & IEEE80211_CONF_CHANGE_POWER) {
    struct mt76_phy *mphy = &dev.mphy;
    dev.txpower_conf = hw.conf.power_level * 2;
    dev.txpower_conf = mt76_get_sar_power(mphy,
    mphy.chandef.chan,
    dev.txpower_conf);
    if (test_bit(MT76_STATE_RUNNING, &mphy.state))
    mt76x0_phy_set_txpower(dev);
    }
    if (changed & IEEE80211_CONF_CHANGE_MONITOR) {
    if (!(hw.conf.flags & IEEE80211_CONF_MONITOR))
    dev.mt76.rxfilter |= MT_RX_FILTR_CFG_PROMISC;
    else
    dev.mt76.rxfilter &= ~MT_RX_FILTR_CFG_PROMISC;
    mt76_wr(dev, MT_RX_FILTR_CFG, dev.mt76.rxfilter);
    }
    mutex_unlock(&dev.mt76.mutex);
    return 0;
    }
    EXPORT_SYMBOL_GPL(mt76x0_config);
