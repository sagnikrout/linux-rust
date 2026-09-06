//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/ts4800_wdt.c
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
// Watchdog driver for TS-4800 based boards
//
// Copyright (c) 2015 - Savoir-faire Linux
//

    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
// possible feed values
pub const TS4800_WDT_FEED_2S: c_uint = 0x1;
pub const TS4800_WDT_FEED_10S: c_uint = 0x2;
pub const TS4800_WDT_DISABLE: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ts4800_wdt {
    pub wdd: watchdog_device,
    pub regmap: *mut regmap,
    pub feed_offset: u32,
    pub feed_val: u32,
}

//
// TS-4800 supports the following timeout values:
//
// value desc
// ---------------------
// 0    feed for 338ms
// 1    feed for 2.706s
// 2    feed for 10.824s
// 3    disable watchdog
//
// Keep the regmap/timeout map ordered by timeout
//
    static const struct {
    const int timeout;
    const int regval;
    } ts4800_wdt_map[] = {
    { 2,  TS4800_WDT_FEED_2S },
    { 10, TS4800_WDT_FEED_10S },
    };

#[no_mangle]
unsafe extern "C" fn ts4800_write_feed(wdt: *mut ts4800_wdt, val: u32) {
    static void ts4800_write_feed(struct ts4800_wdt *wdt, u32 val)
    {
    regmap_write(wdt.regmap, wdt.feed_offset, val);
    }
#[no_mangle]
unsafe extern "C" fn ts4800_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int ts4800_wdt_start(struct watchdog_device *wdd)
    {
    struct ts4800_wdt *wdt = watchdog_get_drvdata(wdd);
    ts4800_write_feed(wdt, wdt.feed_val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ts4800_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int ts4800_wdt_stop(struct watchdog_device *wdd)
    {
    struct ts4800_wdt *wdt = watchdog_get_drvdata(wdd);
    ts4800_write_feed(wdt, TS4800_WDT_DISABLE);
    return 0;
    }
    static int ts4800_wdt_set_timeout(struct watchdog_device *wdd,
    unsigned int timeout)
    {
    struct ts4800_wdt *wdt = watchdog_get_drvdata(wdd);
    int i;
    for (i = 0; i < MAX_TIMEOUT_INDEX; i++) {
    if (ts4800_wdt_map[i].timeout >= timeout)
    break;
    }
    wdd.timeout = ts4800_wdt_map[i].timeout;
    wdt.feed_val = ts4800_wdt_map[i].regval;
    return 0;
    }
    static const struct watchdog_ops ts4800_wdt_ops = {
    .owner = THIS_MODULE,
    .start = ts4800_wdt_start,
    .stop = ts4800_wdt_stop,
    .set_timeout = ts4800_wdt_set_timeout,
    };
    static const struct watchdog_info ts4800_wdt_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_MAGICCLOSE | WDIOF_KEEPALIVEPING,
    .identity = "TS-4800 Watchdog",
    };
#[no_mangle]
unsafe extern "C" fn ts4800_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int ts4800_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct device_node *syscon_np;
    struct watchdog_device *wdd;
    struct ts4800_wdt *wdt;
    u32 reg;
    int ret;
    syscon_np = of_parse_phandle(np, "syscon", 0);
    if (!syscon_np) {
    dev_err(dev, "no syscon property\n");
    return -ENODEV;
    }
    ret = of_property_read_u32_index(np, "syscon", 1, &reg);
    if (ret < 0) {
    dev_err(dev, "no offset in syscon\n");
    of_node_put(syscon_np);
    return ret;
    }
// allocate memory for watchdog struct
    wdt = devm_kzalloc(dev, sizeof(*wdt), GFP_KERNEL);
    if (!wdt) {
    of_node_put(syscon_np);
    return -ENOMEM;
    }
// set regmap and offset to know where to write
    wdt.feed_offset = reg;
    wdt.regmap = syscon_node_to_regmap(syscon_np);
    of_node_put(syscon_np);
    if (IS_ERR(wdt.regmap)) {
    dev_err(dev, "cannot get parent's regmap\n");
    return PTR_ERR(wdt.regmap);
    }
// Initialize struct watchdog_device
    wdd = &wdt.wdd;
    wdd.parent = dev;
    wdd.info = &ts4800_wdt_info;
    wdd.ops = &ts4800_wdt_ops;
    wdd.min_timeout = ts4800_wdt_map[0].timeout;
    wdd.max_timeout = ts4800_wdt_map[MAX_TIMEOUT_INDEX].timeout;
    watchdog_set_drvdata(wdd, wdt);
    watchdog_set_nowayout(wdd, nowayout);
    watchdog_init_timeout(wdd, 0, dev);
//
// As this watchdog supports only a few values, ts4800_wdt_set_timeout
// must be called to initialize timeout and feed_val with valid values.
// Default to maximum timeout if none, or an invalid one, is provided in
// device tree.
//
    if (!wdd.timeout)
    wdd.timeout = wdd.max_timeout;
    ts4800_wdt_set_timeout(wdd, wdd.timeout);
//
// The feed register is write-only, so it is not possible to determine
// watchdog's state. Disable it to be in a known state.
//
    ts4800_wdt_stop(wdd);
    ret = devm_watchdog_register_device(dev, wdd);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, wdt);
    dev_info(dev, "initialized (timeout = %d sec, nowayout = %d)\n",
    wdd.timeout, nowayout);
    return 0;
    }
    static const struct of_device_id ts4800_wdt_of_match[] = {
    { .compatible = "technologic,ts4800-wdt", },
    { },
    };
    MODULE_DEVICE_TABLE(of, ts4800_wdt_of_match);
    static struct platform_driver ts4800_wdt_driver = {
    .probe		= ts4800_wdt_probe,
    .driver		= {
    .name	= "ts4800_wdt",
    .of_match_table = ts4800_wdt_of_match,
    },
    };
    module_platform_driver(ts4800_wdt_driver);
    MODULE_AUTHOR("Damien Riegel <damien.riegel@savoirfairelinux.com>");
    MODULE_DESCRIPTION("Watchdog driver for TS-4800 based boards");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:ts4800_wdt");
