//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/rt2880_wdt.c
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
// Ralink RT288x/RT3xxx/MT76xx built-in hardware watchdog timer
//
// Copyright (C) 2011 Gabor Juhos <juhosg@openwrt.org>
// Copyright (C) 2013 John Crispin <john@phrozen.org>
//
// This driver was based on: drivers/watchdog/softdog.c
//

pub const SYSC_RSTSTAT: c_uint = 0x38;

pub const RALINK_WDT_TIMEOUT: c_int = 30;
pub const RALINK_WDT_PRESCALE: c_int = 65536;
pub const TIMER_REG_TMR1LOAD: c_uint = 0x00;
pub const TIMER_REG_TMR1CTL: c_uint = 0x08;

pub const TMR1CTL_MODE_SHIFT: c_int = 4;
pub const TMR1CTL_MODE_MASK: c_uint = 0x3;
pub const TMR1CTL_MODE_FREE_RUNNING: c_uint = 0x0;
pub const TMR1CTL_MODE_PERIODIC: c_uint = 0x1;
pub const TMR1CTL_MODE_TIMEOUT: c_uint = 0x2;
pub const TMR1CTL_MODE_WDT: c_uint = 0x3;
pub const TMR1CTL_PRESCALE_MASK: c_uint = 0xf;
pub const TMR1CTL_PRESCALE_65536: c_uint = 0xf;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt2880_wdt_data {
    pub base: *mut void __iomem,
    pub freq: c_ulong,
    pub clk: *mut clk,
    pub rst: *mut reset_control,
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
unsafe extern "C" fn rt288x_wdt_ping(w: *mut watchdog_device) -> c_int {
    static int rt288x_wdt_ping(struct watchdog_device *w)
    {
    struct rt2880_wdt_data *drvdata = watchdog_get_drvdata(w);
    rt_wdt_w32(drvdata.base, TIMER_REG_TMR1LOAD, w.timeout * drvdata.freq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rt288x_wdt_start(w: *mut watchdog_device) -> c_int {
    static int rt288x_wdt_start(struct watchdog_device *w)
    {
    struct rt2880_wdt_data *drvdata = watchdog_get_drvdata(w);
    u32 t;
    t = rt_wdt_r32(drvdata.base, TIMER_REG_TMR1CTL);
    t &= ~(TMR1CTL_MODE_MASK << TMR1CTL_MODE_SHIFT |
    TMR1CTL_PRESCALE_MASK);
    t |= (TMR1CTL_MODE_WDT << TMR1CTL_MODE_SHIFT |
    TMR1CTL_PRESCALE_65536);
    rt_wdt_w32(drvdata.base, TIMER_REG_TMR1CTL, t);
    rt288x_wdt_ping(w);
    t = rt_wdt_r32(drvdata.base, TIMER_REG_TMR1CTL);
    t |= TMR1CTL_ENABLE;
    rt_wdt_w32(drvdata.base, TIMER_REG_TMR1CTL, t);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rt288x_wdt_stop(w: *mut watchdog_device) -> c_int {
    static int rt288x_wdt_stop(struct watchdog_device *w)
    {
    struct rt2880_wdt_data *drvdata = watchdog_get_drvdata(w);
    u32 t;
    rt288x_wdt_ping(w);
    t = rt_wdt_r32(drvdata.base, TIMER_REG_TMR1CTL);
    t &= ~TMR1CTL_ENABLE;
    rt_wdt_w32(drvdata.base, TIMER_REG_TMR1CTL, t);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rt288x_wdt_set_timeout(w: *mut watchdog_device, t: c_uint) -> c_int {
    static int rt288x_wdt_set_timeout(struct watchdog_device *w, unsigned int t)
    {
    w.timeout = t;
    rt288x_wdt_ping(w);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rt288x_wdt_bootcause() -> c_int {
    static int rt288x_wdt_bootcause(void)
    {
    if (rt_sysc_r32(SYSC_RSTSTAT) & WDT_RST_CAUSE)
    return WDIOF_CARDRESET;
    return 0;
    }
    static const struct watchdog_info rt288x_wdt_info = {
    .identity = "Ralink Watchdog",
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE,
    };
    static const struct watchdog_ops rt288x_wdt_ops = {
    .owner = THIS_MODULE,
    .start = rt288x_wdt_start,
    .stop = rt288x_wdt_stop,
    .ping = rt288x_wdt_ping,
    .set_timeout = rt288x_wdt_set_timeout,
    };
#[no_mangle]
unsafe extern "C" fn rt288x_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int rt288x_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct watchdog_device *wdt;
    struct rt2880_wdt_data *drvdata;
    int ret;
    drvdata = devm_kzalloc(dev, sizeof(*drvdata), GFP_KERNEL);
    if (!drvdata)
    return -ENOMEM;
    drvdata.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(drvdata.base))
    return PTR_ERR(drvdata.base);
    drvdata.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(drvdata.clk))
    return PTR_ERR(drvdata.clk);
    drvdata.rst = devm_reset_control_get_exclusive(dev, core::ptr::null_mut());
    if (!IS_ERR(drvdata.rst))
    reset_control_deassert(drvdata.rst);
    drvdata.freq = clk_get_rate(drvdata.clk) / RALINK_WDT_PRESCALE;
    wdt = &drvdata.wdt;
    wdt.info = &rt288x_wdt_info;
    wdt.ops = &rt288x_wdt_ops;
    wdt.min_timeout = 1;
    wdt.max_timeout = (0xfffful / drvdata.freq);
    wdt.parent = dev;
    wdt.bootstatus = rt288x_wdt_bootcause();
    watchdog_init_timeout(wdt, wdt.max_timeout, dev);
    watchdog_set_nowayout(wdt, nowayout);
    watchdog_set_drvdata(wdt, drvdata);
    watchdog_stop_on_reboot(wdt);
    ret = devm_watchdog_register_device(dev, &drvdata.wdt);
    if (!ret)
    dev_info(dev, "Initialized\n");
    return 0;
    }
    static const struct of_device_id rt288x_wdt_match[] = {
    { .compatible = "ralink,rt2880-wdt" },
    {},
    };
    MODULE_DEVICE_TABLE(of, rt288x_wdt_match);
    static struct platform_driver rt288x_wdt_driver = {
    .probe		= rt288x_wdt_probe,
    .driver		= {
    .name		= KBUILD_MODNAME,
    .of_match_table	= rt288x_wdt_match,
    },
    };
    module_platform_driver(rt288x_wdt_driver);
    MODULE_DESCRIPTION("MediaTek/Ralink RT288x/RT3xxx hardware watchdog driver");
    MODULE_AUTHOR("Gabor Juhos <juhosg@openwrt.org");
    MODULE_LICENSE("GPL v2");
