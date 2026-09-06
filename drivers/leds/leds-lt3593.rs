//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-lt3593.c
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
// Copyright (c) 2009,2018 Daniel Mack <daniel@zonque.org>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lt3593_led_data {
    pub cdev: led_classdev,
    pub gpiod: *mut gpio_desc,
}

    static int lt3593_led_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    struct lt3593_led_data *led_dat =
    container_of(led_cdev, struct lt3593_led_data, cdev);
    int pulses;
//
// The LT3593 resets its internal current level register to the maximum
// level on the first falling edge on the control pin. Each following
// falling edge decreases the current level by 625uA. Up to 32 pulses
// can be sent, so the maximum power reduction is 20mA.
// After a timeout of 128us, the value is taken from the register and
// applied is to the output driver.
//
    if (value == 0) {
    gpiod_set_value_cansleep(led_dat.gpiod, 0);
    return 0;
    }
    pulses = 32 - (value * 32) / 255;
    if (pulses == 0) {
    gpiod_set_value_cansleep(led_dat.gpiod, 0);
    mdelay(1);
    gpiod_set_value_cansleep(led_dat.gpiod, 1);
    return 0;
    }
    gpiod_set_value_cansleep(led_dat.gpiod, 1);
    while (pulses--) {
    gpiod_set_value_cansleep(led_dat.gpiod, 0);
    udelay(1);
    gpiod_set_value_cansleep(led_dat.gpiod, 1);
    udelay(1);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lt3593_led_probe(pdev: *mut platform_device) -> c_int {
    static int lt3593_led_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct lt3593_led_data *led_data;
    struct fwnode_handle *child;
    int ret, state = LEDS_GPIO_DEFSTATE_OFF;
    let mut init_data: led_init_data = {};
    const char *tmp;
    led_data = devm_kzalloc(dev, sizeof(*led_data), GFP_KERNEL);
    if (!led_data)
    return -ENOMEM;
    if (device_get_child_node_count(dev) != 1) {
    dev_err(dev, "Device must have exactly one LED sub-node.");
    return -EINVAL;
    }
    led_data.gpiod = devm_gpiod_get(dev, "lltc,ctrl", 0);
    if (IS_ERR(led_data.gpiod))
    return PTR_ERR(led_data.gpiod);
    child = device_get_next_child_node(dev, core::ptr::null_mut());
    if (!fwnode_property_read_string(child, "default-state", &tmp)) {
    if (!strcmp(tmp, "on"))
    state = LEDS_GPIO_DEFSTATE_ON;
    }
    led_data.cdev.brightness_set_blocking = lt3593_led_set;
    led_data.cdev.brightness = state ? LED_FULL : LED_OFF;
    init_data.fwnode = child;
    init_data.devicename = LED_LT3593_NAME;
    init_data.default_label = ":";
    ret = devm_led_classdev_register_ext(dev, &led_data.cdev, &init_data);
    fwnode_handle_put(child);
    if (ret < 0)
    return ret;
    platform_set_drvdata(pdev, led_data);
    return 0;
    }
    static const struct of_device_id of_lt3593_leds_match[] = {
    { .compatible = "lltc,lt3593", },
    {},
    };
    MODULE_DEVICE_TABLE(of, of_lt3593_leds_match);
    static struct platform_driver lt3593_led_driver = {
    .probe		= lt3593_led_probe,
    .driver		= {
    .name	= "leds-lt3593",
    .of_match_table = of_lt3593_leds_match,
    },
    };
    module_platform_driver(lt3593_led_driver);
    MODULE_AUTHOR("Daniel Mack <daniel@zonque.org>");
    MODULE_DESCRIPTION("LED driver for LT3593 controllers");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:leds-lt3593");
