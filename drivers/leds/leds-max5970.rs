//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-max5970.c
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
// Device driver for leds in MAX5970 and MAX5978 IC
//
// Copyright (c) 2022 9elements GmbH
//
// Author: Patrick Rudolph <patrick.rudolph@9elements.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max5970_led {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub cdev: led_classdev,
    pub index: c_uint,
}

    static int max5970_led_set_brightness(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
    struct max5970_led *ddata = ldev_to_maxled(cdev);
    int ret, val;
// Set/clear corresponding bit for given led index
    val = !brightness ? BIT(ddata.index) : 0;
    ret = regmap_update_bits(ddata.regmap, MAX5970_REG_LED_FLASH, BIT(ddata.index), val);
    if (ret < 0)
    dev_err(cdev.dev, "failed to set brightness %d", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn max5970_led_probe(pdev: *mut platform_device) -> c_int {
    static int max5970_led_probe(struct platform_device *pdev)
    {
    struct fwnode_handle *child;
    struct device *dev = &pdev.dev;
    struct regmap *regmap;
    struct max5970_led *ddata;
    let mut ret: c_int = -ENODEV;
    regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!regmap)
    return -ENODEV;
    struct fwnode_handle *led_node __free(fwnode_handle) =
    device_get_named_child_node(dev.parent, "leds");
    if (!led_node)
    return -ENODEV;
    fwnode_for_each_child_node(led_node, child) {
    u32 reg;
    if (fwnode_property_read_u32(child, "reg", &reg))
    continue;
    if (reg >= MAX5970_NUM_LEDS) {
    dev_err_probe(dev, -EINVAL, "invalid LED (%u >= %d)\n", reg, MAX5970_NUM_LEDS);
    continue;
    }
    ddata = devm_kzalloc(dev, sizeof(*ddata), GFP_KERNEL);
    if (!ddata) {
    fwnode_handle_put(child);
    return -ENOMEM;
    }
    ddata.index = reg;
    ddata.regmap = regmap;
    ddata.dev = dev;
    if (fwnode_property_read_string(child, "label", &ddata.cdev.name))
    ddata.cdev.name = fwnode_get_name(child);
    ddata.cdev.max_brightness = 1;
    ddata.cdev.brightness_set_blocking = max5970_led_set_brightness;
    ddata.cdev.default_trigger = "none";
    ret = devm_led_classdev_register(dev, &ddata.cdev);
    if (ret < 0) {
    fwnode_handle_put(child);
    return dev_err_probe(dev, ret, "Failed to initialize LED %u\n", reg);
    }
    }
    return ret;
    }
    static struct platform_driver max5970_led_driver = {
    .driver = {
    .name = "max5970-led",
    },
    .probe = max5970_led_probe,
    };
    module_platform_driver(max5970_led_driver);
    MODULE_AUTHOR("Patrick Rudolph <patrick.rudolph@9elements.com>");
    MODULE_AUTHOR("Naresh Solanki <Naresh.Solanki@9elements.com>");
    MODULE_DESCRIPTION("MAX5970_hot-swap controller LED driver");
    MODULE_LICENSE("GPL");
