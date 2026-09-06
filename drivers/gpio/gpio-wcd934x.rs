//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-wcd934x.c
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
// Copyright (c) 2019, Linaro Limited

pub const WCD_REG_DIR_CTL_OFFSET: c_uint = 0x42;
pub const WCD_REG_VAL_CTL_OFFSET: c_uint = 0x43;
pub const WCD934X_NPINS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcd_gpio_data {
    pub map: *mut regmap,
    pub chip: gpio_chip,
}

#[no_mangle]
unsafe extern "C" fn wcd_gpio_get_direction(chip: *mut gpio_chip, pin: c_uint) -> c_int {
    static int wcd_gpio_get_direction(struct gpio_chip *chip, unsigned int pin)
    {
    struct wcd_gpio_data *data = gpiochip_get_data(chip);
    unsigned int value;
    int ret;
    ret = regmap_read(data.map, WCD_REG_DIR_CTL_OFFSET, &value);
    if (ret < 0)
    return ret;
    if (value & WCD_PIN_MASK(pin))
    return GPIO_LINE_DIRECTION_OUT;
    return GPIO_LINE_DIRECTION_IN;
    }
#[no_mangle]
unsafe extern "C" fn wcd_gpio_direction_input(chip: *mut gpio_chip, pin: c_uint) -> c_int {
    static int wcd_gpio_direction_input(struct gpio_chip *chip, unsigned int pin)
    {
    struct wcd_gpio_data *data = gpiochip_get_data(chip);
    return regmap_update_bits(data.map, WCD_REG_DIR_CTL_OFFSET,
    WCD_PIN_MASK(pin), 0);
    }
    static int wcd_gpio_direction_output(struct gpio_chip *chip, unsigned int pin,
    int val)
    {
    struct wcd_gpio_data *data = gpiochip_get_data(chip);
    int ret;
    ret = regmap_update_bits(data.map, WCD_REG_DIR_CTL_OFFSET,
    WCD_PIN_MASK(pin), WCD_PIN_MASK(pin));
    if (ret)
    return ret;
    return regmap_update_bits(data.map, WCD_REG_VAL_CTL_OFFSET,
    WCD_PIN_MASK(pin),
    val ? WCD_PIN_MASK(pin) : 0);
    }
#[no_mangle]
unsafe extern "C" fn wcd_gpio_get(chip: *mut gpio_chip, pin: c_uint) -> c_int {
    static int wcd_gpio_get(struct gpio_chip *chip, unsigned int pin)
    {
    struct wcd_gpio_data *data = gpiochip_get_data(chip);
    unsigned int value;
    regmap_read(data.map, WCD_REG_VAL_CTL_OFFSET, &value);
    return !!(value & WCD_PIN_MASK(pin));
    }
#[no_mangle]
unsafe extern "C" fn wcd_gpio_set(chip: *mut gpio_chip, pin: c_uint, val: c_int) -> c_int {
    static int wcd_gpio_set(struct gpio_chip *chip, unsigned int pin, int val)
    {
    struct wcd_gpio_data *data = gpiochip_get_data(chip);
    return regmap_update_bits(data.map, WCD_REG_VAL_CTL_OFFSET,
    WCD_PIN_MASK(pin),
    val ? WCD_PIN_MASK(pin) : 0);
    }
#[no_mangle]
unsafe extern "C" fn wcd_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int wcd_gpio_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct wcd_gpio_data *data;
    struct gpio_chip *chip;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.map = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!data.map) {
    dev_err(dev, "%s: failed to get regmap\n", __func__);
    return  -EINVAL;
    }
    chip = &data.chip;
    chip.direction_input  = wcd_gpio_direction_input;
    chip.direction_output = wcd_gpio_direction_output;
    chip.get_direction = wcd_gpio_get_direction;
    chip.get = wcd_gpio_get;
    chip.set = wcd_gpio_set;
    chip.parent = dev;
    chip.base = -1;
    chip.ngpio = WCD934X_NPINS;
    chip.label = dev_name(dev);
    chip.can_sleep = true;
    return devm_gpiochip_add_data(dev, chip, data);
    }
    static const struct of_device_id wcd_gpio_of_match[] = {
    { .compatible = "qcom,wcd9340-gpio" },
    { .compatible = "qcom,wcd9341-gpio" },
    { }
    };
    MODULE_DEVICE_TABLE(of, wcd_gpio_of_match);
    static struct platform_driver wcd_gpio_driver = {
    .driver = {
    .name = "wcd934x-gpio",
    .of_match_table = wcd_gpio_of_match,
    },
    .probe = wcd_gpio_probe,
    };
    module_platform_driver(wcd_gpio_driver);
    MODULE_DESCRIPTION("Qualcomm Technologies, Inc WCD GPIO control driver");
    MODULE_LICENSE("GPL v2");
