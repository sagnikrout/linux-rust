//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/siemens/simatic-ipc-batt-f7188x.c
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
// Siemens SIMATIC IPC driver for CMOS battery monitoring
//
// Copyright (c) Siemens AG, 2023
//
// Authors:
// Henning Schild <henning.schild@siemens.com>
//

    static struct gpiod_lookup_table *batt_lookup_table;
    static struct gpiod_lookup_table simatic_ipc_batt_gpio_table_227g = {
    .table = {
    GPIO_LOOKUP_IDX("gpio-f7188x-7", 6, core::ptr::null_mut(), 0, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("gpio-f7188x-7", 5, core::ptr::null_mut(), 1, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("INTC1020:01",  66, core::ptr::null_mut(), 2, GPIO_ACTIVE_HIGH),
    {} /* Terminating entry */
    },
    };
    static struct gpiod_lookup_table simatic_ipc_batt_gpio_table_bx_39a = {
    .table = {
    GPIO_LOOKUP_IDX("gpio-f7188x-6", 4, core::ptr::null_mut(), 0, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("gpio-f7188x-6", 3, core::ptr::null_mut(), 1, GPIO_ACTIVE_HIGH),
    {} /* Terminating entry */
    },
    };
    static struct gpiod_lookup_table simatic_ipc_batt_gpio_table_bx_59a = {
    .table = {
    GPIO_LOOKUP_IDX("gpio-f7188x-7", 6, core::ptr::null_mut(), 0, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("gpio-f7188x-7", 5, core::ptr::null_mut(), 1, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("INTC1056:00", 438, core::ptr::null_mut(), 2, GPIO_ACTIVE_HIGH),
    {} /* Terminating entry */
    }
    };
#[no_mangle]
unsafe extern "C" fn simatic_ipc_batt_f7188x_remove(pdev: *mut platform_device) {
    static void simatic_ipc_batt_f7188x_remove(struct platform_device *pdev)
    {
    simatic_ipc_batt_remove(pdev, batt_lookup_table);
    }
#[no_mangle]
unsafe extern "C" fn simatic_ipc_batt_f7188x_probe(pdev: *mut platform_device) -> c_int {
    static int simatic_ipc_batt_f7188x_probe(struct platform_device *pdev)
    {
    const struct simatic_ipc_platform *plat = pdev.dev.platform_data;
    switch (plat.devmode) {
    case SIMATIC_IPC_DEVICE_227G:
    batt_lookup_table = &simatic_ipc_batt_gpio_table_227g;
    break;
    case SIMATIC_IPC_DEVICE_BX_39A:
    batt_lookup_table = &simatic_ipc_batt_gpio_table_bx_39a;
    break;
    case SIMATIC_IPC_DEVICE_BX_59A:
    batt_lookup_table = &simatic_ipc_batt_gpio_table_bx_59a;
    break;
    default:
    return -ENODEV;
    }
    return simatic_ipc_batt_probe(pdev, batt_lookup_table);
    }
    static struct platform_driver simatic_ipc_batt_driver = {
    .probe = simatic_ipc_batt_f7188x_probe,
    .remove = simatic_ipc_batt_f7188x_remove,
    .driver = {
    .name = KBUILD_MODNAME,
    },
    };
    module_platform_driver(simatic_ipc_batt_driver);
    MODULE_DESCRIPTION("CMOS Battery monitoring for Simatic IPCs based on Nuvoton GPIO");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:" KBUILD_MODNAME);
    MODULE_SOFTDEP("pre: simatic-ipc-batt gpio_f7188x platform:elkhartlake-pinctrl platform:alderlake-pinctrl");
    MODULE_AUTHOR("Henning Schild <henning.schild@siemens.com>");
