//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/menf21bmc_wdt.c
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
// MEN 14F021P00 Board Management Controller (BMC) Watchdog Driver.
//
// Copyright (C) 2014 MEN Mikro Elektronik Nuernberg GmbH
//

pub const BMC_CMD_WD_ON: c_uint = 0x11;
pub const BMC_CMD_WD_OFF: c_uint = 0x12;
pub const BMC_CMD_WD_TRIG: c_uint = 0x13;
pub const BMC_CMD_WD_TIME: c_uint = 0x14;
pub const BMC_CMD_WD_STATE: c_uint = 0x17;
pub const BMC_WD_OFF_VAL: c_uint = 0x69;
pub const BMC_CMD_RST_RSN: c_uint = 0x92;

    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct menf21bmc_wdt {
    pub wdt: watchdog_device,
    pub i2c_client: *mut i2c_client,
}

#[no_mangle]
unsafe extern "C" fn menf21bmc_wdt_set_bootstatus(data: *mut menf21bmc_wdt) -> c_int {
    static int menf21bmc_wdt_set_bootstatus(struct menf21bmc_wdt *data)
    {
    int rst_rsn;
    rst_rsn = i2c_smbus_read_byte_data(data.i2c_client, BMC_CMD_RST_RSN);
    if (rst_rsn < 0)
    return rst_rsn;
    if (rst_rsn == 0x02)
    data.wdt.bootstatus |= WDIOF_CARDRESET;
#[no_mangle]
pub unsafe extern "C" fn if(0x05: rst_rsn ==) -> else {
    else if (rst_rsn == 0x05)
    data.wdt.bootstatus |= WDIOF_EXTERN1;
#[no_mangle]
pub unsafe extern "C" fn if(0x06: rst_rsn ==) -> else {
    else if (rst_rsn == 0x06)
    data.wdt.bootstatus |= WDIOF_EXTERN2;
#[no_mangle]
pub unsafe extern "C" fn if(0x0A: rst_rsn ==) -> else {
    else if (rst_rsn == 0x0A)
    data.wdt.bootstatus |= WDIOF_POWERUNDER;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn menf21bmc_wdt_start(wdt: *mut watchdog_device) -> c_int {
    static int menf21bmc_wdt_start(struct watchdog_device *wdt)
    {
    struct menf21bmc_wdt *drv_data = watchdog_get_drvdata(wdt);
    return i2c_smbus_write_byte(drv_data.i2c_client, BMC_CMD_WD_ON);
    }
#[no_mangle]
unsafe extern "C" fn menf21bmc_wdt_stop(wdt: *mut watchdog_device) -> c_int {
    static int menf21bmc_wdt_stop(struct watchdog_device *wdt)
    {
    struct menf21bmc_wdt *drv_data = watchdog_get_drvdata(wdt);
    return i2c_smbus_write_byte_data(drv_data.i2c_client,
    BMC_CMD_WD_OFF, BMC_WD_OFF_VAL);
    }
    static int
    menf21bmc_wdt_settimeout(struct watchdog_device *wdt, unsigned int timeout)
    {
    int ret;
    struct menf21bmc_wdt *drv_data = watchdog_get_drvdata(wdt);
//
// BMC Watchdog does have a resolution of 100ms.
// Watchdog API defines the timeout in seconds, so we have to
// multiply the value.
//
    ret = i2c_smbus_write_word_data(drv_data.i2c_client,
    BMC_CMD_WD_TIME, timeout * 10);
    if (ret < 0)
    return ret;
    wdt.timeout = timeout;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn menf21bmc_wdt_ping(wdt: *mut watchdog_device) -> c_int {
    static int menf21bmc_wdt_ping(struct watchdog_device *wdt)
    {
    struct menf21bmc_wdt *drv_data = watchdog_get_drvdata(wdt);
    return i2c_smbus_write_byte(drv_data.i2c_client, BMC_CMD_WD_TRIG);
    }
    static const struct watchdog_info menf21bmc_wdt_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING,
    .identity = DEVNAME,
    };
    static const struct watchdog_ops menf21bmc_wdt_ops = {
    .owner		= THIS_MODULE,
    .start		= menf21bmc_wdt_start,
    .stop		= menf21bmc_wdt_stop,
    .ping		= menf21bmc_wdt_ping,
    .set_timeout	= menf21bmc_wdt_settimeout,
    };
#[no_mangle]
unsafe extern "C" fn menf21bmc_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int menf21bmc_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    int ret, bmc_timeout;
    struct menf21bmc_wdt *drv_data;
    struct i2c_client *i2c_client = to_i2c_client(dev.parent);
    drv_data = devm_kzalloc(dev, sizeof(struct menf21bmc_wdt), GFP_KERNEL);
    if (!drv_data)
    return -ENOMEM;
    drv_data.wdt.ops = &menf21bmc_wdt_ops;
    drv_data.wdt.info = &menf21bmc_wdt_info;
    drv_data.wdt.min_timeout = BMC_WD_TIMEOUT_MIN;
    drv_data.wdt.max_timeout = BMC_WD_TIMEOUT_MAX;
    drv_data.wdt.parent = dev;
    drv_data.i2c_client = i2c_client;
//
// Get the current wdt timeout value from the BMC because
// the BMC will save the value set before if the system restarts.
//
    bmc_timeout = i2c_smbus_read_word_data(drv_data.i2c_client,
    BMC_CMD_WD_TIME);
    if (bmc_timeout < 0) {
    dev_err(dev, "failed to get current WDT timeout\n");
    return bmc_timeout;
    }
    watchdog_init_timeout(&drv_data.wdt, bmc_timeout / 10, dev);
    watchdog_set_nowayout(&drv_data.wdt, nowayout);
    watchdog_set_drvdata(&drv_data.wdt, drv_data);
    platform_set_drvdata(pdev, drv_data);
    ret = menf21bmc_wdt_set_bootstatus(drv_data);
    if (ret < 0) {
    dev_err(dev, "failed to set Watchdog bootstatus\n");
    return ret;
    }
    ret = devm_watchdog_register_device(dev, &drv_data.wdt);
    if (ret)
    return ret;
    dev_info(dev, "MEN 14F021P00 BMC Watchdog device enabled\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn menf21bmc_wdt_shutdown(pdev: *mut platform_device) {
    static void menf21bmc_wdt_shutdown(struct platform_device *pdev)
    {
    struct menf21bmc_wdt *drv_data = platform_get_drvdata(pdev);
    i2c_smbus_write_word_data(drv_data.i2c_client,
    BMC_CMD_WD_OFF, BMC_WD_OFF_VAL);
    }
    static struct  platform_driver menf21bmc_wdt = {
    .driver		= {
    .name	= DEVNAME,
    },
    .probe		= menf21bmc_wdt_probe,
    .shutdown	= menf21bmc_wdt_shutdown,
    };
    module_platform_driver(menf21bmc_wdt);
    MODULE_DESCRIPTION("MEN 14F021P00 BMC Watchdog driver");
    MODULE_AUTHOR("Andreas Werner <andreas.werner@men.de>");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:menf21bmc_wdt");
