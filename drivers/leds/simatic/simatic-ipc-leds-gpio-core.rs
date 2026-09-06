//! Automatically rewritten from C to Rust
//! Source: drivers/leds/simatic/simatic-ipc-leds-gpio-core.c
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

    static struct platform_device *simatic_leds_pdev;
    static const struct gpio_led simatic_ipc_gpio_leds[] = {
    { .name = "red:" LED_FUNCTION_STATUS "-1" },
    { .name = "green:" LED_FUNCTION_STATUS "-1" },
    { .name = "red:" LED_FUNCTION_STATUS "-2" },
    { .name = "green:" LED_FUNCTION_STATUS "-2" },
    { .name = "red:" LED_FUNCTION_STATUS "-3" },
    { .name = "green:" LED_FUNCTION_STATUS "-3" },
    };
    static const struct gpio_led_platform_data simatic_ipc_gpio_leds_pdata = {
    .num_leds	= ARRAY_SIZE(simatic_ipc_gpio_leds),
    .leds		= simatic_ipc_gpio_leds,
    };
    void simatic_ipc_leds_gpio_remove(struct platform_device *pdev,
    struct gpiod_lookup_table *table,
    struct gpiod_lookup_table *table_extra)
    {
    gpiod_remove_lookup_table(table);
    gpiod_remove_lookup_table(table_extra);
    platform_device_unregister(simatic_leds_pdev);
    }
    EXPORT_SYMBOL_GPL(simatic_ipc_leds_gpio_remove);
    int simatic_ipc_leds_gpio_probe(struct platform_device *pdev,
    struct gpiod_lookup_table *table,
    struct gpiod_lookup_table *table_extra)
    {
    const struct simatic_ipc_platform *plat = pdev.dev.platform_data;
    struct device *dev = &pdev.dev;
    struct gpio_desc *gpiod;
    int err;
    switch (plat.devmode) {
    case SIMATIC_IPC_DEVICE_127E:
    case SIMATIC_IPC_DEVICE_227G:
    case SIMATIC_IPC_DEVICE_BX_21A:
    case SIMATIC_IPC_DEVICE_BX_59A:
    break;
    default:
    return -ENODEV;
    }
    gpiod_add_lookup_table(table);
    simatic_leds_pdev = platform_device_register_resndata(core::ptr::null_mut(),
    "leds-gpio", PLATFORM_DEVID_NONE, core::ptr::null_mut(), 0,
    &simatic_ipc_gpio_leds_pdata,
    sizeof(simatic_ipc_gpio_leds_pdata));
    if (IS_ERR(simatic_leds_pdev)) {
    err = PTR_ERR(simatic_leds_pdev);
    goto out;
    }
    if (!table_extra)
    return 0;
    table_extra.dev_id = dev_name(dev);
    gpiod_add_lookup_table(table_extra);
// PM_BIOS_BOOT_N
    gpiod = gpiod_get_index(dev, core::ptr::null_mut(), 6, GPIOD_OUT_LOW);
    if (IS_ERR(gpiod)) {
    err = PTR_ERR(gpiod);
    goto out;
    }
    gpiod_put(gpiod);
// PM_WDT_OUT
    gpiod = gpiod_get_index(dev, core::ptr::null_mut(), 7, GPIOD_OUT_LOW);
    if (IS_ERR(gpiod)) {
    err = PTR_ERR(gpiod);
    goto out;
    }
    gpiod_put(gpiod);
    return 0;
    out:
    simatic_ipc_leds_gpio_remove(pdev, table, table_extra);
    return err;
    }
    EXPORT_SYMBOL_GPL(simatic_ipc_leds_gpio_probe);
    MODULE_DESCRIPTION("Siemens SIMATIC IPC core driver for GPIO based LEDs");
    MODULE_LICENSE("GPL v2");
    MODULE_SOFTDEP("pre: platform:leds-gpio");
    MODULE_AUTHOR("Henning Schild <henning.schild@siemens.com>");
