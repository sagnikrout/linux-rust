//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/mt7621_wdt.c
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
// Ralink MT7621/MT7628 built-in hardware watchdog timer
//
// Copyright (C) 2014 John Crispin <john@phrozen.org>
//
// This driver was based on: drivers/watchdog/rt2880_wdt.c
//

pub const SYSC_RSTSTAT: c_uint = 0x38;

pub const RALINK_WDT_TIMEOUT: c_int = 30;
pub const TIMER_REG_TMRSTAT: c_uint = 0x00;
pub const TIMER_REG_TMR1LOAD: c_uint = 0x24;
pub const TIMER_REG_TMR1CTL: c_uint = 0x20;

pub const TMR1CTL_PRESCALE_SHIFT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7621_wdt_data {
    pub base: *mut void __iomem,
    pub rst: *mut reset_control,
    pub sysc: *mut regmap,
    pub wdt: watchdog_device,
}

    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[no_mangle]
pub unsafe extern "C" fn rt_wdt_w32(base: *mut void __iomem, reg: c_uint, val: u32) {
    static inline void rt_wdt_w32(void __iomem *base, unsigned int reg, u32 val)
    {
    iowrite32(val, base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn rt_wdt_r32(base: *mut void __iomem, reg: c_uint) -> u32 {
    static inline u32 rt_wdt_r32(void __iomem *base, unsigned int reg)
    {
    return ioread32(base + reg);
    }
#[no_mangle]
unsafe extern "C" fn mt7621_wdt_ping(w: *mut watchdog_device) -> c_int {
    static int mt7621_wdt_ping(struct watchdog_device *w)
    {
    struct mt7621_wdt_data *drvdata = watchdog_get_drvdata(w);
    rt_wdt_w32(drvdata.base, TIMER_REG_TMRSTAT, TMR1CTL_RESTART);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt7621_wdt_set_timeout(w: *mut watchdog_device, t: c_uint) -> c_int {
    static int mt7621_wdt_set_timeout(struct watchdog_device *w, unsigned int t)
    {
    struct mt7621_wdt_data *drvdata = watchdog_get_drvdata(w);
    w.timeout = t;
    rt_wdt_w32(drvdata.base, TIMER_REG_TMR1LOAD, t * 1000);
    mt7621_wdt_ping(w);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt7621_wdt_start(w: *mut watchdog_device) -> c_int {
    static int mt7621_wdt_start(struct watchdog_device *w)
    {
    struct mt7621_wdt_data *drvdata = watchdog_get_drvdata(w);
    u32 t;
// set the prescaler to 1ms == 1000us
    rt_wdt_w32(drvdata.base, TIMER_REG_TMR1CTL, 1000 << TMR1CTL_PRESCALE_SHIFT);
    mt7621_wdt_set_timeout(w, w.timeout);
    t = rt_wdt_r32(drvdata.base, TIMER_REG_TMR1CTL);
    t |= TMR1CTL_ENABLE;
    rt_wdt_w32(drvdata.base, TIMER_REG_TMR1CTL, t);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt7621_wdt_stop(w: *mut watchdog_device) -> c_int {
    static int mt7621_wdt_stop(struct watchdog_device *w)
    {
    struct mt7621_wdt_data *drvdata = watchdog_get_drvdata(w);
    u32 t;
    mt7621_wdt_ping(w);
    t = rt_wdt_r32(drvdata.base, TIMER_REG_TMR1CTL);
    t &= ~TMR1CTL_ENABLE;
    rt_wdt_w32(drvdata.base, TIMER_REG_TMR1CTL, t);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt7621_wdt_bootcause(d: *mut mt7621_wdt_data) -> c_int {
    static int mt7621_wdt_bootcause(struct mt7621_wdt_data *d)
    {
    u32 val;
    regmap_read(d.sysc, SYSC_RSTSTAT, &val);
    if (val & WDT_RST_CAUSE)
    return WDIOF_CARDRESET;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt7621_wdt_is_running(w: *mut watchdog_device) -> c_int {
    static int mt7621_wdt_is_running(struct watchdog_device *w)
    {
    struct mt7621_wdt_data *drvdata = watchdog_get_drvdata(w);
    return !!(rt_wdt_r32(drvdata.base, TIMER_REG_TMR1CTL) & TMR1CTL_ENABLE);
    }
    static const struct watchdog_info mt7621_wdt_info = {
    .identity = "Mediatek Watchdog",
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE,
    };
    static const struct watchdog_ops mt7621_wdt_ops = {
    .owner = THIS_MODULE,
    .start = mt7621_wdt_start,
    .stop = mt7621_wdt_stop,
    .ping = mt7621_wdt_ping,
    .set_timeout = mt7621_wdt_set_timeout,
    };
#[no_mangle]
unsafe extern "C" fn mt7621_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int mt7621_wdt_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct device *dev = &pdev.dev;
    struct watchdog_device *mt7621_wdt;
    struct mt7621_wdt_data *drvdata;
    int err;
    drvdata = devm_kzalloc(dev, sizeof(*drvdata), GFP_KERNEL);
    if (!drvdata)
    return -ENOMEM;
    drvdata.sysc = syscon_regmap_lookup_by_phandle(np, "mediatek,sysctl");
    if (IS_ERR(drvdata.sysc)) {
    drvdata.sysc = syscon_regmap_lookup_by_compatible("mediatek,mt7621-sysc");
    if (IS_ERR(drvdata.sysc))
    return PTR_ERR(drvdata.sysc);
    }
    drvdata.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(drvdata.base))
    return PTR_ERR(drvdata.base);
    drvdata.rst = devm_reset_control_get_exclusive(dev, core::ptr::null_mut());
    if (!IS_ERR(drvdata.rst))
    reset_control_deassert(drvdata.rst);
    mt7621_wdt = &drvdata.wdt;
    mt7621_wdt.info = &mt7621_wdt_info;
    mt7621_wdt.ops = &mt7621_wdt_ops;
    mt7621_wdt.min_timeout = 1;
    mt7621_wdt.max_timeout = 0xfffful / 1000;
    mt7621_wdt.parent = dev;
    mt7621_wdt.bootstatus = mt7621_wdt_bootcause(drvdata);
    watchdog_init_timeout(mt7621_wdt, mt7621_wdt.max_timeout, dev);
    watchdog_set_nowayout(mt7621_wdt, nowayout);
    watchdog_set_drvdata(mt7621_wdt, drvdata);
    if (mt7621_wdt_is_running(mt7621_wdt)) {
//
// Make sure to apply timeout from watchdog core, taking
// the prescaler of this driver here into account (the
// boot loader might be using a different prescaler).
//
// To avoid spurious resets because of different scaling,
// we first disable the watchdog, set the new prescaler
// and timeout, and then re-enable the watchdog.
//
    mt7621_wdt_stop(mt7621_wdt);
    mt7621_wdt_start(mt7621_wdt);
    set_bit(WDOG_HW_RUNNING, &mt7621_wdt.status);
    }
    err = devm_watchdog_register_device(dev, &drvdata.wdt);
    if (err)
    return err;
    platform_set_drvdata(pdev, drvdata);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt7621_wdt_shutdown(pdev: *mut platform_device) {
    static void mt7621_wdt_shutdown(struct platform_device *pdev)
    {
    struct mt7621_wdt_data *drvdata = platform_get_drvdata(pdev);
    mt7621_wdt_stop(&drvdata.wdt);
    }
    static const struct of_device_id mt7621_wdt_match[] = {
    { .compatible = "mediatek,mt7621-wdt" },
    {},
    };
    MODULE_DEVICE_TABLE(of, mt7621_wdt_match);
    static struct platform_driver mt7621_wdt_driver = {
    .probe		= mt7621_wdt_probe,
    .shutdown	= mt7621_wdt_shutdown,
    .driver		= {
    .name		= KBUILD_MODNAME,
    .of_match_table	= mt7621_wdt_match,
    },
    };
    module_platform_driver(mt7621_wdt_driver);
    MODULE_DESCRIPTION("MediaTek MT762x hardware watchdog driver");
    MODULE_AUTHOR("John Crispin <john@phrozen.org");
    MODULE_LICENSE("GPL v2");
