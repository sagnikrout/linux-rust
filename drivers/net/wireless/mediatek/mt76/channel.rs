//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/mediatek/mt76/channel.c
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (C) 2024 Felix Fietkau <nbd@nbd.name>
//

    static struct mt76_vif_link *
    mt76_alloc_mlink(struct mt76_dev *dev, struct mt76_vif_data *mvif)
    {
    struct mt76_vif_link *mlink;
    mlink = kzalloc(dev.drv.link_data_size, GFP_KERNEL);
    if (!mlink)
    return core::ptr::null_mut();
    mlink.mvif = mvif;
    return mlink;
    }
    static int
    mt76_phy_update_channel(struct mt76_phy *phy,
    struct ieee80211_chanctx_conf *conf)
    {
    phy.radar_enabled = conf.radar_enabled;
    phy.main_chandef = conf.def;
    phy.chanctx = (struct mt76_chanctx *)conf.drv_priv;
    return __mt76_set_channel(phy, &phy.main_chandef, false);
    }
    int mt76_add_chanctx(struct ieee80211_hw *hw,
    struct ieee80211_chanctx_conf *conf)
    {
    struct mt76_chanctx *ctx = (struct mt76_chanctx *)conf.drv_priv;
    struct mt76_phy *phy = hw.priv;
    struct mt76_dev *dev = phy.dev;
    let mut ret: c_int = -EINVAL;
    phy = ctx.phy = dev.band_phys[conf.def.chan.band];
    if (WARN_ON_ONCE(!phy))
    return ret;
    if (dev.scan.phy == phy)
    mt76_abort_scan(dev);
    mutex_lock(&dev.mutex);
    if (!phy.chanctx)
    ret = mt76_phy_update_channel(phy, conf);
    else
    ret = 0;
    mutex_unlock(&dev.mutex);
    return ret;
    }
    EXPORT_SYMBOL_GPL(mt76_add_chanctx);
    void mt76_remove_chanctx(struct ieee80211_hw *hw,
    struct ieee80211_chanctx_conf *conf)
    {
    struct mt76_chanctx *ctx = (struct mt76_chanctx *)conf.drv_priv;
    struct mt76_phy *phy = hw.priv;
    struct mt76_dev *dev = phy.dev;
    phy = ctx.phy;
    if (WARN_ON_ONCE(!phy))
    return;
    if (dev.scan.phy == phy)
    mt76_abort_scan(dev);
    mutex_lock(&dev.mutex);
    if (phy.chanctx == ctx)
    phy.chanctx = core::ptr::null_mut();
    mutex_unlock(&dev.mutex);
    }
    EXPORT_SYMBOL_GPL(mt76_remove_chanctx);
    void mt76_change_chanctx(struct ieee80211_hw *hw,
    struct ieee80211_chanctx_conf *conf,
    u32 changed)
    {
    struct mt76_chanctx *ctx = (struct mt76_chanctx *)conf.drv_priv;
    struct mt76_phy *phy = ctx.phy;
    struct mt76_dev *dev = phy.dev;
    if (!(changed & (IEEE80211_CHANCTX_CHANGE_WIDTH |
    IEEE80211_CHANCTX_CHANGE_RADAR)))
    return;
    if (phy.roc_vif)
    mt76_abort_roc(phy);
    cancel_delayed_work_sync(&phy.mac_work);
    mutex_lock(&dev.mutex);
    mt76_phy_update_channel(phy, conf);
    mutex_unlock(&dev.mutex);
    }
    EXPORT_SYMBOL_GPL(mt76_change_chanctx);
    int mt76_assign_vif_chanctx(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif,
    struct ieee80211_bss_conf *link_conf,
    struct ieee80211_chanctx_conf *conf)
    {
    struct mt76_chanctx *ctx = (struct mt76_chanctx *)conf.drv_priv;
    struct mt76_vif_link *mlink = (struct mt76_vif_link *)vif.drv_priv;
    struct mt76_vif_data *mvif = mlink.mvif;
    let mut link_id: c_int = link_conf.link_id;
    struct mt76_phy *phy = ctx.phy;
    struct mt76_dev *dev = phy.dev;
    let mut mlink_alloc: bool = false;
    let mut ret: c_int = 0;
    if (dev.scan.vif == vif)
    mt76_abort_scan(dev);
    mutex_lock(&dev.mutex);
    if (vif.type == NL80211_IFTYPE_MONITOR &&
    is_zero_ether_addr(vif.addr))
    goto out;
    mlink = mt76_vif_conf_link(dev, vif, link_conf);
    if (!mlink) {
    mlink = mt76_alloc_mlink(dev, mvif);
    if (!mlink) {
    ret = -ENOMEM;
    goto out;
    }
    mlink_alloc = true;
    }
    mlink.ctx = conf;
    ret = dev.drv.vif_link_add(phy, vif, link_conf, mlink);
    if (ret) {
    if (mlink_alloc)
    kfree(mlink);
    goto out;
    }
    if (link_conf != &vif.bss_conf)
    rcu_assign_pointer(mvif.link[link_id], mlink);
    out:
    mutex_unlock(&dev.mutex);
    return ret;
    }
    EXPORT_SYMBOL_GPL(mt76_assign_vif_chanctx);
    void mt76_unassign_vif_chanctx(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif,
    struct ieee80211_bss_conf *link_conf,
    struct ieee80211_chanctx_conf *conf)
    {
    struct mt76_chanctx *ctx = (struct mt76_chanctx *)conf.drv_priv;
    struct mt76_vif_link *mlink = (struct mt76_vif_link *)vif.drv_priv;
    struct mt76_phy *phy = ctx.phy;
    struct mt76_dev *dev = phy.dev;
    if (dev.scan.vif == vif)
    mt76_abort_scan(dev);
    mutex_lock(&dev.mutex);
    if (vif.type == NL80211_IFTYPE_MONITOR &&
    is_zero_ether_addr(vif.addr))
    goto out;
    mlink = mt76_vif_conf_link(dev, vif, link_conf);
    if (!mlink)
    goto out;
    dev.drv.vif_link_remove(phy, vif, link_conf, mlink);
    mlink.ctx = core::ptr::null_mut();
    out:
    mutex_unlock(&dev.mutex);
    }
    EXPORT_SYMBOL_GPL(mt76_unassign_vif_chanctx);
    int mt76_switch_vif_chanctx(struct ieee80211_hw *hw,
    struct ieee80211_vif_chanctx_switch *vifs,
    int n_vifs,
    enum ieee80211_chanctx_switch_mode mode)
    {
    struct ieee80211_vif_chanctx_switch *v;
    struct mt76_chanctx *old_ctx, *new_ctx;
    struct mt76_phy *old_phy, *phy = hw.priv;
    struct mt76_dev *dev = phy.dev;
    struct mt76_vif_link *mlink;
    bool need_update[__MT_MAX_BAND] = {};
    int i, ret = 0;
    for (i = 0; i < n_vifs; i++) {
    v = &vifs[i];
    new_ctx = (struct mt76_chanctx *)v.new_ctx.drv_priv;
    if (mode == CHANCTX_SWMODE_SWAP_CONTEXTS)
    phy = new_ctx.phy = dev.band_phys[v.new_ctx.def.chan.band];
    else
    phy = new_ctx.phy;
    if (!phy)
    return -EINVAL;
    if (need_update[phy.band_idx])
    continue;
    if (phy.chanctx != new_ctx) {
    if (dev.scan.phy == phy)
    mt76_abort_scan(dev);
    cancel_delayed_work_sync(&phy.mac_work);
    need_update[phy.band_idx] = true;
    }
    }
    mutex_lock(&dev.mutex);
    for (i = 0; i < n_vifs; i++) {
    v = &vifs[i];
    old_ctx = (struct mt76_chanctx *)v.old_ctx.drv_priv;
    old_phy = old_ctx.phy;
    new_ctx = (struct mt76_chanctx *)v.new_ctx.drv_priv;
    phy = new_ctx.phy;
    if (mode == CHANCTX_SWMODE_SWAP_CONTEXTS && old_phy.chanctx &&
    old_phy.chanctx == old_ctx && phy != old_phy)
    old_phy.chanctx = core::ptr::null_mut();
    if (need_update[phy.band_idx]) {
    ret = mt76_phy_update_channel(phy, v.new_ctx);
    if (ret)
    goto out;
    need_update[phy.band_idx] = false;
    }
    mlink = mt76_vif_conf_link(dev, v.vif, v.link_conf);
    if (!mlink)
    continue;
    if (old_phy != phy) {
    dev.drv.vif_link_remove(old_phy, v.vif, v.link_conf,
    mlink);
    ret = dev.drv.vif_link_add(phy, v.vif, v.link_conf,
    mlink);
    if (ret)
    goto out;
    }
    mlink.ctx = v.new_ctx;
    if (mlink.beacon_mon_interval)
    WRITE_ONCE(mlink.beacon_mon_last, jiffies);
    }
    out:
    mutex_unlock(&dev.mutex);
    return ret;
    }
    EXPORT_SYMBOL_GPL(mt76_switch_vif_chanctx);
    struct mt76_vif_link *mt76_get_vif_phy_link(struct mt76_phy *phy,
    struct ieee80211_vif *vif)
    {
    struct mt76_vif_link *mlink = (struct mt76_vif_link *)vif.drv_priv;
    struct mt76_vif_data *mvif = mlink.mvif;
    struct mt76_dev *dev = phy.dev;
    int i, ret;
    for (i = 0; i < ARRAY_SIZE(mvif.link); i++) {
    mlink = mt76_dereference(mvif.link[i], dev);
    if (!mlink)
    continue;
    if (mt76_vif_link_phy(mlink) == phy)
    return mlink;
    }
    if (!dev.drv.vif_link_add)
    return ERR_PTR(-EINVAL);
    mlink = mt76_alloc_mlink(dev, mvif);
    if (!mlink)
    return ERR_PTR(-ENOMEM);
    mlink.offchannel = true;
    ret = dev.drv.vif_link_add(phy, vif, &vif.bss_conf, mlink);
    if (ret) {
    kfree(mlink);
    return ERR_PTR(ret);
    }
    rcu_assign_pointer(mvif.offchannel_link, mlink);
    return mlink;
    }
    void mt76_put_vif_phy_link(struct mt76_phy *phy, struct ieee80211_vif *vif,
    struct mt76_vif_link *mlink)
    {
    struct mt76_dev *dev = phy.dev;
    struct mt76_vif_data *mvif;
    if (IS_ERR_OR_NULL(mlink) || !mlink.offchannel)
    return;
    mvif = mlink.mvif;
    rcu_assign_pointer(mvif.offchannel_link, core::ptr::null_mut());
    dev.drv.vif_link_remove(phy, vif, &vif.bss_conf, mlink);
    kfree_rcu(mlink, rcu_head);
    }
