//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-locomo.c
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
// linux/drivers/leds/leds-locomo.c
//
// Copyright (C) 2005 John Lenz <lenz@cs.wisc.edu>
//

    static void locomoled_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness value, int offset)
    {
    struct locomo_dev *locomo_dev = LOCOMO_DEV(led_cdev.dev.parent);
    unsigned long flags;
    local_irq_save(flags);
    if (value)
    locomo_writel(LOCOMO_LPT_TOFH, locomo_dev.mapbase + offset);
    else
    locomo_writel(LOCOMO_LPT_TOFL, locomo_dev.mapbase + offset);
    local_irq_restore(flags);
    }
    static void locomoled_brightness_set0(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    locomoled_brightness_set(led_cdev, value, LOCOMO_LPT0);
    }
    static void locomoled_brightness_set1(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    locomoled_brightness_set(led_cdev, value, LOCOMO_LPT1);
    }
    static struct led_classdev locomo_led0 = {
    .name			= "locomo:amber:charge",
    .default_trigger	= "main-battery-charging",
    .brightness_set		= locomoled_brightness_set0,
    };
    static struct led_classdev locomo_led1 = {
    .name			= "locomo:green:mail",
    .default_trigger	= "nand-disk",
    .brightness_set		= locomoled_brightness_set1,
    };
#[no_mangle]
unsafe extern "C" fn locomoled_probe(ldev: *mut locomo_dev) -> c_int {
    static int locomoled_probe(struct locomo_dev *ldev)
    {
    int ret;
    ret = devm_led_classdev_register(&ldev.dev, &locomo_led0);
    if (ret < 0)
    return ret;
    return  devm_led_classdev_register(&ldev.dev, &locomo_led1);
    }
    static struct locomo_driver locomoled_driver = {
    .drv = {
    .name = "locomoled"
    },
    .devid	= LOCOMO_DEVID_LED,
    .probe	= locomoled_probe,
    };
#[no_mangle]
unsafe extern "C" fn locomoled_init() -> int __init {
    static int __init locomoled_init(void)
    {
    return locomo_driver_register(&locomoled_driver);
    }
    module_init(locomoled_init);
    MODULE_AUTHOR("John Lenz <lenz@cs.wisc.edu>");
    MODULE_DESCRIPTION("Locomo LED driver");
    MODULE_LICENSE("GPL");
