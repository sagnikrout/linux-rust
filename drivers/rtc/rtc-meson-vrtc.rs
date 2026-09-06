//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-meson-vrtc.c
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
// Copyright (C) 2019 BayLibre, SAS
// Author: Neil Armstrong <narmstrong@baylibre.com>
// Copyright (C) 2015 Amlogic, Inc. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_vrtc_data {
    pub io_alarm: *mut void __iomem,
    pub alarm_time: c_ulong,
    pub enabled: bool,
}

#[no_mangle]
unsafe extern "C" fn meson_vrtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int meson_vrtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct timespec64 time;
    dev_dbg(dev, "%s\n", __func__);
    ktime_get_real_ts64(&time);
    rtc_time64_to_tm(time.tv_sec, tm);
    return 0;
    }
    static void meson_vrtc_set_wakeup_time(struct meson_vrtc_data *vrtc,
    unsigned long time)
    {
    writel_relaxed(time, vrtc.io_alarm);
    }
#[no_mangle]
unsafe extern "C" fn meson_vrtc_set_alarm(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int meson_vrtc_set_alarm(struct device *dev, struct rtc_wkalrm *alarm)
    {
    struct meson_vrtc_data *vrtc = dev_get_drvdata(dev);
    dev_dbg(dev, "%s: alarm.enabled=%d\n", __func__, alarm.enabled);
    if (alarm.enabled)
    vrtc.alarm_time = rtc_tm_to_time64(&alarm.time);
    else
    vrtc.alarm_time = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_vrtc_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int meson_vrtc_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    struct meson_vrtc_data *vrtc = dev_get_drvdata(dev);
    vrtc.enabled = enabled;
    return 0;
    }
    static const struct rtc_class_ops meson_vrtc_ops = {
    .read_time = meson_vrtc_read_time,
    .set_alarm = meson_vrtc_set_alarm,
    .alarm_irq_enable = meson_vrtc_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn meson_vrtc_probe(pdev: *mut platform_device) -> c_int {
    static int meson_vrtc_probe(struct platform_device *pdev)
    {
    struct meson_vrtc_data *vrtc;
    struct rtc_device *rtc;
    vrtc = devm_kzalloc(&pdev.dev, sizeof(*vrtc), GFP_KERNEL);
    if (!vrtc)
    return -ENOMEM;
    vrtc.io_alarm = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(vrtc.io_alarm))
    return PTR_ERR(vrtc.io_alarm);
    device_init_wakeup(&pdev.dev, true);
    platform_set_drvdata(pdev, vrtc);
    rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    rtc.ops = &meson_vrtc_ops;
    return devm_rtc_register_device(rtc);
    }
#[no_mangle]
unsafe extern "C" fn meson_vrtc_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused meson_vrtc_suspend(struct device *dev)
    {
    struct meson_vrtc_data *vrtc = dev_get_drvdata(dev);
    dev_dbg(dev, "%s\n", __func__);
    if (vrtc.alarm_time) {
    unsigned long local_time;
    long alarm_secs;
    struct timespec64 time;
    ktime_get_real_ts64(&time);
    local_time = time.tv_sec;
    dev_dbg(dev, "alarm_time = %lus, local_time=%lus\n",
    vrtc.alarm_time, local_time);
    alarm_secs = vrtc.alarm_time - local_time;
    if (alarm_secs > 0) {
    meson_vrtc_set_wakeup_time(vrtc, alarm_secs);
    dev_dbg(dev, "system will wakeup in %lds.\n",
    alarm_secs);
    } else {
    dev_err(dev, "alarm time already passed: %lds.\n",
    alarm_secs);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_vrtc_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused meson_vrtc_resume(struct device *dev)
    {
    struct meson_vrtc_data *vrtc = dev_get_drvdata(dev);
    dev_dbg(dev, "%s\n", __func__);
    vrtc.alarm_time = 0;
    meson_vrtc_set_wakeup_time(vrtc, 0);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(meson_vrtc_pm_ops,
    meson_vrtc_suspend, meson_vrtc_resume);
    static const struct of_device_id meson_vrtc_dt_match[] = {
    { .compatible = "amlogic,meson-vrtc"},
    {},
    };
    MODULE_DEVICE_TABLE(of, meson_vrtc_dt_match);
    static struct platform_driver meson_vrtc_driver = {
    .probe = meson_vrtc_probe,
    .driver = {
    .name = "meson-vrtc",
    .of_match_table = meson_vrtc_dt_match,
    .pm = &meson_vrtc_pm_ops,
    },
    };
    module_platform_driver(meson_vrtc_driver);
    MODULE_DESCRIPTION("Amlogic Virtual Wakeup RTC Timer driver");
    MODULE_LICENSE("GPL");
