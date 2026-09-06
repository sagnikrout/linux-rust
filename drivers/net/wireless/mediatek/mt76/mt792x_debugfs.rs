//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/mediatek/mt76/mt792x_debugfs.c
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
// Copyright (C) 2023 MediaTek Inc.

    static void
    mt792x_ampdu_stat_read_phy(struct mt792x_phy *phy,
    struct seq_file *file)
    {
    struct mt792x_dev *dev = file.private;
    int bound[15], range[4], i;
    if (!phy)
    return;
    mt792x_mac_update_mib_stats(phy);
// Tx ampdu stat
    for (i = 0; i < ARRAY_SIZE(range); i++)
    range[i] = mt76_rr(dev, MT_MIB_ARNG(0, i));
    for (i = 0; i < ARRAY_SIZE(bound); i++)
    bound[i] = MT_MIB_ARNCR_RANGE(range[i / 4], i % 4) + 1;
    seq_puts(file, "\nPhy0\n");
    seq_printf(file, "Length: %8d | ", bound[0]);
    for (i = 0; i < ARRAY_SIZE(bound) - 1; i++)
    seq_printf(file, "%3d  %3d | ", bound[i] + 1, bound[i + 1]);
    seq_puts(file, "\nCount:  ");
    for (i = 0; i < ARRAY_SIZE(bound); i++)
    seq_printf(file, "%8d | ", phy.mt76.aggr_stats[i]);
    seq_puts(file, "\n");
    seq_printf(file, "BA miss count: %d\n", phy.mib.ba_miss_cnt);
    }
#[no_mangle]
pub unsafe extern "C" fn mt792x_tx_stats_show(file: *mut seq_file, data: *mut c_void) -> c_int {
    int mt792x_tx_stats_show(struct seq_file *file, void *data)
    {
    struct mt792x_dev *dev = file.private;
    struct mt792x_phy *phy = &dev.phy;
    struct mt76_mib_stats *mib = &phy.mib;
    int i;
    mt792x_mutex_acquire(dev);
    mt792x_ampdu_stat_read_phy(phy, file);
    seq_puts(file, "Tx MSDU stat:\n");
    for (i = 0; i < ARRAY_SIZE(mib.tx_amsdu); i++) {
    seq_printf(file, "AMSDU pack count of %d MSDU in TXD: %8d ",
    i + 1, mib.tx_amsdu[i]);
    if (mib.tx_amsdu_cnt)
    seq_printf(file, "(%3d%%)\n",
    mib.tx_amsdu[i] * 100 / mib.tx_amsdu_cnt);
    else
    seq_puts(file, "\n");
    }
    mt792x_mutex_release(dev);
    return 0;
    }
    EXPORT_SYMBOL_GPL(mt792x_tx_stats_show);
#[no_mangle]
pub unsafe extern "C" fn mt792x_queues_acq(s: *mut seq_file, data: *mut c_void) -> c_int {
    int mt792x_queues_acq(struct seq_file *s, void *data)
    {
    struct mt792x_dev *dev = dev_get_drvdata(s.private);
    int i;
    mt792x_mutex_acquire(dev);
    for (i = 0; i < 4; i++) {
    u32 ctrl, val, qlen = 0;
    int j;
    val = mt76_rr(dev, MT_PLE_AC_QEMPTY(i));
    ctrl = BIT(31) | BIT(11) | (i << 24);
    for (j = 0; j < 32; j++) {
    if (val & BIT(j))
    continue;
    mt76_wr(dev, MT_PLE_FL_Q0_CTRL, ctrl | j);
    qlen += mt76_get_field(dev, MT_PLE_FL_Q3_CTRL,
    GENMASK(11, 0));
    }
    seq_printf(s, "AC%d: queued=%d\n", i, qlen);
    }
    mt792x_mutex_release(dev);
    return 0;
    }
    EXPORT_SYMBOL_GPL(mt792x_queues_acq);
#[no_mangle]
pub unsafe extern "C" fn mt792x_queues_read(s: *mut seq_file, data: *mut c_void) -> c_int {
    int mt792x_queues_read(struct seq_file *s, void *data)
    {
    struct mt792x_dev *dev = dev_get_drvdata(s.private);
    struct {
    struct mt76_queue *q;
    char *queue;
    } queue_map[] = {
    { dev.mphy.q_tx[MT_TXQ_BE],	 "WFDMA0" },
    { dev.mt76.q_mcu[MT_MCUQ_WM],	 "MCUWM"  },
    { dev.mt76.q_mcu[MT_MCUQ_FWDL], "MCUFWQ" },
    };
    int i;
    for (i = 0; i < ARRAY_SIZE(queue_map); i++) {
    struct mt76_queue *q = queue_map[i].q;
    if (!q)
    continue;
    seq_printf(s,
    "%s:	queued=%d head=%d tail=%d\n",
    queue_map[i].queue, q.queued, q.head,
    q.tail);
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(mt792x_queues_read);
#[no_mangle]
pub unsafe extern "C" fn mt792x_pm_stats(s: *mut seq_file, data: *mut c_void) -> c_int {
    int mt792x_pm_stats(struct seq_file *s, void *data)
    {
    struct mt792x_dev *dev = dev_get_drvdata(s.private);
    struct mt76_connac_pm *pm = &dev.pm;
    let mut awake_time: c_ulong = pm.stats.awake_time;
    let mut doze_time: c_ulong = pm.stats.doze_time;
    if (!test_bit(MT76_STATE_PM, &dev.mphy.state))
    awake_time += jiffies - pm.stats.last_wake_event;
    else
    doze_time += jiffies - pm.stats.last_doze_event;
    seq_printf(s, "awake time: %14u\ndoze time: %15u\n",
    jiffies_to_msecs(awake_time),
    jiffies_to_msecs(doze_time));
    seq_printf(s, "low power wakes: %9d\n", pm.stats.lp_wake);
    return 0;
    }
    EXPORT_SYMBOL_GPL(mt792x_pm_stats);
#[no_mangle]
pub unsafe extern "C" fn mt792x_pm_idle_timeout_set(data: *mut c_void, val: u64) -> c_int {
    int mt792x_pm_idle_timeout_set(void *data, u64 val)
    {
    struct mt792x_dev *dev = data;
    dev.pm.idle_timeout = msecs_to_jiffies(val);
    return 0;
    }
    EXPORT_SYMBOL_GPL(mt792x_pm_idle_timeout_set);
#[no_mangle]
pub unsafe extern "C" fn mt792x_pm_idle_timeout_get(data: *mut c_void, val: *mut u64) -> c_int {
    int mt792x_pm_idle_timeout_get(void *data, u64 *val)
    {
    struct mt792x_dev *dev = data;
// val = jiffies_to_msecs(dev->pm.idle_timeout);
    return 0;
    }
    EXPORT_SYMBOL_GPL(mt792x_pm_idle_timeout_get);
