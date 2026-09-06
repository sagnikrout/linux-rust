//! Automatically rewritten from C to Rust
//! Source: drivers/leds/simatic/simatic-ipc-leds-gpio-apollolake.c
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

    static struct gpiod_lookup_table simatic_ipc_led_gpio_table = {
    .dev_id = "leds-gpio",
    .table = {
    GPIO_LOOKUP_IDX("apollolake-pinctrl.0", 52, core::ptr::null_mut(), 0, GPIO_ACTIVE_LOW),
    GPIO_LOOKUP_IDX("apollolake-pinctrl.0", 53, core::ptr::null_mut(), 1, GPIO_ACTIVE_LOW),
    GPIO_LOOKUP_IDX("apollolake-pinctrl.0", 57, core::ptr::null_mut(), 2, GPIO_ACTIVE_LOW),
    GPIO_LOOKUP_IDX("apollolake-pinctrl.0", 58, core::ptr::null_mut(), 3, GPIO_ACTIVE_LOW),
    GPIO_LOOKUP_IDX("apollolake-pinctrl.0", 60, core::ptr::null_mut(), 4, GPIO_ACTIVE_LOW),
    GPIO_LOOKUP_IDX("apollolake-pinctrl.0", 51, core::ptr::null_mut(), 5, GPIO_ACTIVE_LOW),
    {} /* Terminating entry */
    },
    };
    static struct gpiod_lookup_table simatic_ipc_led_gpio_table_extra = {
    .dev_id = core::ptr::null_mut(), /* Filled during initialization */
    .table = {
    GPIO_LOOKUP_IDX("apollolake-pinctrl.0", 56, core::ptr::null_mut(), 6, GPIO_ACTIVE_LOW),
    GPIO_LOOKUP_IDX("apollolake-pinctrl.0", 59, core::ptr::null_mut(), 7, GPIO_ACTIVE_HIGH),
    {} /* Terminating entry */
    },
    };
#[no_mangle]
unsafe extern "C" fn simatic_ipc_leds_gpio_apollolake_probe(pdev: *mut platform_device) -> c_int {
    static int simatic_ipc_leds_gpio_apollolake_probe(struct platform_device *pdev)
    {
    return simatic_ipc_leds_gpio_probe(pdev, &simatic_ipc_led_gpio_table,
    &simatic_ipc_led_gpio_table_extra);
    }
#[no_mangle]
unsafe extern "C" fn simatic_ipc_leds_gpio_apollolake_remove(pdev: *mut platform_device) {
    static void simatic_ipc_leds_gpio_apollolake_remove(struct platform_device *pdev)
    {
    simatic_ipc_leds_gpio_remove(pdev, &simatic_ipc_led_gpio_table,
    &simatic_ipc_led_gpio_table_extra);
    }
    static struct platform_driver simatic_ipc_led_gpio_apollolake_driver = {
    .probe = simatic_ipc_leds_gpio_apollolake_probe,
    .remove = simatic_ipc_leds_gpio_apollolake_remove,
    .driver = {
    .name = KBUILD_MODNAME,
    },
    };
    module_platform_driver(simatic_ipc_led_gpio_apollolake_driver);
    MODULE_DESCRIPTION("LED driver for Siemens Simatic IPCs based on Intel Apollo Lake GPIO");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:" KBUILD_MODNAME);
    MODULE_SOFTDEP("pre: simatic-ipc-leds-gpio-core platform:apollolake-pinctrl");
    MODULE_AUTHOR("Henning Schild <henning.schild@siemens.com>");
