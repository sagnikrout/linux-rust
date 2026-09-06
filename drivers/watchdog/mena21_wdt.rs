//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/mena21_wdt.c
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
// Watchdog driver for the A21 VME CPU Boards
//
// Copyright (C) 2013 MEN Mikro Elektronik Nuernberg GmbH
//

pub const NUM_GPIOS: c_int = 6;
    enum a21_wdt_gpios {
    GPIO_WD_ENAB,
    GPIO_WD_FAST,
    GPIO_WD_TRIG,
    GPIO_WD_RST0,
    GPIO_WD_RST1,
    GPIO_WD_RST2,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a21_wdt_drv {
    pub wdt: watchdog_device,
    pub gpios: [*mut gpio_desc; NUM_GPIOS],
}

    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[no_mangle]
unsafe extern "C" fn a21_wdt_get_bootstatus(drv: *mut a21_wdt_drv) -> c_uint {
    static unsigned int a21_wdt_get_bootstatus(struct a21_wdt_drv *drv)
    {
    let mut reset: c_int = 0;
    reset |= gpiod_get_value(drv.gpios[GPIO_WD_RST0]) ? (1 << 0) : 0;
    reset |= gpiod_get_value(drv.gpios[GPIO_WD_RST1]) ? (1 << 1) : 0;
    reset |= gpiod_get_value(drv.gpios[GPIO_WD_RST2]) ? (1 << 2) : 0;
    return reset;
    }
#[no_mangle]
unsafe extern "C" fn a21_wdt_start(wdt: *mut watchdog_device) -> c_int {
    static int a21_wdt_start(struct watchdog_device *wdt)
    {
    struct a21_wdt_drv *drv = watchdog_get_drvdata(wdt);
    gpiod_set_value(drv.gpios[GPIO_WD_ENAB], 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn a21_wdt_stop(wdt: *mut watchdog_device) -> c_int {
    static int a21_wdt_stop(struct watchdog_device *wdt)
    {
    struct a21_wdt_drv *drv = watchdog_get_drvdata(wdt);
    gpiod_set_value(drv.gpios[GPIO_WD_ENAB], 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn a21_wdt_ping(wdt: *mut watchdog_device) -> c_int {
    static int a21_wdt_ping(struct watchdog_device *wdt)
    {
    struct a21_wdt_drv *drv = watchdog_get_drvdata(wdt);
    gpiod_set_value(drv.gpios[GPIO_WD_TRIG], 0);
    ndelay(10);
    gpiod_set_value(drv.gpios[GPIO_WD_TRIG], 1);
    return 0;
    }
    static int a21_wdt_set_timeout(struct watchdog_device *wdt,
    unsigned int timeout)
    {
    struct a21_wdt_drv *drv = watchdog_get_drvdata(wdt);
    if (timeout != 1 && timeout != 30) {
    dev_err(wdt.parent, "Only 1 and 30 allowed as timeout\n");
    return -EINVAL;
    }
    if (timeout == 30 && wdt.timeout == 1) {
    dev_err(wdt.parent,
    "Transition from fast to slow mode not allowed\n");
    return -EINVAL;
    }
    if (timeout == 1)
    gpiod_set_value(drv.gpios[GPIO_WD_FAST], 1);
    else
    gpiod_set_value(drv.gpios[GPIO_WD_FAST], 0);
    wdt.timeout = timeout;
    return 0;
    }
    static const struct watchdog_info a21_wdt_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE,
    .identity = "MEN A21 Watchdog",
    };
    static const struct watchdog_ops a21_wdt_ops = {
    .owner = THIS_MODULE,
    .start = a21_wdt_start,
    .stop = a21_wdt_stop,
    .ping = a21_wdt_ping,
    .set_timeout = a21_wdt_set_timeout,
    };
    static struct watchdog_device a21_wdt = {
    .info = &a21_wdt_info,
    .ops = &a21_wdt_ops,
    .min_timeout = 1,
    .max_timeout = 30,
    };
#[no_mangle]
unsafe extern "C" fn a21_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int a21_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct a21_wdt_drv *drv;
    let mut reset: c_uint = 0;
    int num_gpios;
    int ret;
    int i;
    drv = devm_kzalloc(dev, sizeof(struct a21_wdt_drv), GFP_KERNEL);
    if (!drv)
    return -ENOMEM;
    num_gpios = gpiod_count(dev, core::ptr::null_mut());
    if (num_gpios != NUM_GPIOS) {
    dev_err(dev, "gpios DT property wrong, got %d want %d",
    num_gpios, NUM_GPIOS);
    return -ENODEV;
    }
// Request the used GPIOs
    for (i = 0; i < num_gpios; i++) {
    enum gpiod_flags gflags;
    if (i < GPIO_WD_RST0)
    gflags = GPIOD_ASIS;
    else
    gflags = GPIOD_IN;
    drv.gpios[i] = devm_gpiod_get_index(dev, core::ptr::null_mut(), i, gflags);
    if (IS_ERR(drv.gpios[i]))
    return PTR_ERR(drv.gpios[i]);
    gpiod_set_consumer_name(drv.gpios[i], "MEN A21 Watchdog");
//
// Retrieve the initial value from the GPIOs that should be
// output, then set up the line as output with that value.
//
    if (i < GPIO_WD_RST0) {
    int val;
    val = gpiod_get_value(drv.gpios[i]);
    gpiod_direction_output(drv.gpios[i], val);
    }
    }
    watchdog_init_timeout(&a21_wdt, 30, dev);
    watchdog_set_nowayout(&a21_wdt, nowayout);
    watchdog_set_drvdata(&a21_wdt, drv);
    a21_wdt.parent = dev;
    reset = a21_wdt_get_bootstatus(drv);
    if (reset == 2)
    a21_wdt.bootstatus |= WDIOF_EXTERN1;
#[no_mangle]
pub unsafe extern "C" fn if(4: reset ==) -> else {
    else if (reset == 4)
    a21_wdt.bootstatus |= WDIOF_CARDRESET;
#[no_mangle]
pub unsafe extern "C" fn if(5: reset ==) -> else {
    else if (reset == 5)
    a21_wdt.bootstatus |= WDIOF_POWERUNDER;
#[no_mangle]
pub unsafe extern "C" fn if(7: reset ==) -> else {
    else if (reset == 7)
    a21_wdt.bootstatus |= WDIOF_EXTERN2;
    drv.wdt = a21_wdt;
    dev_set_drvdata(dev, drv);
    ret = devm_watchdog_register_device(dev, &a21_wdt);
    if (ret)
    return ret;
    dev_info(dev, "MEN A21 watchdog timer driver enabled\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn a21_wdt_shutdown(pdev: *mut platform_device) {
    static void a21_wdt_shutdown(struct platform_device *pdev)
    {
    struct a21_wdt_drv *drv = dev_get_drvdata(&pdev.dev);
    gpiod_set_value(drv.gpios[GPIO_WD_ENAB], 0);
    }
    static const struct of_device_id a21_wdt_ids[] = {
    { .compatible = "men,a021-wdt" },
    { },
    };
    MODULE_DEVICE_TABLE(of, a21_wdt_ids);
    static struct platform_driver a21_wdt_driver = {
    .probe = a21_wdt_probe,
    .shutdown = a21_wdt_shutdown,
    .driver = {
    .name = "a21-watchdog",
    .of_match_table = a21_wdt_ids,
    },
    };
    module_platform_driver(a21_wdt_driver);
    MODULE_AUTHOR("MEN Mikro Elektronik");
    MODULE_DESCRIPTION("MEN A21 Watchdog");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:a21-watchdog");
