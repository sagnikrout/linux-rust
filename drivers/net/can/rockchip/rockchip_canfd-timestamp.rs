//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/rockchip/rockchip_canfd-timestamp.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2023, 2024 Pengutronix,
// Marc Kleine-Budde <kernel@pengutronix.de>
//

#[no_mangle]
unsafe extern "C" fn rkcanfd_timestamp_read(cc: *mut cyclecounter) -> u64 {
    static u64 rkcanfd_timestamp_read(struct cyclecounter *cc)
    {
    const struct rkcanfd_priv *priv = container_of(cc, struct rkcanfd_priv, cc);
    return rkcanfd_get_timestamp(priv);
    }
    void rkcanfd_skb_set_timestamp(const struct rkcanfd_priv *priv,
    struct sk_buff *skb, const u32 timestamp)
    {
    struct skb_shared_hwtstamps *hwtstamps = skb_hwtstamps(skb);
    u64 ns;
    ns = timecounter_cyc2time(&priv.tc, timestamp);
    hwtstamps.hwtstamp = ns_to_ktime(ns);
    }
#[no_mangle]
unsafe extern "C" fn rkcanfd_timestamp_work(work: *mut work_struct) {
    static void rkcanfd_timestamp_work(struct work_struct *work)
    {
    const struct delayed_work *delayed_work = to_delayed_work(work);
    struct rkcanfd_priv *priv;
    priv = container_of(delayed_work, struct rkcanfd_priv, timestamp);
    timecounter_read(&priv.tc);
    schedule_delayed_work(&priv.timestamp, priv.work_delay_jiffies);
    }
#[no_mangle]
pub unsafe extern "C" fn rkcanfd_timestamp_init(priv: *mut rkcanfd_priv) {
    void rkcanfd_timestamp_init(struct rkcanfd_priv *priv)
    {
    const struct can_bittiming *dbt = &priv.can.fd.data_bittiming;
    const struct can_bittiming *bt = &priv.can.bittiming;
    struct cyclecounter *cc = &priv.cc;
    u32 bitrate, div, reg, rate;
    u64 work_delay_ns;
    u64 max_cycles;
// At the standard clock rate of 300Mhz on the rk3658, the 32
// bit timer overflows every 14s. This means that we have to
// poll it quite often to avoid missing a wrap around.
//
// Divide it down to a reasonable rate, at least twice the bit
// rate.
//
    bitrate = max(bt.bitrate, dbt.bitrate);
    div = min(DIV_ROUND_UP(priv.can.clock.freq, bitrate * 2),
    FIELD_MAX(RKCANFD_REG_TIMESTAMP_CTRL_TIME_BASE_COUNTER_PRESCALE) + 1);
    reg = FIELD_PREP(RKCANFD_REG_TIMESTAMP_CTRL_TIME_BASE_COUNTER_PRESCALE,
    div - 1) |
    RKCANFD_REG_TIMESTAMP_CTRL_TIME_BASE_COUNTER_ENABLE;
    rkcanfd_write(priv, RKCANFD_REG_TIMESTAMP_CTRL, reg);
    cc.read = rkcanfd_timestamp_read;
    cc.mask = CYCLECOUNTER_MASK(32);
    rate = priv.can.clock.freq / div;
    clocks_calc_mult_shift(&cc.mult, &cc.shift, rate, NSEC_PER_SEC,
    RKCANFD_TIMESTAMP_WORK_MAX_DELAY_SEC);
    max_cycles = div_u64(ULLONG_MAX, cc.mult);
    max_cycles = min(max_cycles, cc.mask);
    work_delay_ns = clocksource_cyc2ns(max_cycles, cc.mult, cc.shift);
    priv.work_delay_jiffies = div_u64(work_delay_ns, 3u * NSEC_PER_SEC / HZ);
    INIT_DELAYED_WORK(&priv.timestamp, rkcanfd_timestamp_work);
    netdev_dbg(priv.ndev, "clock=%lu.%02luMHz bitrate=%lu.%02luMBit/s div=%u rate=%lu.%02luMHz mult=%u shift=%u delay=%lus\n",
    priv.can.clock.freq / MEGA,
    priv.can.clock.freq % MEGA / KILO / 10,
    bitrate / MEGA,
    bitrate % MEGA / KILO / 100,
    div,
    rate / MEGA,
    rate % MEGA / KILO / 10,
    cc.mult, cc.shift,
    priv.work_delay_jiffies / HZ);
    }
#[no_mangle]
pub unsafe extern "C" fn rkcanfd_timestamp_start(priv: *mut rkcanfd_priv) {
    void rkcanfd_timestamp_start(struct rkcanfd_priv *priv)
    {
    timecounter_init(&priv.tc, &priv.cc, ktime_get_real_ns());
    schedule_delayed_work(&priv.timestamp, priv.work_delay_jiffies);
    }
#[no_mangle]
pub unsafe extern "C" fn rkcanfd_timestamp_stop(priv: *mut rkcanfd_priv) {
    void rkcanfd_timestamp_stop(struct rkcanfd_priv *priv)
    {
    cancel_delayed_work(&priv.timestamp);
    }
#[no_mangle]
pub unsafe extern "C" fn rkcanfd_timestamp_stop_sync(priv: *mut rkcanfd_priv) {
    void rkcanfd_timestamp_stop_sync(struct rkcanfd_priv *priv)
    {
    cancel_delayed_work_sync(&priv.timestamp);
    }
