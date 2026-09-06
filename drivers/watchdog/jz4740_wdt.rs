//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/jz4740_wdt.c
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
// Copyright (C) 2010, Paul Cercueil <paul@crapouillou.net>
// JZ4740 Watchdog driver
//

pub const DEFAULT_HEARTBEAT: c_int = 5;
pub const MAX_HEARTBEAT: c_int = 2048;
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
    let mut heartbeat: static unsigned int = DEFAULT_HEARTBEAT;
    module_param(heartbeat, uint, 0);
    MODULE_PARM_DESC(heartbeat,
    "Watchdog heartbeat period in seconds from 1 to "
    __MODULE_STRING(MAX_HEARTBEAT) ", default "
    __MODULE_STRING(DEFAULT_HEARTBEAT));
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jz4740_wdt_drvdata {
    pub wdt: watchdog_device,
    pub map: *mut regmap,
    pub clk: *mut clk,
    pub clk_rate: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn jz4740_wdt_ping(wdt_dev: *mut watchdog_device) -> c_int {
    static int jz4740_wdt_ping(struct watchdog_device *wdt_dev)
    {
    struct jz4740_wdt_drvdata *drvdata = watchdog_get_drvdata(wdt_dev);
    regmap_write(drvdata.map, TCU_REG_WDT_TCNT, 0);
    return 0;
    }
    static int jz4740_wdt_set_timeout(struct watchdog_device *wdt_dev,
    unsigned int new_timeout)
    {
    struct jz4740_wdt_drvdata *drvdata = watchdog_get_drvdata(wdt_dev);
    let mut timeout_value: u16 = (u16)(drvdata.clk_rate * new_timeout);
    unsigned int tcer;
    regmap_read(drvdata.map, TCU_REG_WDT_TCER, &tcer);
    regmap_write(drvdata.map, TCU_REG_WDT_TCER, 0);
    regmap_write(drvdata.map, TCU_REG_WDT_TDR, timeout_value);
    regmap_write(drvdata.map, TCU_REG_WDT_TCNT, 0);
    if (tcer & TCU_WDT_TCER_TCEN)
    regmap_write(drvdata.map, TCU_REG_WDT_TCER, TCU_WDT_TCER_TCEN);
    wdt_dev.timeout = new_timeout;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jz4740_wdt_start(wdt_dev: *mut watchdog_device) -> c_int {
    static int jz4740_wdt_start(struct watchdog_device *wdt_dev)
    {
    struct jz4740_wdt_drvdata *drvdata = watchdog_get_drvdata(wdt_dev);
    unsigned int tcer;
    int ret;
    ret = clk_prepare_enable(drvdata.clk);
    if (ret)
    return ret;
    regmap_read(drvdata.map, TCU_REG_WDT_TCER, &tcer);
    jz4740_wdt_set_timeout(wdt_dev, wdt_dev.timeout);
// Start watchdog if it wasn't started already
    if (!(tcer & TCU_WDT_TCER_TCEN))
    regmap_write(drvdata.map, TCU_REG_WDT_TCER, TCU_WDT_TCER_TCEN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jz4740_wdt_stop(wdt_dev: *mut watchdog_device) -> c_int {
    static int jz4740_wdt_stop(struct watchdog_device *wdt_dev)
    {
    struct jz4740_wdt_drvdata *drvdata = watchdog_get_drvdata(wdt_dev);
    regmap_write(drvdata.map, TCU_REG_WDT_TCER, 0);
    clk_disable_unprepare(drvdata.clk);
    return 0;
    }
    static int jz4740_wdt_restart(struct watchdog_device *wdt_dev,
    unsigned long action, void *data)
    {
    wdt_dev.timeout = 0;
    jz4740_wdt_start(wdt_dev);
    return 0;
    }
    static const struct watchdog_info jz4740_wdt_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE,
    .identity = "jz4740 Watchdog",
    };
    static const struct watchdog_ops jz4740_wdt_ops = {
    .owner = THIS_MODULE,
    .start = jz4740_wdt_start,
    .stop = jz4740_wdt_stop,
    .ping = jz4740_wdt_ping,
    .set_timeout = jz4740_wdt_set_timeout,
    .restart = jz4740_wdt_restart,
    };

    static const struct of_device_id jz4740_wdt_of_matches[] = {
    { .compatible = "ingenic,jz4740-watchdog", },
    { .compatible = "ingenic,jz4780-watchdog", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, jz4740_wdt_of_matches);

#[no_mangle]
unsafe extern "C" fn jz4740_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int jz4740_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct jz4740_wdt_drvdata *drvdata;
    struct watchdog_device *jz4740_wdt;
    long rate;
    int ret;
    drvdata = devm_kzalloc(dev, sizeof(struct jz4740_wdt_drvdata),
    GFP_KERNEL);
    if (!drvdata)
    return -ENOMEM;
    drvdata.clk = devm_clk_get(&pdev.dev, "wdt");
    if (IS_ERR(drvdata.clk)) {
    dev_err(&pdev.dev, "cannot find WDT clock\n");
    return PTR_ERR(drvdata.clk);
    }
// Set smallest clock possible
    rate = clk_round_rate(drvdata.clk, 1);
    if (rate < 0)
    return rate;
    ret = clk_set_rate(drvdata.clk, rate);
    if (ret)
    return ret;
    drvdata.clk_rate = rate;
    jz4740_wdt = &drvdata.wdt;
    jz4740_wdt.info = &jz4740_wdt_info;
    jz4740_wdt.ops = &jz4740_wdt_ops;
    jz4740_wdt.min_timeout = 1;
    jz4740_wdt.max_timeout = 0xffff / rate;
    jz4740_wdt.timeout = clamp(heartbeat,
    jz4740_wdt.min_timeout,
    jz4740_wdt.max_timeout);
    jz4740_wdt.parent = dev;
    watchdog_set_nowayout(jz4740_wdt, nowayout);
    watchdog_set_drvdata(jz4740_wdt, drvdata);
    drvdata.map = device_node_to_regmap(dev.parent.of_node);
    if (IS_ERR(drvdata.map)) {
    dev_err(dev, "regmap not found\n");
    return PTR_ERR(drvdata.map);
    }
    return devm_watchdog_register_device(dev, &drvdata.wdt);
    }
    static struct platform_driver jz4740_wdt_driver = {
    .probe = jz4740_wdt_probe,
    .driver = {
    .name = "jz4740-wdt",
    .of_match_table = of_match_ptr(jz4740_wdt_of_matches),
    },
    };
    module_platform_driver(jz4740_wdt_driver);
    MODULE_AUTHOR("Paul Cercueil <paul@crapouillou.net>");
    MODULE_DESCRIPTION("jz4740 Watchdog Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:jz4740-wdt");
