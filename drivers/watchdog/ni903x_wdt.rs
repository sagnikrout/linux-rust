//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/ni903x_wdt.c
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
// Copyright (C) 2016 National Instruments Corp.
//

pub const NIWD_CONTROL: c_uint = 0x01;
pub const NIWD_COUNTER2: c_uint = 0x02;
pub const NIWD_COUNTER1: c_uint = 0x03;
pub const NIWD_COUNTER0: c_uint = 0x04;
pub const NIWD_SEED2: c_uint = 0x05;
pub const NIWD_SEED1: c_uint = 0x06;
pub const NIWD_SEED0: c_uint = 0x07;
pub const NIWD_IO_SIZE: c_uint = 0x08;
pub const NIWD_CONTROL_MODE: c_uint = 0x80;
pub const NIWD_CONTROL_PROC_RESET: c_uint = 0x20;
pub const NIWD_CONTROL_PET: c_uint = 0x10;
pub const NIWD_CONTROL_RUNNING: c_uint = 0x08;
pub const NIWD_CONTROL_CAPTURECOUNTER: c_uint = 0x04;
pub const NIWD_CONTROL_RESET: c_uint = 0x02;
pub const NIWD_CONTROL_ALARM: c_uint = 0x01;
pub const NIWD_PERIOD_NS: c_int = 30720;
pub const NIWD_MIN_TIMEOUT: c_int = 1;
pub const NIWD_MAX_TIMEOUT: c_int = 515;
pub const NIWD_DEFAULT_TIMEOUT: c_int = 60;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ni903x_wdt {
    pub dev: *mut device,
    pub io_base: u16,
    pub wdd: watchdog_device,
}

    static unsigned int timeout;
    module_param(timeout, uint, 0);
    MODULE_PARM_DESC(timeout,
    "Watchdog timeout in seconds. (default="
    __MODULE_STRING(NIWD_DEFAULT_TIMEOUT) ")");
    let mut nowayout: static int = WATCHDOG_NOWAYOUT;
    module_param(nowayout, int, S_IRUGO);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[no_mangle]
