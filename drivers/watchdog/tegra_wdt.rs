//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/tegra_wdt.c
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
// Copyright (c) 2014, NVIDIA CORPORATION.  All rights reserved.
//

// minimum and maximum watchdog trigger timeout, in seconds
pub const MIN_WDT_TIMEOUT: c_int = 1;
pub const MAX_WDT_TIMEOUT: c_int = 255;
//
// Base of the WDT registers, from the timer base address.  There are
// actually 5 watchdogs that can be configured (by pairing with an available
// timer), at bases 0x100 + (WDT ID) * 0x20, where WDT ID is 0 through 4.
// This driver only configures the first watchdog (WDT ID 0).
//
pub const WDT_BASE: c_uint = 0x100;
pub const WDT_ID: c_int = 0;
//
// Register base of the timer that's selected for pairing with the watchdog.
// This driver arbitrarily uses timer 5, which is currently unused by
// other drivers (in particular, the Tegra clocksource driver).  If this
// needs to change, take care that the new timer is not used by the
// clocksource driver.
//
pub const WDT_TIMER_BASE: c_uint = 0x60;
pub const WDT_TIMER_ID: c_int = 5;
// WDT registers
pub const WDT_CFG: c_uint = 0x0;
pub const WDT_CFG_PERIOD_SHIFT: c_int = 4;
pub const WDT_CFG_PERIOD_MASK: c_uint = 0xff;

pub const WDT_STS: c_uint = 0x4;
pub const WDT_STS_COUNT_SHIFT: c_int = 4;
pub const WDT_STS_COUNT_MASK: c_uint = 0xff;
pub const WDT_STS_EXP_SHIFT: c_int = 12;
pub const WDT_STS_EXP_MASK: c_uint = 0x3;
pub const WDT_CMD: c_uint = 0x8;

// Timer registers
pub const TIMER_PTV: c_uint = 0x0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_wdt {
    pub wdd: watchdog_device,
    pub wdt_regs: *mut void __iomem,
    pub tmr_regs: *mut void __iomem,
}

