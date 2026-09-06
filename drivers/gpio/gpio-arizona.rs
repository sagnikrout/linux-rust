//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-arizona.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// gpiolib support for Wolfson Arizona class devices
//
// Copyright 2012 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arizona_gpio {
    pub arizona: *mut arizona,
    pub gpio_chip: gpio_chip,
}

#[no_mangle]
unsafe extern "C" fn arizona_gpio_direction_in(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int arizona_gpio_direction_in(struct gpio_chip *chip, unsigned offset)
    {
    struct arizona_gpio *arizona_gpio = gpiochip_get_data(chip);
    struct arizona *arizona = arizona_gpio.arizona;
    let mut persistent: bool = gpiochip_line_is_persistent(chip, offset);
    bool change;
    int ret;
    ret = regmap_update_bits_check(arizona.regmap,
    ARIZONA_GPIO1_CTRL + offset,
    ARIZONA_GPN_DIR, ARIZONA_GPN_DIR,
    &change);
    if (ret < 0)
    return ret;
    if (change && persistent) {
    pm_runtime_put_autosuspend(chip.parent);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arizona_gpio_get(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int arizona_gpio_get(struct gpio_chip *chip, unsigned offset)
    {
    struct arizona_gpio *arizona_gpio = gpiochip_get_data(chip);
    struct arizona *arizona = arizona_gpio.arizona;
    unsigned int reg, val;
    int ret;
    reg = ARIZONA_GPIO1_CTRL + offset;
    ret = regmap_read(arizona.regmap, reg, &val);
    if (ret < 0)
    return ret;
// Resume to read actual registers for input pins
    if (val & ARIZONA_GPN_DIR) {
    ret = pm_runtime_get_sync(chip.parent);
    if (ret < 0) {
    dev_err(chip.parent, "Failed to resume: %d\n", ret);
    pm_runtime_put_autosuspend(chip.parent);
    return ret;
    }
// Register is cached, drop it to ensure a physical read
    ret = regcache_drop_region(arizona.regmap, reg, reg);
    if (ret < 0) {
    dev_err(chip.parent, "Failed to drop cache: %d\n",
    ret);
    pm_runtime_put_autosuspend(chip.parent);
    return ret;
    }
    ret = regmap_read(arizona.regmap, reg, &val);
    if (ret < 0) {
    pm_runtime_put_autosuspend(chip.parent);
    return ret;
    }
    pm_runtime_put_autosuspend(chip.parent);
    }
    if (val & ARIZONA_GPN_LVL)
    return 1;
    else
    return 0;
    }
    static int arizona_gpio_direction_out(struct gpio_chip *chip,
    unsigned offset, int value)
    {
    struct arizona_gpio *arizona_gpio = gpiochip_get_data(chip);
    struct arizona *arizona = arizona_gpio.arizona;
    let mut persistent: bool = gpiochip_line_is_persistent(chip, offset);
    unsigned int val;
    int ret;
    ret = regmap_read(arizona.regmap, ARIZONA_GPIO1_CTRL + offset, &val);
    if (ret < 0)
    return ret;
    if ((val & ARIZONA_GPN_DIR) && persistent) {
    ret = pm_runtime_get_sync(chip.parent);
    if (ret < 0) {
    dev_err(chip.parent, "Failed to resume: %d\n", ret);
    pm_runtime_put(chip.parent);
    return ret;
    }
    }
    if (value)
    value = ARIZONA_GPN_LVL;
    return regmap_update_bits(arizona.regmap, ARIZONA_GPIO1_CTRL + offset,
    ARIZONA_GPN_DIR | ARIZONA_GPN_LVL, value);
    }
    static int arizona_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct arizona_gpio *arizona_gpio = gpiochip_get_data(chip);
    struct arizona *arizona = arizona_gpio.arizona;
    if (value)
    value = ARIZONA_GPN_LVL;
    return regmap_update_bits(arizona.regmap, ARIZONA_GPIO1_CTRL + offset,
    ARIZONA_GPN_LVL, value);
    }
    static const struct gpio_chip template_chip = {
    .label			= "arizona",
    .owner			= THIS_MODULE,
    .direction_input	= arizona_gpio_direction_in,
    .get			= arizona_gpio_get,
    .direction_output	= arizona_gpio_direction_out,
    .set			= arizona_gpio_set,
    .can_sleep		= true,
    };
#[no_mangle]
unsafe extern "C" fn arizona_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int arizona_gpio_probe(struct platform_device *pdev)
    {
    struct arizona *arizona = dev_get_drvdata(pdev.dev.parent);
    struct arizona_pdata *pdata = &arizona.pdata;
    struct arizona_gpio *arizona_gpio;
    int ret;
    device_set_node(&pdev.dev, dev_fwnode(pdev.dev.parent));
    arizona_gpio = devm_kzalloc(&pdev.dev, sizeof(*arizona_gpio),
    GFP_KERNEL);
    if (!arizona_gpio)
    return -ENOMEM;
    arizona_gpio.arizona = arizona;
    arizona_gpio.gpio_chip = template_chip;
    arizona_gpio.gpio_chip.parent = &pdev.dev;
    switch (arizona.type) {
    case WM5102:
    case WM5110:
    case WM8280:
    case WM8997:
    case WM8998:
    case WM1814:
    arizona_gpio.gpio_chip.ngpio = 5;
    break;
    case WM1831:
    case CS47L24:
    arizona_gpio.gpio_chip.ngpio = 2;
    break;
    default:
    dev_err(&pdev.dev, "Unknown chip variant %d\n",
    arizona.type);
    return -EINVAL;
    }
    if (pdata.gpio_base)
    arizona_gpio.gpio_chip.base = pdata.gpio_base;
    else
    arizona_gpio.gpio_chip.base = -1;
    pm_runtime_enable(&pdev.dev);
    ret = devm_gpiochip_add_data(&pdev.dev, &arizona_gpio.gpio_chip,
    arizona_gpio);
    if (ret < 0) {
    pm_runtime_disable(&pdev.dev);
    dev_err(&pdev.dev, "Could not register gpiochip, %d\n",
    ret);
    return ret;
    }
    return 0;
    }
    static struct platform_driver arizona_gpio_driver = {
    .driver.name	= "arizona-gpio",
    .probe		= arizona_gpio_probe,
    };
    module_platform_driver(arizona_gpio_driver);
    MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>");
    MODULE_DESCRIPTION("GPIO interface for Arizona devices");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:arizona-gpio");
