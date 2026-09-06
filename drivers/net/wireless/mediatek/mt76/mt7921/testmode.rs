//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/mediatek/mt76/mt7921/testmode.c
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

    enum mt7921_testmode_attr {
    MT7921_TM_ATTR_UNSPEC,
    MT7921_TM_ATTR_SET,
    MT7921_TM_ATTR_QUERY,
    MT7921_TM_ATTR_RSP,
// keep last
    NUM_MT7921_TM_ATTRS,
    MT7921_TM_ATTR_MAX = NUM_MT7921_TM_ATTRS - 1,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_tm_cmd {
    pub action: u8,
    pub param0: u32,
    pub param1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_tm_evt {
    pub param0: u32,
    pub param1: u32,
}

    static const struct nla_policy mt7921_tm_policy[NUM_MT7921_TM_ATTRS] = {
    [MT7921_TM_ATTR_SET] = NLA_POLICY_EXACT_LEN(sizeof(struct mt7921_tm_cmd)),
    [MT7921_TM_ATTR_QUERY] = NLA_POLICY_EXACT_LEN(sizeof(struct mt7921_tm_cmd)),
    };
    static int
    mt7921_tm_set(struct mt792x_dev *dev, struct mt7921_tm_cmd *req)
    {
    struct mt7921_rftest_cmd cmd = {
    .action = req.action,
    .param0 = cpu_to_le32(req.param0),
    .param1 = cpu_to_le32(req.param1),
    };
    let mut testmode: bool = false, normal = false;
    struct mt76_connac_pm *pm = &dev.pm;
    struct mt76_phy *phy = &dev.mphy;
    let mut ret: c_int = -ENOTCONN;
    mutex_lock(&dev.mt76.mutex);
    if (req.action == TM_SWITCH_MODE) {
    if (req.param0 == MT7921_TM_NORMAL)
    normal = true;
    else
    testmode = true;
    }
    if (testmode) {
// Make sure testmode running on full power mode
    pm.enable = false;
    cancel_delayed_work_sync(&pm.ps_work);
    cancel_work_sync(&pm.wake_work);
    __mt792x_mcu_drv_pmctrl(dev);
    phy.test.state = MT76_TM_STATE_ON;
    }
    if (!mt76_testmode_enabled(phy))
    goto out;
    ret = mt76_mcu_send_msg(&dev.mt76, MCU_CE_CMD(TEST_CTRL), &cmd,
    sizeof(cmd), false);
    if (ret)
    goto out;
    if (normal) {
// Switch back to the normal world
    phy.test.state = MT76_TM_STATE_OFF;
    pm.enable = true;
    }
    out:
    mutex_unlock(&dev.mt76.mutex);
    return ret;
    }
    static int
    mt7921_tm_query(struct mt792x_dev *dev, struct mt7921_tm_cmd *req,
    struct mt7921_tm_evt *evt_resp)
    {
    struct mt7921_rftest_cmd cmd = {
    .action = req.action,
    .param0 = cpu_to_le32(req.param0),
    .param1 = cpu_to_le32(req.param1),
    };
    struct mt7921_rftest_evt *evt;
    struct sk_buff *skb;
    int ret;
    ret = mt76_mcu_send_and_get_msg(&dev.mt76, MCU_CE_CMD(TEST_CTRL),
    &cmd, sizeof(cmd), true, &skb);
    if (ret)
    goto out;
    evt = (struct mt7921_rftest_evt *)skb.data;
    evt_resp.param0 = le32_to_cpu(evt.param0);
    evt_resp.param1 = le32_to_cpu(evt.param1);
    out:
    dev_kfree_skb(skb);
    return ret;
    }
    int mt7921_testmode_cmd(struct ieee80211_hw *hw, struct ieee80211_vif *vif,
    void *data, int len)
    {
    struct nlattr *tb[NUM_MT76_TM_ATTRS];
    struct mt76_phy *mphy = hw.priv;
    struct mt792x_phy *phy = mphy.priv;
    int err;
    if (!test_bit(MT76_STATE_RUNNING, &mphy.state) ||
    !(hw.conf.flags & IEEE80211_CONF_MONITOR))
    return -ENOTCONN;
    err = nla_parse_deprecated(tb, MT76_TM_ATTR_MAX, data, len,
    mt76_tm_policy, core::ptr::null_mut());
    if (err)
    return err;
    if (tb[MT76_TM_ATTR_DRV_DATA]) {
    struct nlattr *drv_tb[NUM_MT7921_TM_ATTRS], *data;
    int ret;
    data = tb[MT76_TM_ATTR_DRV_DATA];
    ret = nla_parse_nested_deprecated(drv_tb,
    MT7921_TM_ATTR_MAX,
    data, mt7921_tm_policy,
    core::ptr::null_mut());
    if (ret)
    return ret;
    data = drv_tb[MT7921_TM_ATTR_SET];
    if (data)
    return mt7921_tm_set(phy.dev, nla_data(data));
    }
    return -EINVAL;
    }
    int mt7921_testmode_dump(struct ieee80211_hw *hw, struct sk_buff *msg,
    struct netlink_callback *cb, void *data, int len)
    {
    struct nlattr *tb[NUM_MT76_TM_ATTRS];
    struct mt76_phy *mphy = hw.priv;
    struct mt792x_phy *phy = mphy.priv;
    int err;
    if (!test_bit(MT76_STATE_RUNNING, &mphy.state) ||
    !(hw.conf.flags & IEEE80211_CONF_MONITOR) ||
    !mt76_testmode_enabled(mphy))
    return -ENOTCONN;
    if (cb.args[2]++ > 0)
    return -ENOENT;
    err = nla_parse_deprecated(tb, MT76_TM_ATTR_MAX, data, len,
    mt76_tm_policy, core::ptr::null_mut());
    if (err)
    return err;
    if (tb[MT76_TM_ATTR_DRV_DATA]) {
    struct nlattr *drv_tb[NUM_MT7921_TM_ATTRS], *data;
    int ret;
    data = tb[MT76_TM_ATTR_DRV_DATA];
    ret = nla_parse_nested_deprecated(drv_tb,
    MT7921_TM_ATTR_MAX,
    data, mt7921_tm_policy,
    core::ptr::null_mut());
    if (ret)
    return ret;
    data = drv_tb[MT7921_TM_ATTR_QUERY];
    if (data) {
    struct mt7921_tm_evt evt_resp;
    err = mt7921_tm_query(phy.dev, nla_data(data),
    &evt_resp);
    if (err)
    return err;
    return nla_put(msg, MT7921_TM_ATTR_RSP,
    sizeof(evt_resp), &evt_resp);
    }
    }
    return -EINVAL;
    }
