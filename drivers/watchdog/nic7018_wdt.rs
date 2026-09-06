//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/nic7018_wdt.c
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
// Copyright (C) 2016 National Instruments Corp.
//

pub const LOCK: c_uint = 0xA5;
pub const UNLOCK: c_uint = 0x5A;

pub const WDT_CTRL: c_int = 1;
pub const WDT_RELOAD_CTRL: c_int = 2;
pub const WDT_PRESET_PRESCALE: c_int = 4;
pub const WDT_REG_LOCK: c_int = 5;
pub const WDT_COUNT: c_int = 6;
pub const WDT_RELOAD_PORT: c_int = 7;
pub const WDT_MIN_TIMEOUT: c_int = 1;
pub const WDT_MAX_TIMEOUT: c_int = 464;
pub const WDT_DEFAULT_TIMEOUT: c_int = 80;
pub const WDT_MAX_COUNTER: c_int = 15;
    static unsigned int timeout;
    module_param(timeout, uint, 0);
    MODULE_PARM_DESC(timeout,
    "Watchdog timeout in seconds. (default="
    __MODULE_STRING(WDT_DEFAULT_TIMEOUT) ")");
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started. (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nic7018_wdt {
    pub io_base: u16,
    pub period: u32,
    pub wdd: watchdog_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nic7018_config {
    pub period: u32,
    pub divider: u8,
}

    static const struct nic7018_config nic7018_configs[] = {
    {  2, 4 },
    { 32, 5 },
    };
#[no_mangle]
pub unsafe extern "C" fn nic7018_timeout(period: u32, counter: u8) -> u32 {
    static inline u32 nic7018_timeout(u32 period, u8 counter)
    {
    return period * counter - period / 2;
    }
    static const struct nic7018_config *nic7018_get_config(u32 timeout,
    u8 *counter)
    {
    const struct nic7018_config *config;
    u8 count;
    if (timeout < 30 && timeout != 16) {
    config = &nic7018_configs[0];
    count = timeout / 2 + 1;
    } else {
    config = &nic7018_configs[1];
    count = DIV_ROUND_UP(timeout + 16, 32);
    if (count > WDT_MAX_COUNTER)
    count = WDT_MAX_COUNTER;
    }
// counter = count;
    return config;
    }
    static int nic7018_set_timeout(struct watchdog_device *wdd,
    unsigned int timeout)
    {
    struct nic7018_wdt *wdt = watchdog_get_drvdata(wdd);
    const struct nic7018_config *config;
    u8 counter;
    config = nic7018_get_config(timeout, &counter);
    outb(counter << 4 | config.divider,
    wdt.io_base + WDT_PRESET_PRESCALE);
    wdd.timeout = nic7018_timeout(config.period, counter);
    wdt.period = config.period;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nic7018_start(wdd: *mut watchdog_device) -> c_int {
    static int nic7018_start(struct watchdog_device *wdd)
    {
    struct nic7018_wdt *wdt = watchdog_get_drvdata(wdd);
    u8 control;
    nic7018_set_timeout(wdd, wdd.timeout);
    control = inb(wdt.io_base + WDT_RELOAD_CTRL);
    outb(control | WDT_RELOAD_PORT_EN, wdt.io_base + WDT_RELOAD_CTRL);
    outb(1, wdt.io_base + WDT_RELOAD_PORT);
    control = inb(wdt.io_base + WDT_CTRL);
    outb(control | WDT_CTRL_RESET_EN, wdt.io_base + WDT_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nic7018_stop(wdd: *mut watchdog_device) -> c_int {
    static int nic7018_stop(struct watchdog_device *wdd)
    {
    struct nic7018_wdt *wdt = watchdog_get_drvdata(wdd);
    outb(0, wdt.io_base + WDT_CTRL);
    outb(0, wdt.io_base + WDT_RELOAD_CTRL);
    outb(0xF0, wdt.io_base + WDT_PRESET_PRESCALE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nic7018_ping(wdd: *mut watchdog_device) -> c_int {
    static int nic7018_ping(struct watchdog_device *wdd)
    {
    struct nic7018_wdt *wdt = watchdog_get_drvdata(wdd);
    outb(1, wdt.io_base + WDT_RELOAD_PORT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nic7018_get_timeleft(wdd: *mut watchdog_device) -> c_uint {
    static unsigned int nic7018_get_timeleft(struct watchdog_device *wdd)
    {
    struct nic7018_wdt *wdt = watchdog_get_drvdata(wdd);
    u8 count;
    count = inb(wdt.io_base + WDT_COUNT) & 0xF;
    if (!count)
    return 0;
    return nic7018_timeout(wdt.period, count);
    }
    static const struct watchdog_info nic7018_wdd_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE,
    .identity = "NIC7018 Watchdog",
    };
    static const struct watchdog_ops nic7018_wdd_ops = {
    .owner = THIS_MODULE,
    .start = nic7018_start,
    .stop = nic7018_stop,
    .ping = nic7018_ping,
    .set_timeout = nic7018_set_timeout,
    .get_timeleft = nic7018_get_timeleft,
    };
#[no_mangle]
unsafe extern "C" fn nic7018_probe(pdev: *mut platform_device) -> c_int {
    static int nic7018_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct watchdog_device *wdd;
    struct nic7018_wdt *wdt;
    struct resource *io_rc;
    int ret;
    wdt = devm_kzalloc(dev, sizeof(*wdt), GFP_KERNEL);
    if (!wdt)
    return -ENOMEM;
    platform_set_drvdata(pdev, wdt);
    io_rc = platform_get_resource(pdev, IORESOURCE_IO, 0);
    if (!io_rc) {
    dev_err(dev, "missing IO resources\n");
    return -EINVAL;
    }
    if (!devm_request_region(dev, io_rc.start, resource_size(io_rc),
    KBUILD_MODNAME)) {
    dev_err(dev, "failed to get IO region\n");
    return -EBUSY;
    }
    wdt.io_base = io_rc.start;
    wdd = &wdt.wdd;
    wdd.info = &nic7018_wdd_info;
    wdd.ops = &nic7018_wdd_ops;
    wdd.min_timeout = WDT_MIN_TIMEOUT;
    wdd.max_timeout = WDT_MAX_TIMEOUT;
    wdd.timeout = WDT_DEFAULT_TIMEOUT;
    wdd.parent = dev;
    watchdog_set_drvdata(wdd, wdt);
    watchdog_set_nowayout(wdd, nowayout);
    watchdog_init_timeout(wdd, timeout, dev);
// Unlock WDT register
    outb(UNLOCK, wdt.io_base + WDT_REG_LOCK);
    ret = watchdog_register_device(wdd);
    if (ret) {
    outb(LOCK, wdt.io_base + WDT_REG_LOCK);
    return ret;
    }
    dev_dbg(dev, "io_base=0x%04X, timeout=%d, nowayout=%d\n",
    wdt.io_base, timeout, nowayout);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nic7018_remove(pdev: *mut platform_device) {
    static void nic7018_remove(struct platform_device *pdev)
    {
    struct nic7018_wdt *wdt = platform_get_drvdata(pdev);
    watchdog_unregister_device(&wdt.wdd);
// Lock WDT register
    outb(LOCK, wdt.io_base + WDT_REG_LOCK);
    }
    static const struct acpi_device_id nic7018_device_ids[] = {
    { "NIC7018" },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, nic7018_device_ids);
    static struct platform_driver watchdog_driver = {
    .probe = nic7018_probe,
    .remove = nic7018_remove,
    .driver = {
    .name = KBUILD_MODNAME,
    .acpi_match_table = nic7018_device_ids,
    },
    };
    module_platform_driver(watchdog_driver);
    MODULE_DESCRIPTION("National Instruments NIC7018 Watchdog driver");
    MODULE_AUTHOR("Hui Chun Ong <hui.chun.ong@ni.com>");
    MODULE_LICENSE("GPL");
