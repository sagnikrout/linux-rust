//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/meson_wdt.c
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
// Meson Watchdog Driver
//
// Copyright (c) 2014 Carlo Caione
//

pub const MESON_WDT_TC: c_uint = 0x00;

pub const MESON_WDT_RESET: c_uint = 0x04;
pub const MESON_WDT_TIMEOUT: c_int = 30;
pub const MESON_WDT_MIN_TIMEOUT: c_int = 1;

    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    static unsigned int timeout;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_wdt_data {
    pub enable: c_uint,
    pub terminal_count_mask: c_uint,
    pub count_unit: c_uint,
}

    static struct meson_wdt_data meson6_wdt_data = {
    .enable			= BIT(22),
    .terminal_count_mask	= 0x3fffff,
    .count_unit		= 100000, /* 10 us */
    };
    static struct meson_wdt_data meson8b_wdt_data = {
    .enable			= BIT(19),
    .terminal_count_mask	= 0xffff,
    .count_unit		= 7812, /* 128 us */
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_wdt_dev {
    pub wdt_dev: watchdog_device,
    pub wdt_base: *mut void __iomem,
    pub data: *const meson_wdt_data,
}

    static int meson_wdt_restart(struct watchdog_device *wdt_dev,
    unsigned long action, void *data)
    {
    struct meson_wdt_dev *meson_wdt = watchdog_get_drvdata(wdt_dev);
    let mut tc_reboot: u32 = MESON_WDT_DC_RESET;
    tc_reboot |= meson_wdt.data.enable;
    while (1) {
    writel(tc_reboot, meson_wdt.wdt_base + MESON_WDT_TC);
    mdelay(5);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_wdt_ping(wdt_dev: *mut watchdog_device) -> c_int {
    static int meson_wdt_ping(struct watchdog_device *wdt_dev)
    {
    struct meson_wdt_dev *meson_wdt = watchdog_get_drvdata(wdt_dev);
    writel(0, meson_wdt.wdt_base + MESON_WDT_RESET);
    return 0;
    }
    static void meson_wdt_change_timeout(struct watchdog_device *wdt_dev,
    unsigned int timeout)
    {
    struct meson_wdt_dev *meson_wdt = watchdog_get_drvdata(wdt_dev);
    u32 reg;
    reg = readl(meson_wdt.wdt_base + MESON_WDT_TC);
    reg &= ~meson_wdt.data.terminal_count_mask;
    reg |= MESON_SEC_TO_TC(timeout, meson_wdt.data.count_unit);
    writel(reg, meson_wdt.wdt_base + MESON_WDT_TC);
    }
    static int meson_wdt_set_timeout(struct watchdog_device *wdt_dev,
    unsigned int timeout)
    {
    wdt_dev.timeout = timeout;
    meson_wdt_change_timeout(wdt_dev, timeout);
    meson_wdt_ping(wdt_dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_wdt_stop(wdt_dev: *mut watchdog_device) -> c_int {
    static int meson_wdt_stop(struct watchdog_device *wdt_dev)
    {
    struct meson_wdt_dev *meson_wdt = watchdog_get_drvdata(wdt_dev);
    u32 reg;
    reg = readl(meson_wdt.wdt_base + MESON_WDT_TC);
    reg &= ~meson_wdt.data.enable;
    writel(reg, meson_wdt.wdt_base + MESON_WDT_TC);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_wdt_start(wdt_dev: *mut watchdog_device) -> c_int {
    static int meson_wdt_start(struct watchdog_device *wdt_dev)
    {
    struct meson_wdt_dev *meson_wdt = watchdog_get_drvdata(wdt_dev);
    u32 reg;
    meson_wdt_change_timeout(wdt_dev, meson_wdt.wdt_dev.timeout);
    meson_wdt_ping(wdt_dev);
    reg = readl(meson_wdt.wdt_base + MESON_WDT_TC);
    reg |= meson_wdt.data.enable;
    writel(reg, meson_wdt.wdt_base + MESON_WDT_TC);
    return 0;
    }
    static const struct watchdog_info meson_wdt_info = {
    .identity	= DRV_NAME,
    .options	= WDIOF_SETTIMEOUT |
    WDIOF_KEEPALIVEPING |
    WDIOF_MAGICCLOSE,
    };
    static const struct watchdog_ops meson_wdt_ops = {
    .owner		= THIS_MODULE,
    .start		= meson_wdt_start,
    .stop		= meson_wdt_stop,
    .ping		= meson_wdt_ping,
    .set_timeout	= meson_wdt_set_timeout,
    .restart        = meson_wdt_restart,
    };
    static const struct of_device_id meson_wdt_dt_ids[] = {
    { .compatible = "amlogic,meson6-wdt", .data = &meson6_wdt_data },
    { .compatible = "amlogic,meson8-wdt", .data = &meson6_wdt_data },
    { .compatible = "amlogic,meson8b-wdt", .data = &meson8b_wdt_data },
    { .compatible = "amlogic,meson8m2-wdt", .data = &meson8b_wdt_data },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, meson_wdt_dt_ids);
#[no_mangle]
unsafe extern "C" fn meson_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int meson_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct meson_wdt_dev *meson_wdt;
    int err;
    meson_wdt = devm_kzalloc(dev, sizeof(*meson_wdt), GFP_KERNEL);
    if (!meson_wdt)
    return -ENOMEM;
    meson_wdt.wdt_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(meson_wdt.wdt_base))
    return PTR_ERR(meson_wdt.wdt_base);
    meson_wdt.data = device_get_match_data(dev);
    meson_wdt.wdt_dev.parent = dev;
    meson_wdt.wdt_dev.info = &meson_wdt_info;
    meson_wdt.wdt_dev.ops = &meson_wdt_ops;
    meson_wdt.wdt_dev.max_timeout =
    meson_wdt.data.terminal_count_mask / meson_wdt.data.count_unit;
    meson_wdt.wdt_dev.min_timeout = MESON_WDT_MIN_TIMEOUT;
    meson_wdt.wdt_dev.timeout = min_t(unsigned int,
    MESON_WDT_TIMEOUT,
    meson_wdt.wdt_dev.max_timeout);
    watchdog_set_drvdata(&meson_wdt.wdt_dev, meson_wdt);
    watchdog_init_timeout(&meson_wdt.wdt_dev, timeout, dev);
    watchdog_set_nowayout(&meson_wdt.wdt_dev, nowayout);
    watchdog_set_restart_priority(&meson_wdt.wdt_dev, 128);
    meson_wdt_stop(&meson_wdt.wdt_dev);
    watchdog_stop_on_reboot(&meson_wdt.wdt_dev);
    err = devm_watchdog_register_device(dev, &meson_wdt.wdt_dev);
    if (err)
    return err;
    dev_info(dev, "Watchdog enabled (timeout=%d sec, nowayout=%d)",
    meson_wdt.wdt_dev.timeout, nowayout);
    return 0;
    }
    static struct platform_driver meson_wdt_driver = {
    .probe		= meson_wdt_probe,
    .driver		= {
    .name		= DRV_NAME,
    .of_match_table	= meson_wdt_dt_ids,
    },
    };
    module_platform_driver(meson_wdt_driver);
    module_param(timeout, uint, 0);
    MODULE_PARM_DESC(timeout, "Watchdog heartbeat in seconds");
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Carlo Caione <carlo@caione.org>");
    MODULE_DESCRIPTION("Meson Watchdog Timer Driver");
