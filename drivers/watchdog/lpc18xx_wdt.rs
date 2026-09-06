//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/lpc18xx_wdt.c
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
// NXP LPC18xx Watchdog Timer (WDT)
//
// Copyright (c) 2015 Ariel D'Alessandro <ariel@vanguardiasur.com>
//
// Notes
// -----
// The Watchdog consists of a fixed divide-by-4 clock pre-scaler and a 24-bit
// counter which decrements on every clock cycle.
//

// Registers
pub const LPC18XX_WDT_MOD: c_uint = 0x00;

pub const LPC18XX_WDT_TC: c_uint = 0x04;
pub const LPC18XX_WDT_TC_MIN: c_uint = 0xff;
pub const LPC18XX_WDT_TC_MAX: c_uint = 0xffffff;
pub const LPC18XX_WDT_FEED: c_uint = 0x08;
pub const LPC18XX_WDT_FEED_MAGIC1: c_uint = 0xaa;
pub const LPC18XX_WDT_FEED_MAGIC2: c_uint = 0x55;
pub const LPC18XX_WDT_TV: c_uint = 0x0c;
// Clock pre-scaler
pub const LPC18XX_WDT_CLK_DIV: c_int = 4;
// Timeout values in seconds

    static int heartbeat;
    module_param(heartbeat, int, 0);
    MODULE_PARM_DESC(heartbeat, "Watchdog heartbeats in seconds (default="
    __MODULE_STRING(LPC18XX_WDT_DEF_TIMEOUT) ")");
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc18xx_wdt_dev {
    pub wdt_dev: watchdog_device,
    pub reg_clk: *mut clk,
    pub wdt_clk: *mut clk,
    pub clk_rate: c_ulong,
    pub base: *mut void __iomem,
    pub timer: timer_list,
    pub lock: spinlock_t,
}

