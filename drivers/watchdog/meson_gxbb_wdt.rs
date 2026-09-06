//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/meson_gxbb_wdt.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (c) 2016 BayLibre, SAS.
// Author: Neil Armstrong <narmstrong@baylibre.com>
//

pub const GXBB_WDT_CTRL_REG: c_uint = 0x0;
pub const GXBB_WDT_TCNT_REG: c_uint = 0x8;
pub const GXBB_WDT_RSET_REG: c_uint = 0xc;

pub const GXBB_WDT_TCNT_CNT_SHIFT: c_int = 16;
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
    static unsigned int timeout;
    module_param(timeout, uint, 0);
    MODULE_PARM_DESC(timeout, "Watchdog heartbeat in seconds="
    __MODULE_STRING(DEFAULT_TIMEOUT) ")");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_gxbb_wdt {
    pub reg_base: *mut void __iomem,
    pub wdt_dev: watchdog_device,
    pub clk: *mut clk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wdt_params {
    pub rst: u32,
}

#[no_mangle]
unsafe extern "C" fn meson_gxbb_wdt_start(wdt_dev: *mut watchdog_device) -> c_int {
    static int meson_gxbb_wdt_start(struct watchdog_device *wdt_dev)
    {
    struct meson_gxbb_wdt *data = watchdog_get_drvdata(wdt_dev);
    writel(readl(data.reg_base + GXBB_WDT_CTRL_REG) | GXBB_WDT_CTRL_EN,
    data.reg_base + GXBB_WDT_CTRL_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_gxbb_wdt_stop(wdt_dev: *mut watchdog_device) -> c_int {
    static int meson_gxbb_wdt_stop(struct watchdog_device *wdt_dev)
    {
    struct meson_gxbb_wdt *data = watchdog_get_drvdata(wdt_dev);
    writel(readl(data.reg_base + GXBB_WDT_CTRL_REG) & ~GXBB_WDT_CTRL_EN,
    data.reg_base + GXBB_WDT_CTRL_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_gxbb_wdt_ping(wdt_dev: *mut watchdog_device) -> c_int {
    static int meson_gxbb_wdt_ping(struct watchdog_device *wdt_dev)
    {
    struct meson_gxbb_wdt *data = watchdog_get_drvdata(wdt_dev);
    writel(0, data.reg_base + GXBB_WDT_RSET_REG);
    return 0;
    }
    static int meson_gxbb_wdt_set_timeout(struct watchdog_device *wdt_dev,
    unsigned int timeout)
    {
    struct meson_gxbb_wdt *data = watchdog_get_drvdata(wdt_dev);
    let mut tcnt: c_ulong = timeout * 1000;
    if (tcnt > GXBB_WDT_TCNT_SETUP_MASK)
    tcnt = GXBB_WDT_TCNT_SETUP_MASK;
    wdt_dev.timeout = timeout;
    meson_gxbb_wdt_ping(wdt_dev);
    writel(tcnt, data.reg_base + GXBB_WDT_TCNT_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_gxbb_wdt_get_timeleft(wdt_dev: *mut watchdog_device) -> c_uint {
    static unsigned int meson_gxbb_wdt_get_timeleft(struct watchdog_device *wdt_dev)
    {
    struct meson_gxbb_wdt *data = watchdog_get_drvdata(wdt_dev);
    unsigned long reg;
    reg = readl(data.reg_base + GXBB_WDT_TCNT_REG);
    return ((reg & GXBB_WDT_TCNT_SETUP_MASK) -
    (reg >> GXBB_WDT_TCNT_CNT_SHIFT)) / 1000;
    }
    static const struct watchdog_ops meson_gxbb_wdt_ops = {
    .start = meson_gxbb_wdt_start,
    .stop = meson_gxbb_wdt_stop,
    .ping = meson_gxbb_wdt_ping,
    .set_timeout = meson_gxbb_wdt_set_timeout,
    .get_timeleft = meson_gxbb_wdt_get_timeleft,
    };
    static const struct watchdog_info meson_gxbb_wdt_info = {
    .identity = "Meson GXBB Watchdog",
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE,
    };
#[no_mangle]
unsafe extern "C" fn meson_gxbb_wdt_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused meson_gxbb_wdt_resume(struct device *dev)
    {
    struct meson_gxbb_wdt *data = dev_get_drvdata(dev);
    if (watchdog_active(&data.wdt_dev))
    meson_gxbb_wdt_start(&data.wdt_dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_gxbb_wdt_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused meson_gxbb_wdt_suspend(struct device *dev)
    {
    struct meson_gxbb_wdt *data = dev_get_drvdata(dev);
    if (watchdog_active(&data.wdt_dev))
    meson_gxbb_wdt_stop(&data.wdt_dev);
    return 0;
    }
    static const struct dev_pm_ops meson_gxbb_wdt_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(meson_gxbb_wdt_suspend, meson_gxbb_wdt_resume)
    };
    static const struct wdt_params gxbb_params = {
    .rst = BIT(21),
    };
    static const struct wdt_params t7_params = {
    .rst = BIT(22),
    };
    static const struct of_device_id meson_gxbb_wdt_dt_ids[] = {
    { .compatible = "amlogic,meson-gxbb-wdt", .data = &gxbb_params, },
    { .compatible = "amlogic,t7-wdt", .data = &t7_params, },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, meson_gxbb_wdt_dt_ids);
#[no_mangle]
unsafe extern "C" fn meson_gxbb_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int meson_gxbb_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct meson_gxbb_wdt *data;
    struct wdt_params *params;
    u32 ctrl_reg;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(data.reg_base))
    return PTR_ERR(data.reg_base);
    data.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(data.clk))
    return PTR_ERR(data.clk);
    params = (struct wdt_params *)of_device_get_match_data(dev);
    platform_set_drvdata(pdev, data);
    data.wdt_dev.parent = dev;
    data.wdt_dev.info = &meson_gxbb_wdt_info;
    data.wdt_dev.ops = &meson_gxbb_wdt_ops;
    data.wdt_dev.max_hw_heartbeat_ms = GXBB_WDT_TCNT_SETUP_MASK;
    data.wdt_dev.min_timeout = 1;
    data.wdt_dev.timeout = DEFAULT_TIMEOUT;
    watchdog_init_timeout(&data.wdt_dev, timeout, dev);
    watchdog_set_nowayout(&data.wdt_dev, nowayout);
    watchdog_set_drvdata(&data.wdt_dev, data);
    ctrl_reg = readl(data.reg_base + GXBB_WDT_CTRL_REG) &
    GXBB_WDT_CTRL_EN;
    if (ctrl_reg) {
// Watchdog is running - keep it running but extend timeout
// to the maximum while setting the timebase
//
    set_bit(WDOG_HW_RUNNING, &data.wdt_dev.status);
    meson_gxbb_wdt_set_timeout(&data.wdt_dev,
    GXBB_WDT_TCNT_SETUP_MASK / 1000);
    }
// Setup with 1ms timebase
    ctrl_reg |= ((clk_get_rate(data.clk) / 1000) &
    GXBB_WDT_CTRL_DIV_MASK) |
    params.rst |
    GXBB_WDT_CTRL_CLK_EN |
    GXBB_WDT_CTRL_CLKDIV_EN;
    writel(ctrl_reg, data.reg_base + GXBB_WDT_CTRL_REG);
    meson_gxbb_wdt_set_timeout(&data.wdt_dev, data.wdt_dev.timeout);
    return devm_watchdog_register_device(dev, &data.wdt_dev);
    }
    static struct platform_driver meson_gxbb_wdt_driver = {
    .probe	= meson_gxbb_wdt_probe,
    .driver = {
    .name = "meson-gxbb-wdt",
    .pm = &meson_gxbb_wdt_pm_ops,
    .of_match_table	= meson_gxbb_wdt_dt_ids,
    },
    };
    module_platform_driver(meson_gxbb_wdt_driver);
    MODULE_AUTHOR("Neil Armstrong <narmstrong@baylibre.com>");
    MODULE_DESCRIPTION("Amlogic Meson GXBB Watchdog timer driver");
    MODULE_LICENSE("Dual BSD/GPL");
