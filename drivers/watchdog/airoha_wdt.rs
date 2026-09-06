//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/airoha_wdt.c
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
// Airoha Watchdog Driver
//
// Copyright (c) 2024, AIROHA  All rights reserved.
//
// Mayur Kumar <mayur.kumar@airoha.com>
// Christian Marangi <ansuelsmth@gmail.com>
//

// Base address of timer and watchdog registers
pub const TIMER_CTRL: c_uint = 0x0;

// Timer3 is used as Watchdog Timer

pub const WDT_TIMER_LOAD_VALUE: c_uint = 0x2c;
pub const WDT_TIMER_CUR_VALUE: c_uint = 0x30;

pub const WDT_RELOAD: c_uint = 0x38;

// Airoha watchdog structure description
#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_wdt_desc {
    pub wdog_dev: watchdog_device,
    pub wdt_freq: c_uint,
    pub base: *mut void __iomem,
}

pub const WDT_HEARTBEAT: c_int = 24;
    let mut heartbeat: static int = WDT_HEARTBEAT;
    module_param(heartbeat, int, 0);
    MODULE_PARM_DESC(heartbeat, "Watchdog heartbeats in seconds. (default="
    __MODULE_STRING(WDT_HEARTBEAT) ")");
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[no_mangle]
unsafe extern "C" fn airoha_wdt_start(wdog_dev: *mut watchdog_device) -> c_int {
    static int airoha_wdt_start(struct watchdog_device *wdog_dev)
    {
    struct airoha_wdt_desc *airoha_wdt = watchdog_get_drvdata(wdog_dev);
    u32 val;
    val = readl(airoha_wdt.base + TIMER_CTRL);
    val |= (WDT_TIMER_ENABLE | WDT_ENABLE | WDT_TIMER_INTERRUPT);
    writel(val, airoha_wdt.base + TIMER_CTRL);
    val = wdog_dev.timeout * airoha_wdt.wdt_freq;
    writel(val, airoha_wdt.base + WDT_TIMER_LOAD_VALUE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn airoha_wdt_stop(wdog_dev: *mut watchdog_device) -> c_int {
    static int airoha_wdt_stop(struct watchdog_device *wdog_dev)
    {
    struct airoha_wdt_desc *airoha_wdt = watchdog_get_drvdata(wdog_dev);
    u32 val;
    val = readl(airoha_wdt.base + TIMER_CTRL);
    val &= (~WDT_ENABLE & ~WDT_TIMER_ENABLE);
    writel(val, airoha_wdt.base + TIMER_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn airoha_wdt_ping(wdog_dev: *mut watchdog_device) -> c_int {
    static int airoha_wdt_ping(struct watchdog_device *wdog_dev)
    {
    struct airoha_wdt_desc *airoha_wdt = watchdog_get_drvdata(wdog_dev);
    u32 val;
    val = readl(airoha_wdt.base + WDT_RELOAD);
    val |= WDT_RLD;
    writel(val, airoha_wdt.base + WDT_RELOAD);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn airoha_wdt_set_timeout(wdog_dev: *mut watchdog_device, timeout: c_uint) -> c_int {
    static int airoha_wdt_set_timeout(struct watchdog_device *wdog_dev, unsigned int timeout)
    {
    wdog_dev.timeout = timeout;
    if (watchdog_active(wdog_dev)) {
    airoha_wdt_stop(wdog_dev);
    return airoha_wdt_start(wdog_dev);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn airoha_wdt_get_timeleft(wdog_dev: *mut watchdog_device) -> c_uint {
    static unsigned int airoha_wdt_get_timeleft(struct watchdog_device *wdog_dev)
    {
    struct airoha_wdt_desc *airoha_wdt = watchdog_get_drvdata(wdog_dev);
    u32 val;
    val = readl(airoha_wdt.base + WDT_TIMER_CUR_VALUE);
    return DIV_ROUND_UP(val, airoha_wdt.wdt_freq);
    }
    static const struct watchdog_info airoha_wdt_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_MAGICCLOSE | WDIOF_KEEPALIVEPING,
    .identity = "Airoha Watchdog",
    };
    static const struct watchdog_ops airoha_wdt_ops = {
    .owner = THIS_MODULE,
    .start = airoha_wdt_start,
    .stop = airoha_wdt_stop,
    .ping = airoha_wdt_ping,
    .set_timeout = airoha_wdt_set_timeout,
    .get_timeleft = airoha_wdt_get_timeleft,
    };
#[no_mangle]
unsafe extern "C" fn airoha_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int airoha_wdt_probe(struct platform_device *pdev)
    {
    struct airoha_wdt_desc *airoha_wdt;
    struct watchdog_device *wdog_dev;
    struct device *dev = &pdev.dev;
    struct clk *bus_clk;
    int ret;
    airoha_wdt = devm_kzalloc(dev, sizeof(*airoha_wdt), GFP_KERNEL);
    if (!airoha_wdt)
    return -ENOMEM;
    airoha_wdt.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(airoha_wdt.base))
    return PTR_ERR(airoha_wdt.base);
    bus_clk = devm_clk_get_enabled(dev, "bus");
    if (IS_ERR(bus_clk))
    return dev_err_probe(dev, PTR_ERR(bus_clk),
    "failed to enable bus clock\n");
// Watchdog ticks at half the bus rate
    airoha_wdt.wdt_freq = clk_get_rate(bus_clk) / 2;
    if (!airoha_wdt.wdt_freq)
    return dev_err_probe(dev, -EINVAL,
    "invalid clock frequency\n");
// Initialize struct watchdog device
    wdog_dev = &airoha_wdt.wdog_dev;
    wdog_dev.timeout = heartbeat;
    wdog_dev.info = &airoha_wdt_info;
    wdog_dev.ops = &airoha_wdt_ops;
// Bus 300MHz, watchdog 150MHz, 28 seconds
    wdog_dev.max_timeout = FIELD_MAX(WDT_TIMER_VAL) / airoha_wdt.wdt_freq;
    wdog_dev.parent = dev;
    watchdog_set_drvdata(wdog_dev, airoha_wdt);
    watchdog_set_nowayout(wdog_dev, nowayout);
    watchdog_stop_on_unregister(wdog_dev);
    ret = devm_watchdog_register_device(dev, wdog_dev);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, airoha_wdt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn airoha_wdt_suspend(dev: *mut device) -> c_int {
    static int airoha_wdt_suspend(struct device *dev)
    {
    struct airoha_wdt_desc *airoha_wdt = dev_get_drvdata(dev);
    if (watchdog_active(&airoha_wdt.wdog_dev))
    airoha_wdt_stop(&airoha_wdt.wdog_dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn airoha_wdt_resume(dev: *mut device) -> c_int {
    static int airoha_wdt_resume(struct device *dev)
    {
    struct airoha_wdt_desc *airoha_wdt = dev_get_drvdata(dev);
    if (watchdog_active(&airoha_wdt.wdog_dev)) {
    airoha_wdt_start(&airoha_wdt.wdog_dev);
    airoha_wdt_ping(&airoha_wdt.wdog_dev);
    }
    return 0;
    }
    static const struct of_device_id airoha_wdt_of_match[] = {
    { .compatible = "airoha,en7581-wdt", },
    { },
    };
    MODULE_DEVICE_TABLE(of, airoha_wdt_of_match);
    static DEFINE_SIMPLE_DEV_PM_OPS(airoha_wdt_pm_ops, airoha_wdt_suspend, airoha_wdt_resume);
    static struct platform_driver airoha_wdt_driver = {
    .probe = airoha_wdt_probe,
    .driver = {
    .name = "airoha-wdt",
    .pm = pm_sleep_ptr(&airoha_wdt_pm_ops),
    .of_match_table = airoha_wdt_of_match,
    },
    };
    module_platform_driver(airoha_wdt_driver);
    MODULE_AUTHOR("Mayur Kumar <mayur.kumar@airoha.com>");
    MODULE_AUTHOR("Christian Marangi <ansuelsmth@gmail.com>");
    MODULE_DESCRIPTION("Airoha EN7581 Watchdog Driver");
    MODULE_LICENSE("GPL");
