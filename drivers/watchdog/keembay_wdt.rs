//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/keembay_wdt.c
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
// Watchdog driver for Intel Keem Bay non-secure watchdog.
//
// Copyright (C) 2020 Intel Corporation
//

// Non-secure watchdog register offsets
pub const TIM_WATCHDOG: c_uint = 0x0;
pub const TIM_WATCHDOG_INT_THRES: c_uint = 0x4;
pub const TIM_WDOG_EN: c_uint = 0x8;
pub const TIM_SAFE: c_uint = 0xc;

pub const WDT_INT_CLEAR_SMC: c_uint = 0x8200ff18;
pub const WDT_UNLOCK: c_uint = 0xf1d0dead;
pub const WDT_DISABLE: c_uint = 0x0;
pub const WDT_ENABLE: c_uint = 0x1;

pub const WDT_LOAD_MIN: c_int = 1;
pub const WDT_TIMEOUT: c_int = 5;
pub const WDT_PRETIMEOUT: c_int = 4;
    let mut timeout: static unsigned int = WDT_TIMEOUT;
    module_param(timeout, int, 0);
    MODULE_PARM_DESC(timeout, "Watchdog timeout period in seconds (default = "
    __MODULE_STRING(WDT_TIMEOUT) ")");
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default = "
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct keembay_wdt {
    pub wdd: watchdog_device,
    pub clk: *mut clk,
    pub rate: c_uint,
    pub to_irq: c_int,
    pub th_irq: c_int,
    pub base: *mut void __iomem,
}

