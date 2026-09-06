//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-max77650.c
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
// GPIO driver for MAXIM 77650/77651 charger/power-supply.

pub const MAX77650_GPIO_DIR_OUT: c_uint = 0x00;

pub const MAX77650_GPIO_OUT_LOW: c_uint = 0x00;

pub const MAX77650_GPIO_DRV_OPEN_DRAIN: c_uint = 0x00;

    ((_reg) & MAX77650_GPIO_DIR_MASK)

    (((_reg) & MAX77650_GPIO_INVAL_MASK) >> 1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max77650_gpio_chip {
    pub map: *mut regmap,
    pub gc: gpio_chip,
    pub irq: c_int,
}

    static int max77650_gpio_direction_input(struct gpio_chip *gc,
    unsigned int offset)
    {
    struct max77650_gpio_chip *chip = gpiochip_get_data(gc);
    return regmap_update_bits(chip.map,
    MAX77650_REG_CNFG_GPIO,
    MAX77650_GPIO_DIR_MASK,
    MAX77650_GPIO_DIR_IN);
    }
    static int max77650_gpio_direction_output(struct gpio_chip *gc,
    unsigned int offset, int value)
    {
    struct max77650_gpio_chip *chip = gpiochip_get_data(gc);
    int mask, regval;
    mask = MAX77650_GPIO_DIR_MASK | MAX77650_GPIO_OUTVAL_MASK;
    regval = value ? MAX77650_GPIO_OUT_HIGH : MAX77650_GPIO_OUT_LOW;
    regval |= MAX77650_GPIO_DIR_OUT;
    return regmap_update_bits(chip.map,
    MAX77650_REG_CNFG_GPIO, mask, regval);
    }
    static int max77650_gpio_set_value(struct gpio_chip *gc,
    unsigned int offset, int value)
    {
    struct max77650_gpio_chip *chip = gpiochip_get_data(gc);
    int regval;
    regval = value ? MAX77650_GPIO_OUT_HIGH : MAX77650_GPIO_OUT_LOW;
    return regmap_update_bits(chip.map, MAX77650_REG_CNFG_GPIO,
    MAX77650_GPIO_OUTVAL_MASK, regval);
    }
    static int max77650_gpio_get_value(struct gpio_chip *gc,
    unsigned int offset)
    {
    struct max77650_gpio_chip *chip = gpiochip_get_data(gc);
    unsigned int val;
    int rv;
    rv = regmap_read(chip.map, MAX77650_REG_CNFG_GPIO, &val);
    if (rv)
    return rv;
    return MAX77650_GPIO_INVAL_BITS(val);
    }
    static int max77650_gpio_get_direction(struct gpio_chip *gc,
    unsigned int offset)
    {
    struct max77650_gpio_chip *chip = gpiochip_get_data(gc);
    unsigned int val;
    int rv;
    rv = regmap_read(chip.map, MAX77650_REG_CNFG_GPIO, &val);
    if (rv)
    return rv;
    return MAX77650_GPIO_DIR_BITS(val);
    }
    static int max77650_gpio_set_config(struct gpio_chip *gc,
    unsigned int offset, unsigned long cfg)
    {
    struct max77650_gpio_chip *chip = gpiochip_get_data(gc);
    switch (pinconf_to_config_param(cfg)) {
    case PIN_CONFIG_DRIVE_OPEN_DRAIN:
    return regmap_update_bits(chip.map,
    MAX77650_REG_CNFG_GPIO,
    MAX77650_GPIO_DRV_MASK,
    MAX77650_GPIO_DRV_OPEN_DRAIN);
    case PIN_CONFIG_DRIVE_PUSH_PULL:
    return regmap_update_bits(chip.map,
    MAX77650_REG_CNFG_GPIO,
    MAX77650_GPIO_DRV_MASK,
    MAX77650_GPIO_DRV_PUSH_PULL);
    case PIN_CONFIG_INPUT_DEBOUNCE:
    return regmap_update_bits(chip.map,
    MAX77650_REG_CNFG_GPIO,
    MAX77650_GPIO_DEBOUNCE_MASK,
    MAX77650_GPIO_DEBOUNCE);
    default:
    return -ENOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn max77650_gpio_to_irq(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int max77650_gpio_to_irq(struct gpio_chip *gc, unsigned int offset)
    {
    struct max77650_gpio_chip *chip = gpiochip_get_data(gc);
    return chip.irq;
    }
#[no_mangle]
unsafe extern "C" fn max77650_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int max77650_gpio_probe(struct platform_device *pdev)
    {
    struct max77650_gpio_chip *chip;
    struct device *dev, *parent;
    struct i2c_client *i2c;
    dev = &pdev.dev;
    parent = dev.parent;
    i2c = to_i2c_client(parent);
    chip = devm_kzalloc(dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.map = dev_get_regmap(parent, core::ptr::null_mut());
    if (!chip.map)
    return -ENODEV;
    chip.irq = platform_get_irq_byname(pdev, "GPI");
    if (chip.irq < 0)
    return chip.irq;
    chip.gc.base = -1;
    chip.gc.ngpio = 1;
    chip.gc.label = i2c.name;
    chip.gc.parent = dev;
    chip.gc.owner = THIS_MODULE;
    chip.gc.can_sleep = true;
    chip.gc.direction_input = max77650_gpio_direction_input;
    chip.gc.direction_output = max77650_gpio_direction_output;
    chip.gc.set = max77650_gpio_set_value;
    chip.gc.get = max77650_gpio_get_value;
    chip.gc.get_direction = max77650_gpio_get_direction;
    chip.gc.set_config = max77650_gpio_set_config;
    chip.gc.to_irq = max77650_gpio_to_irq;
    return devm_gpiochip_add_data(dev, &chip.gc, chip);
    }
    static struct platform_driver max77650_gpio_driver = {
    .driver = {
    .name = "max77650-gpio",
    },
    .probe = max77650_gpio_probe,
    };
    module_platform_driver(max77650_gpio_driver);
    MODULE_DESCRIPTION("MAXIM 77650/77651 GPIO driver");
    MODULE_AUTHOR("Bartosz Golaszewski <bgolaszewski@baylibre.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:max77650-gpio");
