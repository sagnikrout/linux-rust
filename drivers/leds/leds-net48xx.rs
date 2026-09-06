//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-net48xx.c
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
// LEDs driver for Soekris net48xx
//
// Copyright (C) 2006 Chris Boot <bootc@bootc.net>
//
// Based on leds-ams-delta.c
//

pub const NET48XX_ERROR_LED_GPIO: c_int = 20;
    static struct platform_device *pdev;
    static void net48xx_error_led_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    scx200_gpio_ops.gpio_set(NET48XX_ERROR_LED_GPIO, value ? 1 : 0);
    }
    static struct led_classdev net48xx_error_led = {
    .name		= "net48xx::error",
    .brightness_set	= net48xx_error_led_set,
    .flags		= LED_CORE_SUSPENDRESUME,
    };
#[no_mangle]
unsafe extern "C" fn net48xx_led_probe(pdev: *mut platform_device) -> c_int {
    static int net48xx_led_probe(struct platform_device *pdev)
    {
    return devm_led_classdev_register(&pdev.dev, &net48xx_error_led);
    }
    static struct platform_driver net48xx_led_driver = {
    .probe		= net48xx_led_probe,
    .driver		= {
    .name		= DRVNAME,
    },
    };
#[no_mangle]
unsafe extern "C" fn net48xx_led_init() -> int __init {
    static int __init net48xx_led_init(void)
    {
    int ret;
// small hack, but scx200_gpio doesn't set .dev if the probe fails
    if (!scx200_gpio_ops.dev) {
    ret = -ENODEV;
    goto out;
    }
    ret = platform_driver_register(&net48xx_led_driver);
    if (ret < 0)
    goto out;
    pdev = platform_device_register_simple(DRVNAME, -1, core::ptr::null_mut(), 0);
    if (IS_ERR(pdev)) {
    ret = PTR_ERR(pdev);
    platform_driver_unregister(&net48xx_led_driver);
    goto out;
    }
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn net48xx_led_exit() -> void __exit {
    static void __exit net48xx_led_exit(void)
    {
    platform_device_unregister(pdev);
    platform_driver_unregister(&net48xx_led_driver);
    }
    module_init(net48xx_led_init);
    module_exit(net48xx_led_exit);
    MODULE_AUTHOR("Chris Boot <bootc@bootc.net>");
    MODULE_DESCRIPTION("Soekris net48xx LED driver");
    MODULE_LICENSE("GPL");
