//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/max63xx_wdt.c
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
// drivers/char/watchdog/max63xx_wdt.c
//
// Driver for max63{69,70,71,72,73,74} watchdog timers
//
// Copyright (C) 2009 Marc Zyngier <maz@misterjones.org>
//
// This driver assumes the watchdog pins are memory mapped (as it is
// the case for the Arcom Zeus). Should it be connected over GPIOs or
// another interface, some abstraction will have to be introduced.
//

pub const DEFAULT_HEARTBEAT: c_int = 60;
pub const MAX_HEARTBEAT: c_int = 60;
    let mut heartbeat: static unsigned int = DEFAULT_HEARTBEAT;
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
//
// Memory mapping: a single byte, 3 first lower bits to select bit 3
// to ping the watchdog.
//

pub const MAX6369_WDSET_DISABLED: c_int = 3;
    static int nodelay;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max63xx_wdt {
    pub wdd: watchdog_device,
    pub timeout: *const max63xx_timeout,
// memory mapping
    pub base: *mut void __iomem,
    pub lock: spinlock_t,
// WDI and WSET bits write access routines
    pub wdt): *mut *mut void (ping)(struct max63xx_wdt,
    pub set): *mut *mut *mut void (set)(struct max63xx_wdt wdt, u8,
}

//
// The timeout values used are actually the absolute minimum the chip
// offers. Typical values on my board are slightly over twice as long
// (10s setting ends up with a 25s timeout), and can be up to 3 times
// the nominal setting (according to the datasheet). So please take
// these values with a grain of salt. Same goes for the initial delay
// "feature". Only max6373/74 have a few settings without this initial
// delay (selected with the "nodelay" parameter).
//
// I also decided to remove from the tables any timeout smaller than a
// second, as it looked completly overkill...
//
// Timeouts in second
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max63xx_timeout {
    pub wdset: u8,
    pub tdelay: u8,
    pub twd: u8,
}

    static const struct max63xx_timeout max6369_table[] = {
    { 5,  1,  1 },
    { 6, 10, 10 },
    { 7, 60, 60 },
    { },
    };
    static const struct max63xx_timeout max6371_table[] = {
    { 6, 60,  3 },
    { 7, 60, 60 },
    { },
    };
    static const struct max63xx_timeout max6373_table[] = {
    { 2, 60,  1 },
    { 5,  0,  1 },
    { 1,  3,  3 },
    { 7, 60, 10 },
    { 6,  0, 10 },
    { },
    };
    static const struct max63xx_timeout *
    max63xx_select_timeout(const struct max63xx_timeout *table, int value)
    {
    while (table.twd) {
    if (value <= table.twd) {
    if (nodelay && table.tdelay == 0)
    return table;
    if (!nodelay)
    return table;
    }
    table++;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn max63xx_wdt_ping(wdd: *mut watchdog_device) -> c_int {
    static int max63xx_wdt_ping(struct watchdog_device *wdd)
    {
    struct max63xx_wdt *wdt = watchdog_get_drvdata(wdd);
    wdt.ping(wdt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max63xx_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int max63xx_wdt_start(struct watchdog_device *wdd)
    {
    struct max63xx_wdt *wdt = watchdog_get_drvdata(wdd);
    wdt.set(wdt, wdt.timeout.wdset);
// check for a edge triggered startup
    if (wdt.timeout.tdelay == 0)
    wdt.ping(wdt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max63xx_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int max63xx_wdt_stop(struct watchdog_device *wdd)
    {
    struct max63xx_wdt *wdt = watchdog_get_drvdata(wdd);
    wdt.set(wdt, MAX6369_WDSET_DISABLED);
    return 0;
    }
    static const struct watchdog_ops max63xx_wdt_ops = {
    .owner = THIS_MODULE,
    .start = max63xx_wdt_start,
    .stop = max63xx_wdt_stop,
    .ping = max63xx_wdt_ping,
    };
    static const struct watchdog_info max63xx_wdt_info = {
    .options = WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE,
    .identity = "max63xx Watchdog",
    };
#[no_mangle]
unsafe extern "C" fn max63xx_mmap_ping(wdt: *mut max63xx_wdt) {
    static void max63xx_mmap_ping(struct max63xx_wdt *wdt)
    {
    u8 val;
    spin_lock(&wdt.lock);
    val = __raw_readb(wdt.base);
    __raw_writeb(val | MAX6369_WDI, wdt.base);
    __raw_writeb(val & ~MAX6369_WDI, wdt.base);
    spin_unlock(&wdt.lock);
    }
#[no_mangle]
unsafe extern "C" fn max63xx_mmap_set(wdt: *mut max63xx_wdt, set: u8) {
    static void max63xx_mmap_set(struct max63xx_wdt *wdt, u8 set)
    {
    u8 val;
    spin_lock(&wdt.lock);
    val = __raw_readb(wdt.base);
    val &= ~MAX6369_WDSET;
    val |= set & MAX6369_WDSET;
    __raw_writeb(val, wdt.base);
    spin_unlock(&wdt.lock);
    }
#[no_mangle]
unsafe extern "C" fn max63xx_mmap_init(p: *mut platform_device, wdt: *mut max63xx_wdt) -> c_int {
    static int max63xx_mmap_init(struct platform_device *p, struct max63xx_wdt *wdt)
    {
    wdt.base = devm_platform_ioremap_resource(p, 0);
    if (IS_ERR(wdt.base))
    return PTR_ERR(wdt.base);
    spin_lock_init(&wdt.lock);
    wdt.ping = max63xx_mmap_ping;
    wdt.set = max63xx_mmap_set;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max63xx_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int max63xx_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct max63xx_wdt *wdt;
    const struct max63xx_timeout *table;
    int err;
    wdt = devm_kzalloc(dev, sizeof(*wdt), GFP_KERNEL);
    if (!wdt)
    return -ENOMEM;
// Attempt to use fwnode first
    table = device_get_match_data(dev);
    if (!table)
    table = (struct max63xx_timeout *)pdev.id_entry.driver_data;
    if (heartbeat < 1 || heartbeat > MAX_HEARTBEAT)
    heartbeat = DEFAULT_HEARTBEAT;
    wdt.timeout = max63xx_select_timeout(table, heartbeat);
    if (!wdt.timeout) {
    dev_err(dev, "unable to satisfy %ds heartbeat request\n",
    heartbeat);
    return -EINVAL;
    }
    err = max63xx_mmap_init(pdev, wdt);
    if (err)
    return err;
    platform_set_drvdata(pdev, &wdt.wdd);
    watchdog_set_drvdata(&wdt.wdd, wdt);
    wdt.wdd.parent = dev;
    wdt.wdd.timeout = wdt.timeout.twd;
    wdt.wdd.info = &max63xx_wdt_info;
    wdt.wdd.ops = &max63xx_wdt_ops;
    watchdog_set_nowayout(&wdt.wdd, nowayout);
    err = devm_watchdog_register_device(dev, &wdt.wdd);
    if (err)
    return err;
    dev_info(dev, "using %ds heartbeat with %ds initial delay\n",
    wdt.timeout.twd, wdt.timeout.tdelay);
    return 0;
    }
    static const struct platform_device_id max63xx_id_table[] = {
    { .name = "max6369_wdt", .driver_data = (kernel_ulong_t)max6369_table },
    { .name = "max6370_wdt", .driver_data = (kernel_ulong_t)max6369_table },
    { .name = "max6371_wdt", .driver_data = (kernel_ulong_t)max6371_table },
    { .name = "max6372_wdt", .driver_data = (kernel_ulong_t)max6371_table },
    { .name = "max6373_wdt", .driver_data = (kernel_ulong_t)max6373_table },
    { .name = "max6374_wdt", .driver_data = (kernel_ulong_t)max6373_table },
    { }
    };
    MODULE_DEVICE_TABLE(platform, max63xx_id_table);
    static const struct of_device_id max63xx_dt_id_table[] = {
    { .compatible = "maxim,max6369", .data = max6369_table, },
    { .compatible = "maxim,max6370", .data = max6369_table, },
    { .compatible = "maxim,max6371", .data = max6371_table, },
    { .compatible = "maxim,max6372", .data = max6371_table, },
    { .compatible = "maxim,max6373", .data = max6373_table, },
    { .compatible = "maxim,max6374", .data = max6373_table, },
    { }
    };
    MODULE_DEVICE_TABLE(of, max63xx_dt_id_table);
    static struct platform_driver max63xx_wdt_driver = {
    .probe		= max63xx_wdt_probe,
    .id_table	= max63xx_id_table,
    .driver		= {
    .name	= "max63xx_wdt",
    .of_match_table = max63xx_dt_id_table,
    },
    };
    module_platform_driver(max63xx_wdt_driver);
    MODULE_AUTHOR("Marc Zyngier <maz@misterjones.org>");
    MODULE_DESCRIPTION("max63xx Watchdog Driver");
    module_param(heartbeat, int, 0);
    MODULE_PARM_DESC(heartbeat,
    "Watchdog heartbeat period in seconds from 1 to "
    __MODULE_STRING(MAX_HEARTBEAT) ", default "
    __MODULE_STRING(DEFAULT_HEARTBEAT));
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
    module_param(nodelay, int, 0);
    MODULE_PARM_DESC(nodelay,
    "Force selection of a timeout setting without initial delay "
    "(max6373/74 only, default=0)");
    MODULE_LICENSE("GPL v2");
