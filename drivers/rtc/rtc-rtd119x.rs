//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-rtd119x.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Realtek RTD129x RTC
//
// Copyright (c) 2017 Andreas Färber
//

pub const RTD_RTCSEC: c_uint = 0x00;
pub const RTD_RTCMIN: c_uint = 0x04;
pub const RTD_RTCHR: c_uint = 0x08;
pub const RTD_RTCDATE1: c_uint = 0x0c;
pub const RTD_RTCDATE2: c_uint = 0x10;
pub const RTD_RTCACR: c_uint = 0x28;
pub const RTD_RTCEN: c_uint = 0x2c;
pub const RTD_RTCCR: c_uint = 0x30;
pub const RTD_RTCSEC_RTCSEC_MASK: c_uint = 0x7f;
pub const RTD_RTCMIN_RTCMIN_MASK: c_uint = 0x3f;
pub const RTD_RTCHR_RTCHR_MASK: c_uint = 0x1f;
pub const RTD_RTCDATE1_RTCDATE1_MASK: c_uint = 0xff;
pub const RTD_RTCDATE2_RTCDATE2_MASK: c_uint = 0x7f;

pub const RTD_RTCEN_RTCEN_MASK: c_uint = 0xff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtd119x_rtc {
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub rtcdev: *mut rtc_device,
    pub base_year: c_uint,
}

