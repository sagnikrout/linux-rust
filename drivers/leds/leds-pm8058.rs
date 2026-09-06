//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-pm8058.c
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
// Copyright (c) 2010, 2011, 2016 The Linux Foundation. All rights reserved.
//

pub const PM8058_LED_TYPE_COMMON: c_uint = 0x00;
pub const PM8058_LED_TYPE_KEYPAD: c_uint = 0x01;
pub const PM8058_LED_TYPE_FLASH: c_uint = 0x02;
pub const PM8058_LED_TYPE_COMMON_MASK: c_uint = 0xf8;
pub const PM8058_LED_TYPE_KEYPAD_MASK: c_uint = 0xf0;
pub const PM8058_LED_TYPE_COMMON_SHIFT: c_int = 3;
pub const PM8058_LED_TYPE_KEYPAD_SHIFT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8058_led {
    pub map: *mut regmap,
    pub reg: u32,
    pub ledtype: u32,
    pub cdev: led_classdev,
}

    static void pm8058_led_set(struct led_classdev *cled,
    enum led_brightness value)
    {
    struct pm8058_led *led;
    let mut ret: c_int = 0;
    let mut mask: c_uint = 0;
    let mut val: c_uint = 0;
    led = container_of(cled, struct pm8058_led, cdev);
    switch (led.ledtype) {
    case PM8058_LED_TYPE_COMMON:
    mask = PM8058_LED_TYPE_COMMON_MASK;
    val = value << PM8058_LED_TYPE_COMMON_SHIFT;
    break;
    case PM8058_LED_TYPE_KEYPAD:
    case PM8058_LED_TYPE_FLASH:
    mask = PM8058_LED_TYPE_KEYPAD_MASK;
    val = value << PM8058_LED_TYPE_KEYPAD_SHIFT;
    break;
    default:
    break;
    }
    ret = regmap_update_bits(led.map, led.reg, mask, val);
    if (ret)
    pr_err("Failed to set LED brightness\n");
    }
#[no_mangle]
unsafe extern "C" fn pm8058_led_get(cled: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness pm8058_led_get(struct led_classdev *cled)
    {
    struct pm8058_led *led;
    int ret;
    unsigned int val;
    led = container_of(cled, struct pm8058_led, cdev);
    ret = regmap_read(led.map, led.reg, &val);
    if (ret) {
    pr_err("Failed to get LED brightness\n");
    return LED_OFF;
    }
    switch (led.ledtype) {
    case PM8058_LED_TYPE_COMMON:
    val &= PM8058_LED_TYPE_COMMON_MASK;
    val >>= PM8058_LED_TYPE_COMMON_SHIFT;
    break;
    case PM8058_LED_TYPE_KEYPAD:
    case PM8058_LED_TYPE_FLASH:
    val &= PM8058_LED_TYPE_KEYPAD_MASK;
    val >>= PM8058_LED_TYPE_KEYPAD_SHIFT;
    break;
    default:
    val = LED_OFF;
    break;
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn pm8058_led_probe(pdev: *mut platform_device) -> c_int {
    static int pm8058_led_probe(struct platform_device *pdev)
    {
    let mut init_data: led_init_data = {};
    struct device *dev = &pdev.dev;
    struct pm8058_led *led;
    struct device_node *np;
    int ret;
    struct regmap *map;
    enum led_brightness maxbright;
    enum led_default_state state;
    led = devm_kzalloc(dev, sizeof(*led), GFP_KERNEL);
    if (!led)
    return -ENOMEM;
    led.ledtype = (u32)(unsigned long)of_device_get_match_data(dev);
    map = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!map) {
    dev_err(dev, "Parent regmap unavailable.\n");
    return -ENXIO;
    }
    led.map = map;
    np = dev_of_node(dev);
    ret = of_property_read_u32(np, "reg", &led.reg);
    if (ret) {
    dev_err(dev, "no register offset specified\n");
    return -EINVAL;
    }
    led.cdev.brightness_set = pm8058_led_set;
    led.cdev.brightness_get = pm8058_led_get;
    if (led.ledtype == PM8058_LED_TYPE_COMMON)
    maxbright = 31; /* 5 bits */
    else
    maxbright = 15; /* 4 bits */
    led.cdev.max_brightness = maxbright;
    init_data.fwnode = of_fwnode_handle(np);
    state = led_init_default_state_get(init_data.fwnode);
    switch (state) {
    case LEDS_DEFSTATE_ON:
    led.cdev.brightness = maxbright;
    pm8058_led_set(&led.cdev, maxbright);
    break;
    case LEDS_DEFSTATE_KEEP:
    led.cdev.brightness = pm8058_led_get(&led.cdev);
    break;
    default:
    led.cdev.brightness = LED_OFF;
    pm8058_led_set(&led.cdev, LED_OFF);
    }
    if (led.ledtype == PM8058_LED_TYPE_KEYPAD ||
    led.ledtype == PM8058_LED_TYPE_FLASH)
    led.cdev.flags	= LED_CORE_SUSPENDRESUME;
    ret = devm_led_classdev_register_ext(dev, &led.cdev, &init_data);
    if (ret)
    dev_err(dev, "Failed to register LED for %pOF\n", np);
    return ret;
    }
    static const struct of_device_id pm8058_leds_id_table[] = {
    {
    .compatible = "qcom,pm8058-led",
    .data = (void *)PM8058_LED_TYPE_COMMON
    },
    {
    .compatible = "qcom,pm8058-keypad-led",
    .data = (void *)PM8058_LED_TYPE_KEYPAD
    },
    {
    .compatible = "qcom,pm8058-flash-led",
    .data = (void *)PM8058_LED_TYPE_FLASH
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, pm8058_leds_id_table);
    static struct platform_driver pm8058_led_driver = {
    .probe		= pm8058_led_probe,
    .driver		= {
    .name	= "pm8058-leds",
    .of_match_table = pm8058_leds_id_table,
    },
    };
    module_platform_driver(pm8058_led_driver);
    MODULE_DESCRIPTION("PM8058 LEDs driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:pm8058-leds");
