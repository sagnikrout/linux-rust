//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/cgbc_wdt.c
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
// Congatec Board Controller watchdog driver
//
// Copyright (C) 2024 Bootlin
// Author: Thomas Richard <thomas.richard@bootlin.com>
//

pub const CGBC_WDT_CMD_TRIGGER: c_uint = 0x27;
pub const CGBC_WDT_CMD_INIT: c_uint = 0x28;
pub const CGBC_WDT_DISABLE: c_uint = 0x00;
pub const CGBC_WDT_MODE_SINGLE_EVENT: c_uint = 0x02;
pub const CGBC_WDT_MIN_TIMEOUT: c_int = 1;

pub const CGBC_WDT_DEFAULT_TIMEOUT: c_int = 30;
pub const CGBC_WDT_DEFAULT_PRETIMEOUT: c_int = 0;
    enum action {
    ACTION_INT = 0,
    ACTION_SMI,
    ACTION_RESET,
    ACTION_BUTTON,
    };
    static unsigned int timeout;
    module_param(timeout, uint, 0);
    MODULE_PARM_DESC(timeout,
    "Watchdog timeout in seconds. (>=0, default="
    __MODULE_STRING(CGBC_WDT_DEFAULT_TIMEOUT) ")");
    let mut pretimeout: static unsigned int = CGBC_WDT_DEFAULT_PRETIMEOUT;
    module_param(pretimeout, uint, 0);
    MODULE_PARM_DESC(pretimeout,
    "Watchdog pretimeout in seconds. (>=0, default="
    __MODULE_STRING(CGBC_WDT_DEFAULT_PRETIMEOUT) ")");
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgbc_wdt_data {
    pub cgbc: *mut cgbc_device_data,
    pub wdd: watchdog_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgbc_wdt_cmd_cfg {
    pub cmd: u8,
    pub mode: u8,
    pub action: u8,
    pub timeout1: [u8; 3],
    pub timeout2: [u8; 3],
    pub reserved: [u8; 3],
    pub delay: [u8; 3],
    pub __packed: },
    pub 15): static_assert(sizeof(struct cgbc_wdt_cmd_cfg) ==,
#[no_mangle]
unsafe extern "C" fn cgbc_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int cgbc_wdt_start(struct watchdog_device *wdd)
    {
    pub watchdog_get_drvdata(wdd): *mut *mut cgbc_wdt_data wdt_data =,
    pub wdt_data->cgbc: *mut *mut cgbc_device_data cgbc =,
    pub 1000: *mut *mut unsigned int timeout1 = (wdd->timeout - wdd->pretimeout),
    pub 1000: *mut *mut unsigned int timeout2 = wdd->pretimeout,
    pub action: u8,
    struct cgbc_wdt_cmd_cfg cmd_start = {
    .cmd = CGBC_WDT_CMD_INIT,
    .mode = CGBC_WDT_MODE_SINGLE_EVENT,
    .timeout1[0] = (u8)timeout1,
    .timeout1[1] = (u8)(timeout1 >> 8),
    .timeout1[2] = (u8)(timeout1 >> 16),
    .timeout2[0] = (u8)timeout2,
    .timeout2[1] = (u8)(timeout2 >> 8),
    .timeout2[2] = (u8)(timeout2 >> 16),
}

    if (wdd.pretimeout) {
    action = 2;
    action |= ACTION_SMI << 2;
    action |= ACTION_RESET << 4;
    } else {
    action = 1;
    action |= ACTION_RESET << 2;
    }
    cmd_start.action = action;
    return cgbc_command(cgbc, &cmd_start, sizeof(cmd_start), core::ptr::null_mut(), 0, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn cgbc_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int cgbc_wdt_stop(struct watchdog_device *wdd)
    {
    struct cgbc_wdt_data *wdt_data = watchdog_get_drvdata(wdd);
    struct cgbc_device_data *cgbc = wdt_data.cgbc;
    struct cgbc_wdt_cmd_cfg cmd_stop = {
    .cmd = CGBC_WDT_CMD_INIT,
    .mode = CGBC_WDT_DISABLE,
    };
    return cgbc_command(cgbc, &cmd_stop, sizeof(cmd_stop), core::ptr::null_mut(), 0, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn cgbc_wdt_keepalive(wdd: *mut watchdog_device) -> c_int {
    static int cgbc_wdt_keepalive(struct watchdog_device *wdd)
    {
    struct cgbc_wdt_data *wdt_data = watchdog_get_drvdata(wdd);
    struct cgbc_device_data *cgbc = wdt_data.cgbc;
    let mut cmd_ping: u8 = CGBC_WDT_CMD_TRIGGER;
    return cgbc_command(cgbc, &cmd_ping, sizeof(cmd_ping), core::ptr::null_mut(), 0, core::ptr::null_mut());
    }
    static int cgbc_wdt_set_pretimeout(struct watchdog_device *wdd,
    unsigned int pretimeout)
    {
    wdd.pretimeout = pretimeout;
    if (watchdog_active(wdd))
    return cgbc_wdt_start(wdd);
    return 0;
    }
    static int cgbc_wdt_set_timeout(struct watchdog_device *wdd,
    unsigned int timeout)
    {
    if (timeout < wdd.pretimeout)
    wdd.pretimeout = 0;
    wdd.timeout = timeout;
    if (watchdog_active(wdd))
    return cgbc_wdt_start(wdd);
    return 0;
    }
    static const struct watchdog_info cgbc_wdt_info = {
    .identity	= "CGBC Watchdog",
    .options	= WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING |
    WDIOF_MAGICCLOSE | WDIOF_PRETIMEOUT
    };
    static const struct watchdog_ops cgbc_wdt_ops = {
    .owner		= THIS_MODULE,
    .start		= cgbc_wdt_start,
    .stop		= cgbc_wdt_stop,
    .ping		= cgbc_wdt_keepalive,
    .set_timeout	= cgbc_wdt_set_timeout,
    .set_pretimeout = cgbc_wdt_set_pretimeout,
    };
#[no_mangle]
unsafe extern "C" fn cgbc_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int cgbc_wdt_probe(struct platform_device *pdev)
    {
    struct cgbc_device_data *cgbc = dev_get_drvdata(pdev.dev.parent);
    struct device *dev = &pdev.dev;
    struct cgbc_wdt_data *wdt_data;
    struct watchdog_device *wdd;
    wdt_data = devm_kzalloc(dev, sizeof(*wdt_data), GFP_KERNEL);
    if (!wdt_data)
    return -ENOMEM;
    wdt_data.cgbc = cgbc;
    wdd = &wdt_data.wdd;
    wdd.parent = dev;
    wdd.info = &cgbc_wdt_info;
    wdd.ops = &cgbc_wdt_ops;
    wdd.max_timeout = CGBC_WDT_MAX_TIMEOUT;
    wdd.min_timeout = CGBC_WDT_MIN_TIMEOUT;
    watchdog_set_drvdata(wdd, wdt_data);
    watchdog_set_nowayout(wdd, nowayout);
    wdd.timeout = CGBC_WDT_DEFAULT_TIMEOUT;
    watchdog_init_timeout(wdd, timeout, dev);
    cgbc_wdt_set_pretimeout(wdd, pretimeout);
    platform_set_drvdata(pdev, wdt_data);
    watchdog_stop_on_reboot(wdd);
    watchdog_stop_on_unregister(wdd);
    return devm_watchdog_register_device(dev, wdd);
    }
    static struct platform_driver cgbc_wdt_driver = {
    .driver		= {
    .name	= "cgbc-wdt",
    },
    .probe		= cgbc_wdt_probe,
    };
    module_platform_driver(cgbc_wdt_driver);
    MODULE_DESCRIPTION("Congatec Board Controller Watchdog Driver");
    MODULE_AUTHOR("Thomas Richard <thomas.richard@bootlin.com>");
    MODULE_LICENSE("GPL");
