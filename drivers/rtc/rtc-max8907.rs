//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-max8907.c
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
// RTC driver for Maxim MAX8907
//
// Copyright (c) 2011-2012, NVIDIA Corporation.
//
// Based on drivers/rtc/rtc-max8925.c,
// Copyright (C) 2009-2010 Marvell International Ltd.
//

    enum {
    RTC_SEC = 0,
    RTC_MIN,
    RTC_HOUR,
    RTC_WEEKDAY,
    RTC_DATE,
    RTC_MONTH,
    RTC_YEAR1,
    RTC_YEAR2,
    };
pub const TIME_NUM: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8907_rtc {
    pub max8907: *mut max8907,
    pub regmap: *mut regmap,
    pub rtc_dev: *mut rtc_device,
    pub irq: c_int,
}

#[no_mangle]
unsafe extern "C" fn max8907_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t max8907_irq_handler(int irq, void *data)
    {
    struct max8907_rtc *rtc = data;
    regmap_write(rtc.regmap, MAX8907_REG_ALARM0_CNTL, 0);
    rtc_update_irq(rtc.rtc_dev, 1, RTC_IRQF | RTC_AF);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn regs_to_tm(regs: *mut u8, tm: *mut rtc_time) {
    static void regs_to_tm(u8 *regs, struct rtc_time *tm)
    {
    tm.tm_year = bcd2bin(regs[RTC_YEAR2]) * 100 +
    bcd2bin(regs[RTC_YEAR1]) - 1900;
    tm.tm_mon = bcd2bin(regs[RTC_MONTH] & 0x1f) - 1;
    tm.tm_mday = bcd2bin(regs[RTC_DATE] & 0x3f);
    tm.tm_wday = (regs[RTC_WEEKDAY] & 0x07);
    if (regs[RTC_HOUR] & HOUR_12) {
    tm.tm_hour = bcd2bin(regs[RTC_HOUR] & 0x01f);
    if (tm.tm_hour == 12)
    tm.tm_hour = 0;
    if (regs[RTC_HOUR] & HOUR_AM_PM)
    tm.tm_hour += 12;
    } else {
    tm.tm_hour = bcd2bin(regs[RTC_HOUR] & 0x03f);
    }
    tm.tm_min = bcd2bin(regs[RTC_MIN] & 0x7f);
    tm.tm_sec = bcd2bin(regs[RTC_SEC] & 0x7f);
    }
#[no_mangle]
unsafe extern "C" fn tm_to_regs(tm: *mut rtc_time, regs: *mut u8) {
    static void tm_to_regs(struct rtc_time *tm, u8 *regs)
    {
    u8 high, low;
    high = (tm.tm_year + 1900) / 100;
    low = tm.tm_year % 100;
    regs[RTC_YEAR2] = bin2bcd(high);
    regs[RTC_YEAR1] = bin2bcd(low);
    regs[RTC_MONTH] = bin2bcd(tm.tm_mon + 1);
    regs[RTC_DATE] = bin2bcd(tm.tm_mday);
    regs[RTC_WEEKDAY] = tm.tm_wday;
    regs[RTC_HOUR] = bin2bcd(tm.tm_hour);
    regs[RTC_MIN] = bin2bcd(tm.tm_min);
    regs[RTC_SEC] = bin2bcd(tm.tm_sec);
    }
#[no_mangle]
unsafe extern "C" fn max8907_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int max8907_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct max8907_rtc *rtc = dev_get_drvdata(dev);
    u8 regs[TIME_NUM];
    int ret;
    ret = regmap_bulk_read(rtc.regmap, MAX8907_REG_RTC_SEC, regs,
    TIME_NUM);
    if (ret < 0)
    return ret;
    regs_to_tm(regs, tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max8907_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int max8907_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct max8907_rtc *rtc = dev_get_drvdata(dev);
    u8 regs[TIME_NUM];
    tm_to_regs(tm, regs);
    return regmap_bulk_write(rtc.regmap, MAX8907_REG_RTC_SEC, regs,
    TIME_NUM);
    }
#[no_mangle]
unsafe extern "C" fn max8907_rtc_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int max8907_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct max8907_rtc *rtc = dev_get_drvdata(dev);
    u8 regs[TIME_NUM];
    unsigned int val;
    int ret;
    ret = regmap_bulk_read(rtc.regmap, MAX8907_REG_ALARM0_SEC, regs,
    TIME_NUM);
    if (ret < 0)
    return ret;
    regs_to_tm(regs, &alrm.time);
    ret = regmap_read(rtc.regmap, MAX8907_REG_ALARM0_CNTL, &val);
    if (ret < 0)
    return ret;
    alrm.enabled = !!(val & 0x7f);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max8907_rtc_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int max8907_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct max8907_rtc *rtc = dev_get_drvdata(dev);
    u8 regs[TIME_NUM];
    int ret;
    tm_to_regs(&alrm.time, regs);
// Disable alarm while we update the target time
    ret = regmap_write(rtc.regmap, MAX8907_REG_ALARM0_CNTL, 0);
    if (ret < 0)
    return ret;
    ret = regmap_bulk_write(rtc.regmap, MAX8907_REG_ALARM0_SEC, regs,
    TIME_NUM);
    if (ret < 0)
    return ret;
    if (alrm.enabled)
    ret = regmap_write(rtc.regmap, MAX8907_REG_ALARM0_CNTL, 0x77);
    return ret;
    }
    static const struct rtc_class_ops max8907_rtc_ops = {
    .read_time	= max8907_rtc_read_time,
    .set_time	= max8907_rtc_set_time,
    .read_alarm	= max8907_rtc_read_alarm,
    .set_alarm	= max8907_rtc_set_alarm,
    };
#[no_mangle]
unsafe extern "C" fn max8907_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int max8907_rtc_probe(struct platform_device *pdev)
    {
    struct max8907 *max8907 = dev_get_drvdata(pdev.dev.parent);
    struct max8907_rtc *rtc;
    int ret;
    rtc = devm_kzalloc(&pdev.dev, sizeof(*rtc), GFP_KERNEL);
    if (!rtc)
    return -ENOMEM;
    platform_set_drvdata(pdev, rtc);
    rtc.max8907 = max8907;
    rtc.regmap = max8907.regmap_rtc;
    rtc.rtc_dev = devm_rtc_device_register(&pdev.dev, "max8907-rtc",
    &max8907_rtc_ops, THIS_MODULE);
    if (IS_ERR(rtc.rtc_dev)) {
    ret = PTR_ERR(rtc.rtc_dev);
    dev_err(&pdev.dev, "Failed to register RTC device: %d\n", ret);
    return ret;
    }
    rtc.irq = regmap_irq_get_virq(max8907.irqc_rtc,
    MAX8907_IRQ_RTC_ALARM0);
    if (rtc.irq < 0)
    return rtc.irq;
    ret = devm_request_threaded_irq(&pdev.dev, rtc.irq, core::ptr::null_mut(),
    max8907_irq_handler,
    IRQF_ONESHOT, "max8907-alarm0", rtc);
    if (ret < 0)
    dev_err(&pdev.dev, "Failed to request IRQ%d: %d\n",
    rtc.irq, ret);
    return ret;
    }
    static struct platform_driver max8907_rtc_driver = {
    .driver = {
    .name = "max8907-rtc",
    },
    .probe = max8907_rtc_probe,
    };
    module_platform_driver(max8907_rtc_driver);
    MODULE_DESCRIPTION("Maxim MAX8907 RTC driver");
    MODULE_LICENSE("GPL v2");
