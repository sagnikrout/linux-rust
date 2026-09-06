//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-bd71828.c
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
// Copyright (C) 2018 ROHM Semiconductors

pub const HALL_GPIO_OFFSET: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd71828_gpio {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub gpio: gpio_chip,
}

    static int bd71828_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct bd71828_gpio *bdgpio = gpiochip_get_data(chip);
    let mut val: u8 = (value) ? BD71828_GPIO_OUT_HI : BD71828_GPIO_OUT_LO;
//
// The HALL input pin can only be used as input. If this is the pin
// we are dealing with - then we are done
//
    if (offset == HALL_GPIO_OFFSET)
    return 0;
    return regmap_update_bits(bdgpio.regmap, GPIO_OUT_REG(offset),
    BD71828_GPIO_OUT_MASK, val);
    }
#[no_mangle]
unsafe extern "C" fn bd71828_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int bd71828_gpio_get(struct gpio_chip *chip, unsigned int offset)
    {
    int ret;
    unsigned int val;
    struct bd71828_gpio *bdgpio = gpiochip_get_data(chip);
    if (offset == HALL_GPIO_OFFSET)
    ret = regmap_read(bdgpio.regmap, BD71828_REG_IO_STAT,
    &val);
    else
    ret = regmap_read(bdgpio.regmap, GPIO_OUT_REG(offset),
    &val);
    if (!ret)
    ret = (val & BD71828_GPIO_OUT_MASK);
    return ret;
    }
    static int bd71828_gpio_set_config(struct gpio_chip *chip, unsigned int offset,
    unsigned long config)
    {
    struct bd71828_gpio *bdgpio = gpiochip_get_data(chip);
    if (offset == HALL_GPIO_OFFSET)
    return -ENOTSUPP;
    switch (pinconf_to_config_param(config)) {
    case PIN_CONFIG_DRIVE_OPEN_DRAIN:
    return regmap_update_bits(bdgpio.regmap,
    GPIO_OUT_REG(offset),
    BD71828_GPIO_DRIVE_MASK,
    BD71828_GPIO_OPEN_DRAIN);
    case PIN_CONFIG_DRIVE_PUSH_PULL:
    return regmap_update_bits(bdgpio.regmap,
    GPIO_OUT_REG(offset),
    BD71828_GPIO_DRIVE_MASK,
    BD71828_GPIO_PUSH_PULL);
    default:
    break;
    }
    return -ENOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn bd71828_get_direction(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int bd71828_get_direction(struct gpio_chip *chip, unsigned int offset)
    {
//
// Pin usage is selected by OTP data. We can't read it runtime. Hence
// we trust that if the pin is not excluded by "gpio-reserved-ranges"
// the OTP configuration is set to OUT. (Other pins but HALL input pin
// on BD71828 can't really be used for general purpose input - input
// states are used for specific cases like regulator control or
// PMIC_ON_REQ.
//
    if (offset == HALL_GPIO_OFFSET)
    return GPIO_LINE_DIRECTION_IN;
    return GPIO_LINE_DIRECTION_OUT;
    }
#[no_mangle]
unsafe extern "C" fn bd71828_probe(pdev: *mut platform_device) -> c_int {
    static int bd71828_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct bd71828_gpio *bdgpio;
    bdgpio = devm_kzalloc(dev, sizeof(*bdgpio), GFP_KERNEL);
    if (!bdgpio)
    return -ENOMEM;
    bdgpio.dev = dev;
    bdgpio.gpio.parent = dev.parent;
    bdgpio.gpio.label = "bd71828-gpio";
    bdgpio.gpio.owner = THIS_MODULE;
    bdgpio.gpio.get_direction = bd71828_get_direction;
    bdgpio.gpio.set_config = bd71828_gpio_set_config;
    bdgpio.gpio.can_sleep = true;
    bdgpio.gpio.get = bd71828_gpio_get;
    bdgpio.gpio.set = bd71828_gpio_set;
    bdgpio.gpio.base = -1;
//
// See if we need some implementation to mark some PINs as
// not controllable based on DT info or if core can handle
// "gpio-reserved-ranges" and exclude them from control
//
    bdgpio.gpio.ngpio = 4;
    bdgpio.regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!bdgpio.regmap)
    return -ENODEV;
    return devm_gpiochip_add_data(dev, &bdgpio.gpio, bdgpio);
    }
    static struct platform_driver bd71828_gpio = {
    .driver = {
    .name = "bd71828-gpio"
    },
    .probe = bd71828_probe,
    };
    module_platform_driver(bd71828_gpio);
    MODULE_AUTHOR("Matti Vaittinen <matti.vaittinen@fi.rohmeurope.com>");
    MODULE_DESCRIPTION("BD71828 voltage regulator driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:bd71828-gpio");
