//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/ftwdt010_wdt.c
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
// Watchdog driver for Faraday Technology FTWDT010
//
// Copyright (C) 2017 Linus Walleij <linus.walleij@linaro.org>
//
// Inspired by the out-of-tree drivers from OpenWRT:
// Copyright (C) 2009 Paulius Zaleckas <paulius.zaleckas@teltonika.lt>
//

pub const FTWDT010_WDCOUNTER: c_uint = 0x0;
pub const FTWDT010_WDLOAD: c_uint = 0x4;
pub const FTWDT010_WDRESTART: c_uint = 0x8;
pub const FTWDT010_WDCR: c_uint = 0xC;
pub const WDRESTART_MAGIC: c_uint = 0x5AB9;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftwdt010_wdt {
    pub wdd: watchdog_device,
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub has_irq: bool,
}

    static inline
    struct ftwdt010_wdt *to_ftwdt010_wdt(struct watchdog_device *wdd)
    {
    return container_of(wdd, struct ftwdt010_wdt, wdd);
    }
    static void ftwdt010_enable(struct ftwdt010_wdt *gwdt,
    unsigned int timeout,
    bool need_irq)
    {
    u32 enable;
    writel(timeout * WDT_CLOCK, gwdt.base + FTWDT010_WDLOAD);
    writel(WDRESTART_MAGIC, gwdt.base + FTWDT010_WDRESTART);
// set clock before enabling
    enable = WDCR_CLOCK_5MHZ | WDCR_SYS_RST;
    writel(enable, gwdt.base + FTWDT010_WDCR);
    if (need_irq)
    enable |= WDCR_WDINTR;
    enable |= WDCR_ENABLE;
    writel(enable, gwdt.base + FTWDT010_WDCR);
    }
