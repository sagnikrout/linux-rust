//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/bcm47xx_wdt.c
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
// Watchdog driver for Broadcom BCM47XX
//
// Copyright (C) 2008 Aleksandar Radovanovic <biblbroks@sezampro.rs>
// Copyright (C) 2009 Matthieu CASTET <castet.matthieu@free.fr>
// Copyright (C) 2012-2013 Hauke Mehrtens <hauke@hauke-m.de>
//

    let mut timeout: static int = WDT_DEFAULT_TIME;
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(timeout, int, 0);
    MODULE_PARM_DESC(timeout, "Watchdog time in seconds. (default="
    __MODULE_STRING(WDT_DEFAULT_TIME) ")");
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
    static inline struct bcm47xx_wdt *bcm47xx_wdt_get(struct watchdog_device *wdd)
    {
    return container_of(wdd, struct bcm47xx_wdt, wdd);
    }
#[no_mangle]
unsafe extern "C" fn bcm47xx_wdt_hard_keepalive(wdd: *mut watchdog_device) -> c_int {
    static int bcm47xx_wdt_hard_keepalive(struct watchdog_device *wdd)
    {
    struct bcm47xx_wdt *wdt = bcm47xx_wdt_get(wdd);
    wdt.timer_set_ms(wdt, wdd.timeout * 1000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm47xx_wdt_hard_start(wdd: *mut watchdog_device) -> c_int {
    static int bcm47xx_wdt_hard_start(struct watchdog_device *wdd)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm47xx_wdt_hard_stop(wdd: *mut watchdog_device) -> c_int {
    static int bcm47xx_wdt_hard_stop(struct watchdog_device *wdd)
    {
    struct bcm47xx_wdt *wdt = bcm47xx_wdt_get(wdd);
    wdt.timer_set(wdt, 0);
    return 0;
    }
    static int bcm47xx_wdt_hard_set_timeout(struct watchdog_device *wdd,
    unsigned int new_time)
    {
    struct bcm47xx_wdt *wdt = bcm47xx_wdt_get(wdd);
    let mut max_timer: u32 = wdt.max_timer_ms;
    if (new_time < 1 || new_time > max_timer / 1000) {
    pr_warn("timeout value must be 1<=x<=%d, using %d\n",
    max_timer / 1000, new_time);
    return -EINVAL;
    }
    wdd.timeout = new_time;
    return 0;
    }
    static int bcm47xx_wdt_restart(struct watchdog_device *wdd,
    unsigned long action, void *data)
    {
    struct bcm47xx_wdt *wdt = bcm47xx_wdt_get(wdd);
    wdt.timer_set(wdt, 1);
    return 0;
    }
    static const struct watchdog_ops bcm47xx_wdt_hard_ops = {
    .owner		= THIS_MODULE,
    .start		= bcm47xx_wdt_hard_start,
    .stop		= bcm47xx_wdt_hard_stop,
    .ping		= bcm47xx_wdt_hard_keepalive,
    .set_timeout	= bcm47xx_wdt_hard_set_timeout,
    .restart        = bcm47xx_wdt_restart,
    };
#[no_mangle]
unsafe extern "C" fn bcm47xx_wdt_soft_timer_tick(t: *mut timer_list) {
    static void bcm47xx_wdt_soft_timer_tick(struct timer_list *t)
    {
    struct bcm47xx_wdt *wdt = timer_container_of(wdt, t, soft_timer);
    let mut next_tick: u32 = min(wdt.wdd.timeout * 1000, wdt.max_timer_ms);
    if (!atomic_dec_and_test(&wdt.soft_ticks)) {
    wdt.timer_set_ms(wdt, next_tick);
    mod_timer(&wdt.soft_timer, jiffies + HZ);
    } else {
    pr_crit("Watchdog will fire soon!!!\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn bcm47xx_wdt_soft_keepalive(wdd: *mut watchdog_device) -> c_int {
    static int bcm47xx_wdt_soft_keepalive(struct watchdog_device *wdd)
    {
    struct bcm47xx_wdt *wdt = bcm47xx_wdt_get(wdd);
    atomic_set(&wdt.soft_ticks, wdd.timeout);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm47xx_wdt_soft_start(wdd: *mut watchdog_device) -> c_int {
    static int bcm47xx_wdt_soft_start(struct watchdog_device *wdd)
    {
    struct bcm47xx_wdt *wdt = bcm47xx_wdt_get(wdd);
    bcm47xx_wdt_soft_keepalive(wdd);
    bcm47xx_wdt_soft_timer_tick(&wdt.soft_timer);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm47xx_wdt_soft_stop(wdd: *mut watchdog_device) -> c_int {
    static int bcm47xx_wdt_soft_stop(struct watchdog_device *wdd)
    {
    struct bcm47xx_wdt *wdt = bcm47xx_wdt_get(wdd);
    timer_delete_sync(&wdt.soft_timer);
    wdt.timer_set(wdt, 0);
    return 0;
    }
    static int bcm47xx_wdt_soft_set_timeout(struct watchdog_device *wdd,
    unsigned int new_time)
    {
    if (new_time < 1 || new_time > WDT_SOFTTIMER_MAX) {
    pr_warn("timeout value must be 1<=x<=%d, using %d\n",
    WDT_SOFTTIMER_MAX, new_time);
    return -EINVAL;
    }
    wdd.timeout = new_time;
    return 0;
    }
    static const struct watchdog_info bcm47xx_wdt_info = {
    .identity	= DRV_NAME,
    .options	= WDIOF_SETTIMEOUT |
    WDIOF_KEEPALIVEPING |
    WDIOF_MAGICCLOSE,
    };
    static const struct watchdog_ops bcm47xx_wdt_soft_ops = {
    .owner		= THIS_MODULE,
    .start		= bcm47xx_wdt_soft_start,
    .stop		= bcm47xx_wdt_soft_stop,
    .ping		= bcm47xx_wdt_soft_keepalive,
    .set_timeout	= bcm47xx_wdt_soft_set_timeout,
    .restart        = bcm47xx_wdt_restart,
    };
#[no_mangle]
unsafe extern "C" fn bcm47xx_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int bcm47xx_wdt_probe(struct platform_device *pdev)
    {
    int ret;
    bool soft;
    struct bcm47xx_wdt *wdt = dev_get_platdata(&pdev.dev);
    if (!wdt)
    return -ENXIO;
    soft = wdt.max_timer_ms < WDT_SOFTTIMER_THRESHOLD * 1000;
    if (soft) {
    wdt.wdd.ops = &bcm47xx_wdt_soft_ops;
    timer_setup(&wdt.soft_timer, bcm47xx_wdt_soft_timer_tick, 0);
    } else {
    wdt.wdd.ops = &bcm47xx_wdt_hard_ops;
    }
    wdt.wdd.info = &bcm47xx_wdt_info;
    wdt.wdd.timeout = WDT_DEFAULT_TIME;
    wdt.wdd.parent = &pdev.dev;
    ret = wdt.wdd.ops.set_timeout(&wdt.wdd, timeout);
    if (ret)
    goto err_timer;
    watchdog_set_nowayout(&wdt.wdd, nowayout);
    watchdog_set_restart_priority(&wdt.wdd, 64);
    watchdog_stop_on_reboot(&wdt.wdd);
    ret = devm_watchdog_register_device(&pdev.dev, &wdt.wdd);
    if (ret)
    goto err_timer;
    dev_info(&pdev.dev, "BCM47xx Watchdog Timer enabled (%d seconds%s%s)\n",
    timeout, nowayout ? ", nowayout" : "",
    soft ? ", Software Timer" : "");
    return 0;
    err_timer:
    if (soft)
    timer_delete_sync(&wdt.soft_timer);
    return ret;
    }
    static struct platform_driver bcm47xx_wdt_driver = {
    .driver		= {
    .name	= "bcm47xx-wdt",
    },
    .probe		= bcm47xx_wdt_probe,
    };
    module_platform_driver(bcm47xx_wdt_driver);
    MODULE_AUTHOR("Aleksandar Radovanovic");
    MODULE_AUTHOR("Hauke Mehrtens <hauke@hauke-m.de>");
    MODULE_DESCRIPTION("Watchdog driver for Broadcom BCM47xx");
    MODULE_LICENSE("GPL");
