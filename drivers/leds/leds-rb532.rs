//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-rb532.c
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
// LEDs driver for the "User LED" on Routerboard532
//
// Copyright (C) 2009 Phil Sutter <n0-1@freewrt.org>
//
// Based on leds-cobalt-qube.c by Florian Fainelly and
// rb-diag.c (my own standalone driver for both LED and
// button of Routerboard532).
//

    static void rb532_led_set(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
    if (brightness)
    set_latch_u5(LO_ULED, 0);
    else
    set_latch_u5(0, LO_ULED);
    }
#[no_mangle]
unsafe extern "C" fn rb532_led_get(cdev: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness rb532_led_get(struct led_classdev *cdev)
    {
    return (get_latch_u5() & LO_ULED) ? LED_FULL : LED_OFF;
    }
    static struct led_classdev rb532_uled = {
    .name = "uled",
    .brightness_set = rb532_led_set,
    .brightness_get = rb532_led_get,
    .default_trigger = "nand-disk",
    };
#[no_mangle]
unsafe extern "C" fn rb532_led_probe(pdev: *mut platform_device) -> c_int {
    static int rb532_led_probe(struct platform_device *pdev)
    {
    return led_classdev_register(&pdev.dev, &rb532_uled);
    }
#[no_mangle]
unsafe extern "C" fn rb532_led_remove(pdev: *mut platform_device) {
    static void rb532_led_remove(struct platform_device *pdev)
    {
    led_classdev_unregister(&rb532_uled);
    }
    static struct platform_driver rb532_led_driver = {
    .probe = rb532_led_probe,
    .remove = rb532_led_remove,
    .driver = {
    .name = "rb532-led",
    },
    };
    module_platform_driver(rb532_led_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("User LED support for Routerboard532");
    MODULE_AUTHOR("Phil Sutter <n0-1@freewrt.org>");
    MODULE_ALIAS("platform:rb532-led");
