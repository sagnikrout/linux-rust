//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-by-pinctrl.c
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
// Copyright (C) 2026 Linaro Inc.
// Author: AKASHI takahiro <takahiro.akashi@linaro.org>

#[no_mangle]
unsafe extern "C" fn pin_control_gpio_get_direction(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int pin_control_gpio_get_direction(struct gpio_chip *gc, unsigned int offset)
    {
    unsigned long config;
    int ret;
    config = PIN_CONFIG_OUTPUT_ENABLE;
    ret = pinctrl_gpio_get_config(gc, offset, &config);
    if (ret)
    return ret;
    if (config)
    return GPIO_LINE_DIRECTION_OUT;
    return GPIO_LINE_DIRECTION_IN;
    }
#[no_mangle]
unsafe extern "C" fn pin_control_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int pin_control_gpio_get(struct gpio_chip *chip, unsigned int offset)
    {
    unsigned long config;
    int ret;
    config = PIN_CONFIG_LEVEL;
    ret = pinctrl_gpio_get_config(chip, offset, &config);
    if (ret)
    return ret;
    return !!config;
    }
    static int pin_control_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int val)
    {
    unsigned long config;
    config = pinconf_to_config_packed(PIN_CONFIG_LEVEL, val);
    return pinctrl_gpio_set_config(chip, offset, config);
    }
    static int pin_control_gpio_direction_output(struct gpio_chip *chip,
    unsigned int offset, int val)
    {
    int ret;
    ret = pin_control_gpio_set(chip, offset, val);
    if (ret)
    return ret;
    return pinctrl_gpio_direction_output(chip, offset);
    }
#[no_mangle]
unsafe extern "C" fn pin_control_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int pin_control_gpio_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct gpio_chip *chip;
    chip = devm_kzalloc(dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.label = dev_name(dev);
    chip.parent = dev;
    chip.base = -1;
    chip.request = gpiochip_generic_request;
    chip.free = gpiochip_generic_free;
    chip.get_direction = pin_control_gpio_get_direction;
    chip.direction_input = pinctrl_gpio_direction_input;
    chip.direction_output = pin_control_gpio_direction_output;
    chip.get = pin_control_gpio_get;
    chip.set = pin_control_gpio_set;
    chip.set_config = gpiochip_generic_config;
    return devm_gpiochip_add_data(dev, chip, core::ptr::null_mut());
    }
    static const struct of_device_id pin_control_gpio_match[] = {
    { .compatible = "scmi-pinctrl-gpio" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, pin_control_gpio_match);
    static struct platform_driver pin_control_gpio_driver = {
    .probe = pin_control_gpio_probe,
    .driver = {
    .name = "pin-control-gpio",
    .of_match_table = pin_control_gpio_match,
    },
    };
    module_platform_driver(pin_control_gpio_driver);
    MODULE_AUTHOR("AKASHI Takahiro <takahiro.akashi@linaro.org>");
    MODULE_DESCRIPTION("Pinctrl based GPIO driver");
    MODULE_LICENSE("GPL");
