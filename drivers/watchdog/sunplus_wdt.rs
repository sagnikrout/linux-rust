//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/sunplus_wdt.c
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
// sunplus Watchdog Driver
//
// Copyright (C) 2021 Sunplus Technology Co., Ltd.
//

pub const WDT_CTRL: c_uint = 0x00;
pub const WDT_CNT: c_uint = 0x04;
pub const WDT_STOP: c_uint = 0x3877;
pub const WDT_RESUME: c_uint = 0x4A4B;
pub const WDT_CLRIRQ: c_uint = 0x7482;
pub const WDT_UNLOCK: c_uint = 0xAB00;
pub const WDT_LOCK: c_uint = 0xAB01;
pub const WDT_CONMAX: c_uint = 0xDEAF;
// TIMEOUT_MAX = ffff0/90kHz =11.65, so longer than 11 seconds will time out.

pub const SP_WDT_DEFAULT_TIMEOUT: c_int = 10;
pub const STC_CLK: c_int = 90000;

    static unsigned int timeout;
    module_param(timeout, int, 0);
    MODULE_PARM_DESC(timeout, "Watchdog timeout in seconds");
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sp_wdt_priv {
    pub wdev: watchdog_device,
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub rstc: *mut reset_control,
}

    static int sp_wdt_restart(struct watchdog_device *wdev,
    unsigned long action, void *data)
    {
    struct sp_wdt_priv *priv = watchdog_get_drvdata(wdev);
    void __iomem *base = priv.base;
    writel(WDT_STOP, base + WDT_CTRL);
    writel(WDT_UNLOCK, base + WDT_CTRL);
    writel(0x0001, base + WDT_CNT);
    writel(WDT_LOCK, base + WDT_CTRL);
    writel(WDT_RESUME, base + WDT_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sp_wdt_ping(wdev: *mut watchdog_device) -> c_int {
    static int sp_wdt_ping(struct watchdog_device *wdev)
    {
    struct sp_wdt_priv *priv = watchdog_get_drvdata(wdev);
    void __iomem *base = priv.base;
    u32 count;
    if (wdev.timeout > SP_WDT_MAX_TIMEOUT) {
// WDT_CONMAX sets the count to the maximum (down-counting).
    writel(WDT_CONMAX, base + WDT_CTRL);
    } else {
    writel(WDT_UNLOCK, base + WDT_CTRL);
//
// Watchdog timer is a 20-bit down-counting based on STC_CLK.
// This register bits[16:0] is from bit[19:4] of the watchdog
// timer counter.
//
    count = (wdev.timeout * STC_CLK) >> 4;
    writel(count, base + WDT_CNT);
    writel(WDT_LOCK, base + WDT_CTRL);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sp_wdt_stop(wdev: *mut watchdog_device) -> c_int {
    static int sp_wdt_stop(struct watchdog_device *wdev)
    {
    struct sp_wdt_priv *priv = watchdog_get_drvdata(wdev);
    void __iomem *base = priv.base;
    writel(WDT_STOP, base + WDT_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sp_wdt_start(wdev: *mut watchdog_device) -> c_int {
    static int sp_wdt_start(struct watchdog_device *wdev)
    {
    struct sp_wdt_priv *priv = watchdog_get_drvdata(wdev);
    void __iomem *base = priv.base;
    writel(WDT_RESUME, base + WDT_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sp_wdt_get_timeleft(wdev: *mut watchdog_device) -> c_uint {
    static unsigned int sp_wdt_get_timeleft(struct watchdog_device *wdev)
    {
    struct sp_wdt_priv *priv = watchdog_get_drvdata(wdev);
    void __iomem *base = priv.base;
    u32 val;
    val = readl(base + WDT_CNT);
    val &= 0xffff;
    val = val << 4;
    return val;
    }
    static const struct watchdog_info sp_wdt_info = {
    .identity	= DEVICE_NAME,
    .options	= WDIOF_SETTIMEOUT |
    WDIOF_MAGICCLOSE |
    WDIOF_KEEPALIVEPING,
    };
    static const struct watchdog_ops sp_wdt_ops = {
    .owner		= THIS_MODULE,
    .start		= sp_wdt_start,
    .stop		= sp_wdt_stop,
    .ping		= sp_wdt_ping,
    .get_timeleft	= sp_wdt_get_timeleft,
    .restart	= sp_wdt_restart,
    };
#[no_mangle]
unsafe extern "C" fn sp_reset_control_assert(data: *mut c_void) {
    static void sp_reset_control_assert(void *data)
    {
    reset_control_assert(data);
    }
#[no_mangle]
unsafe extern "C" fn sp_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int sp_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct sp_wdt_priv *priv;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk))
    return dev_err_probe(dev, PTR_ERR(priv.clk), "Failed to enable clock\n");
// The timer and watchdog shared the STC reset
    priv.rstc = devm_reset_control_get_shared(dev, core::ptr::null_mut());
    if (IS_ERR(priv.rstc))
    return dev_err_probe(dev, PTR_ERR(priv.rstc), "Failed to get reset\n");
    reset_control_deassert(priv.rstc);
    ret = devm_add_action_or_reset(dev, sp_reset_control_assert, priv.rstc);
    if (ret)
    return ret;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    priv.wdev.info = &sp_wdt_info;
    priv.wdev.ops = &sp_wdt_ops;
    priv.wdev.timeout = SP_WDT_DEFAULT_TIMEOUT;
    priv.wdev.max_hw_heartbeat_ms = SP_WDT_MAX_TIMEOUT * 1000;
    priv.wdev.min_timeout = 1;
    priv.wdev.parent = dev;
    watchdog_set_drvdata(&priv.wdev, priv);
    watchdog_init_timeout(&priv.wdev, timeout, dev);
    watchdog_set_nowayout(&priv.wdev, nowayout);
    watchdog_stop_on_reboot(&priv.wdev);
    watchdog_set_restart_priority(&priv.wdev, 128);
    return devm_watchdog_register_device(dev, &priv.wdev);
    }
    static const struct of_device_id sp_wdt_of_match[] = {
    {.compatible = "sunplus,sp7021-wdt", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, sp_wdt_of_match);
    static struct platform_driver sp_wdt_driver = {
    .probe = sp_wdt_probe,
    .driver = {
    .name = DEVICE_NAME,
    .of_match_table = sp_wdt_of_match,
    },
    };
    module_platform_driver(sp_wdt_driver);
    MODULE_AUTHOR("Xiantao Hu <xt.hu@cqplus1.com>");
    MODULE_DESCRIPTION("Sunplus Watchdog Timer Driver");
    MODULE_LICENSE("GPL");
