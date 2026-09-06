//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/da9052_wdt.c
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
// System monitoring driver for DA9052 PMICs.
//
// Copyright(c) 2012 Dialog Semiconductor Ltd.
//
// Author: Anthony Olech <Anthony.Olech@diasemi.com>
//

pub const DA9052_DEF_TIMEOUT: c_int = 4;
pub const DA9052_TWDMIN: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9052_wdt_data {
    pub wdt: watchdog_device,
    pub da9052: *mut da9052,
    pub jpast: c_ulong,
}

    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
    static int timeout;
    module_param(timeout, int, 0);
    MODULE_PARM_DESC(timeout,
    "Watchdog timeout in seconds. (default = "
    __MODULE_STRING(WDT_DEFAULT_TIMEOUT) ")");
    static const struct {
    u8 reg_val;
    int time;  /* Seconds */
    } da9052_wdt_maps[] = {
    { 1, 2 },
    { 2, 4 },
    { 3, 8 },
    { 4, 16 },
    { 5, 32 },
    { 5, 33 },  /* Actual time  32.768s so included both 32s and 33s */
    { 6, 65 },
    { 6, 66 },  /* Actual time 65.536s so include both, 65s and 66s */
    { 7, 131 },
    };
    static int da9052_wdt_set_timeout(struct watchdog_device *wdt_dev,
    unsigned int timeout)
    {
    struct da9052_wdt_data *driver_data = watchdog_get_drvdata(wdt_dev);
    struct da9052 *da9052 = driver_data.da9052;
    int ret, i;
//
// Disable the Watchdog timer before setting
// new time out.
//
    ret = da9052_reg_update(da9052, DA9052_CONTROL_D_REG,
    DA9052_CONTROLD_TWDSCALE, 0);
    if (ret < 0) {
    dev_err(da9052.dev, "Failed to disable watchdog bit, %d\n",
    ret);
    return ret;
    }
    if (timeout) {
//
// To change the timeout, da9052 needs to
// be disabled for at least 150 us.
//
    udelay(150);
// Set the desired timeout
    for (i = 0; i < ARRAY_SIZE(da9052_wdt_maps); i++)
    if (da9052_wdt_maps[i].time == timeout)
    break;
    if (i == ARRAY_SIZE(da9052_wdt_maps))
    ret = -EINVAL;
    else
    ret = da9052_reg_update(da9052, DA9052_CONTROL_D_REG,
    DA9052_CONTROLD_TWDSCALE,
    da9052_wdt_maps[i].reg_val);
    if (ret < 0) {
    dev_err(da9052.dev,
    "Failed to update timescale bit, %d\n", ret);
    return ret;
    }
    wdt_dev.timeout = timeout;
    driver_data.jpast = jiffies;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn da9052_wdt_start(wdt_dev: *mut watchdog_device) -> c_int {
    static int da9052_wdt_start(struct watchdog_device *wdt_dev)
    {
    return da9052_wdt_set_timeout(wdt_dev, wdt_dev.timeout);
    }
#[no_mangle]
unsafe extern "C" fn da9052_wdt_stop(wdt_dev: *mut watchdog_device) -> c_int {
    static int da9052_wdt_stop(struct watchdog_device *wdt_dev)
    {
    return da9052_wdt_set_timeout(wdt_dev, 0);
    }
#[no_mangle]
unsafe extern "C" fn da9052_wdt_ping(wdt_dev: *mut watchdog_device) -> c_int {
    static int da9052_wdt_ping(struct watchdog_device *wdt_dev)
    {
    struct da9052_wdt_data *driver_data = watchdog_get_drvdata(wdt_dev);
    struct da9052 *da9052 = driver_data.da9052;
    unsigned long msec, jnow = jiffies;
    int ret;
//
// We have a minimum time for watchdog window called TWDMIN. A write
// to the watchdog before this elapsed time should cause an error.
//
    msec = (jnow - driver_data.jpast) * 1000/HZ;
    if (msec < DA9052_TWDMIN)
    mdelay(msec);
// Reset the watchdog timer
    ret = da9052_reg_update(da9052, DA9052_CONTROL_D_REG,
    DA9052_CONTROLD_WATCHDOG, 1 << 7);
    if (ret < 0)
    return ret;
//
// FIXME: Reset the watchdog core, in general PMIC
// is supposed to do this
//
    return da9052_reg_update(da9052, DA9052_CONTROL_D_REG,
    DA9052_CONTROLD_WATCHDOG, 0 << 7);
    }
    static const struct watchdog_info da9052_wdt_info = {
    .options =	WDIOF_SETTIMEOUT |
    WDIOF_KEEPALIVEPING |
    WDIOF_CARDRESET |
    WDIOF_OVERHEAT |
    WDIOF_POWERUNDER,
    .identity	= "DA9052 Watchdog",
    };
    static const struct watchdog_ops da9052_wdt_ops = {
    .owner = THIS_MODULE,
    .start = da9052_wdt_start,
    .stop = da9052_wdt_stop,
    .ping = da9052_wdt_ping,
    .set_timeout = da9052_wdt_set_timeout,
    };
#[no_mangle]
unsafe extern "C" fn da9052_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int da9052_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct da9052 *da9052 = dev_get_drvdata(dev.parent);
    struct da9052_wdt_data *driver_data;
    struct watchdog_device *da9052_wdt;
    int ret;
    driver_data = devm_kzalloc(dev, sizeof(*driver_data), GFP_KERNEL);
    if (!driver_data)
    return -ENOMEM;
    driver_data.da9052 = da9052;
    da9052_wdt = &driver_data.wdt;
    da9052_wdt.timeout = DA9052_DEF_TIMEOUT;
    da9052_wdt.min_hw_heartbeat_ms = DA9052_TWDMIN;
    da9052_wdt.info = &da9052_wdt_info;
    da9052_wdt.ops = &da9052_wdt_ops;
    da9052_wdt.parent = dev;
    watchdog_set_drvdata(da9052_wdt, driver_data);
    watchdog_init_timeout(da9052_wdt, timeout, dev);
    watchdog_set_nowayout(da9052_wdt, nowayout);
    if (da9052.fault_log & DA9052_FAULTLOG_TWDERROR)
    da9052_wdt.bootstatus |= WDIOF_CARDRESET;
    if (da9052.fault_log & DA9052_FAULTLOG_TEMPOVER)
    da9052_wdt.bootstatus |= WDIOF_OVERHEAT;
    if (da9052.fault_log & DA9052_FAULTLOG_VDDFAULT)
    da9052_wdt.bootstatus |= WDIOF_POWERUNDER;
    ret = da9052_reg_read(da9052, DA9052_CONTROL_D_REG);
    if (ret < 0)
    return ret;
// Check if FW enabled the watchdog
    if (ret & DA9052_CONTROLD_TWDSCALE) {
// Ensure proper initialization
    da9052_wdt_start(da9052_wdt);
    set_bit(WDOG_HW_RUNNING, &da9052_wdt.status);
    }
    return devm_watchdog_register_device(dev, &driver_data.wdt);
    }
    static struct platform_driver da9052_wdt_driver = {
    .probe = da9052_wdt_probe,
    .driver = {
    .name	= "da9052-watchdog",
    },
    };
    module_platform_driver(da9052_wdt_driver);
    MODULE_AUTHOR("Anthony Olech <Anthony.Olech@diasemi.com>");
    MODULE_DESCRIPTION("DA9052 SM Device Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:da9052-watchdog");