unsafe extern "C" fn ni903x_start(wdt: *mut ni903x_wdt) {
    static void ni903x_start(struct ni903x_wdt *wdt)
    {
    let mut control: u8 = inb(wdt.io_base + NIWD_CONTROL);
    outb(control | NIWD_CONTROL_RESET, wdt.io_base + NIWD_CONTROL);
    outb(control | NIWD_CONTROL_PET, wdt.io_base + NIWD_CONTROL);
    }
    static int ni903x_wdd_set_timeout(struct watchdog_device *wdd,
    unsigned int timeout)
    {
    struct ni903x_wdt *wdt = watchdog_get_drvdata(wdd);
    let mut counter: u32 = timeout * (1000000000 / NIWD_PERIOD_NS);
    outb(((0x00FF0000 & counter) >> 16), wdt.io_base + NIWD_SEED2);
    outb(((0x0000FF00 & counter) >> 8), wdt.io_base + NIWD_SEED1);
    outb((0x000000FF & counter), wdt.io_base + NIWD_SEED0);
    wdd.timeout = timeout;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ni903x_wdd_get_timeleft(wdd: *mut watchdog_device) -> c_uint {
    static unsigned int ni903x_wdd_get_timeleft(struct watchdog_device *wdd)
    {
    struct ni903x_wdt *wdt = watchdog_get_drvdata(wdd);
    u8 control, counter0, counter1, counter2;
    u32 counter;
    control = inb(wdt.io_base + NIWD_CONTROL);
    control |= NIWD_CONTROL_CAPTURECOUNTER;
    outb(control, wdt.io_base + NIWD_CONTROL);
    counter2 = inb(wdt.io_base + NIWD_COUNTER2);
    counter1 = inb(wdt.io_base + NIWD_COUNTER1);
    counter0 = inb(wdt.io_base + NIWD_COUNTER0);
    counter = (counter2 << 16) | (counter1 << 8) | counter0;
    return counter / (1000000000 / NIWD_PERIOD_NS);
    }
#[no_mangle]
unsafe extern "C" fn ni903x_wdd_ping(wdd: *mut watchdog_device) -> c_int {
    static int ni903x_wdd_ping(struct watchdog_device *wdd)
    {
    struct ni903x_wdt *wdt = watchdog_get_drvdata(wdd);
    u8 control;
    control = inb(wdt.io_base + NIWD_CONTROL);
    outb(control | NIWD_CONTROL_PET, wdt.io_base + NIWD_CONTROL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ni903x_wdd_start(wdd: *mut watchdog_device) -> c_int {
    static int ni903x_wdd_start(struct watchdog_device *wdd)
    {
    struct ni903x_wdt *wdt = watchdog_get_drvdata(wdd);
    outb(NIWD_CONTROL_RESET | NIWD_CONTROL_PROC_RESET,
    wdt.io_base + NIWD_CONTROL);
    ni903x_wdd_set_timeout(wdd, wdd.timeout);
    ni903x_start(wdt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ni903x_wdd_stop(wdd: *mut watchdog_device) -> c_int {
    static int ni903x_wdd_stop(struct watchdog_device *wdd)
    {
    struct ni903x_wdt *wdt = watchdog_get_drvdata(wdd);
    outb(NIWD_CONTROL_RESET, wdt.io_base + NIWD_CONTROL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ni903x_resources(res: *mut acpi_resource, data: *mut c_void) -> acpi_status {
    static acpi_status ni903x_resources(struct acpi_resource *res, void *data)
    {
    struct ni903x_wdt *wdt = data;
    u16 io_size;
    switch (res.type) {
    case ACPI_RESOURCE_TYPE_IO:
    if (wdt.io_base != 0) {
    dev_err(wdt.dev, "too many IO resources\n");
    return AE_ERROR;
    }
    wdt.io_base = res.data.io.minimum;
    io_size = res.data.io.address_length;
    if (io_size < NIWD_IO_SIZE) {
    dev_err(wdt.dev, "memory region too small\n");
    return AE_ERROR;
    }
    if (!devm_request_region(wdt.dev, wdt.io_base, io_size,
    NIWD_NAME)) {
    dev_err(wdt.dev, "failed to get memory region\n");
    return AE_ERROR;
    }
    return AE_OK;
    case ACPI_RESOURCE_TYPE_END_TAG:
    default:
// Ignore unsupported resources, e.g. IRQ
    return AE_OK;
    }
    }
    static const struct watchdog_info ni903x_wdd_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE,
    .identity = "NI Watchdog",
    };
    static const struct watchdog_ops ni903x_wdd_ops = {
    .owner = THIS_MODULE,
    .start = ni903x_wdd_start,
    .stop = ni903x_wdd_stop,
    .ping = ni903x_wdd_ping,
    .set_timeout = ni903x_wdd_set_timeout,
    .get_timeleft = ni903x_wdd_get_timeleft,
    };
#[no_mangle]
unsafe extern "C" fn ni903x_acpi_probe(pdev: *mut platform_device) -> c_int {
    static int ni903x_acpi_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct watchdog_device *wdd;
    struct ni903x_wdt *wdt;
    acpi_handle handle;
    acpi_status status;
    int ret;
    handle = ACPI_HANDLE(dev);
    if (!handle)
    return -ENODEV;
    wdt = devm_kzalloc(dev, sizeof(*wdt), GFP_KERNEL);
    if (!wdt)
    return -ENOMEM;
    platform_set_drvdata(pdev, wdt);
    wdt.dev = dev;
    status = acpi_walk_resources(handle, METHOD_NAME__CRS,
    ni903x_resources, wdt);
    if (ACPI_FAILURE(status) || wdt.io_base == 0) {
    dev_err(dev, "failed to get resources\n");
    return -ENODEV;
    }
    wdd = &wdt.wdd;
    wdd.info = &ni903x_wdd_info;
    wdd.ops = &ni903x_wdd_ops;
    wdd.min_timeout = NIWD_MIN_TIMEOUT;
    wdd.max_timeout = NIWD_MAX_TIMEOUT;
    wdd.timeout = NIWD_DEFAULT_TIMEOUT;
    wdd.parent = dev;
    watchdog_set_drvdata(wdd, wdt);
    watchdog_set_nowayout(wdd, nowayout);
    watchdog_init_timeout(wdd, timeout, dev);
    ret = watchdog_register_device(wdd);
    if (ret)
    return ret;
// Switch from boot mode to user mode
    outb(NIWD_CONTROL_RESET | NIWD_CONTROL_MODE,
    wdt.io_base + NIWD_CONTROL);
    dev_dbg(dev, "io_base=0x%04X, timeout=%d, nowayout=%d\n",
    wdt.io_base, timeout, nowayout);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ni903x_acpi_remove(pdev: *mut platform_device) {
    static void ni903x_acpi_remove(struct platform_device *pdev)
    {
    struct ni903x_wdt *wdt = platform_get_drvdata(pdev);
    ni903x_wdd_stop(&wdt.wdd);
    watchdog_unregister_device(&wdt.wdd);
    }
    static const struct acpi_device_id ni903x_device_ids[] = {
    {"NIC775C", 0},
    {"", 0},
    };
    MODULE_DEVICE_TABLE(acpi, ni903x_device_ids);
    static struct platform_driver ni903x_acpi_driver = {
    .probe = ni903x_acpi_probe,
    .remove = ni903x_acpi_remove,
    .driver = {
    .name = NIWD_NAME,
    .acpi_match_table = ni903x_device_ids,
    },
    };
    module_platform_driver(ni903x_acpi_driver);
    MODULE_DESCRIPTION("NI 903x Watchdog");
    MODULE_AUTHOR("Jeff Westfahl <jeff.westfahl@ni.com>");
    MODULE_AUTHOR("Kyle Roeschley <kyle.roeschley@ni.com>");
    MODULE_LICENSE("GPL");
