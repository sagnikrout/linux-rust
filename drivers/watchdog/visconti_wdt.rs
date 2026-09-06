//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/visconti_wdt.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2020 TOSHIBA CORPORATION
// Copyright (c) 2020 Toshiba Electronic Devices & Storage Corporation
// Copyright (c) 2020 Nobuhiro Iwamatsu <nobuhiro1.iwamatsu@toshiba.co.jp>
//

pub const WDT_CNT: c_uint = 0x00;
pub const WDT_MIN: c_uint = 0x04;
pub const WDT_MAX: c_uint = 0x08;
pub const WDT_CTL: c_uint = 0x0c;
pub const WDT_CMD: c_uint = 0x10;
pub const WDT_CMD_CLEAR: c_uint = 0x4352;
pub const WDT_CMD_START_STOP: c_uint = 0x5354;
pub const WDT_DIV: c_uint = 0x30;

    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(
    nowayout,
    "Watchdog cannot be stopped once started (default=" __MODULE_STRING(WATCHDOG_NOWAYOUT)")");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct visconti_wdt_priv {
    pub wdev: watchdog_device,
    pub base: *mut void __iomem,
    pub div: u32,
}

#[no_mangle]
unsafe extern "C" fn visconti_wdt_start(wdev: *mut watchdog_device) -> c_int {
    static int visconti_wdt_start(struct watchdog_device *wdev)
    {
    struct visconti_wdt_priv *priv = watchdog_get_drvdata(wdev);
    let mut timeout: u32 = wdev.timeout * VISCONTI_WDT_FREQ;
    writel(priv.div, priv.base + WDT_DIV);
    writel(0, priv.base + WDT_MIN);
    writel(timeout, priv.base + WDT_MAX);
    writel(0, priv.base + WDT_CTL);
    writel(WDT_CMD_START_STOP, priv.base + WDT_CMD);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn visconti_wdt_stop(wdev: *mut watchdog_device) -> c_int {
    static int visconti_wdt_stop(struct watchdog_device *wdev)
    {
    struct visconti_wdt_priv *priv = watchdog_get_drvdata(wdev);
    writel(1, priv.base + WDT_CTL);
    writel(WDT_CMD_START_STOP, priv.base + WDT_CMD);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn visconti_wdt_ping(wdd: *mut watchdog_device) -> c_int {
    static int visconti_wdt_ping(struct watchdog_device *wdd)
    {
    struct visconti_wdt_priv *priv = watchdog_get_drvdata(wdd);
    writel(WDT_CMD_CLEAR, priv.base + WDT_CMD);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn visconti_wdt_get_timeleft(wdev: *mut watchdog_device) -> c_uint {
    static unsigned int visconti_wdt_get_timeleft(struct watchdog_device *wdev)
    {
    struct visconti_wdt_priv *priv = watchdog_get_drvdata(wdev);
    let mut timeout: u32 = wdev.timeout * VISCONTI_WDT_FREQ;
    let mut cnt: u32 = readl(priv.base + WDT_CNT);
    if (timeout <= cnt)
    return 0;
    timeout -= cnt;
    return timeout / VISCONTI_WDT_FREQ;
    }
#[no_mangle]
unsafe extern "C" fn visconti_wdt_set_timeout(wdev: *mut watchdog_device, timeout: c_uint) -> c_int {
    static int visconti_wdt_set_timeout(struct watchdog_device *wdev, unsigned int timeout)
    {
    u32 val;
    struct visconti_wdt_priv *priv = watchdog_get_drvdata(wdev);
    wdev.timeout = timeout;
    val = wdev.timeout * VISCONTI_WDT_FREQ;
// Clear counter before setting timeout because WDT expires
    writel(WDT_CMD_CLEAR, priv.base + WDT_CMD);
    writel(val, priv.base + WDT_MAX);
    return 0;
    }
    static const struct watchdog_info visconti_wdt_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_MAGICCLOSE | WDIOF_KEEPALIVEPING,
    .identity = "Visconti Watchdog",
    };
    static const struct watchdog_ops visconti_wdt_ops = {
    .owner		= THIS_MODULE,
    .start		= visconti_wdt_start,
    .stop		= visconti_wdt_stop,
    .ping		= visconti_wdt_ping,
    .get_timeleft	= visconti_wdt_get_timeleft,
    .set_timeout	= visconti_wdt_set_timeout,
    };
#[no_mangle]
unsafe extern "C" fn visconti_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int visconti_wdt_probe(struct platform_device *pdev)
    {
    struct watchdog_device *wdev;
    struct visconti_wdt_priv *priv;
    struct device *dev = &pdev.dev;
    struct clk *clk;
    unsigned long clk_freq;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return dev_err_probe(dev, PTR_ERR(clk), "Could not get clock\n");
    clk_freq = clk_get_rate(clk);
    if (!clk_freq)
    return -EINVAL;
    priv.div = clk_freq / VISCONTI_WDT_FREQ;
// Initialize struct watchdog_device.
    wdev = &priv.wdev;
    wdev.info = &visconti_wdt_info;
    wdev.ops = &visconti_wdt_ops;
    wdev.parent = dev;
    wdev.min_timeout = 1;
    wdev.max_timeout = 0xffffffff / VISCONTI_WDT_FREQ;
    wdev.timeout = min(wdev.max_timeout, WDT_DEFAULT_TIMEOUT);
    watchdog_set_drvdata(wdev, priv);
    watchdog_set_nowayout(wdev, nowayout);
    watchdog_stop_on_unregister(wdev);
// This overrides the default timeout only if DT configuration was found
    watchdog_init_timeout(wdev, 0, dev);
    return devm_watchdog_register_device(dev, wdev);
    }
    static const struct of_device_id visconti_wdt_of_match[] = {
    { .compatible = "toshiba,visconti-wdt", },
    {}
    };
    MODULE_DEVICE_TABLE(of, visconti_wdt_of_match);
    static struct platform_driver visconti_wdt_driver = {
    .driver = {
    .name = "visconti_wdt",
    .of_match_table = visconti_wdt_of_match,
    },
    .probe = visconti_wdt_probe,
    };
    module_platform_driver(visconti_wdt_driver);
    MODULE_DESCRIPTION("TOSHIBA Visconti Watchdog Driver");
    MODULE_AUTHOR("Nobuhiro Iwamatsu <nobuhiro1.iwamatsu@toshiba.co.jp");
    MODULE_LICENSE("GPL v2");
