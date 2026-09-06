//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/db8500_wdt.c
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
// Copyright (C) ST-Ericsson SA 2011-2013
//
// Author: Mathieu Poirier <mathieu.poirier@linaro.org> for ST-Ericsson
// Author: Jonas Aaberg <jonas.aberg@stericsson.com> for ST-Ericsson
//

pub const WATCHDOG_MIN: c_int = 0;

    let mut timeout: static unsigned int = WATCHDOG_TIMEOUT;
    module_param(timeout, uint, 0);
    MODULE_PARM_DESC(timeout,
    "Watchdog timeout in seconds. default="
    __MODULE_STRING(WATCHDOG_TIMEOUT) ".");
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[no_mangle]
unsafe extern "C" fn db8500_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int db8500_wdt_start(struct watchdog_device *wdd)
    {
    return db8500_prcmu_enable_a9wdog(PRCMU_WDOG_ALL);
    }
#[no_mangle]
unsafe extern "C" fn db8500_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int db8500_wdt_stop(struct watchdog_device *wdd)
    {
    return db8500_prcmu_disable_a9wdog(PRCMU_WDOG_ALL);
    }
#[no_mangle]
unsafe extern "C" fn db8500_wdt_keepalive(wdd: *mut watchdog_device) -> c_int {
    static int db8500_wdt_keepalive(struct watchdog_device *wdd)
    {
    return db8500_prcmu_kick_a9wdog(PRCMU_WDOG_ALL);
    }
    static int db8500_wdt_set_timeout(struct watchdog_device *wdd,
    unsigned int timeout)
    {
    db8500_wdt_stop(wdd);
    db8500_prcmu_load_a9wdog(PRCMU_WDOG_ALL, timeout * 1000);
    db8500_wdt_start(wdd);
    return 0;
    }
    static const struct watchdog_info db8500_wdt_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE,
    .identity = "DB8500 WDT",
    .firmware_version = 1,
    };
    static const struct watchdog_ops db8500_wdt_ops = {
    .owner = THIS_MODULE,
    .start = db8500_wdt_start,
    .stop  = db8500_wdt_stop,
    .ping  = db8500_wdt_keepalive,
    .set_timeout = db8500_wdt_set_timeout,
    };
    static struct watchdog_device db8500_wdt = {
    .info = &db8500_wdt_info,
    .ops = &db8500_wdt_ops,
    .min_timeout = WATCHDOG_MIN,
    .max_timeout = WATCHDOG_MAX28,
    };
#[no_mangle]
unsafe extern "C" fn db8500_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int db8500_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    int ret;
    timeout = 600; /* Default to 10 minutes */
    db8500_wdt.parent = dev;
    watchdog_set_nowayout(&db8500_wdt, nowayout);
// disable auto off on sleep
    db8500_prcmu_config_a9wdog(PRCMU_WDOG_CPU1, false);
// set HW initial value
    db8500_prcmu_load_a9wdog(PRCMU_WDOG_ALL, timeout * 1000);
    ret = devm_watchdog_register_device(dev, &db8500_wdt);
    if (ret)
    return ret;
    dev_info(dev, "initialized\n");
    return 0;
    }
    static int db8500_wdt_suspend(struct platform_device *pdev,
    pm_message_t state)
    {
    if (watchdog_active(&db8500_wdt)) {
    db8500_wdt_stop(&db8500_wdt);
    db8500_prcmu_config_a9wdog(PRCMU_WDOG_CPU1, true);
    db8500_prcmu_load_a9wdog(PRCMU_WDOG_ALL, timeout * 1000);
    db8500_wdt_start(&db8500_wdt);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn db8500_wdt_resume(pdev: *mut platform_device) -> c_int {
    static int db8500_wdt_resume(struct platform_device *pdev)
    {
    if (watchdog_active(&db8500_wdt)) {
    db8500_wdt_stop(&db8500_wdt);
    db8500_prcmu_config_a9wdog(PRCMU_WDOG_CPU1, false);
    db8500_prcmu_load_a9wdog(PRCMU_WDOG_ALL, timeout * 1000);
    db8500_wdt_start(&db8500_wdt);
    }
    return 0;
    }
    static struct platform_driver db8500_wdt_driver = {
    .probe		= db8500_wdt_probe,
    .suspend	= pm_ptr(db8500_wdt_suspend),
    .resume		= pm_ptr(db8500_wdt_resume),
    .driver		= {
    .name	= "db8500_wdt",
    },
    };
    module_platform_driver(db8500_wdt_driver);
    MODULE_AUTHOR("Jonas Aaberg <jonas.aberg@stericsson.com>");
    MODULE_DESCRIPTION("DB8500 Watchdog Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:db8500_wdt");
