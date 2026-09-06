//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/ts72xx_wdt.c
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
// Watchdog driver for Technologic Systems TS-72xx based SBCs
// (TS-7200, TS-7250 and TS-7260). These boards have external
// glue logic CPLD chip, which includes programmable watchdog
// timer.
//
// Copyright (c) 2009 Mika Westerberg <mika.westerberg@iki.fi>
//
// This driver is based on ep93xx_wdt and wm831x_wdt drivers.
//

pub const TS72XX_WDT_DEFAULT_TIMEOUT: c_int = 30;
    static int timeout;
    module_param(timeout, int, 0);
    MODULE_PARM_DESC(timeout, "Watchdog timeout in seconds.");
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Disable watchdog shutdown on close");
// priv->control_reg
pub const TS72XX_WDT_CTRL_DISABLE: c_uint = 0x00;
pub const TS72XX_WDT_CTRL_250MS: c_uint = 0x01;
pub const TS72XX_WDT_CTRL_500MS: c_uint = 0x02;
pub const TS72XX_WDT_CTRL_1SEC: c_uint = 0x03;
pub const TS72XX_WDT_CTRL_RESERVED: c_uint = 0x04;
pub const TS72XX_WDT_CTRL_2SEC: c_uint = 0x05;
pub const TS72XX_WDT_CTRL_4SEC: c_uint = 0x06;
pub const TS72XX_WDT_CTRL_8SEC: c_uint = 0x07;
// priv->feed_reg
pub const TS72XX_WDT_FEED_VAL: c_uint = 0x05;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ts72xx_wdt_priv {
    pub control_reg: *mut void __iomem,
    pub feed_reg: *mut void __iomem,
    pub wdd: watchdog_device,
    pub regval: c_uchar,
}

#[no_mangle]
unsafe extern "C" fn ts72xx_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int ts72xx_wdt_start(struct watchdog_device *wdd)
    {
    struct ts72xx_wdt_priv *priv = watchdog_get_drvdata(wdd);
    writeb(TS72XX_WDT_FEED_VAL, priv.feed_reg);
    writeb(priv.regval, priv.control_reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ts72xx_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int ts72xx_wdt_stop(struct watchdog_device *wdd)
    {
    struct ts72xx_wdt_priv *priv = watchdog_get_drvdata(wdd);
    writeb(TS72XX_WDT_FEED_VAL, priv.feed_reg);
    writeb(TS72XX_WDT_CTRL_DISABLE, priv.control_reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ts72xx_wdt_ping(wdd: *mut watchdog_device) -> c_int {
    static int ts72xx_wdt_ping(struct watchdog_device *wdd)
    {
    struct ts72xx_wdt_priv *priv = watchdog_get_drvdata(wdd);
    writeb(TS72XX_WDT_FEED_VAL, priv.feed_reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ts72xx_wdt_settimeout(wdd: *mut watchdog_device, to: c_uint) -> c_int {
    static int ts72xx_wdt_settimeout(struct watchdog_device *wdd, unsigned int to)
    {
    struct ts72xx_wdt_priv *priv = watchdog_get_drvdata(wdd);
    if (to == 1) {
    priv.regval = TS72XX_WDT_CTRL_1SEC;
    } else if (to == 2) {
    priv.regval = TS72XX_WDT_CTRL_2SEC;
    } else if (to <= 4) {
    priv.regval = TS72XX_WDT_CTRL_4SEC;
    to = 4;
    } else {
    priv.regval = TS72XX_WDT_CTRL_8SEC;
    if (to <= 8)
    to = 8;
    }
    wdd.timeout = to;
    if (watchdog_active(wdd)) {
    ts72xx_wdt_stop(wdd);
    ts72xx_wdt_start(wdd);
    }
    return 0;
    }
    static const struct watchdog_info ts72xx_wdt_ident = {
    .options		= WDIOF_KEEPALIVEPING |
    WDIOF_SETTIMEOUT |
    WDIOF_MAGICCLOSE,
    .firmware_version	= 1,
    .identity		= "TS-72XX WDT",
    };
    static const struct watchdog_ops ts72xx_wdt_ops = {
    .owner		= THIS_MODULE,
    .start		= ts72xx_wdt_start,
    .stop		= ts72xx_wdt_stop,
    .ping		= ts72xx_wdt_ping,
    .set_timeout	= ts72xx_wdt_settimeout,
    };
#[no_mangle]
unsafe extern "C" fn ts72xx_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int ts72xx_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct ts72xx_wdt_priv *priv;
    struct watchdog_device *wdd;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.control_reg = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.control_reg))
    return PTR_ERR(priv.control_reg);
    priv.feed_reg = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(priv.feed_reg))
    return PTR_ERR(priv.feed_reg);
    wdd = &priv.wdd;
    wdd.info = &ts72xx_wdt_ident;
    wdd.ops = &ts72xx_wdt_ops;
    wdd.min_timeout = 1;
    wdd.max_hw_heartbeat_ms = 8000;
    wdd.parent = dev;
    watchdog_set_nowayout(wdd, nowayout);
    wdd.timeout = TS72XX_WDT_DEFAULT_TIMEOUT;
    watchdog_init_timeout(wdd, timeout, dev);
    watchdog_set_drvdata(wdd, priv);
    ret = devm_watchdog_register_device(dev, wdd);
    if (ret)
    return ret;
    dev_info(dev, "TS-72xx Watchdog driver\n");
    return 0;
    }
    static const struct of_device_id ts72xx_wdt_of_ids[] = {
    { .compatible = "technologic,ts7200-wdt" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ts72xx_wdt_of_ids);
    static struct platform_driver ts72xx_wdt_driver = {
    .probe		= ts72xx_wdt_probe,
    .driver		= {
    .name	= "ts72xx-wdt",
    .of_match_table = ts72xx_wdt_of_ids,
    },
    };
    module_platform_driver(ts72xx_wdt_driver);
    MODULE_AUTHOR("Mika Westerberg <mika.westerberg@iki.fi>");
    MODULE_DESCRIPTION("TS-72xx SBC Watchdog");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:ts72xx-wdt");
