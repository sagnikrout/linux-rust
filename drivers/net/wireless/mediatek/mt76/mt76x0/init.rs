//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/mediatek/mt76/mt76x0/init.c
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
// (c) Copyright 2002-2010, Ralink Technology, Inc.
// Copyright (C) 2014 Felix Fietkau <nbd@openwrt.org>
// Copyright (C) 2015 Jakub Kicinski <kubakici@wp.pl>
// Copyright (C) 2018 Stanislaw Gruszka <stf_xl@wp.pl>
//

    static void
    mt76x0_set_wlan_state(struct mt76x02_dev *dev, u32 val, bool enable)
    {
    let mut mask: u32 = MT_CMB_CTRL_XTAL_RDY | MT_CMB_CTRL_PLL_LD;
// Note: we don't turn off WLAN_CLK because that makes the device
// not respond properly on the probe path.
// In case anyone (PSM?) wants to use this function we can
// bring the clock stuff back and fixup the probe path.
//
    if (enable)
    val |= (MT_WLAN_FUN_CTRL_WLAN_EN |
    MT_WLAN_FUN_CTRL_WLAN_CLK_EN);
    else
    val &= ~(MT_WLAN_FUN_CTRL_WLAN_EN);
    mt76_wr(dev, MT_WLAN_FUN_CTRL, val);
    udelay(20);
// Note: vendor driver tries to disable/enable wlan here and retry
// but the code which does it is so buggy it must have never
// triggered, so don't bother.
//
    if (enable && !mt76_poll(dev, MT_CMB_CTRL, mask, mask, 2000))
    dev_err(dev.mt76.dev, "PLL and XTAL check failed\n");
    }
