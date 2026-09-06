//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/sl28cpld_wdt.c
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
// sl28cpld watchdog driver
//
// Copyright 2020 Kontron Europe GmbH
//

//
// Watchdog timer block registers.
//
pub const WDT_CTRL: c_uint = 0x00;

pub const WDT_TIMEOUT: c_uint = 0x01;
pub const WDT_KICK: c_uint = 0x02;
pub const WDT_KICK_VALUE: c_uint = 0x6b;
pub const WDT_COUNT: c_uint = 0x03;
pub const WDT_DEFAULT_TIMEOUT: c_int = 10;
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
    static int timeout;
    module_param(timeout, int, 0);
    MODULE_PARM_DESC(timeout, "Initial watchdog timeout in seconds");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sl28cpld_wdt {
    pub wdd: watchdog_device,
    pub regmap: *mut regmap,
    pub offset: u32,
    pub assert_wdt_timeout: bool,
}

#[no_mangle]
unsafe extern "C" fn sl28cpld_wdt_ping(wdd: *mut watchdog_device) -> c_int {
    static int sl28cpld_wdt_ping(struct watchdog_device *wdd)
    {
    struct sl28cpld_wdt *wdt = watchdog_get_drvdata(wdd);
    return regmap_write(wdt.regmap, wdt.offset + WDT_KICK,
    WDT_KICK_VALUE);
    }
#[no_mangle]
unsafe extern "C" fn sl28cpld_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int sl28cpld_wdt_start(struct watchdog_device *wdd)
    {
    struct sl28cpld_wdt *wdt = watchdog_get_drvdata(wdd);
    unsigned int val;
    val = WDT_CTRL_EN | WDT_CTRL_ASSERT_SYS_RESET;
    if (wdt.assert_wdt_timeout)
    val |= WDT_CTRL_ASSERT_WDT_TIMEOUT;
    if (nowayout)
    val |= WDT_CTRL_LOCK;
    return regmap_update_bits(wdt.regmap, wdt.offset + WDT_CTRL,
    val, val);
    }
#[no_mangle]
unsafe extern "C" fn sl28cpld_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int sl28cpld_wdt_stop(struct watchdog_device *wdd)
    {
    struct sl28cpld_wdt *wdt = watchdog_get_drvdata(wdd);
    return regmap_update_bits(wdt.regmap, wdt.offset + WDT_CTRL,
    WDT_CTRL_EN, 0);
    }
#[no_mangle]
unsafe extern "C" fn sl28cpld_wdt_get_timeleft(wdd: *mut watchdog_device) -> c_uint {
    static unsigned int sl28cpld_wdt_get_timeleft(struct watchdog_device *wdd)
    {
    struct sl28cpld_wdt *wdt = watchdog_get_drvdata(wdd);
    unsigned int val;
    int ret;
    ret = regmap_read(wdt.regmap, wdt.offset + WDT_COUNT, &val);
    if (ret)
    return 0;
    return val;
    }
    static int sl28cpld_wdt_set_timeout(struct watchdog_device *wdd,
    unsigned int timeout)
    {
    struct sl28cpld_wdt *wdt = watchdog_get_drvdata(wdd);
    int ret;
    ret = regmap_write(wdt.regmap, wdt.offset + WDT_TIMEOUT, timeout);
    if (ret)
    return ret;
    wdd.timeout = timeout;
    return 0;
    }
    static const struct watchdog_info sl28cpld_wdt_info = {
    .options = WDIOF_MAGICCLOSE | WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING,
    .identity = "sl28cpld watchdog",
    };
    static const struct watchdog_ops sl28cpld_wdt_ops = {
    .owner = THIS_MODULE,
    .start = sl28cpld_wdt_start,
    .stop = sl28cpld_wdt_stop,
    .ping = sl28cpld_wdt_ping,
    .set_timeout = sl28cpld_wdt_set_timeout,
    .get_timeleft = sl28cpld_wdt_get_timeleft,
    };
#[no_mangle]
unsafe extern "C" fn sl28cpld_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int sl28cpld_wdt_probe(struct platform_device *pdev)
    {
    struct watchdog_device *wdd;
    struct sl28cpld_wdt *wdt;
    unsigned int status;
    unsigned int val;
    int ret;
    if (!pdev.dev.parent)
    return -ENODEV;
    wdt = devm_kzalloc(&pdev.dev, sizeof(*wdt), GFP_KERNEL);
    if (!wdt)
    return -ENOMEM;
    wdt.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!wdt.regmap)
    return -ENODEV;
    ret = device_property_read_u32(&pdev.dev, "reg", &wdt.offset);
    if (ret)
    return -EINVAL;
    wdt.assert_wdt_timeout = device_property_read_bool(&pdev.dev,
    "kontron,assert-wdt-timeout-pin");
// initialize struct watchdog_device
    wdd = &wdt.wdd;
    wdd.parent = &pdev.dev;
    wdd.info = &sl28cpld_wdt_info;
    wdd.ops = &sl28cpld_wdt_ops;
    wdd.min_timeout = 1;
    wdd.max_timeout = 255;
    watchdog_set_drvdata(wdd, wdt);
    watchdog_stop_on_reboot(wdd);
//
// Read the status early, in case of an error, we haven't modified the
// hardware.
//
    ret = regmap_read(wdt.regmap, wdt.offset + WDT_CTRL, &status);
    if (ret)
    return ret;
//
// Initial timeout value, may be overwritten by device tree or module
// parameter in watchdog_init_timeout().
//
// Reading a zero here means that either the hardware has a default
// value of zero (which is very unlikely and definitely a hardware
// bug) or the bootloader set it to zero. In any case, we handle
// this case gracefully and set out own timeout.
//
    ret = regmap_read(wdt.regmap, wdt.offset + WDT_TIMEOUT, &val);
    if (ret)
    return ret;
    if (val)
    wdd.timeout = val;
    else
    wdd.timeout = WDT_DEFAULT_TIMEOUT;
    watchdog_init_timeout(wdd, timeout, &pdev.dev);
    sl28cpld_wdt_set_timeout(wdd, wdd.timeout);
// if the watchdog is locked, we set nowayout
    if (status & WDT_CTRL_LOCK)
    nowayout = true;
    watchdog_set_nowayout(wdd, nowayout);
//
// If watchdog is already running, keep it enabled, but make
// sure its mode is set correctly.
//
    if (status & WDT_CTRL_EN) {
    sl28cpld_wdt_start(wdd);
    set_bit(WDOG_HW_RUNNING, &wdd.status);
    }
    ret = devm_watchdog_register_device(&pdev.dev, wdd);
    if (ret < 0)
    return ret;
    dev_info(&pdev.dev, "initial timeout %d sec%s\n",
    wdd.timeout, nowayout ? ", nowayout" : "");
    return 0;
    }
    static const struct of_device_id sl28cpld_wdt_of_match[] = {
    { .compatible = "kontron,sl28cpld-wdt" },
    {}
    };
    MODULE_DEVICE_TABLE(of, sl28cpld_wdt_of_match);
    static struct platform_driver sl28cpld_wdt_driver = {
    .probe = sl28cpld_wdt_probe,
    .driver = {
    .name = "sl28cpld-wdt",
    .of_match_table = sl28cpld_wdt_of_match,
    },
    };
    module_platform_driver(sl28cpld_wdt_driver);
    MODULE_DESCRIPTION("sl28cpld Watchdog Driver");
    MODULE_AUTHOR("Michael Walle <michael@walle.cc>");
    MODULE_LICENSE("GPL");