#[no_mangle]
pub unsafe extern "C" fn keembay_wdt_readl(wdt: *mut keembay_wdt, offset: u32) -> u32 {
    static inline u32 keembay_wdt_readl(struct keembay_wdt *wdt, u32 offset)
    {
    return readl(wdt.base + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn keembay_wdt_writel(wdt: *mut keembay_wdt, offset: u32, val: u32) {
    static inline void keembay_wdt_writel(struct keembay_wdt *wdt, u32 offset, u32 val)
    {
    writel(WDT_UNLOCK, wdt.base + TIM_SAFE);
    writel(val, wdt.base + offset);
    }
#[no_mangle]
unsafe extern "C" fn keembay_wdt_set_timeout_reg(wdog: *mut watchdog_device) {
    static void keembay_wdt_set_timeout_reg(struct watchdog_device *wdog)
    {
    struct keembay_wdt *wdt = watchdog_get_drvdata(wdog);
    keembay_wdt_writel(wdt, TIM_WATCHDOG, wdog.timeout * wdt.rate);
    }
#[no_mangle]
unsafe extern "C" fn keembay_wdt_set_pretimeout_reg(wdog: *mut watchdog_device) {
    static void keembay_wdt_set_pretimeout_reg(struct watchdog_device *wdog)
    {
    struct keembay_wdt *wdt = watchdog_get_drvdata(wdog);
    let mut th_val: u32 = 0;
    if (wdog.pretimeout)
    th_val = wdog.timeout - wdog.pretimeout;
    keembay_wdt_writel(wdt, TIM_WATCHDOG_INT_THRES, th_val * wdt.rate);
    }
#[no_mangle]
unsafe extern "C" fn keembay_wdt_start(wdog: *mut watchdog_device) -> c_int {
    static int keembay_wdt_start(struct watchdog_device *wdog)
    {
    struct keembay_wdt *wdt = watchdog_get_drvdata(wdog);
    keembay_wdt_writel(wdt, TIM_WDOG_EN, WDT_ENABLE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn keembay_wdt_stop(wdog: *mut watchdog_device) -> c_int {
    static int keembay_wdt_stop(struct watchdog_device *wdog)
    {
    struct keembay_wdt *wdt = watchdog_get_drvdata(wdog);
    keembay_wdt_writel(wdt, TIM_WDOG_EN, WDT_DISABLE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn keembay_wdt_ping(wdog: *mut watchdog_device) -> c_int {
    static int keembay_wdt_ping(struct watchdog_device *wdog)
    {
    keembay_wdt_set_timeout_reg(wdog);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn keembay_wdt_set_timeout(wdog: *mut watchdog_device, t: u32) -> c_int {
    static int keembay_wdt_set_timeout(struct watchdog_device *wdog, u32 t)
    {
    wdog.timeout = t;
    keembay_wdt_set_timeout_reg(wdog);
    keembay_wdt_set_pretimeout_reg(wdog);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn keembay_wdt_set_pretimeout(wdog: *mut watchdog_device, t: u32) -> c_int {
    static int keembay_wdt_set_pretimeout(struct watchdog_device *wdog, u32 t)
    {
    if (t > wdog.timeout)
    return -EINVAL;
    wdog.pretimeout = t;
    keembay_wdt_set_pretimeout_reg(wdog);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn keembay_wdt_get_timeleft(wdog: *mut watchdog_device) -> c_uint {
    static unsigned int keembay_wdt_get_timeleft(struct watchdog_device *wdog)
    {
    struct keembay_wdt *wdt = watchdog_get_drvdata(wdog);
    return keembay_wdt_readl(wdt, TIM_WATCHDOG) / wdt.rate;
    }
//
// SMC call is used to clear the interrupt bits, because the TIM_GEN_CONFIG
// register is in the secure bank.
//
#[no_mangle]
unsafe extern "C" fn keembay_wdt_to_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t keembay_wdt_to_isr(int irq, void *dev_id)
    {
    struct keembay_wdt *wdt = dev_id;
    struct arm_smccc_res res;
    arm_smccc_smc(WDT_INT_CLEAR_SMC, WDT_TO_INT_MASK, 0, 0, 0, 0, 0, 0, &res);
    dev_crit(wdt.wdd.parent, "Intel Keem Bay non-secure wdt timeout.\n");
    emergency_restart();
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn keembay_wdt_th_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t keembay_wdt_th_isr(int irq, void *dev_id)
    {
    struct keembay_wdt *wdt = dev_id;
    struct arm_smccc_res res;
    keembay_wdt_set_pretimeout(&wdt.wdd, 0x0);
    arm_smccc_smc(WDT_INT_CLEAR_SMC, WDT_TH_INT_MASK, 0, 0, 0, 0, 0, 0, &res);
    dev_crit(wdt.wdd.parent, "Intel Keem Bay non-secure wdt pre-timeout.\n");
    watchdog_notify_pretimeout(&wdt.wdd);
    return IRQ_HANDLED;
    }
    static const struct watchdog_info keembay_wdt_info = {
    .identity	= "Intel Keem Bay Watchdog Timer",
    .options	= WDIOF_SETTIMEOUT |
    WDIOF_PRETIMEOUT |
    WDIOF_MAGICCLOSE |
    WDIOF_KEEPALIVEPING,
    };
    static const struct watchdog_ops keembay_wdt_ops = {
    .owner		= THIS_MODULE,
    .start		= keembay_wdt_start,
    .stop		= keembay_wdt_stop,
    .ping		= keembay_wdt_ping,
    .set_timeout	= keembay_wdt_set_timeout,
    .set_pretimeout	= keembay_wdt_set_pretimeout,
    .get_timeleft	= keembay_wdt_get_timeleft,
    };
#[no_mangle]
unsafe extern "C" fn keembay_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int keembay_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct keembay_wdt *wdt;
    int ret;
    wdt = devm_kzalloc(dev, sizeof(*wdt), GFP_KERNEL);
    if (!wdt)
    return -ENOMEM;
    wdt.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(wdt.base))
    return PTR_ERR(wdt.base);
// we do not need to enable the clock as it is enabled by default
    wdt.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(wdt.clk))
    return dev_err_probe(dev, PTR_ERR(wdt.clk), "Failed to get clock\n");
    wdt.rate = clk_get_rate(wdt.clk);
    if (!wdt.rate)
    return dev_err_probe(dev, -EINVAL, "Failed to get clock rate\n");
    wdt.th_irq = platform_get_irq_byname(pdev, "threshold");
    if (wdt.th_irq < 0)
    return dev_err_probe(dev, wdt.th_irq, "Failed to get IRQ for threshold\n");
    ret = devm_request_irq(dev, wdt.th_irq, keembay_wdt_th_isr, 0,
    "keembay-wdt", wdt);
    if (ret)
    return ret;
    wdt.to_irq = platform_get_irq_byname(pdev, "timeout");
    if (wdt.to_irq < 0)
    return dev_err_probe(dev, wdt.to_irq, "Failed to get IRQ for timeout\n");
    ret = devm_request_irq(dev, wdt.to_irq, keembay_wdt_to_isr, 0,
    "keembay-wdt", wdt);
    if (ret)
    return ret;
    wdt.wdd.parent		= dev;
    wdt.wdd.info		= &keembay_wdt_info;
    wdt.wdd.ops		= &keembay_wdt_ops;
    wdt.wdd.min_timeout	= WDT_LOAD_MIN;
    wdt.wdd.max_timeout	= WDT_LOAD_MAX / wdt.rate;
    wdt.wdd.timeout	= WDT_TIMEOUT;
    wdt.wdd.pretimeout	= WDT_PRETIMEOUT;
    watchdog_set_drvdata(&wdt.wdd, wdt);
    watchdog_set_nowayout(&wdt.wdd, nowayout);
    watchdog_init_timeout(&wdt.wdd, timeout, dev);
    keembay_wdt_set_timeout(&wdt.wdd, wdt.wdd.timeout);
    keembay_wdt_set_pretimeout(&wdt.wdd, wdt.wdd.pretimeout);
    ret = devm_watchdog_register_device(dev, &wdt.wdd);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to register watchdog device.\n");
    platform_set_drvdata(pdev, wdt);
    dev_info(dev, "Initial timeout %d sec%s.\n",
    wdt.wdd.timeout, nowayout ? ", nowayout" : "");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn keembay_wdt_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused keembay_wdt_suspend(struct device *dev)
    {
    struct keembay_wdt *wdt = dev_get_drvdata(dev);
    if (watchdog_active(&wdt.wdd))
    return keembay_wdt_stop(&wdt.wdd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn keembay_wdt_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused keembay_wdt_resume(struct device *dev)
    {
    struct keembay_wdt *wdt = dev_get_drvdata(dev);
    if (watchdog_active(&wdt.wdd))
    return keembay_wdt_start(&wdt.wdd);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(keembay_wdt_pm_ops, keembay_wdt_suspend,
    keembay_wdt_resume);
    static const struct of_device_id keembay_wdt_match[] = {
    { .compatible = "intel,keembay-wdt" },
    { }
    };
    MODULE_DEVICE_TABLE(of, keembay_wdt_match);
    static struct platform_driver keembay_wdt_driver = {
    .probe	= keembay_wdt_probe,
    .driver	= {
    .name		= "keembay_wdt",
    .of_match_table	= keembay_wdt_match,
    .pm		= &keembay_wdt_pm_ops,
    },
    };
    module_platform_driver(keembay_wdt_driver);
    MODULE_DESCRIPTION("Intel Keem Bay SoC watchdog driver");
    MODULE_AUTHOR("Wan Ahmad Zainie <wan.ahmad.zainie.wan.mohamad@intel.com");
    MODULE_LICENSE("GPL v2");