#[no_mangle]
pub unsafe extern "C" fn mt76x0_chip_onoff(dev: *mut mt76x02_dev, enable: bool, reset: bool) {
    void mt76x0_chip_onoff(struct mt76x02_dev *dev, bool enable, bool reset)
    {
    u32 val;
    val = mt76_rr(dev, MT_WLAN_FUN_CTRL);
    if (reset) {
    val |= MT_WLAN_FUN_CTRL_GPIO_OUT_EN;
    val &= ~MT_WLAN_FUN_CTRL_FRC_WL_ANT_SEL;
    if (val & MT_WLAN_FUN_CTRL_WLAN_EN) {
    val |= (MT_WLAN_FUN_CTRL_WLAN_RESET |
    MT_WLAN_FUN_CTRL_WLAN_RESET_RF);
    mt76_wr(dev, MT_WLAN_FUN_CTRL, val);
    udelay(20);
    val &= ~(MT_WLAN_FUN_CTRL_WLAN_RESET |
    MT_WLAN_FUN_CTRL_WLAN_RESET_RF);
    }
    }
    mt76_wr(dev, MT_WLAN_FUN_CTRL, val);
    udelay(20);
    mt76x0_set_wlan_state(dev, val, enable);
    }
    EXPORT_SYMBOL_GPL(mt76x0_chip_onoff);
#[no_mangle]
unsafe extern "C" fn mt76x0_reset_csr_bbp(dev: *mut mt76x02_dev) {
    static void mt76x0_reset_csr_bbp(struct mt76x02_dev *dev)
    {
    mt76_wr(dev, MT_MAC_SYS_CTRL,
    MT_MAC_SYS_CTRL_RESET_CSR |
    MT_MAC_SYS_CTRL_RESET_BBP);
    msleep(200);
    mt76_clear(dev, MT_MAC_SYS_CTRL,
    MT_MAC_SYS_CTRL_RESET_CSR |
    MT_MAC_SYS_CTRL_RESET_BBP);
    }

    mt76_wr_rp(dev, MT_MCU_MEMMAP_WLAN,	\
    tab, ARRAY_SIZE(tab))
#[no_mangle]
unsafe extern "C" fn mt76x0_init_bbp(dev: *mut mt76x02_dev) -> c_int {
    static int mt76x0_init_bbp(struct mt76x02_dev *dev)
    {
    int ret, i;
    ret = mt76x0_phy_wait_bbp_ready(dev);
    if (ret)
    return ret;
    RANDOM_WRITE(dev, mt76x0_bbp_init_tab);
    for (i = 0; i < ARRAY_SIZE(mt76x0_bbp_switch_tab); i++) {
    const struct mt76x0_bbp_switch_item *item = &mt76x0_bbp_switch_tab[i];
    const struct mt76_reg_pair *pair = &item.reg_pair;
    if (((RF_G_BAND | RF_BW_20) & item.bw_band) == (RF_G_BAND | RF_BW_20))
    mt76_wr(dev, pair.reg, pair.value);
    }
    RANDOM_WRITE(dev, mt76x0_dcoc_tab);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt76x0_init_mac_registers(dev: *mut mt76x02_dev) {
    static void mt76x0_init_mac_registers(struct mt76x02_dev *dev)
    {
    RANDOM_WRITE(dev, common_mac_reg_table);
// Enable PBF and MAC clock SYS_CTRL[11:10] = 0x3
    RANDOM_WRITE(dev, mt76x0_mac_reg_table);
// Release BBP and MAC reset MAC_SYS_CTRL[1:0] = 0x0
    mt76_clear(dev, MT_MAC_SYS_CTRL, 0x3);
// Set 0x141C[15:12]=0xF
    mt76_set(dev, MT_EXT_CCA_CFG, 0xf000);
    mt76_clear(dev, MT_FCE_L2_STUFF, MT_FCE_L2_STUFF_WR_MPDU_LEN_EN);
//
// tx_ring 9 is for mgmt frame
// tx_ring 8 is for in-band command frame.
// WMM_RG0_TXQMA: this register setting is for FCE to
// define the rule of tx_ring 9
// WMM_RG1_TXQMA: this register setting is for FCE to
// define the rule of tx_ring 8
//
    mt76_rmw(dev, MT_WMM_CTRL, 0x3ff, 0x201);
    }
#[no_mangle]
pub unsafe extern "C" fn mt76x0_mac_stop(dev: *mut mt76x02_dev) {
    void mt76x0_mac_stop(struct mt76x02_dev *dev)
    {
    let mut i: c_int = 200, ok = 0;
    mt76_clear(dev, MT_TXOP_CTRL_CFG, MT_TXOP_ED_CCA_EN);
// Page count on TxQ
    while (i-- && ((mt76_rr(dev, 0x0438) & 0xffffffff) ||
    (mt76_rr(dev, 0x0a30) & 0x000000ff) ||
    (mt76_rr(dev, 0x0a34) & 0x00ff00ff)))
    msleep(10);
    if (!mt76_poll(dev, MT_MAC_STATUS, MT_MAC_STATUS_TX, 0, 1000))
    dev_warn(dev.mt76.dev, "Warning: MAC TX did not stop!\n");
    mt76_clear(dev, MT_MAC_SYS_CTRL, MT_MAC_SYS_CTRL_ENABLE_RX |
    MT_MAC_SYS_CTRL_ENABLE_TX);
// Page count on RxQ
    for (i = 0; i < 200; i++) {
    if (!(mt76_rr(dev, MT_RXQ_STA) & 0x00ff0000) &&
    !mt76_rr(dev, 0x0a30) &&
    !mt76_rr(dev, 0x0a34)) {
    if (ok++ > 5)
    break;
    continue;
    }
    msleep(1);
    }
    if (!mt76_poll(dev, MT_MAC_STATUS, MT_MAC_STATUS_RX, 0, 1000))
    dev_warn(dev.mt76.dev, "Warning: MAC RX did not stop!\n");
    }
    EXPORT_SYMBOL_GPL(mt76x0_mac_stop);
#[no_mangle]
pub unsafe extern "C" fn mt76x0_init_hardware(dev: *mut mt76x02_dev) -> c_int {
    int mt76x0_init_hardware(struct mt76x02_dev *dev)
    {
    int ret, i, k;
    if (!mt76x02_wait_for_wpdma(&dev.mt76, 1000))
    return -EIO;
// Wait for ASIC ready after FW load.
    if (!mt76x02_wait_for_mac(&dev.mt76))
    return -ETIMEDOUT;
    mt76x0_reset_csr_bbp(dev);
    ret = mt76x02_mcu_function_select(dev, Q_SELECT, 1);
    if (ret)
    return ret;
    mt76x0_init_mac_registers(dev);
    if (!mt76x02_wait_for_txrx_idle(&dev.mt76))
    return -EIO;
    ret = mt76x0_init_bbp(dev);
    if (ret)
    return ret;
    dev.mt76.rxfilter = mt76_rr(dev, MT_RX_FILTR_CFG);
    for (i = 0; i < 16; i++)
    for (k = 0; k < 4; k++)
    mt76x02_mac_shared_key_setup(dev, i, k, core::ptr::null_mut());
    for (i = 0; i < 256; i++)
    mt76x02_mac_wcid_setup(dev, i, 0, core::ptr::null_mut());
    ret = mt76x0_eeprom_init(dev);
    if (ret)
    return ret;
    mt76x0_phy_init(dev);
    return 0;
    }
    EXPORT_SYMBOL_GPL(mt76x0_init_hardware);
    static void
    mt76x0_init_txpower(struct mt76x02_dev *dev,
    struct ieee80211_supported_band *sband)
    {
    struct ieee80211_channel *chan;
    struct mt76x02_rate_power t;
    s8 tp;
    int i;
    for (i = 0; i < sband.n_channels; i++) {
    chan = &sband.channels[i];
    mt76x0_get_tx_power_per_rate(dev, chan, &t);
    mt76x0_get_power_info(dev, chan, &tp);
    chan.orig_mpwr = (mt76x02_get_max_rate_power(&t) + tp) / 2;
    chan.max_power = min_t(int, chan.max_reg_power,
    chan.orig_mpwr);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mt76x0_register_device(dev: *mut mt76x02_dev) -> c_int {
    int mt76x0_register_device(struct mt76x02_dev *dev)
    {
    int ret;
    ret = mt76x02_init_device(dev);
    if (ret)
    return ret;
    mt76x02_config_mac_addr_list(dev);
    ret = mt76_register_device(&dev.mt76, true, mt76x02_rates,
    ARRAY_SIZE(mt76x02_rates));
    if (ret)
    return ret;
    if (dev.mphy.cap.has_5ghz) {
    struct ieee80211_supported_band *sband;
    sband = &dev.mphy.sband_5g.sband;
    sband.vht_cap.cap &= ~IEEE80211_VHT_CAP_RXLDPC;
    mt76x0_init_txpower(dev, sband);
    }
    if (dev.mphy.cap.has_2ghz)
    mt76x0_init_txpower(dev, &dev.mphy.sband_2g.sband);
    mt76x02_init_debugfs(dev);
    return 0;
    }
    EXPORT_SYMBOL_GPL(mt76x0_register_device);
