//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-max8925.c
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
// RTC driver for Maxim MAX8925
//
// Copyright (C) 2009-2010 Marvell International Ltd.
// Haojian Zhuang <haojian.zhuang@marvell.com>
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
pub const MAX8925_RTC_SEC: c_uint = 0x00;
pub const MAX8925_RTC_MIN: c_uint = 0x01;
pub const MAX8925_RTC_HOUR: c_uint = 0x02;
pub const MAX8925_RTC_WEEKDAY: c_uint = 0x03;
pub const MAX8925_RTC_DATE: c_uint = 0x04;
pub const MAX8925_RTC_MONTH: c_uint = 0x05;
pub const MAX8925_RTC_YEAR1: c_uint = 0x06;
pub const MAX8925_RTC_YEAR2: c_uint = 0x07;
pub const MAX8925_ALARM0_SEC: c_uint = 0x08;
pub const MAX8925_ALARM0_MIN: c_uint = 0x09;
pub const MAX8925_ALARM0_HOUR: c_uint = 0x0a;
pub const MAX8925_ALARM0_WEEKDAY: c_uint = 0x0b;
pub const MAX8925_ALARM0_DATE: c_uint = 0x0c;
pub const MAX8925_ALARM0_MON: c_uint = 0x0d;
pub const MAX8925_ALARM0_YEAR1: c_uint = 0x0e;
pub const MAX8925_ALARM0_YEAR2: c_uint = 0x0f;
pub const MAX8925_ALARM1_SEC: c_uint = 0x10;
pub const MAX8925_ALARM1_MIN: c_uint = 0x11;
pub const MAX8925_ALARM1_HOUR: c_uint = 0x12;
pub const MAX8925_ALARM1_WEEKDAY: c_uint = 0x13;
pub const MAX8925_ALARM1_DATE: c_uint = 0x14;
pub const MAX8925_ALARM1_MON: c_uint = 0x15;
pub const MAX8925_ALARM1_YEAR1: c_uint = 0x16;
pub const MAX8925_ALARM1_YEAR2: c_uint = 0x17;
pub const MAX8925_RTC_CNTL: c_uint = 0x1b;
pub const MAX8925_RTC_STATUS: c_uint = 0x20;
pub const TIME_NUM: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8925_rtc_info {
    pub rtc_dev: *mut rtc_device,
    pub chip: *mut max8925_chip,
    pub rtc: *mut i2c_client,
    pub dev: *mut device,
    pub irq: c_int,
}

