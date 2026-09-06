//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/stpmic1_wdt.c
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
// Copyright (C) STMicroelectronics 2018
// Author: Pascal Paillet <p.paillet@st.com> for STMicroelectronics.

// WATCHDOG CONTROL REGISTER bit

pub const WDT_STOP: c_int = 0;
pub const PMIC_WDT_MIN_TIMEOUT: c_int = 1;
pub const PMIC_WDT_MAX_TIMEOUT: c_int = 256;
pub const PMIC_WDT_DEFAULT_TIMEOUT: c_int = 30;
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stpmic1_wdt {
    pub pmic: *mut stpmic1,
    pub wdtdev: watchdog_device,
}

#[no_mangle]
unsafe extern "C" fn pmic_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int pmic_wdt_start(struct watchdog_device *wdd)
    {
    struct stpmic1_wdt *wdt = watchdog_get_drvdata(wdd);
    return regmap_update_bits(wdt.pmic.regmap,
    WCHDG_CR, WDT_START_MASK, WDT_START);
    }
#[no_mangle]
unsafe extern "C" fn pmic_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int pmic_wdt_stop(struct watchdog_device *wdd)
    {
    struct stpmic1_wdt *wdt = watchdog_get_drvdata(wdd);
    return regmap_update_bits(wdt.pmic.regmap,
    WCHDG_CR, WDT_START_MASK, WDT_STOP);
    }
#[no_mangle]
unsafe extern "C" fn pmic_wdt_ping(wdd: *mut watchdog_device) -> c_int {
    static int pmic_wdt_ping(struct watchdog_device *wdd)
    {
    struct stpmic1_wdt *wdt = watchdog_get_drvdata(wdd);
    return regmap_update_bits(wdt.pmic.regmap,
    WCHDG_CR, WDT_PING_MASK, WDT_PING);
    }
    static int pmic_wdt_set_timeout(struct watchdog_device *wdd,
    unsigned int timeout)
    {
    struct stpmic1_wdt *wdt = watchdog_get_drvdata(wdd);
    wdd.timeout = timeout;
// timeout is equal to register value + 1
    return regmap_write(wdt.pmic.regmap, WCHDG_TIMER_CR, timeout - 1);
    }
    static const struct watchdog_info pmic_watchdog_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE,
    .identity = "STPMIC1 PMIC Watchdog",
    };
    static const struct watchdog_ops pmic_watchdog_ops = {
    .owner = THIS_MODULE,
    .start = pmic_wdt_start,
    .stop = pmic_wdt_stop,
    .ping = pmic_wdt_ping,
    .set_timeout = pmic_wdt_set_timeout,
    };
#[no_mangle]
unsafe extern "C" fn pmic_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int pmic_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    int ret;
    struct stpmic1 *pmic;
    struct stpmic1_wdt *wdt;
    if (!dev.parent)
    return -EINVAL;
    pmic = dev_get_drvdata(dev.parent);
    if (!pmic)
    return -EINVAL;
    wdt = devm_kzalloc(dev, sizeof(struct stpmic1_wdt), GFP_KERNEL);
    if (!wdt)
    return -ENOMEM;
    wdt.pmic = pmic;
    wdt.wdtdev.info = &pmic_watchdog_info;
    wdt.wdtdev.ops = &pmic_watchdog_ops;
    wdt.wdtdev.min_timeout = PMIC_WDT_MIN_TIMEOUT;
    wdt.wdtdev.max_timeout = PMIC_WDT_MAX_TIMEOUT;
    wdt.wdtdev.parent = dev;
    wdt.wdtdev.timeout = PMIC_WDT_DEFAULT_TIMEOUT;
    watchdog_init_timeout(&wdt.wdtdev, 0, dev);
    watchdog_set_nowayout(&wdt.wdtdev, nowayout);
    watchdog_set_drvdata(&wdt.wdtdev, wdt);
    ret = devm_watchdog_register_device(dev, &wdt.wdtdev);
    if (ret)
    return ret;
    dev_dbg(wdt.pmic.dev, "PMIC Watchdog driver probed\n");
    return 0;
    }
    static const struct of_device_id of_pmic_wdt_match[] = {
    { .compatible = "st,stpmic1-wdt" },
    { },
    };
    MODULE_DEVICE_TABLE(of, of_pmic_wdt_match);
    static struct platform_driver stpmic1_wdt_driver = {
    .probe = pmic_wdt_probe,
    .driver = {
    .name = "stpmic1-wdt",
    .of_match_table = of_pmic_wdt_match,
    },
    };
    module_platform_driver(stpmic1_wdt_driver);
    MODULE_DESCRIPTION("Watchdog driver for STPMIC1 device");
    MODULE_AUTHOR("Pascal Paillet <p.paillet@st.com>");
    MODULE_LICENSE("GPL v2");
