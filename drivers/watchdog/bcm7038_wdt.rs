//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/bcm7038_wdt.c
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
// Copyright (C) 2015 Broadcom Corporation
//

pub const WDT_START_1: c_uint = 0xff00;
pub const WDT_START_2: c_uint = 0x00ff;
pub const WDT_STOP_1: c_uint = 0xee00;
pub const WDT_STOP_2: c_uint = 0x00ee;
pub const WDT_TIMEOUT_REG: c_uint = 0x0;
pub const WDT_CMD_REG: c_uint = 0x4;

pub const WDT_DEFAULT_RATE: c_int = 27000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm7038_watchdog {
    pub base: *mut void __iomem,
    pub wdd: watchdog_device,
    pub rate: u32,
    pub clk: *mut clk,
}

    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
#[no_mangle]
pub unsafe extern "C" fn bcm7038_wdt_write(value: u32, addr: *mut void __iomem) {
    static inline void bcm7038_wdt_write(u32 value, void __iomem *addr)
    {
// MIPS chips strapped for BE will automagically configure the
// peripheral registers for CPU-native byte order.
//
    if (IS_ENABLED(CONFIG_MIPS) && IS_ENABLED(CONFIG_CPU_BIG_ENDIAN))
    __raw_writel(value, addr);
    else
    writel_relaxed(value, addr);
    }
#[no_mangle]
pub unsafe extern "C" fn bcm7038_wdt_read(addr: *mut void __iomem) -> u32 {
    static inline u32 bcm7038_wdt_read(void __iomem *addr)
    {
    if (IS_ENABLED(CONFIG_MIPS) && IS_ENABLED(CONFIG_CPU_BIG_ENDIAN))
    return __raw_readl(addr);
    else
    return readl_relaxed(addr);
    }
#[no_mangle]
unsafe extern "C" fn bcm7038_wdt_set_timeout_reg(wdog: *mut watchdog_device) {
    static void bcm7038_wdt_set_timeout_reg(struct watchdog_device *wdog)
    {
    struct bcm7038_watchdog *wdt = watchdog_get_drvdata(wdog);
    u32 timeout;
    timeout = wdt.rate * wdog.timeout;
    bcm7038_wdt_write(timeout, wdt.base + WDT_TIMEOUT_REG);
    }
#[no_mangle]
unsafe extern "C" fn bcm7038_wdt_ping(wdog: *mut watchdog_device) -> c_int {
    static int bcm7038_wdt_ping(struct watchdog_device *wdog)
    {
    struct bcm7038_watchdog *wdt = watchdog_get_drvdata(wdog);
    bcm7038_wdt_write(WDT_START_1, wdt.base + WDT_CMD_REG);
    bcm7038_wdt_write(WDT_START_2, wdt.base + WDT_CMD_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm7038_wdt_start(wdog: *mut watchdog_device) -> c_int {
    static int bcm7038_wdt_start(struct watchdog_device *wdog)
    {
    bcm7038_wdt_set_timeout_reg(wdog);
    bcm7038_wdt_ping(wdog);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm7038_wdt_stop(wdog: *mut watchdog_device) -> c_int {
    static int bcm7038_wdt_stop(struct watchdog_device *wdog)
    {
    struct bcm7038_watchdog *wdt = watchdog_get_drvdata(wdog);
    bcm7038_wdt_write(WDT_STOP_1, wdt.base + WDT_CMD_REG);
    bcm7038_wdt_write(WDT_STOP_2, wdt.base + WDT_CMD_REG);
    return 0;
    }
    static int bcm7038_wdt_set_timeout(struct watchdog_device *wdog,
    unsigned int t)
    {
// Can't modify timeout value if watchdog timer is running
    bcm7038_wdt_stop(wdog);
    wdog.timeout = t;
    bcm7038_wdt_start(wdog);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm7038_wdt_get_timeleft(wdog: *mut watchdog_device) -> c_uint {
    static unsigned int bcm7038_wdt_get_timeleft(struct watchdog_device *wdog)
    {
    struct bcm7038_watchdog *wdt = watchdog_get_drvdata(wdog);
    u32 time_left;
    time_left = bcm7038_wdt_read(wdt.base + WDT_CMD_REG);
    return time_left / wdt.rate;
    }
    static const struct watchdog_info bcm7038_wdt_info = {
    .identity	= "Broadcom BCM7038 Watchdog Timer",
    .options	= WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING |
    WDIOF_MAGICCLOSE
    };
    static const struct watchdog_ops bcm7038_wdt_ops = {
    .owner		= THIS_MODULE,
    .start		= bcm7038_wdt_start,
    .stop		= bcm7038_wdt_stop,
    .set_timeout	= bcm7038_wdt_set_timeout,
    .get_timeleft	= bcm7038_wdt_get_timeleft,
    };
#[no_mangle]
unsafe extern "C" fn bcm7038_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int bcm7038_wdt_probe(struct platform_device *pdev)
    {
    struct bcm7038_wdt_platform_data *pdata = pdev.dev.platform_data;
    struct device *dev = &pdev.dev;
    struct bcm7038_watchdog *wdt;
    const char *clk_name = core::ptr::null_mut();
    int err;
    wdt = devm_kzalloc(dev, sizeof(*wdt), GFP_KERNEL);
    if (!wdt)
    return -ENOMEM;
    platform_set_drvdata(pdev, wdt);
    wdt.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(wdt.base))
    return PTR_ERR(wdt.base);
    if (pdata && pdata.clk_name)
    clk_name = pdata.clk_name;
    wdt.clk = devm_clk_get_enabled(dev, clk_name);
// If unable to get clock, use default frequency
    if (!IS_ERR(wdt.clk)) {
    wdt.rate = clk_get_rate(wdt.clk);
// Prevent divide-by-zero exception
    if (!wdt.rate)
    wdt.rate = WDT_DEFAULT_RATE;
    } else {
    wdt.rate = WDT_DEFAULT_RATE;
    wdt.clk = core::ptr::null_mut();
    }
    wdt.wdd.info		= &bcm7038_wdt_info;
    wdt.wdd.ops		= &bcm7038_wdt_ops;
    wdt.wdd.min_timeout	= WDT_MIN_TIMEOUT;
    wdt.wdd.timeout	= WDT_DEFAULT_TIMEOUT;
    wdt.wdd.max_timeout	= 0xffffffff / wdt.rate;
    wdt.wdd.parent		= dev;
    watchdog_set_drvdata(&wdt.wdd, wdt);
    watchdog_stop_on_reboot(&wdt.wdd);
    watchdog_stop_on_unregister(&wdt.wdd);
    err = devm_watchdog_register_device(dev, &wdt.wdd);
    if (err)
    return err;
    dev_info(dev, "Registered BCM7038 Watchdog\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm7038_wdt_suspend(dev: *mut device) -> c_int {
    static int bcm7038_wdt_suspend(struct device *dev)
    {
    struct bcm7038_watchdog *wdt = dev_get_drvdata(dev);
    if (watchdog_active(&wdt.wdd))
    return bcm7038_wdt_stop(&wdt.wdd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm7038_wdt_resume(dev: *mut device) -> c_int {
    static int bcm7038_wdt_resume(struct device *dev)
    {
    struct bcm7038_watchdog *wdt = dev_get_drvdata(dev);
    if (watchdog_active(&wdt.wdd))
    return bcm7038_wdt_start(&wdt.wdd);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(bcm7038_wdt_pm_ops,
    bcm7038_wdt_suspend, bcm7038_wdt_resume);
    static const struct of_device_id bcm7038_wdt_match[] = {
    { .compatible = "brcm,bcm6345-wdt" },
    { .compatible = "brcm,bcm7038-wdt" },
    {},
    };
    MODULE_DEVICE_TABLE(of, bcm7038_wdt_match);
    static const struct platform_device_id bcm7038_wdt_devtype[] = {
    { .name = "bcm63xx-wdt" },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(platform, bcm7038_wdt_devtype);
    static struct platform_driver bcm7038_wdt_driver = {
    .probe		= bcm7038_wdt_probe,
    .id_table	= bcm7038_wdt_devtype,
    .driver		= {
    .name		= "bcm7038-wdt",
    .of_match_table	= bcm7038_wdt_match,
    .pm		= pm_sleep_ptr(&bcm7038_wdt_pm_ops),
    }
    };
    module_platform_driver(bcm7038_wdt_driver);
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Driver for Broadcom 7038 SoCs Watchdog");
    MODULE_AUTHOR("Justin Chen");
