//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/moxart_wdt.c
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
// MOXA ART SoCs watchdog driver.
//
// Copyright (C) 2013 Jonas Jensen
//
// Jonas Jensen <jonas.jensen@gmail.com>
//

pub const REG_COUNT: c_uint = 0x4;
pub const REG_MODE: c_uint = 0x8;
pub const REG_ENABLE: c_uint = 0xC;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct moxart_wdt_dev {
    pub dev: watchdog_device,
    pub base: *mut void __iomem,
    pub clock_frequency: c_uint,
}

    static int heartbeat;
    static int moxart_wdt_restart(struct watchdog_device *wdt_dev,
    unsigned long action, void *data)
    {
    struct moxart_wdt_dev *moxart_wdt = watchdog_get_drvdata(wdt_dev);
    writel(1, moxart_wdt.base + REG_COUNT);
    writel(0x5ab9, moxart_wdt.base + REG_MODE);
    writel(0x03, moxart_wdt.base + REG_ENABLE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn moxart_wdt_stop(wdt_dev: *mut watchdog_device) -> c_int {
    static int moxart_wdt_stop(struct watchdog_device *wdt_dev)
    {
    struct moxart_wdt_dev *moxart_wdt = watchdog_get_drvdata(wdt_dev);
    writel(0, moxart_wdt.base + REG_ENABLE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn moxart_wdt_start(wdt_dev: *mut watchdog_device) -> c_int {
    static int moxart_wdt_start(struct watchdog_device *wdt_dev)
    {
    struct moxart_wdt_dev *moxart_wdt = watchdog_get_drvdata(wdt_dev);
    writel(moxart_wdt.clock_frequency * wdt_dev.timeout,
    moxart_wdt.base + REG_COUNT);
    writel(0x5ab9, moxart_wdt.base + REG_MODE);
    writel(0x03, moxart_wdt.base + REG_ENABLE);
    return 0;
    }
    static int moxart_wdt_set_timeout(struct watchdog_device *wdt_dev,
    unsigned int timeout)
    {
    wdt_dev.timeout = timeout;
    return 0;
    }
    static const struct watchdog_info moxart_wdt_info = {
    .identity       = "moxart-wdt",
    .options        = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING |
    WDIOF_MAGICCLOSE,
    };
    static const struct watchdog_ops moxart_wdt_ops = {
    .owner          = THIS_MODULE,
    .start          = moxart_wdt_start,
    .stop           = moxart_wdt_stop,
    .set_timeout    = moxart_wdt_set_timeout,
    .restart        = moxart_wdt_restart,
    };
#[no_mangle]
unsafe extern "C" fn moxart_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int moxart_wdt_probe(struct platform_device *pdev)
    {
    struct moxart_wdt_dev *moxart_wdt;
    struct device *dev = &pdev.dev;
    struct clk *clk;
    int err;
    unsigned int max_timeout;
    let mut nowayout: bool = WATCHDOG_NOWAYOUT;
    moxart_wdt = devm_kzalloc(dev, sizeof(*moxart_wdt), GFP_KERNEL);
    if (!moxart_wdt)
    return -ENOMEM;
    platform_set_drvdata(pdev, moxart_wdt);
    moxart_wdt.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(moxart_wdt.base))
    return PTR_ERR(moxart_wdt.base);
    clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(clk)) {
    pr_err("%s: of_clk_get failed\n", __func__);
    return PTR_ERR(clk);
    }
    moxart_wdt.clock_frequency = clk_get_rate(clk);
    if (moxart_wdt.clock_frequency == 0) {
    pr_err("%s: incorrect clock frequency\n", __func__);
    return -EINVAL;
    }
    max_timeout = UINT_MAX / moxart_wdt.clock_frequency;
    moxart_wdt.dev.info = &moxart_wdt_info;
    moxart_wdt.dev.ops = &moxart_wdt_ops;
    moxart_wdt.dev.timeout = max_timeout;
    moxart_wdt.dev.min_timeout = 1;
    moxart_wdt.dev.max_timeout = max_timeout;
    moxart_wdt.dev.parent = dev;
    watchdog_init_timeout(&moxart_wdt.dev, heartbeat, dev);
    watchdog_set_nowayout(&moxart_wdt.dev, nowayout);
    watchdog_set_restart_priority(&moxart_wdt.dev, 128);
    watchdog_set_drvdata(&moxart_wdt.dev, moxart_wdt);
    watchdog_stop_on_unregister(&moxart_wdt.dev);
    err = devm_watchdog_register_device(dev, &moxart_wdt.dev);
    if (err)
    return err;
    dev_dbg(dev, "Watchdog enabled (heartbeat=%d sec, nowayout=%d)\n",
    moxart_wdt.dev.timeout, nowayout);
    return 0;
    }
    static const struct of_device_id moxart_watchdog_match[] = {
    { .compatible = "moxa,moxart-watchdog" },
    { },
    };
    MODULE_DEVICE_TABLE(of, moxart_watchdog_match);
    static struct platform_driver moxart_wdt_driver = {
    .probe      = moxart_wdt_probe,
    .driver     = {
    .name		= "moxart-watchdog",
    .of_match_table	= moxart_watchdog_match,
    },
    };
    module_platform_driver(moxart_wdt_driver);
    module_param(heartbeat, int, 0);
    MODULE_PARM_DESC(heartbeat, "Watchdog heartbeat in seconds");
    MODULE_DESCRIPTION("MOXART watchdog driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Jonas Jensen <jonas.jensen@gmail.com>");
