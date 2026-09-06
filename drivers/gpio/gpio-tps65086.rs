//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-tps65086.c
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
// Copyright (C) 2015-2023 Texas Instruments Incorporated - https://www.ti.com
// Andrew Davis <afd@ti.com>
//
// Based on the TPS65912 driver
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps65086_gpio {
    pub chip: gpio_chip,
    pub tps: *mut tps65086,
}

    static int tps65086_gpio_get_direction(struct gpio_chip *chip,
    unsigned offset)
    {
// This device is output only
    return GPIO_LINE_DIRECTION_OUT;
    }
    static int tps65086_gpio_direction_input(struct gpio_chip *chip,
    unsigned offset)
    {
// This device is output only
    return -EINVAL;
    }
    static int tps65086_gpio_direction_output(struct gpio_chip *chip,
    unsigned offset, int value)
    {
    struct tps65086_gpio *gpio = gpiochip_get_data(chip);
// Set the initial value
    return regmap_update_bits(gpio.tps.regmap, TPS65086_GPOCTRL,
    BIT(4 + offset), value ? BIT(4 + offset) : 0);
    }
#[no_mangle]
unsafe extern "C" fn tps65086_gpio_get(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int tps65086_gpio_get(struct gpio_chip *chip, unsigned offset)
    {
    struct tps65086_gpio *gpio = gpiochip_get_data(chip);
    int ret, val;
    ret = regmap_read(gpio.tps.regmap, TPS65086_GPOCTRL, &val);
    if (ret < 0)
    return ret;
    return !!(val & BIT(4 + offset));
    }
    static int tps65086_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct tps65086_gpio *gpio = gpiochip_get_data(chip);
    return regmap_update_bits(gpio.tps.regmap, TPS65086_GPOCTRL,
    BIT(4 + offset), value ? BIT(4 + offset) : 0);
    }
    static const struct gpio_chip template_chip = {
    .label			= "tps65086-gpio",
    .owner			= THIS_MODULE,
    .get_direction		= tps65086_gpio_get_direction,
    .direction_input	= tps65086_gpio_direction_input,
    .direction_output	= tps65086_gpio_direction_output,
    .get			= tps65086_gpio_get,
    .set			= tps65086_gpio_set,
    .base			= -1,
    .ngpio			= 4,
    .can_sleep		= true,
    };
#[no_mangle]
unsafe extern "C" fn tps65086_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int tps65086_gpio_probe(struct platform_device *pdev)
    {
    struct tps65086_gpio *gpio;
    gpio = devm_kzalloc(&pdev.dev, sizeof(*gpio), GFP_KERNEL);
    if (!gpio)
    return -ENOMEM;
    gpio.tps = dev_get_drvdata(pdev.dev.parent);
    gpio.chip = template_chip;
    gpio.chip.parent = gpio.tps.dev;
    return devm_gpiochip_add_data(&pdev.dev, &gpio.chip, gpio);
    }
    static const struct platform_device_id tps65086_gpio_id_table[] = {
    { .name = "tps65086-gpio" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(platform, tps65086_gpio_id_table);
    static struct platform_driver tps65086_gpio_driver = {
    .driver = {
    .name = "tps65086-gpio",
    },
    .probe = tps65086_gpio_probe,
    .id_table = tps65086_gpio_id_table,
    };
    module_platform_driver(tps65086_gpio_driver);
    MODULE_AUTHOR("Andrew Davis <afd@ti.com>");
    MODULE_DESCRIPTION("TPS65086 GPIO driver");
    MODULE_LICENSE("GPL v2");
