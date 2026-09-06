//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-lm36274.c
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
// TI LM36274 LED chip family driver
// Copyright (C) 2019 Texas Instruments Incorporated - https://www.ti.com

pub const LM36274_MAX_STRINGS: c_int = 4;

//
// struct lm36274
// @pdev: platform device
// @led_dev: led class device
// @lmu_data: Register and setting values for common code
// @regmap: Devices register map
// @dev: Pointer to the devices device struct
// @led_sources: The LED strings supported in this array
// @num_leds: Number of LED strings are supported in this array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm36274 {
    pub pdev: *mut platform_device,
    pub led_dev: led_classdev,
    pub lmu_data: ti_lmu_bank,
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub led_sources: [u32; LM36274_MAX_STRINGS],
    pub num_leds: c_int,
}

    static int lm36274_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness brt_val)
    {
    struct lm36274 *chip = container_of(led_cdev, struct lm36274, led_dev);
    return ti_lmu_common_set_brightness(&chip.lmu_data, brt_val);
    }
#[no_mangle]
unsafe extern "C" fn lm36274_init(chip: *mut lm36274) -> c_int {
    static int lm36274_init(struct lm36274 *chip)
    {
    let mut enable_val: c_int = 0;
    int i;
    for (i = 0; i < chip.num_leds; i++)
    enable_val |= (1 << chip.led_sources[i]);
    if (!enable_val) {
    dev_err(chip.dev, "No LEDs were enabled\n");
    return -EINVAL;
    }
    enable_val |= LM36274_BL_EN;
    return regmap_write(chip.regmap, LM36274_REG_BL_EN, enable_val);
    }
    static int lm36274_parse_dt(struct lm36274 *chip,
    struct led_init_data *init_data)
    {
    struct device *dev = chip.dev;
    struct fwnode_handle *child;
    int ret;
// There should only be 1 node
    if (device_get_child_node_count(dev) != 1)
    return -EINVAL;
    child = device_get_next_child_node(dev, core::ptr::null_mut());
    init_data.fwnode = child;
    init_data.devicename = chip.pdev.name;
// for backwards compatibility when `label` property is not present
    init_data.default_label = ":";
    chip.num_leds = fwnode_property_count_u32(child, "led-sources");
    if (chip.num_leds <= 0) {
    ret = -ENODEV;
    goto err;
    }
    ret = fwnode_property_read_u32_array(child, "led-sources",
    chip.led_sources, chip.num_leds);
    if (ret) {
    dev_err(dev, "led-sources property missing\n");
    goto err;
    }
    return 0;
    err:
    fwnode_handle_put(child);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lm36274_probe(pdev: *mut platform_device) -> c_int {
    static int lm36274_probe(struct platform_device *pdev)
    {
    struct ti_lmu *lmu = dev_get_drvdata(pdev.dev.parent);
    let mut init_data: led_init_data = {};
    struct lm36274 *chip;
    int ret;
    chip = devm_kzalloc(&pdev.dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.pdev = pdev;
    chip.dev = &pdev.dev;
    chip.regmap = lmu.regmap;
    platform_set_drvdata(pdev, chip);
    ret = lm36274_parse_dt(chip, &init_data);
    if (ret) {
    dev_err(chip.dev, "Failed to parse DT node\n");
    return ret;
    }
    ret = lm36274_init(chip);
    if (ret) {
    fwnode_handle_put(init_data.fwnode);
    dev_err(chip.dev, "Failed to init the device\n");
    return ret;
    }
    chip.lmu_data.regmap = chip.regmap;
    chip.lmu_data.max_brightness = MAX_BRIGHTNESS_11BIT;
    chip.lmu_data.msb_brightness_reg = LM36274_REG_BRT_MSB;
    chip.lmu_data.lsb_brightness_reg = LM36274_REG_BRT_LSB;
    chip.led_dev.max_brightness = MAX_BRIGHTNESS_11BIT;
    chip.led_dev.brightness_set_blocking = lm36274_brightness_set;
    ret = devm_led_classdev_register_ext(chip.dev, &chip.led_dev,
    &init_data);
    if (ret)
    dev_err(chip.dev, "Failed to register LED for node %pfw\n",
    init_data.fwnode);
    fwnode_handle_put(init_data.fwnode);
    return ret;
    }
    static const struct of_device_id of_lm36274_leds_match[] = {
    { .compatible = "ti,lm36274-backlight", },
    {},
    };
    MODULE_DEVICE_TABLE(of, of_lm36274_leds_match);
    static struct platform_driver lm36274_driver = {
    .probe  = lm36274_probe,
    .driver = {
    .name = "lm36274-leds",
    .of_match_table = of_lm36274_leds_match,
    },
    };
    module_platform_driver(lm36274_driver)
    MODULE_DESCRIPTION("Texas Instruments LM36274 LED driver");
    MODULE_AUTHOR("Dan Murphy <dmurphy@ti.com>");
    MODULE_LICENSE("GPL v2");
