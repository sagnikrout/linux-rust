//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/rzn1_wdt.c
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
// Renesas RZ/N1 Watchdog timer.
// This is a 12-bit timer driver from a (62.5/16384) MHz clock. It can't even
// cope with 2 seconds.
//
// Copyright 2018 Renesas Electronics Europe Ltd.
//
// Derived from Ralink RT288x watchdog timer.
//

pub const DEFAULT_TIMEOUT: c_int = 60;
pub const RZN1_WDT_RETRIGGER: c_uint = 0x0;
pub const RZN1_WDT_RETRIGGER_RELOAD_VAL: c_int = 0;
pub const RZN1_WDT_RETRIGGER_RELOAD_VAL_MASK: c_uint = 0xfff;

pub const RZN1_WDT_PRESCALER: c_int = 16384;
pub const RZN1_WDT_MAX: c_int = 4095;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzn1_watchdog {
    pub wdtdev: watchdog_device,
    pub base: *mut void __iomem,
    pub clk_rate_khz: c_ulong,
}

#[no_mangle]
pub unsafe extern "C" fn max_heart_beat_ms(clk_rate_khz: c_ulong) -> u32 {
    static inline uint32_t max_heart_beat_ms(unsigned long clk_rate_khz)
    {
    return (RZN1_WDT_MAX * RZN1_WDT_PRESCALER) / clk_rate_khz;
    }
    static inline uint32_t compute_reload_value(uint32_t tick_ms,
    unsigned long clk_rate_khz)
    {
    return (tick_ms * clk_rate_khz) / RZN1_WDT_PRESCALER;
    }
#[no_mangle]
unsafe extern "C" fn rzn1_wdt_ping(w: *mut watchdog_device) -> c_int {
    static int rzn1_wdt_ping(struct watchdog_device *w)
    {
    struct rzn1_watchdog *wdt = watchdog_get_drvdata(w);
// Any value retriggers the watchdog
    writel(0, wdt.base + RZN1_WDT_RETRIGGER);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzn1_wdt_start(w: *mut watchdog_device) -> c_int {
    static int rzn1_wdt_start(struct watchdog_device *w)
    {
    struct rzn1_watchdog *wdt = watchdog_get_drvdata(w);
    u32 val;
//
// The hardware allows you to write to this reg only once.
// Since this includes the reload value, there is no way to change the
// timeout once started. Also note that the WDT clock is half the bus
// fabric clock rate, so if the bus fabric clock rate is changed after
// the WDT is started, the WDT interval will be wrong.
//
    val = RZN1_WDT_RETRIGGER_WDSI;
    val |= RZN1_WDT_RETRIGGER_ENABLE;
    val |= RZN1_WDT_RETRIGGER_PRESCALE;
    val |= compute_reload_value(w.max_hw_heartbeat_ms, wdt.clk_rate_khz);
    writel(val, wdt.base + RZN1_WDT_RETRIGGER);
    return 0;
    }
    static struct watchdog_info rzn1_wdt_info = {
    .identity = "RZ/N1 Watchdog",
    .options = WDIOF_MAGICCLOSE | WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING,
    };
    static const struct watchdog_ops rzn1_wdt_ops = {
    .owner = THIS_MODULE,
    .start = rzn1_wdt_start,
    .ping = rzn1_wdt_ping,
    };
#[no_mangle]
unsafe extern "C" fn rzn1_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int rzn1_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct rzn1_watchdog *wdt;
    unsigned long clk_rate;
    struct clk *clk;
    int ret;
    wdt = devm_kzalloc(dev, sizeof(*wdt), GFP_KERNEL);
    if (!wdt)
    return -ENOMEM;
    wdt.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(wdt.base))
    return PTR_ERR(wdt.base);
    clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return dev_err_probe(dev, PTR_ERR(clk), "failed to get the clock\n");
    clk_rate = clk_get_rate(clk);
    if (!clk_rate)
    return dev_err_probe(dev, -EINVAL, "failed to get the clock rate\n");
    wdt.clk_rate_khz = clk_rate / 1000;
    wdt.wdtdev.info = &rzn1_wdt_info;
    wdt.wdtdev.ops = &rzn1_wdt_ops;
    wdt.wdtdev.status = WATCHDOG_NOWAYOUT_INIT_STATUS;
    wdt.wdtdev.parent = dev;
//
// The period of the watchdog cannot be changed once set
// and is limited to a very short period.
// Configure it for a 1s period once and for all, and
// rely on the heart-beat provided by the watchdog core
// to make this usable by the user-space.
//
    wdt.wdtdev.max_hw_heartbeat_ms = max_heart_beat_ms(wdt.clk_rate_khz);
    if (wdt.wdtdev.max_hw_heartbeat_ms > 1000)
    wdt.wdtdev.max_hw_heartbeat_ms = 1000;
    wdt.wdtdev.timeout = DEFAULT_TIMEOUT;
    ret = watchdog_init_timeout(&wdt.wdtdev, 0, dev);
    if (ret)
    return ret;
    watchdog_set_drvdata(&wdt.wdtdev, wdt);
    return devm_watchdog_register_device(dev, &wdt.wdtdev);
    }
    static const struct of_device_id rzn1_wdt_match[] = {
    { .compatible = "renesas,rzn1-wdt" },
    {},
    };
    MODULE_DEVICE_TABLE(of, rzn1_wdt_match);
    static struct platform_driver rzn1_wdt_driver = {
    .probe		= rzn1_wdt_probe,
    .driver		= {
    .name		= KBUILD_MODNAME,
    .of_match_table	= rzn1_wdt_match,
    },
    };
    module_platform_driver(rzn1_wdt_driver);
    MODULE_DESCRIPTION("Renesas RZ/N1 hardware watchdog");
    MODULE_AUTHOR("Phil Edworthy <phil.edworthy@renesas.com>");
    MODULE_LICENSE("GPL");
