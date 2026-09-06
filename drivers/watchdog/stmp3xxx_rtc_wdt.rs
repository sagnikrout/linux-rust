//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/stmp3xxx_rtc_wdt.c
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
// Watchdog driver for the RTC based watchdog in STMP3xxx and i.MX23/28
//
// Author: Wolfram Sang <kernel@pengutronix.de>
//
// Copyright (C) 2011-12 Wolfram Sang, Pengutronix
//

pub const STMP3XXX_DEFAULT_TIMEOUT: c_int = 19;

    let mut heartbeat: static int = STMP3XXX_DEFAULT_TIMEOUT;
    module_param(heartbeat, uint, 0);
    MODULE_PARM_DESC(heartbeat, "Watchdog heartbeat period in seconds from 1 to "
    __MODULE_STRING(STMP3XXX_MAX_TIMEOUT) ", default "
    __MODULE_STRING(STMP3XXX_DEFAULT_TIMEOUT));
#[no_mangle]
unsafe extern "C" fn wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int wdt_start(struct watchdog_device *wdd)
    {
    struct device *dev = watchdog_get_drvdata(wdd);
    struct stmp3xxx_wdt_pdata *pdata = dev_get_platdata(dev);
    pdata.wdt_set_timeout(dev.parent, wdd.timeout * WDOG_TICK_RATE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int wdt_stop(struct watchdog_device *wdd)
    {
    struct device *dev = watchdog_get_drvdata(wdd);
    struct stmp3xxx_wdt_pdata *pdata = dev_get_platdata(dev);
    pdata.wdt_set_timeout(dev.parent, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wdt_set_timeout(wdd: *mut watchdog_device, new_timeout: unsigned) -> c_int {
    static int wdt_set_timeout(struct watchdog_device *wdd, unsigned new_timeout)
    {
    wdd.timeout = new_timeout;
    return wdt_start(wdd);
    }
    static const struct watchdog_info stmp3xxx_wdt_ident = {
    .options = WDIOF_MAGICCLOSE | WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING,
    .identity = "STMP3XXX RTC Watchdog",
    };
    static const struct watchdog_ops stmp3xxx_wdt_ops = {
    .owner = THIS_MODULE,
    .start = wdt_start,
    .stop = wdt_stop,
    .set_timeout = wdt_set_timeout,
    };
    static struct watchdog_device stmp3xxx_wdd = {
    .info = &stmp3xxx_wdt_ident,
    .ops = &stmp3xxx_wdt_ops,
    .min_timeout = 1,
    .max_timeout = STMP3XXX_MAX_TIMEOUT,
    .status = WATCHDOG_NOWAYOUT_INIT_STATUS,
    };
    static int wdt_notify_sys(struct notifier_block *nb, unsigned long code,
    void *unused)
    {
    switch (code) {
    case SYS_DOWN:	/* keep enabled, system might crash while going down */
    break;
    case SYS_HALT:	/* allow the system to actually halt */
    case SYS_POWER_OFF:
    wdt_stop(&stmp3xxx_wdd);
    break;
    }
    return NOTIFY_DONE;
    }
    static struct notifier_block wdt_notifier = {
    .notifier_call = wdt_notify_sys,
    };
#[no_mangle]
unsafe extern "C" fn stmp3xxx_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int stmp3xxx_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    int ret;
    watchdog_set_drvdata(&stmp3xxx_wdd, dev);
    stmp3xxx_wdd.timeout = clamp_t(unsigned, heartbeat, 1, STMP3XXX_MAX_TIMEOUT);
    stmp3xxx_wdd.parent = dev;
    ret = devm_watchdog_register_device(dev, &stmp3xxx_wdd);
    if (ret < 0)
    return ret;
    if (register_reboot_notifier(&wdt_notifier))
    dev_warn(dev, "cannot register reboot notifier\n");
    dev_info(dev, "initialized watchdog with heartbeat %ds\n",
    stmp3xxx_wdd.timeout);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stmp3xxx_wdt_remove(pdev: *mut platform_device) {
    static void stmp3xxx_wdt_remove(struct platform_device *pdev)
    {
    unregister_reboot_notifier(&wdt_notifier);
    }
#[no_mangle]
unsafe extern "C" fn stmp3xxx_wdt_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused stmp3xxx_wdt_suspend(struct device *dev)
    {
    struct watchdog_device *wdd = &stmp3xxx_wdd;
    if (watchdog_active(wdd))
    return wdt_stop(wdd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stmp3xxx_wdt_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused stmp3xxx_wdt_resume(struct device *dev)
    {
    struct watchdog_device *wdd = &stmp3xxx_wdd;
    if (watchdog_active(wdd))
    return wdt_start(wdd);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(stmp3xxx_wdt_pm_ops,
    stmp3xxx_wdt_suspend, stmp3xxx_wdt_resume);
    static struct platform_driver stmp3xxx_wdt_driver = {
    .driver = {
    .name = "stmp3xxx_rtc_wdt",
    .pm = &stmp3xxx_wdt_pm_ops,
    },
    .probe = stmp3xxx_wdt_probe,
    .remove = stmp3xxx_wdt_remove,
    };
    module_platform_driver(stmp3xxx_wdt_driver);
    MODULE_DESCRIPTION("STMP3XXX RTC Watchdog Driver");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Wolfram Sang <kernel@pengutronix.de>");