#[no_mangle]
pub unsafe extern "C" fn rtd119x_rtc_days_in_year(year: c_int) -> c_int {
    static inline int rtd119x_rtc_days_in_year(int year)
    {
    return 365 + (is_leap_year(year) ? 1 : 0);
    }
#[no_mangle]
unsafe extern "C" fn rtd119x_rtc_reset(dev: *mut device) {
    static void rtd119x_rtc_reset(struct device *dev)
    {
    struct rtd119x_rtc *data = dev_get_drvdata(dev);
    u32 val;
    val = readl_relaxed(data.base + RTD_RTCCR);
    val |= RTD_RTCCR_RTCRST;
    writel_relaxed(val, data.base + RTD_RTCCR);
    val &= ~RTD_RTCCR_RTCRST;
    writel(val, data.base + RTD_RTCCR);
    }
#[no_mangle]
unsafe extern "C" fn rtd119x_rtc_set_enabled(dev: *mut device, enable: bool) {
    static void rtd119x_rtc_set_enabled(struct device *dev, bool enable)
    {
    struct rtd119x_rtc *data = dev_get_drvdata(dev);
    u32 val;
    val = readl_relaxed(data.base + RTD_RTCEN);
    if (enable) {
    if ((val & RTD_RTCEN_RTCEN_MASK) == 0x5a)
    return;
    writel_relaxed(0x5a, data.base + RTD_RTCEN);
    } else {
    writel_relaxed(0, data.base + RTD_RTCEN);
    }
    }
#[no_mangle]
unsafe extern "C" fn rtd119x_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int rtd119x_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct rtd119x_rtc *data = dev_get_drvdata(dev);
    s32 day;
    u32 sec;
    unsigned int year;
    let mut tries: c_int = 0;
    while (true) {
    tm.tm_sec = (readl_relaxed(data.base + RTD_RTCSEC) & RTD_RTCSEC_RTCSEC_MASK) >> 1;
    tm.tm_min  = readl_relaxed(data.base + RTD_RTCMIN) & RTD_RTCMIN_RTCMIN_MASK;
    tm.tm_hour = readl_relaxed(data.base + RTD_RTCHR) & RTD_RTCHR_RTCHR_MASK;
    day  =  readl_relaxed(data.base + RTD_RTCDATE1) & RTD_RTCDATE1_RTCDATE1_MASK;
    day |= (readl_relaxed(data.base + RTD_RTCDATE2) & RTD_RTCDATE2_RTCDATE2_MASK) << 8;
    sec  = (readl_relaxed(data.base + RTD_RTCSEC) & RTD_RTCSEC_RTCSEC_MASK) >> 1;
    tries++;
    if (sec == tm.tm_sec)
    break;
    if (tries >= 3)
    return -EINVAL;
    }
    if (tries > 1)
    dev_dbg(dev, "%s: needed %i tries\n", __func__, tries);
    year = data.base_year;
    while (day >= rtd119x_rtc_days_in_year(year)) {
    day -= rtd119x_rtc_days_in_year(year);
    year++;
    }
    tm.tm_year = year - 1900;
    tm.tm_yday = day;
    tm.tm_mon = 0;
    while (day >= rtc_month_days(tm.tm_mon, year)) {
    day -= rtc_month_days(tm.tm_mon, year);
    tm.tm_mon++;
    }
    tm.tm_mday = day + 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtd119x_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int rtd119x_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct rtd119x_rtc *data = dev_get_drvdata(dev);
    unsigned int day;
    int i;
    if (1900 + tm.tm_year < data.base_year)
    return -EINVAL;
    day = 0;
    for (i = data.base_year; i < 1900 + tm.tm_year; i++)
    day += rtd119x_rtc_days_in_year(i);
    day += tm.tm_yday;
    if (day > 0x7fff)
    return -EINVAL;
    rtd119x_rtc_set_enabled(dev, false);
    writel_relaxed((tm.tm_sec << 1) & RTD_RTCSEC_RTCSEC_MASK, data.base + RTD_RTCSEC);
    writel_relaxed(tm.tm_min & RTD_RTCMIN_RTCMIN_MASK, data.base + RTD_RTCMIN);
    writel_relaxed(tm.tm_hour & RTD_RTCHR_RTCHR_MASK, data.base + RTD_RTCHR);
    writel_relaxed(day & RTD_RTCDATE1_RTCDATE1_MASK, data.base + RTD_RTCDATE1);
    writel_relaxed((day >> 8) & RTD_RTCDATE2_RTCDATE2_MASK, data.base + RTD_RTCDATE2);
    rtd119x_rtc_set_enabled(dev, true);
    return 0;
    }
    static const struct rtc_class_ops rtd119x_rtc_ops = {
    .read_time	= rtd119x_rtc_read_time,
    .set_time	= rtd119x_rtc_set_time,
    };
    static const struct of_device_id rtd119x_rtc_dt_ids[] = {
    { .compatible = "realtek,rtd1295-rtc" },
    { }
    };
#[no_mangle]
unsafe extern "C" fn rtd119x_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int rtd119x_rtc_probe(struct platform_device *pdev)
    {
    struct rtd119x_rtc *data;
    u32 val;
    int ret;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    platform_set_drvdata(pdev, data);
    data.base_year = 2014;
    data.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(data.base))
    return PTR_ERR(data.base);
    data.clk = of_clk_get(pdev.dev.of_node, 0);
    if (IS_ERR(data.clk))
    return PTR_ERR(data.clk);
    ret = clk_prepare_enable(data.clk);
    if (ret) {
    clk_put(data.clk);
    return ret;
    }
    val = readl_relaxed(data.base + RTD_RTCACR);
    if (!(val & RTD_RTCACR_RTCPWR)) {
    writel_relaxed(RTD_RTCACR_RTCPWR, data.base + RTD_RTCACR);
    rtd119x_rtc_reset(&pdev.dev);
    writel_relaxed(0, data.base + RTD_RTCMIN);
    writel_relaxed(0, data.base + RTD_RTCHR);
    writel_relaxed(0, data.base + RTD_RTCDATE1);
    writel_relaxed(0, data.base + RTD_RTCDATE2);
    }
    rtd119x_rtc_set_enabled(&pdev.dev, true);
    data.rtcdev = devm_rtc_device_register(&pdev.dev, "rtc",
    &rtd119x_rtc_ops, THIS_MODULE);
    if (IS_ERR(data.rtcdev)) {
    dev_err(&pdev.dev, "failed to register rtc device");
    clk_disable_unprepare(data.clk);
    clk_put(data.clk);
    return PTR_ERR(data.rtcdev);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtd119x_rtc_remove(pdev: *mut platform_device) {
    static void rtd119x_rtc_remove(struct platform_device *pdev)
    {
    struct rtd119x_rtc *data = platform_get_drvdata(pdev);
    rtd119x_rtc_set_enabled(&pdev.dev, false);
    clk_disable_unprepare(data.clk);
    clk_put(data.clk);
    }
    static struct platform_driver rtd119x_rtc_driver = {
    .probe = rtd119x_rtc_probe,
    .remove = rtd119x_rtc_remove,
    .driver = {
    .name = "rtd1295-rtc",
    .of_match_table	= rtd119x_rtc_dt_ids,
    },
    };
    builtin_platform_driver(rtd119x_rtc_driver);