#[no_mangle]
unsafe extern "C" fn lpc18xx_wdt_feed(wdt_dev: *mut watchdog_device) -> c_int {
    static int lpc18xx_wdt_feed(struct watchdog_device *wdt_dev)
    {
    struct lpc18xx_wdt_dev *lpc18xx_wdt = watchdog_get_drvdata(wdt_dev);
    unsigned long flags;
//
// An abort condition will occur if an interrupt happens during the feed
// sequence.
//
    spin_lock_irqsave(&lpc18xx_wdt.lock, flags);
    writel(LPC18XX_WDT_FEED_MAGIC1, lpc18xx_wdt.base + LPC18XX_WDT_FEED);
    writel(LPC18XX_WDT_FEED_MAGIC2, lpc18xx_wdt.base + LPC18XX_WDT_FEED);
    spin_unlock_irqrestore(&lpc18xx_wdt.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc18xx_wdt_timer_feed(t: *mut timer_list) {
    static void lpc18xx_wdt_timer_feed(struct timer_list *t)
    {
    struct lpc18xx_wdt_dev *lpc18xx_wdt = timer_container_of(lpc18xx_wdt,
    t, timer);
    struct watchdog_device *wdt_dev = &lpc18xx_wdt.wdt_dev;
    lpc18xx_wdt_feed(wdt_dev);
// Use safe value (1/2 of real timeout)
    mod_timer(&lpc18xx_wdt.timer, jiffies +
    msecs_to_jiffies((wdt_dev.timeout * MSEC_PER_SEC) / 2));
    }
//
// Since LPC18xx Watchdog cannot be disabled in hardware, we must keep feeding
// it with a timer until userspace watchdog software takes over.
//
#[no_mangle]
unsafe extern "C" fn lpc18xx_wdt_stop(wdt_dev: *mut watchdog_device) -> c_int {
    static int lpc18xx_wdt_stop(struct watchdog_device *wdt_dev)
    {
    struct lpc18xx_wdt_dev *lpc18xx_wdt = watchdog_get_drvdata(wdt_dev);
    lpc18xx_wdt_timer_feed(&lpc18xx_wdt.timer);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __lpc18xx_wdt_set_timeout(lpc18xx_wdt: *mut lpc18xx_wdt_dev) {
    static void __lpc18xx_wdt_set_timeout(struct lpc18xx_wdt_dev *lpc18xx_wdt)
    {
    unsigned int val;
    val = DIV_ROUND_UP(lpc18xx_wdt.wdt_dev.timeout * lpc18xx_wdt.clk_rate,
    LPC18XX_WDT_CLK_DIV);
    writel(val, lpc18xx_wdt.base + LPC18XX_WDT_TC);
    }
    static int lpc18xx_wdt_set_timeout(struct watchdog_device *wdt_dev,
    unsigned int new_timeout)
    {
    struct lpc18xx_wdt_dev *lpc18xx_wdt = watchdog_get_drvdata(wdt_dev);
    lpc18xx_wdt.wdt_dev.timeout = new_timeout;
    __lpc18xx_wdt_set_timeout(lpc18xx_wdt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc18xx_wdt_get_timeleft(wdt_dev: *mut watchdog_device) -> c_uint {
    static unsigned int lpc18xx_wdt_get_timeleft(struct watchdog_device *wdt_dev)
    {
    struct lpc18xx_wdt_dev *lpc18xx_wdt = watchdog_get_drvdata(wdt_dev);
    unsigned int val;
    val = readl(lpc18xx_wdt.base + LPC18XX_WDT_TV);
    return (val * LPC18XX_WDT_CLK_DIV) / lpc18xx_wdt.clk_rate;
    }
#[no_mangle]
unsafe extern "C" fn lpc18xx_wdt_start(wdt_dev: *mut watchdog_device) -> c_int {
    static int lpc18xx_wdt_start(struct watchdog_device *wdt_dev)
    {
    struct lpc18xx_wdt_dev *lpc18xx_wdt = watchdog_get_drvdata(wdt_dev);
    unsigned int val;
    if (timer_pending(&lpc18xx_wdt.timer))
    timer_delete(&lpc18xx_wdt.timer);
    val = readl(lpc18xx_wdt.base + LPC18XX_WDT_MOD);
    val |= LPC18XX_WDT_MOD_WDEN;
    val |= LPC18XX_WDT_MOD_WDRESET;
    writel(val, lpc18xx_wdt.base + LPC18XX_WDT_MOD);
//
// Setting the WDEN bit in the WDMOD register is not sufficient to
// enable the Watchdog. A valid feed sequence must be completed after
// setting WDEN before the Watchdog is capable of generating a reset.
//
    lpc18xx_wdt_feed(wdt_dev);
    return 0;
    }
    static int lpc18xx_wdt_restart(struct watchdog_device *wdt_dev,
    unsigned long action, void *data)
    {
    struct lpc18xx_wdt_dev *lpc18xx_wdt = watchdog_get_drvdata(wdt_dev);
    unsigned long flags;
    int val;
//
// Incorrect feed sequence causes immediate watchdog reset if enabled.
//
    spin_lock_irqsave(&lpc18xx_wdt.lock, flags);
    val = readl(lpc18xx_wdt.base + LPC18XX_WDT_MOD);
    val |= LPC18XX_WDT_MOD_WDEN;
    val |= LPC18XX_WDT_MOD_WDRESET;
    writel(val, lpc18xx_wdt.base + LPC18XX_WDT_MOD);
    writel(LPC18XX_WDT_FEED_MAGIC1, lpc18xx_wdt.base + LPC18XX_WDT_FEED);
    writel(LPC18XX_WDT_FEED_MAGIC2, lpc18xx_wdt.base + LPC18XX_WDT_FEED);
    writel(LPC18XX_WDT_FEED_MAGIC1, lpc18xx_wdt.base + LPC18XX_WDT_FEED);
    writel(LPC18XX_WDT_FEED_MAGIC1, lpc18xx_wdt.base + LPC18XX_WDT_FEED);
    spin_unlock_irqrestore(&lpc18xx_wdt.lock, flags);
    return 0;
    }
    static const struct watchdog_info lpc18xx_wdt_info = {
    .identity	= "NXP LPC18xx Watchdog",
    .options	= WDIOF_SETTIMEOUT |
    WDIOF_KEEPALIVEPING |
    WDIOF_MAGICCLOSE,
    };
    static const struct watchdog_ops lpc18xx_wdt_ops = {
    .owner		= THIS_MODULE,
    .start		= lpc18xx_wdt_start,
    .stop		= lpc18xx_wdt_stop,
    .ping		= lpc18xx_wdt_feed,
    .set_timeout	= lpc18xx_wdt_set_timeout,
    .get_timeleft	= lpc18xx_wdt_get_timeleft,
    .restart        = lpc18xx_wdt_restart,
    };
#[no_mangle]
unsafe extern "C" fn lpc18xx_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int lpc18xx_wdt_probe(struct platform_device *pdev)
    {
    struct lpc18xx_wdt_dev *lpc18xx_wdt;
    struct device *dev = &pdev.dev;
    lpc18xx_wdt = devm_kzalloc(dev, sizeof(*lpc18xx_wdt), GFP_KERNEL);
    if (!lpc18xx_wdt)
    return -ENOMEM;
    lpc18xx_wdt.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(lpc18xx_wdt.base))
    return PTR_ERR(lpc18xx_wdt.base);
    lpc18xx_wdt.reg_clk = devm_clk_get_enabled(dev, "reg");
    if (IS_ERR(lpc18xx_wdt.reg_clk)) {
    dev_err(dev, "failed to get the reg clock\n");
    return PTR_ERR(lpc18xx_wdt.reg_clk);
    }
    lpc18xx_wdt.wdt_clk = devm_clk_get_enabled(dev, "wdtclk");
    if (IS_ERR(lpc18xx_wdt.wdt_clk)) {
    dev_err(dev, "failed to get the wdt clock\n");
    return PTR_ERR(lpc18xx_wdt.wdt_clk);
    }
// We use the clock rate to calculate timeouts
    lpc18xx_wdt.clk_rate = clk_get_rate(lpc18xx_wdt.wdt_clk);
    if (lpc18xx_wdt.clk_rate == 0) {
    dev_err(dev, "failed to get clock rate\n");
    return -EINVAL;
    }
    lpc18xx_wdt.wdt_dev.info = &lpc18xx_wdt_info;
    lpc18xx_wdt.wdt_dev.ops = &lpc18xx_wdt_ops;
    lpc18xx_wdt.wdt_dev.min_timeout = DIV_ROUND_UP(LPC18XX_WDT_TC_MIN *
    LPC18XX_WDT_CLK_DIV, lpc18xx_wdt.clk_rate);
    lpc18xx_wdt.wdt_dev.max_timeout = (LPC18XX_WDT_TC_MAX *
    LPC18XX_WDT_CLK_DIV) / lpc18xx_wdt.clk_rate;
    lpc18xx_wdt.wdt_dev.timeout = min(lpc18xx_wdt.wdt_dev.max_timeout,
    LPC18XX_WDT_DEF_TIMEOUT);
    spin_lock_init(&lpc18xx_wdt.lock);
    lpc18xx_wdt.wdt_dev.parent = dev;
    watchdog_set_drvdata(&lpc18xx_wdt.wdt_dev, lpc18xx_wdt);
    watchdog_init_timeout(&lpc18xx_wdt.wdt_dev, heartbeat, dev);
    __lpc18xx_wdt_set_timeout(lpc18xx_wdt);
    timer_setup(&lpc18xx_wdt.timer, lpc18xx_wdt_timer_feed, 0);
    watchdog_set_nowayout(&lpc18xx_wdt.wdt_dev, nowayout);
    watchdog_set_restart_priority(&lpc18xx_wdt.wdt_dev, 128);
    platform_set_drvdata(pdev, lpc18xx_wdt);
    watchdog_stop_on_reboot(&lpc18xx_wdt.wdt_dev);
    return devm_watchdog_register_device(dev, &lpc18xx_wdt.wdt_dev);
    }
#[no_mangle]
unsafe extern "C" fn lpc18xx_wdt_remove(pdev: *mut platform_device) {
    static void lpc18xx_wdt_remove(struct platform_device *pdev)
    {
    struct lpc18xx_wdt_dev *lpc18xx_wdt = platform_get_drvdata(pdev);
    dev_warn(&pdev.dev, "I quit now, hardware will probably reboot!\n");
    timer_delete_sync(&lpc18xx_wdt.timer);
    }
    static const struct of_device_id lpc18xx_wdt_match[] = {
    { .compatible = "nxp,lpc1850-wwdt" },
    {}
    };
    MODULE_DEVICE_TABLE(of, lpc18xx_wdt_match);
    static struct platform_driver lpc18xx_wdt_driver = {
    .driver = {
    .name = "lpc18xx-wdt",
    .of_match_table	= lpc18xx_wdt_match,
    },
    .probe = lpc18xx_wdt_probe,
    .remove = lpc18xx_wdt_remove,
    };
    module_platform_driver(lpc18xx_wdt_driver);
    MODULE_AUTHOR("Ariel D'Alessandro <ariel@vanguardiasur.com.ar>");
    MODULE_DESCRIPTION("NXP LPC18xx Watchdog Timer Driver");
    MODULE_LICENSE("GPL v2");