pub const WDT_HEARTBEAT: c_int = 120;
    let mut heartbeat: static int = WDT_HEARTBEAT;
    module_param(heartbeat, int, 0);
    MODULE_PARM_DESC(heartbeat,
    "Watchdog heartbeats in seconds. (default = "
    __MODULE_STRING(WDT_HEARTBEAT) ")");
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[no_mangle]
unsafe extern "C" fn tegra_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int tegra_wdt_start(struct watchdog_device *wdd)
    {
    struct tegra_wdt *wdt = watchdog_get_drvdata(wdd);
    u32 val;
//
// This thing has a fixed 1MHz clock.  Normally, we would set the
// period to 1 second by writing 1000000ul, but the watchdog system
// reset actually occurs on the 4th expiration of this counter,
// so we set the period to 1/4 of this amount.
//
    val = 1000000ul / 4;
    val |= (TIMER_EN | TIMER_PERIODIC);
    writel(val, wdt.tmr_regs + TIMER_PTV);
//
// Set number of periods and start counter.
//
// Interrupt handler is not required for user space
// WDT accesses, since the caller is responsible to ping the
// WDT to reset the counter before expiration, through ioctls.
//
    val = WDT_TIMER_ID |
    (wdd.timeout << WDT_CFG_PERIOD_SHIFT) |
    WDT_CFG_PMC2CAR_RST_EN;
    writel(val, wdt.wdt_regs + WDT_CFG);
    writel(WDT_CMD_START_COUNTER, wdt.wdt_regs + WDT_CMD);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int tegra_wdt_stop(struct watchdog_device *wdd)
    {
    struct tegra_wdt *wdt = watchdog_get_drvdata(wdd);
    writel(WDT_UNLOCK_PATTERN, wdt.wdt_regs + WDT_UNLOCK);
    writel(WDT_CMD_DISABLE_COUNTER, wdt.wdt_regs + WDT_CMD);
    writel(0, wdt.tmr_regs + TIMER_PTV);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_wdt_ping(wdd: *mut watchdog_device) -> c_int {
    static int tegra_wdt_ping(struct watchdog_device *wdd)
    {
    struct tegra_wdt *wdt = watchdog_get_drvdata(wdd);
    writel(WDT_CMD_START_COUNTER, wdt.wdt_regs + WDT_CMD);
    return 0;
    }
    static int tegra_wdt_set_timeout(struct watchdog_device *wdd,
    unsigned int timeout)
    {
    wdd.timeout = timeout;
    if (watchdog_active(wdd)) {
    tegra_wdt_stop(wdd);
    return tegra_wdt_start(wdd);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_wdt_get_timeleft(wdd: *mut watchdog_device) -> c_uint {
    static unsigned int tegra_wdt_get_timeleft(struct watchdog_device *wdd)
    {
    struct tegra_wdt *wdt = watchdog_get_drvdata(wdd);
    u32 val;
    int count;
    int exp;
    val = readl(wdt.wdt_regs + WDT_STS);
// Current countdown (from timeout)
    count = (val >> WDT_STS_COUNT_SHIFT) & WDT_STS_COUNT_MASK;
// Number of expirations (we are waiting for the 4th expiration)
    exp = (val >> WDT_STS_EXP_SHIFT) & WDT_STS_EXP_MASK;
//
// The entire thing is divided by 4 because we are ticking down 4 times
// faster due to needing to wait for the 4th expiration.
//
    return (((3 - exp) * wdd.timeout) + count) / 4;
    }
    static const struct watchdog_info tegra_wdt_info = {
    .options	= WDIOF_SETTIMEOUT |
    WDIOF_MAGICCLOSE |
    WDIOF_KEEPALIVEPING,
    .firmware_version = 0,
    .identity	= "Tegra Watchdog",
    };
    static const struct watchdog_ops tegra_wdt_ops = {
    .owner = THIS_MODULE,
    .start = tegra_wdt_start,
    .stop = tegra_wdt_stop,
    .ping = tegra_wdt_ping,
    .set_timeout = tegra_wdt_set_timeout,
    .get_timeleft = tegra_wdt_get_timeleft,
    };
#[no_mangle]
unsafe extern "C" fn tegra_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int tegra_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct watchdog_device *wdd;
    struct tegra_wdt *wdt;
    void __iomem *regs;
    int ret;
// This is the timer base.
    regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
//
// Allocate our watchdog driver data, which has the
// struct watchdog_device nested within it.
//
    wdt = devm_kzalloc(dev, sizeof(*wdt), GFP_KERNEL);
    if (!wdt)
    return -ENOMEM;
// Initialize struct tegra_wdt.
    wdt.wdt_regs = regs + WDT_BASE;
    wdt.tmr_regs = regs + WDT_TIMER_BASE;
// Initialize struct watchdog_device.
    wdd = &wdt.wdd;
    wdd.timeout = heartbeat;
    wdd.info = &tegra_wdt_info;
    wdd.ops = &tegra_wdt_ops;
    wdd.min_timeout = MIN_WDT_TIMEOUT;
    wdd.max_timeout = MAX_WDT_TIMEOUT;
    wdd.parent = dev;
    watchdog_set_drvdata(wdd, wdt);
    watchdog_set_nowayout(wdd, nowayout);
    watchdog_stop_on_unregister(wdd);
    ret = devm_watchdog_register_device(dev, wdd);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, wdt);
    dev_info(dev, "initialized (heartbeat = %d sec, nowayout = %d)\n",
    heartbeat, nowayout);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_wdt_suspend(dev: *mut device) -> c_int {
    static int tegra_wdt_suspend(struct device *dev)
    {
    struct tegra_wdt *wdt = dev_get_drvdata(dev);
    if (watchdog_active(&wdt.wdd))
    tegra_wdt_stop(&wdt.wdd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_wdt_resume(dev: *mut device) -> c_int {
    static int tegra_wdt_resume(struct device *dev)
    {
    struct tegra_wdt *wdt = dev_get_drvdata(dev);
    if (watchdog_active(&wdt.wdd))
    tegra_wdt_start(&wdt.wdd);
    return 0;
    }
    static const struct of_device_id tegra_wdt_of_match[] = {
    { .compatible = "nvidia,tegra30-timer", },
    { },
    };
    MODULE_DEVICE_TABLE(of, tegra_wdt_of_match);
    static DEFINE_SIMPLE_DEV_PM_OPS(tegra_wdt_pm_ops,
    tegra_wdt_suspend, tegra_wdt_resume);
    static struct platform_driver tegra_wdt_driver = {
    .probe		= tegra_wdt_probe,
    .driver		= {
    .name	= "tegra-wdt",
    .pm	= pm_sleep_ptr(&tegra_wdt_pm_ops),
    .of_match_table = tegra_wdt_of_match,
    },
    };
    module_platform_driver(tegra_wdt_driver);
    MODULE_AUTHOR("NVIDIA Corporation");
    MODULE_DESCRIPTION("Tegra Watchdog Driver");
    MODULE_LICENSE("GPL v2");
