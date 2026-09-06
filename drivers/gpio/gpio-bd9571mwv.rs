//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-bd9571mwv.c
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
// ROHM BD9571MWV-M and BD9574MWF-M GPIO driver
//
// Copyright (C) 2017 Marek Vasut <marek.vasut+renesas@gmail.com>
//
// Based on the TPS65086 driver
//
// NOTE: Interrupts are not supported yet.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd9571mwv_gpio {
    pub regmap: *mut regmap,
    pub chip: gpio_chip,
}

    static int bd9571mwv_gpio_get_direction(struct gpio_chip *chip,
    unsigned int offset)
    {
    struct bd9571mwv_gpio *gpio = gpiochip_get_data(chip);
    int ret, val;
    ret = regmap_read(gpio.regmap, BD9571MWV_GPIO_DIR, &val);
    if (ret < 0)
    return ret;
    if (val & BIT(offset))
    return GPIO_LINE_DIRECTION_IN;
    return GPIO_LINE_DIRECTION_OUT;
    }
    static int bd9571mwv_gpio_direction_input(struct gpio_chip *chip,
    unsigned int offset)
    {
    struct bd9571mwv_gpio *gpio = gpiochip_get_data(chip);
    regmap_update_bits(gpio.regmap, BD9571MWV_GPIO_DIR, BIT(offset), 0);
    return 0;
    }
    static int bd9571mwv_gpio_direction_output(struct gpio_chip *chip,
    unsigned int offset, int value)
    {
    struct bd9571mwv_gpio *gpio = gpiochip_get_data(chip);
// Set the initial value
    regmap_update_bits(gpio.regmap, BD9571MWV_GPIO_OUT,
    BIT(offset), value ? BIT(offset) : 0);
    regmap_update_bits(gpio.regmap, BD9571MWV_GPIO_DIR,
    BIT(offset), BIT(offset));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bd9571mwv_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int bd9571mwv_gpio_get(struct gpio_chip *chip, unsigned int offset)
    {
    struct bd9571mwv_gpio *gpio = gpiochip_get_data(chip);
    int ret, val;
    ret = regmap_read(gpio.regmap, BD9571MWV_GPIO_IN, &val);
    if (ret < 0)
    return ret;
    return !!(val & BIT(offset));
    }
    static int bd9571mwv_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct bd9571mwv_gpio *gpio = gpiochip_get_data(chip);
    return regmap_update_bits(gpio.regmap, BD9571MWV_GPIO_OUT,
    BIT(offset), value ? BIT(offset) : 0);
    }
    static const struct gpio_chip template_chip = {
    .label			= "bd9571mwv-gpio",
    .owner			= THIS_MODULE,
    .get_direction		= bd9571mwv_gpio_get_direction,
    .direction_input	= bd9571mwv_gpio_direction_input,
    .direction_output	= bd9571mwv_gpio_direction_output,
    .get			= bd9571mwv_gpio_get,
    .set			= bd9571mwv_gpio_set,
    .base			= -1,
    .ngpio			= 2,
    .can_sleep		= true,
    };
#[no_mangle]
unsafe extern "C" fn bd9571mwv_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int bd9571mwv_gpio_probe(struct platform_device *pdev)
    {
    struct bd9571mwv_gpio *gpio;
    gpio = devm_kzalloc(&pdev.dev, sizeof(*gpio), GFP_KERNEL);
    if (!gpio)
    return -ENOMEM;
    gpio.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    gpio.chip = template_chip;
    gpio.chip.parent = pdev.dev.parent;
    return devm_gpiochip_add_data(&pdev.dev, &gpio.chip, gpio);
    }
    static const struct platform_device_id bd9571mwv_gpio_id_table[] = {
    { .name = "bd9571mwv-gpio", .driver_data = ROHM_CHIP_TYPE_BD9571 },
    { .name = "bd9574mwf-gpio", .driver_data = ROHM_CHIP_TYPE_BD9574 },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(platform, bd9571mwv_gpio_id_table);
    static struct platform_driver bd9571mwv_gpio_driver = {
    .driver = {
    .name = "bd9571mwv-gpio",
    },
    .probe = bd9571mwv_gpio_probe,
    .id_table = bd9571mwv_gpio_id_table,
    };
    module_platform_driver(bd9571mwv_gpio_driver);
    MODULE_AUTHOR("Marek Vasut <marek.vasut+renesas@gmail.com>");
    MODULE_DESCRIPTION("BD9571MWV GPIO driver");
    MODULE_LICENSE("GPL v2");