#[no_mangle]
pub unsafe extern "C" fn mt76_roc_complete(phy: *mut mt76_phy) {
    void mt76_roc_complete(struct mt76_phy *phy)
    {
    struct mt76_vif_link *mlink = phy.roc_link;
    struct mt76_dev *dev = phy.dev;
    if (!phy.roc_vif)
    return;
    if (mlink)
    mlink.mvif.roc_phy = core::ptr::null_mut();
    if (phy.chanctx && phy.main_chandef.chan && phy.offchannel &&
    !test_bit(MT76_MCU_RESET, &dev.phy.state)) {
    __mt76_set_channel(phy, &phy.main_chandef, false);
    mt76_offchannel_notify(phy, false);
    }
    mt76_put_vif_phy_link(phy, phy.roc_vif, phy.roc_link);
    phy.roc_vif = core::ptr::null_mut();
    phy.roc_link = core::ptr::null_mut();
    if (!test_bit(MT76_MCU_RESET, &dev.phy.state))
    ieee80211_remain_on_channel_expired(phy.hw);
    }
#[no_mangle]
pub unsafe extern "C" fn mt76_roc_complete_work(work: *mut work_struct) {
    void mt76_roc_complete_work(struct work_struct *work)
    {
    struct mt76_phy *phy = container_of(work, struct mt76_phy, roc_work.work);
    struct mt76_dev *dev = phy.dev;
    mutex_lock(&dev.mutex);
    mt76_roc_complete(phy);
    mutex_unlock(&dev.mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn mt76_abort_roc(phy: *mut mt76_phy) {
    void mt76_abort_roc(struct mt76_phy *phy)
    {
    struct mt76_dev *dev = phy.dev;
    cancel_delayed_work_sync(&phy.roc_work);
    mutex_lock(&dev.mutex);
    mt76_roc_complete(phy);
    mutex_unlock(&dev.mutex);
    }
    EXPORT_SYMBOL_GPL(mt76_abort_roc);
    int mt76_remain_on_channel(struct ieee80211_hw *hw, struct ieee80211_vif *vif,
    struct ieee80211_channel *chan, int duration,
    enum ieee80211_roc_type type)
    {
    let mut chandef: cfg80211_chan_def = {};
    struct mt76_phy *phy = hw.priv;
    struct mt76_dev *dev = phy.dev;
    struct mt76_vif_link *mlink;
    bool offchannel;
    let mut ret: c_int = 0;
    phy = dev.band_phys[chan.band];
    if (!phy)
    return -EINVAL;
    cancel_delayed_work_sync(&phy.mac_work);
    mutex_lock(&dev.mutex);
    if (phy.roc_vif || dev.scan.phy == phy ||
    test_bit(MT76_MCU_RESET, &dev.phy.state)) {
    ret = -EBUSY;
    goto out;
    }
    mlink = mt76_get_vif_phy_link(phy, vif);
    if (IS_ERR(mlink)) {
    ret = PTR_ERR(mlink);
    goto out;
    }
    mlink.mvif.roc_phy = phy;
    phy.roc_vif = vif;
    phy.roc_link = mlink;
    offchannel = mt76_offchannel_chandef(phy, chan, &chandef);
    if (offchannel)
    mt76_offchannel_notify(phy, true);
    ret = __mt76_set_channel(phy, &chandef, offchannel);
    if (ret) {
    mlink.mvif.roc_phy = core::ptr::null_mut();
    phy.roc_vif = core::ptr::null_mut();
    phy.roc_link = core::ptr::null_mut();
    mt76_put_vif_phy_link(phy, vif, mlink);
    goto out;
    }
    ieee80211_ready_on_channel(hw);
    ieee80211_queue_delayed_work(phy.hw, &phy.roc_work,
    msecs_to_jiffies(duration));
    out:
    mutex_unlock(&dev.mutex);
    return ret;
    }
    EXPORT_SYMBOL_GPL(mt76_remain_on_channel);
    int mt76_cancel_remain_on_channel(struct ieee80211_hw *hw,
    struct ieee80211_vif *vif)
    {
    struct mt76_vif_link *mlink = (struct mt76_vif_link *)vif.drv_priv;
    struct mt76_vif_data *mvif = mlink.mvif;
    struct mt76_phy *phy = mvif.roc_phy;
    if (!phy)
    return 0;
    mt76_abort_roc(phy);
    return 0;
    }
    EXPORT_SYMBOL_GPL(mt76_cancel_remain_on_channel);
