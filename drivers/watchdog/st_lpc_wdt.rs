//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/st_lpc_wdt.c
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
// ST's LPC Watchdog
//
// Copyright (C) 2014 STMicroelectronics -- All Rights Reserved
//
// Author: David Paris <david.paris@st.com> for STMicroelectronics
// Lee Jones <lee.jones@linaro.org> for STMicroelectronics
//

// Low Power Alarm
pub const LPC_LPA_LSB_OFF: c_uint = 0x410;
pub const LPC_LPA_START_OFF: c_uint = 0x418;
// LPC as WDT
pub const LPC_WDT_OFF: c_uint = 0x510;
    static struct watchdog_device st_wdog_dev;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_wdog_syscfg {
    pub reset_type_reg: c_uint,
    pub reset_type_mask: c_uint,
    pub enable_reg: c_uint,
    pub enable_mask: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_wdog {
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub syscfg: *const st_wdog_syscfg,
    pub clk: *mut clk,
    pub clkrate: c_ulong,
    pub warm_reset: bool,
}

    static struct st_wdog_syscfg stih407_syscfg = {
    .enable_reg		= 0x204,
    .enable_mask		= BIT(19),
    };
    static const struct of_device_id st_wdog_match[] = {
    {
    .compatible = "st,stih407-lpc",
    .data = &stih407_syscfg,
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, st_wdog_match);
#[no_mangle]
unsafe extern "C" fn st_wdog_setup(st_wdog: *mut st_wdog, enable: bool) {
    static void st_wdog_setup(struct st_wdog *st_wdog, bool enable)
    {
// Type of watchdog reset - 0: Cold 1: Warm
    if (st_wdog.syscfg.reset_type_reg)
    regmap_update_bits(st_wdog.regmap,
    st_wdog.syscfg.reset_type_reg,
    st_wdog.syscfg.reset_type_mask,
    st_wdog.warm_reset);
// Mask/unmask watchdog reset
    regmap_update_bits(st_wdog.regmap,
    st_wdog.syscfg.enable_reg,
    st_wdog.syscfg.enable_mask,
    enable ? 0 : st_wdog.syscfg.enable_mask);
    }
#[no_mangle]
unsafe extern "C" fn st_wdog_load_timer(st_wdog: *mut st_wdog, timeout: c_uint) {
    static void st_wdog_load_timer(struct st_wdog *st_wdog, unsigned int timeout)
    {
    let mut clkrate: c_ulong = st_wdog.clkrate;
    writel_relaxed(timeout * clkrate, st_wdog.base + LPC_LPA_LSB_OFF);
    writel_relaxed(1, st_wdog.base + LPC_LPA_START_OFF);
    }
#[no_mangle]
unsafe extern "C" fn st_wdog_start(wdd: *mut watchdog_device) -> c_int {
    static int st_wdog_start(struct watchdog_device *wdd)
    {
    struct st_wdog *st_wdog = watchdog_get_drvdata(wdd);
    writel_relaxed(1, st_wdog.base + LPC_WDT_OFF);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st_wdog_stop(wdd: *mut watchdog_device) -> c_int {
    static int st_wdog_stop(struct watchdog_device *wdd)
    {
    struct st_wdog *st_wdog = watchdog_get_drvdata(wdd);
    writel_relaxed(0, st_wdog.base + LPC_WDT_OFF);
    return 0;
    }
    static int st_wdog_set_timeout(struct watchdog_device *wdd,
    unsigned int timeout)
    {
    struct st_wdog *st_wdog = watchdog_get_drvdata(wdd);
    wdd.timeout = timeout;
    st_wdog_load_timer(st_wdog, timeout);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st_wdog_keepalive(wdd: *mut watchdog_device) -> c_int {
    static int st_wdog_keepalive(struct watchdog_device *wdd)
    {
    struct st_wdog *st_wdog = watchdog_get_drvdata(wdd);
    st_wdog_load_timer(st_wdog, wdd.timeout);
    return 0;
    }
    static const struct watchdog_info st_wdog_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE,
    .identity = "ST LPC WDT",
    };
    static const struct watchdog_ops st_wdog_ops = {
    .owner		= THIS_MODULE,
    .start		= st_wdog_start,
    .stop		= st_wdog_stop,
    .ping		= st_wdog_keepalive,
    .set_timeout	= st_wdog_set_timeout,
    };
    static struct watchdog_device st_wdog_dev = {
    .info		= &st_wdog_info,
    .ops		= &st_wdog_ops,
    };
#[no_mangle]
unsafe extern "C" fn st_clk_disable_unprepare(data: *mut c_void) {
    static void st_clk_disable_unprepare(void *data)
    {
    clk_disable_unprepare(data);
    }
#[no_mangle]
unsafe extern "C" fn st_wdog_probe(pdev: *mut platform_device) -> c_int {
    static int st_wdog_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct st_wdog *st_wdog;
    struct regmap *regmap;
    struct clk *clk;
    void __iomem *base;
    uint32_t mode;
    int ret;
    ret = of_property_read_u32(np, "st,lpc-mode", &mode);
    if (ret) {
    dev_err(dev, "An LPC mode must be provided\n");
    return -EINVAL;
    }
// LPC can either run as a Clocksource or in RTC or WDT mode
    if (mode != ST_LPC_MODE_WDT)
    return -ENODEV;
    st_wdog = devm_kzalloc(dev, sizeof(*st_wdog), GFP_KERNEL);
    if (!st_wdog)
    return -ENOMEM;
    st_wdog.syscfg	= (struct st_wdog_syscfg *)device_get_match_data(dev);
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    regmap = syscon_regmap_lookup_by_phandle(np, "st,syscfg");
    if (IS_ERR(regmap)) {
    dev_err(dev, "No syscfg phandle specified\n");
    return PTR_ERR(regmap);
    }
    clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(clk)) {
    dev_err(dev, "Unable to request clock\n");
    return PTR_ERR(clk);
    }
    st_wdog.dev		= dev;
    st_wdog.base		= base;
    st_wdog.clk		= clk;
    st_wdog.regmap		= regmap;
    st_wdog.warm_reset	= of_property_read_bool(np, "st,warm_reset");
    st_wdog.clkrate	= clk_get_rate(st_wdog.clk);
    if (!st_wdog.clkrate) {
    dev_err(dev, "Unable to fetch clock rate\n");
    return -EINVAL;
    }
    st_wdog_dev.max_timeout = 0xFFFFFFFF / st_wdog.clkrate;
    st_wdog_dev.parent = dev;
    ret = clk_prepare_enable(clk);
    if (ret) {
    dev_err(dev, "Unable to enable clock\n");
    return ret;
    }
    ret = devm_add_action_or_reset(dev, st_clk_disable_unprepare, clk);
    if (ret)
    return ret;
    watchdog_set_drvdata(&st_wdog_dev, st_wdog);
    watchdog_set_nowayout(&st_wdog_dev, WATCHDOG_NOWAYOUT);
// Init Watchdog timeout with value in DT
    ret = watchdog_init_timeout(&st_wdog_dev, 0, dev);
    if (ret)
    return ret;
    ret = devm_watchdog_register_device(dev, &st_wdog_dev);
    if (ret)
    return ret;
    st_wdog_setup(st_wdog, true);
    dev_info(dev, "LPC Watchdog driver registered, reset type is %s",
    st_wdog.warm_reset ? "warm" : "cold");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn st_wdog_remove(pdev: *mut platform_device) {
    static void st_wdog_remove(struct platform_device *pdev)
    {
    struct st_wdog *st_wdog = watchdog_get_drvdata(&st_wdog_dev);
    st_wdog_setup(st_wdog, false);
    }
#[no_mangle]
unsafe extern "C" fn st_wdog_suspend(dev: *mut device) -> c_int {
    static int st_wdog_suspend(struct device *dev)
    {
    struct st_wdog *st_wdog = watchdog_get_drvdata(&st_wdog_dev);
    if (watchdog_active(&st_wdog_dev))
    st_wdog_stop(&st_wdog_dev);
    st_wdog_setup(st_wdog, false);
    clk_disable(st_wdog.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st_wdog_resume(dev: *mut device) -> c_int {
    static int st_wdog_resume(struct device *dev)
    {
    struct st_wdog *st_wdog = watchdog_get_drvdata(&st_wdog_dev);
    int ret;
    ret = clk_enable(st_wdog.clk);
    if (ret) {
    dev_err(dev, "Unable to re-enable clock\n");
    watchdog_unregister_device(&st_wdog_dev);
    clk_unprepare(st_wdog.clk);
    return ret;
    }
    st_wdog_setup(st_wdog, true);
    if (watchdog_active(&st_wdog_dev)) {
    st_wdog_load_timer(st_wdog, st_wdog_dev.timeout);
    st_wdog_start(&st_wdog_dev);
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(st_wdog_pm_ops,
    st_wdog_suspend, st_wdog_resume);
    static struct platform_driver st_wdog_driver = {
    .driver	= {
    .name = "st-lpc-wdt",
    .pm = pm_sleep_ptr(&st_wdog_pm_ops),
    .of_match_table = st_wdog_match,
    },
    .probe = st_wdog_probe,
    .remove = st_wdog_remove,
    };
    module_platform_driver(st_wdog_driver);
    MODULE_AUTHOR("David Paris <david.paris@st.com>");
    MODULE_DESCRIPTION("ST LPC Watchdog Driver");
    MODULE_LICENSE("GPL");
