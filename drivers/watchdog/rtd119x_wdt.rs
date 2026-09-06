//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/rtd119x_wdt.c
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
// Realtek RTD129x watchdog
//
// Copyright (c) 2017 Andreas Färber
//

pub const RTD119X_TCWCR: c_uint = 0x0;
pub const RTD119X_TCWTR: c_uint = 0x4;
pub const RTD119X_TCWOV: c_uint = 0xc;
pub const RTD119X_TCWCR_WDEN_DISABLED: c_uint = 0xa5;
pub const RTD119X_TCWCR_WDEN_ENABLED: c_uint = 0xff;
pub const RTD119X_TCWCR_WDEN_MASK: c_uint = 0xff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtd119x_watchdog_device {
    pub wdt_dev: watchdog_device,
    pub base: *mut void __iomem,
    pub clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn rtd119x_wdt_start(wdev: *mut watchdog_device) -> c_int {
    static int rtd119x_wdt_start(struct watchdog_device *wdev)
    {
    struct rtd119x_watchdog_device *data = watchdog_get_drvdata(wdev);
    u32 val;
    val = readl_relaxed(data.base + RTD119X_TCWCR);
    val &= ~RTD119X_TCWCR_WDEN_MASK;
    val |= RTD119X_TCWCR_WDEN_ENABLED;
    writel(val, data.base + RTD119X_TCWCR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtd119x_wdt_stop(wdev: *mut watchdog_device) -> c_int {
    static int rtd119x_wdt_stop(struct watchdog_device *wdev)
    {
    struct rtd119x_watchdog_device *data = watchdog_get_drvdata(wdev);
    u32 val;
    val = readl_relaxed(data.base + RTD119X_TCWCR);
    val &= ~RTD119X_TCWCR_WDEN_MASK;
    val |= RTD119X_TCWCR_WDEN_DISABLED;
    writel(val, data.base + RTD119X_TCWCR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtd119x_wdt_ping(wdev: *mut watchdog_device) -> c_int {
    static int rtd119x_wdt_ping(struct watchdog_device *wdev)
    {
    struct rtd119x_watchdog_device *data = watchdog_get_drvdata(wdev);
    writel_relaxed(RTD119X_TCWTR_WDCLR, data.base + RTD119X_TCWTR);
    return rtd119x_wdt_start(wdev);
    }
#[no_mangle]
unsafe extern "C" fn rtd119x_wdt_set_timeout(wdev: *mut watchdog_device, val: c_uint) -> c_int {
    static int rtd119x_wdt_set_timeout(struct watchdog_device *wdev, unsigned int val)
    {
    struct rtd119x_watchdog_device *data = watchdog_get_drvdata(wdev);
    writel(val * clk_get_rate(data.clk), data.base + RTD119X_TCWOV);
    data.wdt_dev.timeout = val;
    return 0;
    }
    static const struct watchdog_ops rtd119x_wdt_ops = {
    .owner = THIS_MODULE,
    .start		= rtd119x_wdt_start,
    .stop		= rtd119x_wdt_stop,
    .ping		= rtd119x_wdt_ping,
    .set_timeout	= rtd119x_wdt_set_timeout,
    };
    static const struct watchdog_info rtd119x_wdt_info = {
    .identity = "rtd119x-wdt",
    .options = 0,
    };
    static const struct of_device_id rtd119x_wdt_dt_ids[] = {
    { .compatible = "realtek,rtd1295-watchdog" },
    { }
    };
#[no_mangle]
unsafe extern "C" fn rtd119x_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int rtd119x_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct rtd119x_watchdog_device *data;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(data.base))
    return PTR_ERR(data.base);
    data.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(data.clk))
    return PTR_ERR(data.clk);
    data.wdt_dev.info = &rtd119x_wdt_info;
    data.wdt_dev.ops = &rtd119x_wdt_ops;
    data.wdt_dev.timeout = 120;
    data.wdt_dev.max_timeout = 0xffffffff / clk_get_rate(data.clk);
    data.wdt_dev.min_timeout = 1;
    data.wdt_dev.parent = dev;
    watchdog_stop_on_reboot(&data.wdt_dev);
    watchdog_set_drvdata(&data.wdt_dev, data);
    platform_set_drvdata(pdev, data);
    writel_relaxed(RTD119X_TCWTR_WDCLR, data.base + RTD119X_TCWTR);
    rtd119x_wdt_set_timeout(&data.wdt_dev, data.wdt_dev.timeout);
    rtd119x_wdt_stop(&data.wdt_dev);
    return devm_watchdog_register_device(dev, &data.wdt_dev);
    }
    static struct platform_driver rtd119x_wdt_driver = {
    .probe = rtd119x_wdt_probe,
    .driver = {
    .name = "rtd1295-watchdog",
    .of_match_table	= rtd119x_wdt_dt_ids,
    },
    };
    builtin_platform_driver(rtd119x_wdt_driver);
