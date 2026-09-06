//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-hp6xx.c
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
// LED Triggers Core
// For the HP Jornada 620/660/680/690 handhelds
//
// Copyright 2008 Kristoffer Ericson <kristoffer.ericson@gmail.com>
// this driver is based on leds-spitz.c by Richard Purdie.
//

    static void hp6xxled_green_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    u8 v8;
    v8 = inb(PKDR);
    if (value)
    outb(v8 & (~PKDR_LED_GREEN), PKDR);
    else
    outb(v8 | PKDR_LED_GREEN, PKDR);
    }
    static void hp6xxled_red_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    u16 v16;
    v16 = inw(HD64461_GPBDR);
    if (value)
    outw(v16 & (~HD64461_GPBDR_LED_RED), HD64461_GPBDR);
    else
    outw(v16 | HD64461_GPBDR_LED_RED, HD64461_GPBDR);
    }
    static struct led_classdev hp6xx_red_led = {
    .name			= "hp6xx:red",
    .default_trigger	= "hp6xx-charge",
    .brightness_set		= hp6xxled_red_set,
    .flags			= LED_CORE_SUSPENDRESUME,
    };
    static struct led_classdev hp6xx_green_led = {
    .name			= "hp6xx:green",
    .default_trigger	= "disk-activity",
    .brightness_set		= hp6xxled_green_set,
    .flags			= LED_CORE_SUSPENDRESUME,
    };
#[no_mangle]
unsafe extern "C" fn hp6xxled_probe(pdev: *mut platform_device) -> c_int {
    static int hp6xxled_probe(struct platform_device *pdev)
    {
    int ret;
    ret = devm_led_classdev_register(&pdev.dev, &hp6xx_red_led);
    if (ret < 0)
    return ret;
    return devm_led_classdev_register(&pdev.dev, &hp6xx_green_led);
    }
    static struct platform_driver hp6xxled_driver = {
    .probe		= hp6xxled_probe,
    .driver		= {
    .name		= "hp6xx-led",
    },
    };
    module_platform_driver(hp6xxled_driver);
    MODULE_AUTHOR("Kristoffer Ericson <kristoffer.ericson@gmail.com>");
    MODULE_DESCRIPTION("HP Jornada 6xx LED driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:hp6xx-led");
