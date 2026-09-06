//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-madera.c
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
// GPIO support for Cirrus Logic Madera codecs
//
// Copyright (C) 2015-2018 Cirrus Logic
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct madera_gpio {
    pub madera: *mut madera,
// storage space for the gpio_chip we're using
    pub gpio_chip: gpio_chip,
}

    static int madera_gpio_get_direction(struct gpio_chip *chip,
    unsigned int offset)
    {
    struct madera_gpio *madera_gpio = gpiochip_get_data(chip);
    struct madera *madera = madera_gpio.madera;
    let mut reg_offset: c_uint = 2 * offset;
    unsigned int val;
    int ret;
    ret = regmap_read(madera.regmap, MADERA_GPIO1_CTRL_2 + reg_offset,
    &val);
    if (ret < 0)
    return ret;
    if (val & MADERA_GP1_DIR_MASK)
    return GPIO_LINE_DIRECTION_IN;
    return GPIO_LINE_DIRECTION_OUT;
    }
#[no_mangle]
unsafe extern "C" fn madera_gpio_direction_in(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int madera_gpio_direction_in(struct gpio_chip *chip, unsigned int offset)
    {
    struct madera_gpio *madera_gpio = gpiochip_get_data(chip);
    struct madera *madera = madera_gpio.madera;
    let mut reg_offset: c_uint = 2 * offset;
    return regmap_update_bits(madera.regmap,
    MADERA_GPIO1_CTRL_2 + reg_offset,
    MADERA_GP1_DIR_MASK, MADERA_GP1_DIR);
    }
#[no_mangle]
unsafe extern "C" fn madera_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int madera_gpio_get(struct gpio_chip *chip, unsigned int offset)
    {
    struct madera_gpio *madera_gpio = gpiochip_get_data(chip);
    struct madera *madera = madera_gpio.madera;
    let mut reg_offset: c_uint = 2 * offset;
    unsigned int val;
    int ret;
    ret = regmap_read(madera.regmap, MADERA_GPIO1_CTRL_1 + reg_offset,
    &val);
    if (ret < 0)
    return ret;
    return !!(val & MADERA_GP1_LVL_MASK);
    }
    static int madera_gpio_direction_out(struct gpio_chip *chip,
    unsigned int offset, int value)
    {
    struct madera_gpio *madera_gpio = gpiochip_get_data(chip);
    struct madera *madera = madera_gpio.madera;
    let mut reg_offset: c_uint = 2 * offset;
    let mut reg_val: c_uint = value ? MADERA_GP1_LVL : 0;
    int ret;
    ret = regmap_update_bits(madera.regmap,
    MADERA_GPIO1_CTRL_2 + reg_offset,
    MADERA_GP1_DIR_MASK, 0);
    if (ret < 0)
    return ret;
    return regmap_update_bits(madera.regmap,
    MADERA_GPIO1_CTRL_1 + reg_offset,
    MADERA_GP1_LVL_MASK, reg_val);
    }
    static int madera_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct madera_gpio *madera_gpio = gpiochip_get_data(chip);
    struct madera *madera = madera_gpio.madera;
    let mut reg_offset: c_uint = 2 * offset;
    let mut reg_val: c_uint = value ? MADERA_GP1_LVL : 0;
    return regmap_update_bits(madera.regmap,
    MADERA_GPIO1_CTRL_1 + reg_offset,
    MADERA_GP1_LVL_MASK, reg_val);
    }
    static const struct gpio_chip madera_gpio_chip = {
    .label			= "madera",
    .owner			= THIS_MODULE,
    .request		= gpiochip_generic_request,
    .free			= gpiochip_generic_free,
    .get_direction		= madera_gpio_get_direction,
    .direction_input	= madera_gpio_direction_in,
    .get			= madera_gpio_get,
    .direction_output	= madera_gpio_direction_out,
    .set			= madera_gpio_set,
    .set_config		= gpiochip_generic_config,
    .can_sleep		= true,
    };
#[no_mangle]
unsafe extern "C" fn madera_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int madera_gpio_probe(struct platform_device *pdev)
    {
    struct madera *madera = dev_get_drvdata(pdev.dev.parent);
    struct madera_pdata *pdata = &madera.pdata;
    struct madera_gpio *madera_gpio;
    int ret;
    madera_gpio = devm_kzalloc(&pdev.dev, sizeof(*madera_gpio),
    GFP_KERNEL);
    if (!madera_gpio)
    return -ENOMEM;
    madera_gpio.madera = madera;
// Construct suitable gpio_chip from the template in madera_gpio_chip
    madera_gpio.gpio_chip = madera_gpio_chip;
    madera_gpio.gpio_chip.parent = pdev.dev.parent;
    switch (madera.type) {
    case CS47L15:
    madera_gpio.gpio_chip.ngpio = CS47L15_NUM_GPIOS;
    break;
    case CS47L35:
    madera_gpio.gpio_chip.ngpio = CS47L35_NUM_GPIOS;
    break;
    case CS47L85:
    case WM1840:
    madera_gpio.gpio_chip.ngpio = CS47L85_NUM_GPIOS;
    break;
    case CS47L90:
    case CS47L91:
    madera_gpio.gpio_chip.ngpio = CS47L90_NUM_GPIOS;
    break;
    case CS42L92:
    case CS47L92:
    case CS47L93:
    madera_gpio.gpio_chip.ngpio = CS47L92_NUM_GPIOS;
    break;
    default:
    dev_err(&pdev.dev, "Unknown chip variant %d\n", madera.type);
    return -EINVAL;
    }
// We want to be usable on systems that don't use devicetree or acpi
    if (pdata.gpio_base)
    madera_gpio.gpio_chip.base = pdata.gpio_base;
    else
    madera_gpio.gpio_chip.base = -1;
    ret = devm_gpiochip_add_data(&pdev.dev,
    &madera_gpio.gpio_chip,
    madera_gpio);
    if (ret < 0) {
    dev_dbg(&pdev.dev, "Could not register gpiochip, %d\n", ret);
    return ret;
    }
//
// This is part of a composite MFD device which can only be used with
// the corresponding pinctrl driver. On all supported silicon the GPIO
// to pinctrl mapping is fixed in the silicon, so we register it
// explicitly instead of requiring a redundant gpio-ranges in the
// devicetree.
// In any case we also want to work on systems that don't use devicetree
// or acpi.
//
    ret = gpiochip_add_pin_range(&madera_gpio.gpio_chip, "madera-pinctrl",
    0, 0, madera_gpio.gpio_chip.ngpio);
    if (ret) {
    dev_dbg(&pdev.dev, "Failed to add pin range (%d)\n", ret);
    return ret;
    }
    return 0;
    }
    static struct platform_driver madera_gpio_driver = {
    .driver = {
    .name	= "madera-gpio",
    },
    .probe		= madera_gpio_probe,
    };
    module_platform_driver(madera_gpio_driver);
    MODULE_SOFTDEP("pre: pinctrl-madera");
    MODULE_DESCRIPTION("GPIO interface for Madera codecs");
    MODULE_AUTHOR("Nariman Poushin <nariman@opensource.cirrus.com>");
    MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:madera-gpio");
