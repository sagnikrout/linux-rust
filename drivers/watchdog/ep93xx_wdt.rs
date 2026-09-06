//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/ep93xx_wdt.c
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
// Watchdog driver for Cirrus Logic EP93xx family of devices.
//
// Copyright (c) 2004 Ray Lehtiniemi
// Copyright (c) 2006 Tower Technologies
// Based on ep93xx driver, bits from alim7101_wdt.c
//
// Authors: Ray Lehtiniemi <rayl@mail.com>,
// Alessandro Zummo <a.zummo@towertech.it>
//
// Copyright (c) 2012 H Hartley Sweeten <hsweeten@visionengravers.com>
// Convert to a platform device and use the watchdog framework API
//
// This watchdog fires after 250msec, which is a too short interval
// for us to rely on the user space daemon alone. So we ping the
// wdt each ~200msec and eventually stop doing it if the user space
// daemon dies.
//

// default timeout (secs)
pub const WDT_TIMEOUT: c_int = 30;
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started");
    static unsigned int timeout;
    module_param(timeout, uint, 0);
    MODULE_PARM_DESC(timeout, "Watchdog timeout in seconds.");
pub const EP93XX_WATCHDOG: c_uint = 0x00;
pub const EP93XX_WDSTATUS: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep93xx_wdt_priv {
    pub mmio: *mut void __iomem,
    pub wdd: watchdog_device,
}

#[no_mangle]
unsafe extern "C" fn ep93xx_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int ep93xx_wdt_start(struct watchdog_device *wdd)
    {
    struct ep93xx_wdt_priv *priv = watchdog_get_drvdata(wdd);
    writel(0xaaaa, priv.mmio + EP93XX_WATCHDOG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int ep93xx_wdt_stop(struct watchdog_device *wdd)
    {
    struct ep93xx_wdt_priv *priv = watchdog_get_drvdata(wdd);
    writel(0xaa55, priv.mmio + EP93XX_WATCHDOG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_wdt_ping(wdd: *mut watchdog_device) -> c_int {
    static int ep93xx_wdt_ping(struct watchdog_device *wdd)
    {
    struct ep93xx_wdt_priv *priv = watchdog_get_drvdata(wdd);
    writel(0x5555, priv.mmio + EP93XX_WATCHDOG);
    return 0;
    }
    static const struct watchdog_info ep93xx_wdt_ident = {
    .options	= WDIOF_CARDRESET |
    WDIOF_SETTIMEOUT |
    WDIOF_MAGICCLOSE |
    WDIOF_KEEPALIVEPING,
    .identity	= "EP93xx Watchdog",
    };
    static const struct watchdog_ops ep93xx_wdt_ops = {
    .owner		= THIS_MODULE,
    .start		= ep93xx_wdt_start,
    .stop		= ep93xx_wdt_stop,
    .ping		= ep93xx_wdt_ping,
    };
#[no_mangle]
unsafe extern "C" fn ep93xx_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int ep93xx_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct ep93xx_wdt_priv *priv;
    struct watchdog_device *wdd;
    unsigned long val;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.mmio = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.mmio))
    return PTR_ERR(priv.mmio);
    val = readl(priv.mmio + EP93XX_WATCHDOG);
    wdd = &priv.wdd;
    wdd.bootstatus = (val & 0x01) ? WDIOF_CARDRESET : 0;
    wdd.info = &ep93xx_wdt_ident;
    wdd.ops = &ep93xx_wdt_ops;
    wdd.min_timeout = 1;
    wdd.max_hw_heartbeat_ms = 200;
    wdd.parent = dev;
    watchdog_set_nowayout(wdd, nowayout);
    wdd.timeout = WDT_TIMEOUT;
    watchdog_init_timeout(wdd, timeout, dev);
    watchdog_set_drvdata(wdd, priv);
    ret = devm_watchdog_register_device(dev, wdd);
    if (ret)
    return ret;
    dev_info(dev, "EP93XX watchdog driver %s\n",
    (val & 0x08) ? " (nCS1 disable detected)" : "");
    return 0;
    }
    static struct platform_driver ep93xx_wdt_driver = {
    .driver		= {
    .name	= "ep93xx-wdt",
    },
    .probe		= ep93xx_wdt_probe,
    };
    module_platform_driver(ep93xx_wdt_driver);
    MODULE_AUTHOR("Ray Lehtiniemi <rayl@mail.com>");
    MODULE_AUTHOR("Alessandro Zummo <a.zummo@towertech.it>");
    MODULE_AUTHOR("H Hartley Sweeten <hsweeten@visionengravers.com>");
    MODULE_DESCRIPTION("EP93xx Watchdog");
    MODULE_LICENSE("GPL");
