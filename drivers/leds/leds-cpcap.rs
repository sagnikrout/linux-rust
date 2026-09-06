//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-cpcap.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 2017 Sebastian Reichel <sre@kernel.org>
//

pub const CPCAP_LED_NO_CURRENT: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpcap_led_info {
    pub reg: u16,
    pub mask: u16,
    pub limit: u16,
    pub init_mask: u16,
    pub init_val: u16,
}

    static const struct cpcap_led_info cpcap_led_red = {
    .reg	= CPCAP_REG_REDC,
    .mask	= 0x03FF,
    .limit	= 31,
    };
    static const struct cpcap_led_info cpcap_led_green = {
    .reg	= CPCAP_REG_GREENC,
    .mask	= 0x03FF,
    .limit	= 31,
    };
    static const struct cpcap_led_info cpcap_led_blue = {
    .reg	= CPCAP_REG_BLUEC,
    .mask	= 0x03FF,
    .limit	= 31,
    };
// aux display light
    static const struct cpcap_led_info cpcap_led_adl = {
    .reg		= CPCAP_REG_ADLC,
    .mask		= 0x000F,
    .limit		= 1,
    .init_mask	= 0x7FFF,
    .init_val	= 0x5FF0,
    };
// camera privacy led
    static const struct cpcap_led_info cpcap_led_cp = {
    .reg		= CPCAP_REG_CLEDC,
    .mask		= 0x0007,
    .limit		= 1,
    .init_mask	= 0x03FF,
    .init_val	= 0x0008,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpcap_led {
    pub led: led_classdev,
    pub info: *const cpcap_led_info,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub update_lock: mutex,
    pub vdd: *mut regulator,
    pub powered: bool,
    pub current_limit: u32,
}

#[no_mangle]
unsafe extern "C" fn cpcap_led_val(current_limit: u8, duty_cycle: u8) -> u16 {
    static u16 cpcap_led_val(u8 current_limit, u8 duty_cycle)
    {
    current_limit &= 0x1f; /* 5 bit */
    duty_cycle &= 0x0f; /* 4 bit */
    return current_limit << 4 | duty_cycle;
    }
#[no_mangle]
unsafe extern "C" fn cpcap_led_set_power(led: *mut cpcap_led, status: bool) -> c_int {
    static int cpcap_led_set_power(struct cpcap_led *led, bool status)
    {
    int err;
    if (status == led.powered)
    return 0;
    if (status)
    err = regulator_enable(led.vdd);
    else
    err = regulator_disable(led.vdd);
    if (err) {
    dev_err(led.dev, "regulator failure: %d", err);
    return err;
    }
    led.powered = status;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpcap_led_set(ledc: *mut led_classdev, value: enum led_brightness) -> c_int {
    static int cpcap_led_set(struct led_classdev *ledc, enum led_brightness value)
    {
    struct cpcap_led *led = container_of(ledc, struct cpcap_led, led);
    int brightness;
    int err;
    mutex_lock(&led.update_lock);
    if (value > LED_OFF) {
    err = cpcap_led_set_power(led, true);
    if (err)
    goto exit;
    }
    if (value == LED_OFF) {
// Avoid HW issue by turning off current before duty cycle
    err = regmap_update_bits(led.regmap,
    led.info.reg, led.info.mask, CPCAP_LED_NO_CURRENT);
    if (err) {
    dev_err(led.dev, "regmap failed: %d", err);
    goto exit;
    }
    brightness = cpcap_led_val(value, LED_OFF);
    } else {
    brightness = cpcap_led_val(value, LED_ON);
    }
    err = regmap_update_bits(led.regmap, led.info.reg, led.info.mask,
    brightness);
    if (err) {
    dev_err(led.dev, "regmap failed: %d", err);
    goto exit;
    }
    if (value == LED_OFF) {
    err = cpcap_led_set_power(led, false);
    if (err)
    goto exit;
    }
    exit:
    mutex_unlock(&led.update_lock);
    return err;
    }
    static const struct of_device_id cpcap_led_of_match[] = {
    { .compatible = "motorola,cpcap-led-red", .data = &cpcap_led_red },
    { .compatible = "motorola,cpcap-led-green", .data = &cpcap_led_green },
    { .compatible = "motorola,cpcap-led-blue",  .data = &cpcap_led_blue },
    { .compatible = "motorola,cpcap-led-adl", .data = &cpcap_led_adl },
    { .compatible = "motorola,cpcap-led-cp", .data = &cpcap_led_cp },
    {},
    };
    MODULE_DEVICE_TABLE(of, cpcap_led_of_match);
#[no_mangle]
unsafe extern "C" fn cpcap_led_probe(pdev: *mut platform_device) -> c_int {
    static int cpcap_led_probe(struct platform_device *pdev)
    {
    struct cpcap_led *led;
    int err;
    led = devm_kzalloc(&pdev.dev, sizeof(*led), GFP_KERNEL);
    if (!led)
    return -ENOMEM;
    platform_set_drvdata(pdev, led);
    led.info = device_get_match_data(&pdev.dev);
    led.dev = &pdev.dev;
    if (led.info.reg == 0x0000) {
    dev_err(led.dev, "Unsupported LED");
    return -ENODEV;
    }
    led.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!led.regmap)
    return -ENODEV;
    led.vdd = devm_regulator_get(&pdev.dev, "vdd");
    if (IS_ERR(led.vdd)) {
    err = PTR_ERR(led.vdd);
    dev_err(led.dev, "Couldn't get regulator: %d", err);
    return err;
    }
    err = device_property_read_string(&pdev.dev, "label", &led.led.name);
    if (err) {
    dev_err(led.dev, "Couldn't read LED label: %d", err);
    return err;
    }
    if (led.info.init_mask) {
    err = regmap_update_bits(led.regmap, led.info.reg,
    led.info.init_mask, led.info.init_val);
    if (err) {
    dev_err(led.dev, "regmap failed: %d", err);
    return err;
    }
    }
    mutex_init(&led.update_lock);
    led.led.max_brightness = led.info.limit;
    led.led.brightness_set_blocking = cpcap_led_set;
    err = devm_led_classdev_register(&pdev.dev, &led.led);
    if (err) {
    dev_err(led.dev, "Couldn't register LED: %d", err);
    return err;
    }
    return 0;
    }
    static struct platform_driver cpcap_led_driver = {
    .probe = cpcap_led_probe,
    .driver = {
    .name = "cpcap-led",
    .of_match_table = cpcap_led_of_match,
    },
    };
    module_platform_driver(cpcap_led_driver);
    MODULE_DESCRIPTION("CPCAP LED driver");
    MODULE_AUTHOR("Sebastian Reichel <sre@kernel.org>");
    MODULE_LICENSE("GPL");
