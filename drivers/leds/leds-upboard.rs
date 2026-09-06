//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-upboard.c
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
// UP board LED driver.
//
// Copyright (c) AAEON. All rights reserved.
// Copyright (C) 2024 Bootlin
//
// Author: Gary Wang <garywang@aaeon.com.tw>
// Author: Thomas Richard <thomas.richard@bootlin.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct upboard_led {
    pub field: *mut regmap_field,
    pub cdev: led_classdev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct upboard_led_profile {
    pub name: *const c_char,
    pub bit: c_uint,
}

    static struct upboard_led_profile upboard_up_led_profile[] = {
    { "upboard:yellow:" LED_FUNCTION_STATUS, 0 },
    { "upboard:green:" LED_FUNCTION_STATUS, 1 },
    { "upboard:red:" LED_FUNCTION_STATUS, 2 },
    };
    static struct upboard_led_profile upboard_up2_led_profile[] = {
    { "upboard:blue:" LED_FUNCTION_STATUS, 0 },
    { "upboard:yellow:" LED_FUNCTION_STATUS, 1 },
    { "upboard:green:" LED_FUNCTION_STATUS, 2 },
    { "upboard:red:" LED_FUNCTION_STATUS, 3 },
    };
#[no_mangle]
unsafe extern "C" fn upboard_led_brightness_get(cdev: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness upboard_led_brightness_get(struct led_classdev *cdev)
    {
    struct upboard_led *led = led_cdev_to_led_upboard(cdev);
    int brightness, ret;
    ret = regmap_field_read(led.field, &brightness);
    return ret ? LED_OFF : brightness;
    };
#[no_mangle]
unsafe extern "C" fn upboard_led_brightness_set(cdev: *mut led_classdev, brightness: enum led_brightness) -> c_int {
    static int upboard_led_brightness_set(struct led_classdev *cdev, enum led_brightness brightness)
    {
    struct upboard_led *led = led_cdev_to_led_upboard(cdev);
    return regmap_field_write(led.field, brightness != LED_OFF);
    };
#[no_mangle]
unsafe extern "C" fn upboard_led_probe(pdev: *mut platform_device) -> c_int {
    static int upboard_led_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct upboard_fpga *fpga = dev_get_drvdata(dev.parent);
    struct upboard_led_profile *led_profile;
    struct upboard_led *led;
    int led_instances, ret, i;
    switch (fpga.fpga_data.type) {
    case UPBOARD_UP_FPGA:
    led_profile = upboard_up_led_profile;
    led_instances = ARRAY_SIZE(upboard_up_led_profile);
    break;
    case UPBOARD_UP2_FPGA:
    led_profile = upboard_up2_led_profile;
    led_instances = ARRAY_SIZE(upboard_up2_led_profile);
    break;
    default:
    return dev_err_probe(dev, -EINVAL, "Unknown device type %d\n",
    fpga.fpga_data.type);
    }
    for (i = 0; i < led_instances; i++) {
    const struct reg_field fldconf = {
    .reg = UPBOARD_REG_FUNC_EN0,
    .lsb = led_profile[i].bit,
    .msb = led_profile[i].bit,
    };
    led = devm_kzalloc(dev, sizeof(*led), GFP_KERNEL);
    if (!led)
    return -ENOMEM;
    led.field = devm_regmap_field_alloc(&pdev.dev, fpga.regmap, fldconf);
    if (IS_ERR(led.field))
    return PTR_ERR(led.field);
    led.cdev.brightness_get = upboard_led_brightness_get;
    led.cdev.brightness_set_blocking = upboard_led_brightness_set;
    led.cdev.max_brightness = LED_ON;
    led.cdev.name = led_profile[i].name;
    ret = devm_led_classdev_register(dev, &led.cdev);
    if (ret)
    return ret;
    }
    return 0;
    }
    static struct platform_driver upboard_led_driver = {
    .driver = {
    .name = "upboard-leds",
    },
    .probe = upboard_led_probe,
    };
    module_platform_driver(upboard_led_driver);
    MODULE_AUTHOR("Gary Wang <garywang@aaeon.com.tw>");
    MODULE_AUTHOR("Thomas Richard <thomas.richard@bootlin.com>");
    MODULE_DESCRIPTION("UP Board LED driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:upboard-leds");
