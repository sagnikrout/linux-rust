//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-wrap.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// LEDs driver for PCEngines WRAP
//
// Copyright (C) 2006 Kristian Kielhofner <kris@krisk.org>
//
// Based on leds-net48xx.c
//

pub const WRAP_POWER_LED_GPIO: c_int = 2;
pub const WRAP_ERROR_LED_GPIO: c_int = 3;
pub const WRAP_EXTRA_LED_GPIO: c_int = 18;
    static struct platform_device *pdev;
    static void wrap_power_led_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    if (value)
    scx200_gpio_set_low(WRAP_POWER_LED_GPIO);
    else
    scx200_gpio_set_high(WRAP_POWER_LED_GPIO);
    }
    static void wrap_error_led_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    if (value)
    scx200_gpio_set_low(WRAP_ERROR_LED_GPIO);
    else
    scx200_gpio_set_high(WRAP_ERROR_LED_GPIO);
    }
    static void wrap_extra_led_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    if (value)
    scx200_gpio_set_low(WRAP_EXTRA_LED_GPIO);
    else
    scx200_gpio_set_high(WRAP_EXTRA_LED_GPIO);
    }
    static struct led_classdev wrap_power_led = {
    .name			= "wrap::power",
    .brightness_set		= wrap_power_led_set,
    .default_trigger	= "default-on",
    .flags			= LED_CORE_SUSPENDRESUME,
    };
    static struct led_classdev wrap_error_led = {
    .name		= "wrap::error",
    .brightness_set	= wrap_error_led_set,
    .flags			= LED_CORE_SUSPENDRESUME,
    };
    static struct led_classdev wrap_extra_led = {
    .name           = "wrap::extra",
    .brightness_set = wrap_extra_led_set,
    .flags			= LED_CORE_SUSPENDRESUME,
    };
#[no_mangle]
unsafe extern "C" fn wrap_led_probe(pdev: *mut platform_device) -> c_int {
    static int wrap_led_probe(struct platform_device *pdev)
    {
    int ret;
    ret = devm_led_classdev_register(&pdev.dev, &wrap_power_led);
    if (ret < 0)
    return ret;
    ret = devm_led_classdev_register(&pdev.dev, &wrap_error_led);
    if (ret < 0)
    return ret;
    return  devm_led_classdev_register(&pdev.dev, &wrap_extra_led);
    }
    static struct platform_driver wrap_led_driver = {
    .probe		= wrap_led_probe,
    .driver		= {
    .name		= DRVNAME,
    },
    };
#[no_mangle]
unsafe extern "C" fn wrap_led_init() -> int __init {
    static int __init wrap_led_init(void)
    {
    int ret;
    if (!scx200_gpio_present()) {
    ret = -ENODEV;
    goto out;
    }
    ret = platform_driver_register(&wrap_led_driver);
    if (ret < 0)
    goto out;
    pdev = platform_device_register_simple(DRVNAME, -1, core::ptr::null_mut(), 0);
    if (IS_ERR(pdev)) {
    ret = PTR_ERR(pdev);
    platform_driver_unregister(&wrap_led_driver);
    goto out;
    }
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn wrap_led_exit() -> void __exit {
    static void __exit wrap_led_exit(void)
    {
    platform_device_unregister(pdev);
    platform_driver_unregister(&wrap_led_driver);
    }
    module_init(wrap_led_init);
    module_exit(wrap_led_exit);
    MODULE_AUTHOR("Kristian Kielhofner <kris@krisk.org>");
    MODULE_DESCRIPTION("PCEngines WRAP LED driver");
    MODULE_LICENSE("GPL");
