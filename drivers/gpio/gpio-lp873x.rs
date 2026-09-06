//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-lp873x.c
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
// Copyright (C) 2016 Texas Instruments Incorporated - http://www.ti.com
// Keerthy <j-keerthy@ti.com>
//
// Based on the TPS65218 driver
//

pub const BITS_PER_GPO: c_uint = 0x4;
pub const LP873X_GPO_CTRL_OD: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp873x_gpio {
    pub chip: gpio_chip,
    pub lp873: *mut lp873x,
}

    static int lp873x_gpio_get_direction(struct gpio_chip *chip,
    unsigned int offset)
    {
// This device is output only
    return GPIO_LINE_DIRECTION_OUT;
    }
    static int lp873x_gpio_direction_input(struct gpio_chip *chip,
    unsigned int offset)
    {
// This device is output only
    return -EINVAL;
    }
    static int lp873x_gpio_direction_output(struct gpio_chip *chip,
    unsigned int offset, int value)
    {
    struct lp873x_gpio *gpio = gpiochip_get_data(chip);
// Set the initial value
    return regmap_update_bits(gpio.lp873.regmap, LP873X_REG_GPO_CTRL,
    BIT(offset * BITS_PER_GPO),
    value ? BIT(offset * BITS_PER_GPO) : 0);
    }
#[no_mangle]
unsafe extern "C" fn lp873x_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int lp873x_gpio_get(struct gpio_chip *chip, unsigned int offset)
    {
    struct lp873x_gpio *gpio = gpiochip_get_data(chip);
    int ret, val;
    ret = regmap_read(gpio.lp873.regmap, LP873X_REG_GPO_CTRL, &val);
    if (ret < 0)
    return ret;
    return !!(val & BIT(offset * BITS_PER_GPO));
    }
    static int lp873x_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct lp873x_gpio *gpio = gpiochip_get_data(chip);
    return regmap_update_bits(gpio.lp873.regmap, LP873X_REG_GPO_CTRL,
    BIT(offset * BITS_PER_GPO),
    value ? BIT(offset * BITS_PER_GPO) : 0);
    }
#[no_mangle]
unsafe extern "C" fn lp873x_gpio_request(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int lp873x_gpio_request(struct gpio_chip *gc, unsigned int offset)
    {
    struct lp873x_gpio *gpio = gpiochip_get_data(gc);
    int ret;
    switch (offset) {
    case 0:
// No MUX Set up Needed for GPO
    break;
    case 1:
// Setup the CLKIN_PIN_SEL MUX to GPO2
    ret = regmap_update_bits(gpio.lp873.regmap, LP873X_REG_CONFIG,
    LP873X_CONFIG_CLKIN_PIN_SEL, 0);
    if (ret)
    return ret;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int lp873x_gpio_set_config(struct gpio_chip *gc, unsigned offset,
    unsigned long config)
    {
    struct lp873x_gpio *gpio = gpiochip_get_data(gc);
    switch (pinconf_to_config_param(config)) {
    case PIN_CONFIG_DRIVE_OPEN_DRAIN:
    return regmap_update_bits(gpio.lp873.regmap,
    LP873X_REG_GPO_CTRL,
    BIT(offset * BITS_PER_GPO +
    LP873X_GPO_CTRL_OD),
    BIT(offset * BITS_PER_GPO +
    LP873X_GPO_CTRL_OD));
    case PIN_CONFIG_DRIVE_PUSH_PULL:
    return regmap_update_bits(gpio.lp873.regmap,
    LP873X_REG_GPO_CTRL,
    BIT(offset * BITS_PER_GPO +
    LP873X_GPO_CTRL_OD), 0);
    default:
    return -ENOTSUPP;
    }
    }
    static const struct gpio_chip template_chip = {
    .label			= "lp873x-gpio",
    .owner			= THIS_MODULE,
    .request		= lp873x_gpio_request,
    .get_direction		= lp873x_gpio_get_direction,
    .direction_input	= lp873x_gpio_direction_input,
    .direction_output	= lp873x_gpio_direction_output,
    .get			= lp873x_gpio_get,
    .set			= lp873x_gpio_set,
    .set_config		= lp873x_gpio_set_config,
    .base			= -1,
    .ngpio			= 2,
    .can_sleep		= true,
    };
#[no_mangle]
unsafe extern "C" fn lp873x_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int lp873x_gpio_probe(struct platform_device *pdev)
    {
    struct lp873x_gpio *gpio;
    int ret;
    gpio = devm_kzalloc(&pdev.dev, sizeof(*gpio), GFP_KERNEL);
    if (!gpio)
    return -ENOMEM;
    platform_set_drvdata(pdev, gpio);
    gpio.lp873 = dev_get_drvdata(pdev.dev.parent);
    gpio.chip = template_chip;
    gpio.chip.parent = gpio.lp873.dev;
    ret = devm_gpiochip_add_data(&pdev.dev, &gpio.chip, gpio);
    if (ret < 0) {
    dev_err(&pdev.dev, "Could not register gpiochip, %d\n", ret);
    return ret;
    }
    return 0;
    }
    static const struct platform_device_id lp873x_gpio_id_table[] = {
    { .name = "lp873x-gpio" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(platform, lp873x_gpio_id_table);
    static struct platform_driver lp873x_gpio_driver = {
    .driver = {
    .name = "lp873x-gpio",
    },
    .probe = lp873x_gpio_probe,
    .id_table = lp873x_gpio_id_table,
    };
    module_platform_driver(lp873x_gpio_driver);
    MODULE_AUTHOR("Keerthy <j-keerthy@ti.com>");
    MODULE_DESCRIPTION("LP873X GPIO driver");
    MODULE_LICENSE("GPL v2");
