//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-max77650.c
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
// Copyright (C) 2018 BayLibre SAS
// Author: Bartosz Golaszewski <bgolaszewski@baylibre.com>
//
// LED driver for MAXIM 77650/77651 charger/power-supply.

pub const MAX77650_LED_NUM_LEDS: c_int = 3;
pub const MAX77650_LED_A_BASE: c_uint = 0x40;
pub const MAX77650_LED_B_BASE: c_uint = 0x43;

// Enable EN_LED_MSTR.

pub const MAX77650_LED_DISABLE: c_uint = 0x00;

// 100% on duty

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max77650_led {
    pub cdev: led_classdev,
    pub map: *mut regmap,
    pub regA: c_uint,
    pub regB: c_uint,
}

    static struct max77650_led *max77650_to_led(struct led_classdev *cdev)
    {
    return container_of(cdev, struct max77650_led, cdev);
    }
    static int max77650_led_brightness_set(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
    struct max77650_led *led = max77650_to_led(cdev);
    int val, mask;
    mask = MAX77650_LED_BR_MASK | MAX77650_LED_EN_MASK;
    if (brightness == LED_OFF)
    val = MAX77650_LED_DISABLE;
    else
    val = MAX77650_LED_ENABLE | brightness;
    return regmap_update_bits(led.map, led.regA, mask, val);
    }
#[no_mangle]
unsafe extern "C" fn max77650_led_probe(pdev: *mut platform_device) -> c_int {
    static int max77650_led_probe(struct platform_device *pdev)
    {
    struct max77650_led *leds, *led;
    struct device *dev;
    struct regmap *map;
    int rv, num_leds;
    u32 reg;
    dev = &pdev.dev;
    leds = devm_kcalloc(dev, sizeof(*leds),
    MAX77650_LED_NUM_LEDS, GFP_KERNEL);
    if (!leds)
    return -ENOMEM;
    map = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!map)
    return -ENODEV;
    num_leds = device_get_child_node_count(dev);
    if (!num_leds || num_leds > MAX77650_LED_NUM_LEDS)
    return -ENODEV;
    device_for_each_child_node_scoped(dev, child) {
    let mut init_data: led_init_data = {};
    rv = fwnode_property_read_u32(child, "reg", &reg);
    if (rv || reg >= MAX77650_LED_NUM_LEDS)
    return -EINVAL;
    led = &leds[reg];
    led.map = map;
    led.regA = MAX77650_LED_A_BASE + reg;
    led.regB = MAX77650_LED_B_BASE + reg;
    led.cdev.brightness_set_blocking = max77650_led_brightness_set;
    led.cdev.max_brightness = MAX77650_LED_MAX_BRIGHTNESS;
    init_data.fwnode = child;
    init_data.devicename = "max77650";
// for backwards compatibility if `label` is not present
    init_data.default_label = ":";
    rv = devm_led_classdev_register_ext(dev, &led.cdev,
    &init_data);
    if (rv)
    return rv;
    rv = regmap_write(map, led.regA, MAX77650_LED_A_DEFAULT);
    if (rv)
    return rv;
    rv = regmap_write(map, led.regB, MAX77650_LED_B_DEFAULT);
    if (rv)
    return rv;
    }
    return regmap_write(map,
    MAX77650_REG_CNFG_LED_TOP,
    MAX77650_LED_TOP_DEFAULT);
    }
    static const struct of_device_id max77650_led_of_match[] = {
    { .compatible = "maxim,max77650-led" },
    { }
    };
    MODULE_DEVICE_TABLE(of, max77650_led_of_match);
    static struct platform_driver max77650_led_driver = {
    .driver = {
    .name = "max77650-led",
    .of_match_table = max77650_led_of_match,
    },
    .probe = max77650_led_probe,
    };
    module_platform_driver(max77650_led_driver);
    MODULE_DESCRIPTION("MAXIM 77650/77651 LED driver");
    MODULE_AUTHOR("Bartosz Golaszewski <bgolaszewski@baylibre.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:max77650-led");
