//! Automatically rewritten from C to Rust
//! Source: drivers/leds/simatic/simatic-ipc-leds-gpio-f7188x.c
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
// Siemens SIMATIC IPC driver for GPIO based LEDs
//
// Copyright (c) Siemens AG, 2023
//
// Author:
// Henning Schild <henning.schild@siemens.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simatic_ipc_led_tables {
    pub led_lookup_table: *mut gpiod_lookup_table,
    pub led_lookup_table_extra: *mut gpiod_lookup_table,
}

    static struct gpiod_lookup_table simatic_ipc_led_gpio_table_227g = {
    .dev_id = "leds-gpio",
    .table = {
    GPIO_LOOKUP_IDX("gpio-f7188x-2", 0, core::ptr::null_mut(), 0, GPIO_ACTIVE_LOW),
    GPIO_LOOKUP_IDX("gpio-f7188x-2", 1, core::ptr::null_mut(), 1, GPIO_ACTIVE_LOW),
    GPIO_LOOKUP_IDX("gpio-f7188x-2", 2, core::ptr::null_mut(), 2, GPIO_ACTIVE_LOW),
    GPIO_LOOKUP_IDX("gpio-f7188x-2", 3, core::ptr::null_mut(), 3, GPIO_ACTIVE_LOW),
    GPIO_LOOKUP_IDX("gpio-f7188x-2", 4, core::ptr::null_mut(), 4, GPIO_ACTIVE_LOW),
    GPIO_LOOKUP_IDX("gpio-f7188x-2", 5, core::ptr::null_mut(), 5, GPIO_ACTIVE_LOW),
    {} /* Terminating entry */
    },
    };
    static struct gpiod_lookup_table simatic_ipc_led_gpio_table_extra_227g = {
    .dev_id = core::ptr::null_mut(), /* Filled during initialization */
    .table = {
    GPIO_LOOKUP_IDX("gpio-f7188x-3", 6, core::ptr::null_mut(), 6, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("gpio-f7188x-3", 7, core::ptr::null_mut(), 7, GPIO_ACTIVE_HIGH),
    {} /* Terminating entry */
    },
    };
    static struct gpiod_lookup_table simatic_ipc_led_gpio_table_bx_59a = {
    .dev_id = "leds-gpio",
    .table = {
    GPIO_LOOKUP_IDX("gpio-f7188x-2", 0, core::ptr::null_mut(), 0, GPIO_ACTIVE_LOW),
    GPIO_LOOKUP_IDX("gpio-f7188x-2", 3, core::ptr::null_mut(), 1, GPIO_ACTIVE_LOW),
    GPIO_LOOKUP_IDX("gpio-f7188x-5", 3, core::ptr::null_mut(), 2, GPIO_ACTIVE_LOW),
    GPIO_LOOKUP_IDX("gpio-f7188x-5", 2, core::ptr::null_mut(), 3, GPIO_ACTIVE_LOW),
    GPIO_LOOKUP_IDX("gpio-f7188x-7", 7, core::ptr::null_mut(), 4, GPIO_ACTIVE_LOW),
    GPIO_LOOKUP_IDX("gpio-f7188x-7", 4, core::ptr::null_mut(), 5, GPIO_ACTIVE_LOW),
    {} /* Terminating entry */
    }
    };
#[no_mangle]
unsafe extern "C" fn simatic_ipc_leds_gpio_f7188x_probe(pdev: *mut platform_device) -> c_int {
    static int simatic_ipc_leds_gpio_f7188x_probe(struct platform_device *pdev)
    {
    const struct simatic_ipc_platform *plat = dev_get_platdata(&pdev.dev);
    struct simatic_ipc_led_tables *led_tables;
    led_tables = devm_kzalloc(&pdev.dev, sizeof(*led_tables), GFP_KERNEL);
    if (!led_tables)
    return -ENOMEM;
    switch (plat.devmode) {
    case SIMATIC_IPC_DEVICE_227G:
    led_tables.led_lookup_table = &simatic_ipc_led_gpio_table_227g;
    led_tables.led_lookup_table_extra = &simatic_ipc_led_gpio_table_extra_227g;
    break;
    case SIMATIC_IPC_DEVICE_BX_59A:
    led_tables.led_lookup_table = &simatic_ipc_led_gpio_table_bx_59a;
    break;
    default:
    return -ENODEV;
    }
    platform_set_drvdata(pdev, led_tables);
    return simatic_ipc_leds_gpio_probe(pdev, led_tables.led_lookup_table,
    led_tables.led_lookup_table_extra);
    }
#[no_mangle]
unsafe extern "C" fn simatic_ipc_leds_gpio_f7188x_remove(pdev: *mut platform_device) {
    static void simatic_ipc_leds_gpio_f7188x_remove(struct platform_device *pdev)
    {
    struct simatic_ipc_led_tables *led_tables = platform_get_drvdata(pdev);
    simatic_ipc_leds_gpio_remove(pdev, led_tables.led_lookup_table,
    led_tables.led_lookup_table_extra);
    }
    static struct platform_driver simatic_ipc_led_gpio_driver = {
    .probe = simatic_ipc_leds_gpio_f7188x_probe,
    .remove = simatic_ipc_leds_gpio_f7188x_remove,
    .driver = {
    .name = KBUILD_MODNAME,
    },
    };
    module_platform_driver(simatic_ipc_led_gpio_driver);
    MODULE_DESCRIPTION("LED driver for Siemens Simatic IPCs based on Nuvoton GPIO");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:" KBUILD_MODNAME);
    MODULE_SOFTDEP("pre: simatic-ipc-leds-gpio-core gpio_f7188x");
    MODULE_AUTHOR("Henning Schild <henning.schild@siemens.com>");
