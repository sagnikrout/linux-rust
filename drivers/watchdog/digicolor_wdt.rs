//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/digicolor_wdt.c
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
// Watchdog driver for Conexant Digicolor
//
// Copyright (C) 2015 Paradox Innovation Ltd.
//

pub const TIMER_A_CONTROL: c_int = 0;
pub const TIMER_A_COUNT: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_wdt {
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub lock: spinlock_t,
}

    static unsigned timeout;
    module_param(timeout, uint, 0);
    MODULE_PARM_DESC(timeout, "Watchdog timeout in seconds");
#[no_mangle]
unsafe extern "C" fn dc_wdt_set(wdt: *mut dc_wdt, ticks: u32) {
    static void dc_wdt_set(struct dc_wdt *wdt, u32 ticks)
    {
    unsigned long flags;
    spin_lock_irqsave(&wdt.lock, flags);
    writel_relaxed(0, wdt.base + TIMER_A_CONTROL);
    writel_relaxed(ticks, wdt.base + TIMER_A_COUNT);
    writel_relaxed(TIMER_A_ENABLE_COUNT | TIMER_A_ENABLE_WATCHDOG,
    wdt.base + TIMER_A_CONTROL);
    spin_unlock_irqrestore(&wdt.lock, flags);
    }
    static int dc_wdt_restart(struct watchdog_device *wdog, unsigned long action,
    void *data)
    {
    struct dc_wdt *wdt = watchdog_get_drvdata(wdog);
    dc_wdt_set(wdt, 1);
// wait for reset to assert...
    mdelay(500);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dc_wdt_start(wdog: *mut watchdog_device) -> c_int {
    static int dc_wdt_start(struct watchdog_device *wdog)
    {
    struct dc_wdt *wdt = watchdog_get_drvdata(wdog);
    dc_wdt_set(wdt, wdog.timeout * clk_get_rate(wdt.clk));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dc_wdt_stop(wdog: *mut watchdog_device) -> c_int {
    static int dc_wdt_stop(struct watchdog_device *wdog)
    {
    struct dc_wdt *wdt = watchdog_get_drvdata(wdog);
    writel_relaxed(0, wdt.base + TIMER_A_CONTROL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dc_wdt_set_timeout(wdog: *mut watchdog_device, t: c_uint) -> c_int {
    static int dc_wdt_set_timeout(struct watchdog_device *wdog, unsigned int t)
    {
    struct dc_wdt *wdt = watchdog_get_drvdata(wdog);
    dc_wdt_set(wdt, t * clk_get_rate(wdt.clk));
    wdog.timeout = t;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dc_wdt_get_timeleft(wdog: *mut watchdog_device) -> c_uint {
    static unsigned int dc_wdt_get_timeleft(struct watchdog_device *wdog)
    {
    struct dc_wdt *wdt = watchdog_get_drvdata(wdog);
    let mut count: u32 = readl_relaxed(wdt.base + TIMER_A_COUNT);
    return count / clk_get_rate(wdt.clk);
    }
    static const struct watchdog_ops dc_wdt_ops = {
    .owner		= THIS_MODULE,
    .start		= dc_wdt_start,
    .stop		= dc_wdt_stop,
    .set_timeout	= dc_wdt_set_timeout,
    .get_timeleft	= dc_wdt_get_timeleft,
    .restart        = dc_wdt_restart,
    };
    static const struct watchdog_info dc_wdt_info = {
    .options	= WDIOF_SETTIMEOUT | WDIOF_MAGICCLOSE
    | WDIOF_KEEPALIVEPING,
    .identity	= "Conexant Digicolor Watchdog",
    };
    static struct watchdog_device dc_wdt_wdd = {
    .info		= &dc_wdt_info,
    .ops		= &dc_wdt_ops,
    .min_timeout	= 1,
    };
#[no_mangle]
unsafe extern "C" fn dc_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int dc_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct dc_wdt *wdt;
    wdt = devm_kzalloc(dev, sizeof(struct dc_wdt), GFP_KERNEL);
    if (!wdt)
    return -ENOMEM;
    wdt.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(wdt.base))
    return PTR_ERR(wdt.base);
    wdt.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(wdt.clk))
    return PTR_ERR(wdt.clk);
    dc_wdt_wdd.max_timeout = U32_MAX / clk_get_rate(wdt.clk);
    dc_wdt_wdd.timeout = dc_wdt_wdd.max_timeout;
    dc_wdt_wdd.parent = dev;
    spin_lock_init(&wdt.lock);
    watchdog_set_drvdata(&dc_wdt_wdd, wdt);
    watchdog_set_restart_priority(&dc_wdt_wdd, 128);
    watchdog_init_timeout(&dc_wdt_wdd, timeout, dev);
    watchdog_stop_on_reboot(&dc_wdt_wdd);
    return devm_watchdog_register_device(dev, &dc_wdt_wdd);
    }
    static const struct of_device_id dc_wdt_of_match[] = {
    { .compatible = "cnxt,cx92755-wdt", },
    {},
    };
    MODULE_DEVICE_TABLE(of, dc_wdt_of_match);
    static struct platform_driver dc_wdt_driver = {
    .probe		= dc_wdt_probe,
    .driver = {
    .name =		"digicolor-wdt",
    .of_match_table = dc_wdt_of_match,
    },
    };
    module_platform_driver(dc_wdt_driver);
    MODULE_AUTHOR("Baruch Siach <baruch@tkos.co.il>");
    MODULE_DESCRIPTION("Driver for Conexant Digicolor watchdog timer");
    MODULE_LICENSE("GPL");