#[no_mangle]
unsafe extern "C" fn ftwdt010_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int ftwdt010_wdt_start(struct watchdog_device *wdd)
    {
    struct ftwdt010_wdt *gwdt = to_ftwdt010_wdt(wdd);
    ftwdt010_enable(gwdt, wdd.timeout, gwdt.has_irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ftwdt010_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int ftwdt010_wdt_stop(struct watchdog_device *wdd)
    {
    struct ftwdt010_wdt *gwdt = to_ftwdt010_wdt(wdd);
    writel(0, gwdt.base + FTWDT010_WDCR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ftwdt010_wdt_ping(wdd: *mut watchdog_device) -> c_int {
    static int ftwdt010_wdt_ping(struct watchdog_device *wdd)
    {
    struct ftwdt010_wdt *gwdt = to_ftwdt010_wdt(wdd);
    writel(WDRESTART_MAGIC, gwdt.base + FTWDT010_WDRESTART);
    return 0;
    }
    static int ftwdt010_wdt_set_timeout(struct watchdog_device *wdd,
    unsigned int timeout)
    {
    wdd.timeout = timeout;
    if (watchdog_active(wdd))
    ftwdt010_wdt_start(wdd);
    return 0;
    }
    static int ftwdt010_wdt_restart(struct watchdog_device *wdd,
    unsigned long action, void *data)
    {
    ftwdt010_enable(to_ftwdt010_wdt(wdd), 0, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ftwdt010_wdt_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t ftwdt010_wdt_interrupt(int irq, void *data)
    {
    struct ftwdt010_wdt *gwdt = data;
    watchdog_notify_pretimeout(&gwdt.wdd);
    return IRQ_HANDLED;
    }
    static const struct watchdog_ops ftwdt010_wdt_ops = {
    .start		= ftwdt010_wdt_start,
    .stop		= ftwdt010_wdt_stop,
    .ping		= ftwdt010_wdt_ping,
    .set_timeout	= ftwdt010_wdt_set_timeout,
    .restart	= ftwdt010_wdt_restart,
    .owner		= THIS_MODULE,
    };
    static const struct watchdog_info ftwdt010_wdt_info = {
    .options	= WDIOF_KEEPALIVEPING
    | WDIOF_MAGICCLOSE
    | WDIOF_SETTIMEOUT,
    .identity	= KBUILD_MODNAME,
    };
#[no_mangle]
unsafe extern "C" fn ftwdt010_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int ftwdt010_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct ftwdt010_wdt *gwdt;
    unsigned int reg;
    int irq;
    int ret;
    gwdt = devm_kzalloc(dev, sizeof(*gwdt), GFP_KERNEL);
    if (!gwdt)
    return -ENOMEM;
    gwdt.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(gwdt.base))
    return PTR_ERR(gwdt.base);
    gwdt.dev = dev;
    gwdt.wdd.info = &ftwdt010_wdt_info;
    gwdt.wdd.ops = &ftwdt010_wdt_ops;
    gwdt.wdd.min_timeout = 1;
    gwdt.wdd.max_timeout = 0xFFFFFFFF / WDT_CLOCK;
    gwdt.wdd.parent = dev;
//
// If 'timeout-sec' unspecified in devicetree, assume a 13 second
// default.
//
    gwdt.wdd.timeout = 13U;
    watchdog_init_timeout(&gwdt.wdd, 0, dev);
    reg = readw(gwdt.base + FTWDT010_WDCR);
    if (reg & WDCR_ENABLE) {
// Watchdog was enabled by the bootloader, disable it.
    reg &= ~WDCR_ENABLE;
    writel(reg, gwdt.base + FTWDT010_WDCR);
    }
    irq = platform_get_irq(pdev, 0);
    if (irq > 0) {
    ret = devm_request_irq(dev, irq, ftwdt010_wdt_interrupt, 0,
    "watchdog bark", gwdt);
    if (ret)
    return ret;
    gwdt.has_irq = true;
    }
    ret = devm_watchdog_register_device(dev, &gwdt.wdd);
    if (ret)
    return ret;
// Set up platform driver data
    platform_set_drvdata(pdev, gwdt);
    dev_info(dev, "FTWDT010 watchdog driver enabled\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ftwdt010_wdt_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ftwdt010_wdt_suspend(struct device *dev)
    {
    struct ftwdt010_wdt *gwdt = dev_get_drvdata(dev);
    unsigned int reg;
    reg = readw(gwdt.base + FTWDT010_WDCR);
    reg &= ~WDCR_ENABLE;
    writel(reg, gwdt.base + FTWDT010_WDCR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ftwdt010_wdt_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ftwdt010_wdt_resume(struct device *dev)
    {
    struct ftwdt010_wdt *gwdt = dev_get_drvdata(dev);
    unsigned int reg;
    if (watchdog_active(&gwdt.wdd)) {
    reg = readw(gwdt.base + FTWDT010_WDCR);
    reg |= WDCR_ENABLE;
    writel(reg, gwdt.base + FTWDT010_WDCR);
    }
    return 0;
    }
    static const struct dev_pm_ops ftwdt010_wdt_dev_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(ftwdt010_wdt_suspend,
    ftwdt010_wdt_resume)
    };
    static const struct of_device_id ftwdt010_wdt_match[] = {
    { .compatible = "faraday,ftwdt010" },
    { .compatible = "cortina,gemini-watchdog" },
    {},
    };
    MODULE_DEVICE_TABLE(of, ftwdt010_wdt_match);
    static struct platform_driver ftwdt010_wdt_driver = {
    .probe		= ftwdt010_wdt_probe,
    .driver		= {
    .name	= "ftwdt010-wdt",
    .of_match_table = ftwdt010_wdt_match,
    .pm = &ftwdt010_wdt_dev_pm_ops,
    },
    };
    module_platform_driver(ftwdt010_wdt_driver);
    MODULE_AUTHOR("Linus Walleij");
    MODULE_DESCRIPTION("Watchdog driver for Faraday Technology FTWDT010");
    MODULE_LICENSE("GPL");