#[no_mangle]
unsafe extern "C" fn rtc_update_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t rtc_update_handler(int irq, void *data)
    {
    struct max8925_rtc_info *info = (struct max8925_rtc_info *)data;
// disable ALARM0 except for 1SEC alarm
    max8925_set_bits(info.rtc, MAX8925_ALARM0_CNTL, 0x7f, 0);
    rtc_update_irq(info.rtc_dev, 1, RTC_IRQF | RTC_AF);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn tm_calc(tm: *mut rtc_time, buf: *mut c_uchar, len: c_int) -> c_int {
    static int tm_calc(struct rtc_time *tm, unsigned char *buf, int len)
    {
    if (len < TIME_NUM)
    return -EINVAL;
    tm.tm_year = (buf[RTC_YEAR2] >> 4) * 1000
    + (buf[RTC_YEAR2] & 0xf) * 100
    + (buf[RTC_YEAR1] >> 4) * 10
    + (buf[RTC_YEAR1] & 0xf);
    tm.tm_year -= 1900;
    tm.tm_mon = ((buf[RTC_MONTH] >> 4) & 0x01) * 10
    + (buf[RTC_MONTH] & 0x0f);
    tm.tm_mday = ((buf[RTC_DATE] >> 4) & 0x03) * 10
    + (buf[RTC_DATE] & 0x0f);
    tm.tm_wday = buf[RTC_WEEKDAY] & 0x07;
    if (buf[RTC_HOUR] & HOUR_12) {
    tm.tm_hour = ((buf[RTC_HOUR] >> 4) & 0x1) * 10
    + (buf[RTC_HOUR] & 0x0f);
    if (buf[RTC_HOUR] & HOUR_AM_PM)
    tm.tm_hour += 12;
    } else
    tm.tm_hour = ((buf[RTC_HOUR] >> 4) & 0x03) * 10
    + (buf[RTC_HOUR] & 0x0f);
    tm.tm_min = ((buf[RTC_MIN] >> 4) & 0x7) * 10
    + (buf[RTC_MIN] & 0x0f);
    tm.tm_sec = ((buf[RTC_SEC] >> 4) & 0x7) * 10
    + (buf[RTC_SEC] & 0x0f);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn data_calc(buf: *mut c_uchar, tm: *mut rtc_time, len: c_int) -> c_int {
    static int data_calc(unsigned char *buf, struct rtc_time *tm, int len)
    {
    unsigned char high, low;
    if (len < TIME_NUM)
    return -EINVAL;
    high = (tm.tm_year + 1900) / 1000;
    low = (tm.tm_year + 1900) / 100;
    low = low - high * 10;
    buf[RTC_YEAR2] = (high << 4) + low;
    high = (tm.tm_year + 1900) / 10;
    low = tm.tm_year + 1900;
    low = low - high * 10;
    high = high - (high / 10) * 10;
    buf[RTC_YEAR1] = (high << 4) + low;
    high = tm.tm_mon / 10;
    low = tm.tm_mon;
    low = low - high * 10;
    buf[RTC_MONTH] = (high << 4) + low;
    high = tm.tm_mday / 10;
    low = tm.tm_mday;
    low = low - high * 10;
    buf[RTC_DATE] = (high << 4) + low;
    buf[RTC_WEEKDAY] = tm.tm_wday;
    high = tm.tm_hour / 10;
    low = tm.tm_hour;
    low = low - high * 10;
    buf[RTC_HOUR] = (high << 4) + low;
    high = tm.tm_min / 10;
    low = tm.tm_min;
    low = low - high * 10;
    buf[RTC_MIN] = (high << 4) + low;
    high = tm.tm_sec / 10;
    low = tm.tm_sec;
    low = low - high * 10;
    buf[RTC_SEC] = (high << 4) + low;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max8925_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int max8925_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct max8925_rtc_info *info = dev_get_drvdata(dev);
    unsigned char buf[TIME_NUM];
    int ret;
    ret = max8925_bulk_read(info.rtc, MAX8925_RTC_SEC, TIME_NUM, buf);
    if (ret < 0)
    goto out;
    ret = tm_calc(tm, buf, TIME_NUM);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn max8925_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int max8925_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct max8925_rtc_info *info = dev_get_drvdata(dev);
    unsigned char buf[TIME_NUM];
    int ret;
    ret = data_calc(buf, tm, TIME_NUM);
    if (ret < 0)
    goto out;
    ret = max8925_bulk_write(info.rtc, MAX8925_RTC_SEC, TIME_NUM, buf);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn max8925_rtc_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int max8925_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct max8925_rtc_info *info = dev_get_drvdata(dev);
    unsigned char buf[TIME_NUM];
    int ret;
    ret = max8925_bulk_read(info.rtc, MAX8925_ALARM0_SEC, TIME_NUM, buf);
    if (ret < 0)
    goto out;
    ret = tm_calc(&alrm.time, buf, TIME_NUM);
    if (ret < 0)
    goto out;
    ret = max8925_reg_read(info.rtc, MAX8925_RTC_IRQ_MASK);
    if (ret < 0)
    goto out;
    if (ret & ALARM0_IRQ) {
    alrm.enabled = 0;
    } else {
    ret = max8925_reg_read(info.rtc, MAX8925_ALARM0_CNTL);
    if (ret < 0)
    goto out;
    if (!ret)
    alrm.enabled = 0;
    else
    alrm.enabled = 1;
    }
    ret = max8925_reg_read(info.rtc, MAX8925_RTC_STATUS);
    if (ret < 0)
    goto out;
    if (ret & ALARM0_STATUS)
    alrm.pending = 1;
    else
    alrm.pending = 0;
    return 0;
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn max8925_rtc_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int max8925_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct max8925_rtc_info *info = dev_get_drvdata(dev);
    unsigned char buf[TIME_NUM];
    int ret;
    ret = data_calc(buf, &alrm.time, TIME_NUM);
    if (ret < 0)
    goto out;
    ret = max8925_bulk_write(info.rtc, MAX8925_ALARM0_SEC, TIME_NUM, buf);
    if (ret < 0)
    goto out;
    if (alrm.enabled)
// only enable alarm on year/month/day/hour/min/sec
    ret = max8925_reg_write(info.rtc, MAX8925_ALARM0_CNTL, 0x77);
    else
    ret = max8925_reg_write(info.rtc, MAX8925_ALARM0_CNTL, 0x0);
    out:
    return ret;
    }
    static const struct rtc_class_ops max8925_rtc_ops = {
    .read_time	= max8925_rtc_read_time,
    .set_time	= max8925_rtc_set_time,
    .read_alarm	= max8925_rtc_read_alarm,
    .set_alarm	= max8925_rtc_set_alarm,
    };
#[no_mangle]
unsafe extern "C" fn max8925_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int max8925_rtc_probe(struct platform_device *pdev)
    {
    struct max8925_chip *chip = dev_get_drvdata(pdev.dev.parent);
    struct max8925_rtc_info *info;
    int ret;
    info = devm_kzalloc(&pdev.dev, sizeof(struct max8925_rtc_info),
    GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    info.chip = chip;
    info.rtc = chip.rtc;
    info.dev = &pdev.dev;
    info.irq = platform_get_irq(pdev, 0);
    ret = devm_request_threaded_irq(&pdev.dev, info.irq, core::ptr::null_mut(),
    rtc_update_handler, IRQF_ONESHOT,
    "rtc-alarm0", info);
    if (ret < 0) {
    dev_err(chip.dev, "Failed to request IRQ: #%d: %d\n",
    info.irq, ret);
    return ret;
    }
    dev_set_drvdata(&pdev.dev, info);
// XXX - isn't this redundant?
    platform_set_drvdata(pdev, info);
    device_init_wakeup(&pdev.dev, true);
    info.rtc_dev = devm_rtc_device_register(&pdev.dev, "max8925-rtc",
    &max8925_rtc_ops, THIS_MODULE);
    ret = PTR_ERR(info.rtc_dev);
    if (IS_ERR(info.rtc_dev)) {
    dev_err(&pdev.dev, "Failed to register RTC device: %d\n", ret);
    return ret;
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn max8925_rtc_suspend(dev: *mut device) -> c_int {
    static int max8925_rtc_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct max8925_chip *chip = dev_get_drvdata(pdev.dev.parent);
    if (device_may_wakeup(dev))
    chip.wakeup_flag |= 1 << MAX8925_IRQ_RTC_ALARM0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max8925_rtc_resume(dev: *mut device) -> c_int {
    static int max8925_rtc_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct max8925_chip *chip = dev_get_drvdata(pdev.dev.parent);
    if (device_may_wakeup(dev))
    chip.wakeup_flag &= ~(1 << MAX8925_IRQ_RTC_ALARM0);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(max8925_rtc_pm_ops, max8925_rtc_suspend, max8925_rtc_resume);
    static struct platform_driver max8925_rtc_driver = {
    .driver		= {
    .name	= "max8925-rtc",
    .pm     = &max8925_rtc_pm_ops,
    },
    .probe		= max8925_rtc_probe,
    };
    module_platform_driver(max8925_rtc_driver);
    MODULE_DESCRIPTION("Maxim MAX8925 RTC driver");
    MODULE_AUTHOR("Haojian Zhuang <haojian.zhuang@marvell.com>");
    MODULE_LICENSE("GPL");
