//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/retu_wdt.c
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
// Retu watchdog driver
//
// Copyright (C) 2004, 2005 Nokia Corporation
//
// Based on code written by Amit Kucheria and Michael Buesch.
// Rewritten by Aaro Koskinen.
//

// Watchdog timer values in seconds
pub const RETU_WDT_MAX_TIMER: c_int = 63;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct retu_wdt_dev {
    pub rdev: *mut retu_dev,
    pub dev: *mut device,
    pub ping_work: delayed_work,
}

//
// Since Retu watchdog cannot be disabled in hardware, we must kick it
// with a timer until userspace watchdog software takes over. If
// CONFIG_WATCHDOG_NOWAYOUT is set, we never start the feeding.
//
#[no_mangle]
unsafe extern "C" fn retu_wdt_ping_enable(wdev: *mut retu_wdt_dev) {
    static void retu_wdt_ping_enable(struct retu_wdt_dev *wdev)
    {
    retu_write(wdev.rdev, RETU_REG_WATCHDOG, RETU_WDT_MAX_TIMER);
    schedule_delayed_work(&wdev.ping_work,
    round_jiffies_relative(RETU_WDT_MAX_TIMER * HZ / 2));
    }
#[no_mangle]
unsafe extern "C" fn retu_wdt_ping_disable(wdev: *mut retu_wdt_dev) {
    static void retu_wdt_ping_disable(struct retu_wdt_dev *wdev)
    {
    retu_write(wdev.rdev, RETU_REG_WATCHDOG, RETU_WDT_MAX_TIMER);
    cancel_delayed_work_sync(&wdev.ping_work);
    }
#[no_mangle]
unsafe extern "C" fn retu_wdt_ping_work(work: *mut work_struct) {
    static void retu_wdt_ping_work(struct work_struct *work)
    {
    struct retu_wdt_dev *wdev = container_of(to_delayed_work(work),
    struct retu_wdt_dev, ping_work);
    retu_wdt_ping_enable(wdev);
    }
#[no_mangle]
unsafe extern "C" fn retu_wdt_start(wdog: *mut watchdog_device) -> c_int {
    static int retu_wdt_start(struct watchdog_device *wdog)
    {
    struct retu_wdt_dev *wdev = watchdog_get_drvdata(wdog);
    retu_wdt_ping_disable(wdev);
    return retu_write(wdev.rdev, RETU_REG_WATCHDOG, wdog.timeout);
    }
#[no_mangle]
unsafe extern "C" fn retu_wdt_stop(wdog: *mut watchdog_device) -> c_int {
    static int retu_wdt_stop(struct watchdog_device *wdog)
    {
    struct retu_wdt_dev *wdev = watchdog_get_drvdata(wdog);
    retu_wdt_ping_enable(wdev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn retu_wdt_ping(wdog: *mut watchdog_device) -> c_int {
    static int retu_wdt_ping(struct watchdog_device *wdog)
    {
    struct retu_wdt_dev *wdev = watchdog_get_drvdata(wdog);
    return retu_write(wdev.rdev, RETU_REG_WATCHDOG, wdog.timeout);
    }
    static int retu_wdt_set_timeout(struct watchdog_device *wdog,
    unsigned int timeout)
    {
    struct retu_wdt_dev *wdev = watchdog_get_drvdata(wdog);
    wdog.timeout = timeout;
    return retu_write(wdev.rdev, RETU_REG_WATCHDOG, wdog.timeout);
    }
    static const struct watchdog_info retu_wdt_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_MAGICCLOSE | WDIOF_KEEPALIVEPING,
    .identity = "Retu watchdog",
    };
    static const struct watchdog_ops retu_wdt_ops = {
    .owner		= THIS_MODULE,
    .start		= retu_wdt_start,
    .stop		= retu_wdt_stop,
    .ping		= retu_wdt_ping,
    .set_timeout	= retu_wdt_set_timeout,
    };
#[no_mangle]
unsafe extern "C" fn retu_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int retu_wdt_probe(struct platform_device *pdev)
    {
    struct retu_dev *rdev = dev_get_drvdata(pdev.dev.parent);
    let mut nowayout: bool = WATCHDOG_NOWAYOUT;
    struct watchdog_device *retu_wdt;
    struct retu_wdt_dev *wdev;
    int ret;
    retu_wdt = devm_kzalloc(&pdev.dev, sizeof(*retu_wdt), GFP_KERNEL);
    if (!retu_wdt)
    return -ENOMEM;
    wdev = devm_kzalloc(&pdev.dev, sizeof(*wdev), GFP_KERNEL);
    if (!wdev)
    return -ENOMEM;
    retu_wdt.info		= &retu_wdt_info;
    retu_wdt.ops		= &retu_wdt_ops;
    retu_wdt.timeout	= RETU_WDT_MAX_TIMER;
    retu_wdt.min_timeout	= 0;
    retu_wdt.max_timeout	= RETU_WDT_MAX_TIMER;
    retu_wdt.parent	= &pdev.dev;
    watchdog_set_drvdata(retu_wdt, wdev);
    watchdog_set_nowayout(retu_wdt, nowayout);
    wdev.rdev		= rdev;
    wdev.dev		= &pdev.dev;
    ret = devm_delayed_work_autocancel(&pdev.dev, &wdev.ping_work,
    retu_wdt_ping_work);
    if (ret)
    return ret;
    ret = devm_watchdog_register_device(&pdev.dev, retu_wdt);
    if (ret < 0)
    return ret;
    if (nowayout)
    retu_wdt_ping(retu_wdt);
    else
    retu_wdt_ping_enable(wdev);
    return 0;
    }
    static struct platform_driver retu_wdt_driver = {
    .probe		= retu_wdt_probe,
    .driver		= {
    .name	= "retu-wdt",
    },
    };
    module_platform_driver(retu_wdt_driver);
    MODULE_ALIAS("platform:retu-wdt");
    MODULE_DESCRIPTION("Retu watchdog");
    MODULE_AUTHOR("Amit Kucheria");
    MODULE_AUTHOR("Aaro Koskinen <aaro.koskinen@iki.fi>");
    MODULE_LICENSE("GPL");
