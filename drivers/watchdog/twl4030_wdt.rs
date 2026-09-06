//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/twl4030_wdt.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) Nokia Corporation
//
// Written by Timo Kokkonen <timo.t.kokkonen at nokia.com>
//

pub const TWL4030_WATCHDOG_CFG_REG_OFFS: c_uint = 0x3;
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started "
    "(default=" __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[no_mangle]
unsafe extern "C" fn twl4030_wdt_write(val: c_uchar) -> c_int {
    static int twl4030_wdt_write(unsigned char val)
    {
    return twl_i2c_write_u8(TWL_MODULE_PM_RECEIVER, val,
    TWL4030_WATCHDOG_CFG_REG_OFFS);
    }
#[no_mangle]
unsafe extern "C" fn twl4030_wdt_start(wdt: *mut watchdog_device) -> c_int {
    static int twl4030_wdt_start(struct watchdog_device *wdt)
    {
    return twl4030_wdt_write(wdt.timeout + 1);
    }
#[no_mangle]
unsafe extern "C" fn twl4030_wdt_stop(wdt: *mut watchdog_device) -> c_int {
    static int twl4030_wdt_stop(struct watchdog_device *wdt)
    {
    return twl4030_wdt_write(0);
    }
    static int twl4030_wdt_set_timeout(struct watchdog_device *wdt,
    unsigned int timeout)
    {
    wdt.timeout = timeout;
    return 0;
    }
    static const struct watchdog_info twl4030_wdt_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_MAGICCLOSE | WDIOF_KEEPALIVEPING,
    .identity = "TWL4030 Watchdog",
    };
    static const struct watchdog_ops twl4030_wdt_ops = {
    .owner		= THIS_MODULE,
    .start		= twl4030_wdt_start,
    .stop		= twl4030_wdt_stop,
    .set_timeout	= twl4030_wdt_set_timeout,
    };
#[no_mangle]
unsafe extern "C" fn twl4030_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int twl4030_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct watchdog_device *wdt;
    wdt = devm_kzalloc(dev, sizeof(*wdt), GFP_KERNEL);
    if (!wdt)
    return -ENOMEM;
    wdt.info		= &twl4030_wdt_info;
    wdt.ops		= &twl4030_wdt_ops;
    wdt.status		= 0;
    wdt.timeout		= 30;
    wdt.min_timeout	= 1;
    wdt.max_timeout	= 30;
    wdt.parent = dev;
    watchdog_set_nowayout(wdt, nowayout);
    platform_set_drvdata(pdev, wdt);
    twl4030_wdt_stop(wdt);
    return devm_watchdog_register_device(dev, wdt);
    }
#[no_mangle]
unsafe extern "C" fn twl4030_wdt_suspend(pdev: *mut platform_device, state: pm_message_t) -> c_int {
    static int twl4030_wdt_suspend(struct platform_device *pdev, pm_message_t state)
    {
    struct watchdog_device *wdt = platform_get_drvdata(pdev);
    if (watchdog_active(wdt))
    return twl4030_wdt_stop(wdt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_wdt_resume(pdev: *mut platform_device) -> c_int {
    static int twl4030_wdt_resume(struct platform_device *pdev)
    {
    struct watchdog_device *wdt = platform_get_drvdata(pdev);
    if (watchdog_active(wdt))
    return twl4030_wdt_start(wdt);
    return 0;
    }
    static const struct of_device_id twl_wdt_of_match[] = {
    { .compatible = "ti,twl4030-wdt", },
    { },
    };
    MODULE_DEVICE_TABLE(of, twl_wdt_of_match);
    static struct platform_driver twl4030_wdt_driver = {
    .probe		= twl4030_wdt_probe,
    .suspend	= pm_ptr(twl4030_wdt_suspend),
    .resume		= pm_ptr(twl4030_wdt_resume),
    .driver		= {
    .name		= "twl4030_wdt",
    .of_match_table	= twl_wdt_of_match,
    },
    };
    module_platform_driver(twl4030_wdt_driver);
    MODULE_AUTHOR("Nokia Corporation");
    MODULE_DESCRIPTION("TWL4030 Watchdog");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:twl4030_wdt");
