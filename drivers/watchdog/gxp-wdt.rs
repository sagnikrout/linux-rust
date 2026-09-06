//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/gxp-wdt.c
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
// Copyright (C) 2022 Hewlett-Packard Enterprise Development Company, L.P.

pub const MASK_WDGCS_ENABLE: c_uint = 0x01;
pub const MASK_WDGCS_RELOAD: c_uint = 0x04;
pub const MASK_WDGCS_NMIEN: c_uint = 0x08;
pub const MASK_WDGCS_WARN: c_uint = 0x80;
pub const WDT_MAX_TIMEOUT_MS: c_int = 655350;
pub const WDT_DEFAULT_TIMEOUT: c_int = 30;

pub const GXP_WDT_CNT_OFS: c_uint = 0x10;
pub const GXP_WDT_CTRL_OFS: c_uint = 0x16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gxp_wdt {
    pub base: *mut void __iomem,
    pub wdd: watchdog_device,
}

#[no_mangle]
unsafe extern "C" fn gxp_wdt_enable_reload(drvdata: *mut gxp_wdt) {
    static void gxp_wdt_enable_reload(struct gxp_wdt *drvdata)
    {
    u8 val;
    val = readb(drvdata.base + GXP_WDT_CTRL_OFS);
    val |= (MASK_WDGCS_ENABLE | MASK_WDGCS_RELOAD);
    writeb(val, drvdata.base + GXP_WDT_CTRL_OFS);
    }
#[no_mangle]
unsafe extern "C" fn gxp_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int gxp_wdt_start(struct watchdog_device *wdd)
    {
    struct gxp_wdt *drvdata = watchdog_get_drvdata(wdd);
    writew(SECS_TO_WDOG_TICKS(wdd.timeout), drvdata.base + GXP_WDT_CNT_OFS);
    gxp_wdt_enable_reload(drvdata);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gxp_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int gxp_wdt_stop(struct watchdog_device *wdd)
    {
    struct gxp_wdt *drvdata = watchdog_get_drvdata(wdd);
    u8 val;
    val = readb_relaxed(drvdata.base + GXP_WDT_CTRL_OFS);
    val &= ~MASK_WDGCS_ENABLE;
    writeb(val, drvdata.base + GXP_WDT_CTRL_OFS);
    return 0;
    }
    static int gxp_wdt_set_timeout(struct watchdog_device *wdd,
    unsigned int timeout)
    {
    struct gxp_wdt *drvdata = watchdog_get_drvdata(wdd);
    u32 actual;
    wdd.timeout = timeout;
    actual = min(timeout * 100, wdd.max_hw_heartbeat_ms / 10);
    writew(actual, drvdata.base + GXP_WDT_CNT_OFS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gxp_wdt_get_timeleft(wdd: *mut watchdog_device) -> c_uint {
    static unsigned int gxp_wdt_get_timeleft(struct watchdog_device *wdd)
    {
    struct gxp_wdt *drvdata = watchdog_get_drvdata(wdd);
    let mut val: u32 = readw(drvdata.base + GXP_WDT_CNT_OFS);
    return WDOG_TICKS_TO_SECS(val);
    }
#[no_mangle]
unsafe extern "C" fn gxp_wdt_ping(wdd: *mut watchdog_device) -> c_int {
    static int gxp_wdt_ping(struct watchdog_device *wdd)
    {
    struct gxp_wdt *drvdata = watchdog_get_drvdata(wdd);
    gxp_wdt_enable_reload(drvdata);
    return 0;
    }
    static int gxp_restart(struct watchdog_device *wdd, unsigned long action,
    void *data)
    {
    struct gxp_wdt *drvdata = watchdog_get_drvdata(wdd);
    writew(1, drvdata.base + GXP_WDT_CNT_OFS);
    gxp_wdt_enable_reload(drvdata);
    mdelay(100);
    return 0;
    }
    static const struct watchdog_ops gxp_wdt_ops = {
    .owner =	THIS_MODULE,
    .start =	gxp_wdt_start,
    .stop =		gxp_wdt_stop,
    .ping =		gxp_wdt_ping,
    .set_timeout =	gxp_wdt_set_timeout,
    .get_timeleft =	gxp_wdt_get_timeleft,
    .restart =	gxp_restart,
    };
    static const struct watchdog_info gxp_wdt_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_MAGICCLOSE | WDIOF_KEEPALIVEPING,
    .identity = "HPE GXP Watchdog timer",
    };
#[no_mangle]
unsafe extern "C" fn gxp_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int gxp_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct gxp_wdt *drvdata;
    int err;
    u8 val;
    drvdata = devm_kzalloc(dev, sizeof(struct gxp_wdt), GFP_KERNEL);
    if (!drvdata)
    return -ENOMEM;
//
// The register area where the timer and watchdog reside is disarranged.
// Hence mapping individual register blocks for the timer and watchdog
// is not recommended as they would have access to each others
// registers. Based on feedback the watchdog is no longer part of the
// device tree file and the timer driver now creates the watchdog as a
// child device. During the watchdogs creation, the timer driver passes
// the base address to the watchdog over the private interface.
//
    drvdata.base = (void __iomem *)dev.platform_data;
    drvdata.wdd.info = &gxp_wdt_info;
    drvdata.wdd.ops = &gxp_wdt_ops;
    drvdata.wdd.max_hw_heartbeat_ms = WDT_MAX_TIMEOUT_MS;
    drvdata.wdd.parent = dev;
    drvdata.wdd.timeout = WDT_DEFAULT_TIMEOUT;
    watchdog_set_drvdata(&drvdata.wdd, drvdata);
    watchdog_set_nowayout(&drvdata.wdd, WATCHDOG_NOWAYOUT);
    val = readb(drvdata.base + GXP_WDT_CTRL_OFS);
    if (val & MASK_WDGCS_ENABLE)
    set_bit(WDOG_HW_RUNNING, &drvdata.wdd.status);
    watchdog_set_restart_priority(&drvdata.wdd, 128);
    watchdog_stop_on_reboot(&drvdata.wdd);
    err = devm_watchdog_register_device(dev, &drvdata.wdd);
    if (err)
    return err;
    dev_info(dev, "HPE GXP watchdog timer");
    return 0;
    }
    static struct platform_driver gxp_wdt_driver = {
    .probe = gxp_wdt_probe,
    .driver = {
    .name =	"gxp-wdt",
    },
    };
    module_platform_driver(gxp_wdt_driver);
    MODULE_AUTHOR("Nick Hawkins <nick.hawkins@hpe.com>");
    MODULE_AUTHOR("Jean-Marie Verdun <verdun@hpe.com>");
    MODULE_DESCRIPTION("Driver for GXP watchdog timer");
    MODULE_LICENSE("GPL");
