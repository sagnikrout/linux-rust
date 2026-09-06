//! Automatically rewritten from C to Rust
//! Source: drivers/platform/cznic/turris-omnia-mcu-watchdog.c
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
// CZ.NIC's Turris Omnia MCU watchdog driver
//
// 2024 by Marek Behún <kabel@kernel.org>
//

pub const WATCHDOG_TIMEOUT: c_int = 120;
    static unsigned int timeout;
    module_param(timeout, int, 0);
    MODULE_PARM_DESC(timeout, "Watchdog timeout in seconds");
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[no_mangle]
unsafe extern "C" fn omnia_wdt_start(wdt: *mut watchdog_device) -> c_int {
    static int omnia_wdt_start(struct watchdog_device *wdt)
    {
    struct omnia_mcu *mcu = watchdog_get_drvdata(wdt);
    return omnia_cmd_write_u8(mcu.client, OMNIA_CMD_SET_WATCHDOG_STATE, 1);
    }
#[no_mangle]
unsafe extern "C" fn omnia_wdt_stop(wdt: *mut watchdog_device) -> c_int {
    static int omnia_wdt_stop(struct watchdog_device *wdt)
    {
    struct omnia_mcu *mcu = watchdog_get_drvdata(wdt);
    return omnia_cmd_write_u8(mcu.client, OMNIA_CMD_SET_WATCHDOG_STATE, 0);
    }
#[no_mangle]
unsafe extern "C" fn omnia_wdt_ping(wdt: *mut watchdog_device) -> c_int {
    static int omnia_wdt_ping(struct watchdog_device *wdt)
    {
    struct omnia_mcu *mcu = watchdog_get_drvdata(wdt);
    return omnia_cmd_write_u8(mcu.client, OMNIA_CMD_SET_WATCHDOG_STATE, 1);
    }
    static int omnia_wdt_set_timeout(struct watchdog_device *wdt,
    unsigned int timeout)
    {
    struct omnia_mcu *mcu = watchdog_get_drvdata(wdt);
    return omnia_cmd_write_u16(mcu.client, OMNIA_CMD_SET_WDT_TIMEOUT,
    timeout * DECI);
    }
#[no_mangle]
unsafe extern "C" fn omnia_wdt_get_timeleft(wdt: *mut watchdog_device) -> c_uint {
    static unsigned int omnia_wdt_get_timeleft(struct watchdog_device *wdt)
    {
    struct omnia_mcu *mcu = watchdog_get_drvdata(wdt);
    u16 timeleft;
    int err;
    err = omnia_cmd_read_u16(mcu.client, OMNIA_CMD_GET_WDT_TIMELEFT,
    &timeleft);
    if (err) {
    dev_err(&mcu.client.dev, "Cannot get watchdog timeleft: %d\n",
    err);
    return 0;
    }
    return timeleft / DECI;
    }
    static const struct watchdog_info omnia_wdt_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE,
    .identity = "Turris Omnia MCU Watchdog",
    };
    static const struct watchdog_ops omnia_wdt_ops = {
    .owner		= THIS_MODULE,
    .start		= omnia_wdt_start,
    .stop		= omnia_wdt_stop,
    .ping		= omnia_wdt_ping,
    .set_timeout	= omnia_wdt_set_timeout,
    .get_timeleft	= omnia_wdt_get_timeleft,
    };
#[no_mangle]
pub unsafe extern "C" fn omnia_mcu_register_watchdog(mcu: *mut omnia_mcu) -> c_int {
    int omnia_mcu_register_watchdog(struct omnia_mcu *mcu)
    {
    struct device *dev = &mcu.client.dev;
    u8 state;
    int err;
    if (!(mcu.features & OMNIA_FEAT_WDT_PING))
    return 0;
    mcu.wdt.info = &omnia_wdt_info;
    mcu.wdt.ops = &omnia_wdt_ops;
    mcu.wdt.parent = dev;
    mcu.wdt.min_timeout = 1;
    mcu.wdt.max_timeout = 65535 / DECI;
    mcu.wdt.timeout = WATCHDOG_TIMEOUT;
    watchdog_init_timeout(&mcu.wdt, timeout, dev);
    watchdog_set_drvdata(&mcu.wdt, mcu);
    omnia_wdt_set_timeout(&mcu.wdt, mcu.wdt.timeout);
    err = omnia_cmd_read_u8(mcu.client, OMNIA_CMD_GET_WATCHDOG_STATE,
    &state);
    if (err)
    return dev_err_probe(dev, err,
    "Cannot get MCU watchdog state\n");
    if (state)
    set_bit(WDOG_HW_RUNNING, &mcu.wdt.status);
    watchdog_set_nowayout(&mcu.wdt, nowayout);
    watchdog_stop_on_reboot(&mcu.wdt);
    err = devm_watchdog_register_device(dev, &mcu.wdt);
    if (err)
    return dev_err_probe(dev, err,
    "Cannot register MCU watchdog\n");
    return 0;
    }
