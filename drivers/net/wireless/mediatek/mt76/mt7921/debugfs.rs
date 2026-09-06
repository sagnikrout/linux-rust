//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/mediatek/mt76/mt7921/debugfs.c
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
// Copyright (C) 2020 MediaTek Inc.

    static int
    mt7921_reg_set(void *data, u64 val)
    {
    struct mt792x_dev *dev = data;
    mt792x_mutex_acquire(dev);
    mt76_wr(dev, dev.mt76.debugfs_reg, val);
    mt792x_mutex_release(dev);
    return 0;
    }
    static int
    mt7921_reg_get(void *data, u64 *val)
    {
    struct mt792x_dev *dev = data;
    mt792x_mutex_acquire(dev);
// val = mt76_rr(dev, dev->mt76.debugfs_reg);
    mt792x_mutex_release(dev);
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(fops_regval, mt7921_reg_get, mt7921_reg_set,
    "0x%08llx\n");
    static int
    mt7921_fw_debug_set(void *data, u64 val)
    {
    struct mt792x_dev *dev = data;
    mt792x_mutex_acquire(dev);
    dev.fw_debug = (u8)val;
    mt7921_mcu_fw_log_2_host(dev, dev.fw_debug);
    mt792x_mutex_release(dev);
    return 0;
    }
    static int
    mt7921_fw_debug_get(void *data, u64 *val)
    {
    struct mt792x_dev *dev = data;
// val = dev->fw_debug;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(fops_fw_debug, mt7921_fw_debug_get,
    mt7921_fw_debug_set, "%lld\n");
    DEFINE_SHOW_ATTRIBUTE(mt792x_tx_stats);
    static void
    mt7921_seq_puts_array(struct seq_file *file, const char *str,
    s8 *val, int len)
    {
    int i;
    seq_printf(file, "%-16s:", str);
    for (i = 0; i < len; i++)
    if (val[i] == 127)
    seq_printf(file, " %6s", "N.A");
    else
    seq_printf(file, " %6d", val[i]);
    seq_puts(file, "\n");
    }

    ({									\
    mt7921_seq_puts_array(s, #prefix " (user)",			\
    txpwr.data[TXPWR_USER].rate,		\
    ARRAY_SIZE(txpwr.data[TXPWR_USER].rate)); \
    mt7921_seq_puts_array(s, #prefix " (eeprom)",			\
    txpwr.data[TXPWR_EEPROM].rate,		\
    ARRAY_SIZE(txpwr.data[TXPWR_EEPROM].rate)); \
    mt7921_seq_puts_array(s, #prefix " (tmac)",			\
    txpwr.data[TXPWR_MAC].rate,		\
    ARRAY_SIZE(txpwr.data[TXPWR_MAC].rate));	\
    })
    static int
    mt7921_txpwr(struct seq_file *s, void *data)
    {
    struct mt792x_dev *dev = dev_get_drvdata(s.private);
    struct mt7921_txpwr txpwr;
    int ret;
    mt792x_mutex_acquire(dev);
    ret = mt7921_get_txpwr_info(dev, &txpwr);
    mt792x_mutex_release(dev);
    if (ret)
    return ret;
    seq_printf(s, "Tx power table (channel %d)\n", txpwr.ch);
    seq_printf(s, "%-16s  %6s %6s %6s %6s\n",
    " ", "1m", "2m", "5m", "11m");
    mt7921_print_txpwr_entry(CCK, cck);
    seq_printf(s, "%-16s  %6s %6s %6s %6s %6s %6s %6s %6s\n",
    " ", "6m", "9m", "12m", "18m", "24m", "36m",
    "48m", "54m");
    mt7921_print_txpwr_entry(OFDM, ofdm);
    seq_printf(s, "%-16s  %6s %6s %6s %6s %6s %6s %6s %6s\n",
    " ", "mcs0", "mcs1", "mcs2", "mcs3", "mcs4", "mcs5",
    "mcs6", "mcs7");
    mt7921_print_txpwr_entry(HT20, ht20);
    seq_printf(s, "%-16s  %6s %6s %6s %6s %6s %6s %6s %6s %6s\n",
    " ", "mcs0", "mcs1", "mcs2", "mcs3", "mcs4", "mcs5",
    "mcs6", "mcs7", "mcs32");
    mt7921_print_txpwr_entry(HT40, ht40);
    seq_printf(s, "%-16s  %6s %6s %6s %6s %6s %6s %6s %6s %6s %6s %6s %6s\n",
    " ", "mcs0", "mcs1", "mcs2", "mcs3", "mcs4", "mcs5",
    "mcs6", "mcs7", "mcs8", "mcs9", "mcs10", "mcs11");
    mt7921_print_txpwr_entry(VHT20, vht20);
    mt7921_print_txpwr_entry(VHT40, vht40);
    mt7921_print_txpwr_entry(VHT80, vht80);
    mt7921_print_txpwr_entry(VHT160, vht160);
    mt7921_print_txpwr_entry(HE26, he26);
    mt7921_print_txpwr_entry(HE52, he52);
    mt7921_print_txpwr_entry(HE106, he106);
    mt7921_print_txpwr_entry(HE242, he242);
    mt7921_print_txpwr_entry(HE484, he484);
    mt7921_print_txpwr_entry(HE996, he996);
    mt7921_print_txpwr_entry(HE996x2, he996x2);
    return 0;
    }
    static int
    mt7921_pm_set(void *data, u64 val)
    {
    struct mt792x_dev *dev = data;
    struct mt76_connac_pm *pm = &dev.pm;
    if (mt76_is_usb(&dev.mt76))
    return -EOPNOTSUPP;
    mutex_lock(&dev.mt76.mutex);
    if (val == pm.enable_user)
    goto out;
    if (!pm.enable_user) {
    pm.stats.last_wake_event = jiffies;
    pm.stats.last_doze_event = jiffies;
    }
// make sure the chip is awake here and ps_work is scheduled
// just at end of the this routine.
//
    pm.enable = false;
    mt76_connac_pm_wake(&dev.mphy, pm);
    pm.enable_user = val;
    mt7921_set_runtime_pm(dev);
    mt76_connac_power_save_sched(&dev.mphy, pm);
    out:
    mutex_unlock(&dev.mt76.mutex);
    return 0;
    }
    static int
    mt7921_pm_get(void *data, u64 *val)
    {
    struct mt792x_dev *dev = data;
// val = dev->pm.enable_user;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(fops_pm, mt7921_pm_get, mt7921_pm_set, "%lld\n");
    static int
    mt7921_deep_sleep_set(void *data, u64 val)
    {
    struct mt792x_dev *dev = data;
    struct mt76_connac_pm *pm = &dev.pm;
    let mut monitor: bool = !!(dev.mphy.hw.conf.flags & IEEE80211_CONF_MONITOR);
    let mut enable: bool = !!val;
    if (mt76_is_usb(&dev.mt76))
    return -EOPNOTSUPP;
    mt792x_mutex_acquire(dev);
    if (pm.ds_enable_user == enable)
    goto out;
    pm.ds_enable_user = enable;
    pm.ds_enable = enable && !monitor;
    mt76_connac_mcu_set_deep_sleep(&dev.mt76, pm.ds_enable);
    out:
    mt792x_mutex_release(dev);
    return 0;
    }
    static int
    mt7921_deep_sleep_get(void *data, u64 *val)
    {
    struct mt792x_dev *dev = data;
// val = dev->pm.ds_enable_user;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(fops_ds, mt7921_deep_sleep_get,
    mt7921_deep_sleep_set, "%lld\n");
    DEFINE_DEBUGFS_ATTRIBUTE(fops_pm_idle_timeout, mt792x_pm_idle_timeout_get,
    mt792x_pm_idle_timeout_set, "%lld\n");
#[no_mangle]
unsafe extern "C" fn mt7921_chip_reset(data: *mut c_void, val: u64) -> c_int {
    static int mt7921_chip_reset(void *data, u64 val)
    {
    struct mt792x_dev *dev = data;
    let mut ret: c_int = 0;
    switch (val) {
    case 1:
// Reset wifisys directly.
    mt792x_reset(&dev.mt76);
    break;
    default:
// Collect the core dump before reset wifisys.
    mt792x_mutex_acquire(dev);
    ret = mt76_connac_mcu_chip_config(&dev.mt76);
    mt792x_mutex_release(dev);
    break;
    }
    return ret;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(fops_reset, core::ptr::null_mut(), mt7921_chip_reset, "%lld\n");
    static int
    mt7921s_sched_quota_read(struct seq_file *s, void *data)
    {
    struct mt792x_dev *dev = dev_get_drvdata(s.private);
    struct mt76_sdio *sdio = &dev.mt76.sdio;
    seq_printf(s, "pse_data_quota\t%d\n", sdio.sched.pse_data_quota);
    seq_printf(s, "ple_data_quota\t%d\n", sdio.sched.ple_data_quota);
    seq_printf(s, "pse_mcu_quota\t%d\n", sdio.sched.pse_mcu_quota);
    seq_printf(s, "sched_deficit\t%d\n", sdio.sched.deficit);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mt7921_init_debugfs(dev: *mut mt792x_dev) -> c_int {
    int mt7921_init_debugfs(struct mt792x_dev *dev)
    {
    struct dentry *dir;
    dir = mt76_register_debugfs_fops(&dev.mphy, &fops_regval);
    if (mt76_is_mmio(&dev.mt76))
    debugfs_create_devm_seqfile(dev.mt76.dev, "xmit-queues",
    dir, mt792x_queues_read);
    else
    debugfs_create_devm_seqfile(dev.mt76.dev, "xmit-queues",
    dir, mt76_queues_read);
    debugfs_create_devm_seqfile(dev.mt76.dev, "acq", dir,
    mt792x_queues_acq);
    debugfs_create_devm_seqfile(dev.mt76.dev, "txpower_sku", dir,
    mt7921_txpwr);
    debugfs_create_file("tx_stats", 0400, dir, dev, &mt792x_tx_stats_fops);
    debugfs_create_file("fw_debug", 0600, dir, dev, &fops_fw_debug);
    debugfs_create_file("runtime-pm", 0600, dir, dev, &fops_pm);
    debugfs_create_file("idle-timeout", 0600, dir, dev,
    &fops_pm_idle_timeout);
    debugfs_create_file("chip_reset", 0600, dir, dev, &fops_reset);
    debugfs_create_devm_seqfile(dev.mt76.dev, "runtime_pm_stats", dir,
    mt792x_pm_stats);
    debugfs_create_file("deep-sleep", 0600, dir, dev, &fops_ds);
    if (mt76_is_sdio(&dev.mt76))
    debugfs_create_devm_seqfile(dev.mt76.dev, "sched-quota", dir,
    mt7921s_sched_quota_read);
    return 0;
    }
