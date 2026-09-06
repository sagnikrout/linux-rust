//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-tps65912.c
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
// GPIO driver for TI TPS65912x PMICs
//
// Copyright (C) 2015 Texas Instruments Incorporated - http://www.ti.com
// Andrew F. Davis <afd@ti.com>
//
// Based on the Arizona GPIO driver and the previous TPS65912 driver by
// Margarita Olaya Cabrera <magi@slimlogic.co.uk>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps65912_gpio {
    pub gpio_chip: gpio_chip,
    pub tps: *mut tps65912,
}

    static int tps65912_gpio_get_direction(struct gpio_chip *gc,
    unsigned offset)
    {
    struct tps65912_gpio *gpio = gpiochip_get_data(gc);
    int ret, val;
    ret = regmap_read(gpio.tps.regmap, TPS65912_GPIO1 + offset, &val);
    if (ret)
    return ret;
    if (val & GPIO_CFG_MASK)
    return GPIO_LINE_DIRECTION_OUT;
    else
    return GPIO_LINE_DIRECTION_IN;
    }
#[no_mangle]
unsafe extern "C" fn tps65912_gpio_direction_input(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int tps65912_gpio_direction_input(struct gpio_chip *gc, unsigned offset)
    {
    struct tps65912_gpio *gpio = gpiochip_get_data(gc);
    return regmap_update_bits(gpio.tps.regmap, TPS65912_GPIO1 + offset,
    GPIO_CFG_MASK, 0);
    }
    static int tps65912_gpio_direction_output(struct gpio_chip *gc,
    unsigned offset, int value)
    {
    struct tps65912_gpio *gpio = gpiochip_get_data(gc);
    int ret;
// Set the initial value
    ret = regmap_update_bits(gpio.tps.regmap, TPS65912_GPIO1 + offset,
    GPIO_SET_MASK, value ? GPIO_SET_MASK : 0);
    if (ret)
    return ret;
    return regmap_update_bits(gpio.tps.regmap, TPS65912_GPIO1 + offset,
    GPIO_CFG_MASK, GPIO_CFG_MASK);
    }
#[no_mangle]
unsafe extern "C" fn tps65912_gpio_get(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int tps65912_gpio_get(struct gpio_chip *gc, unsigned offset)
    {
    struct tps65912_gpio *gpio = gpiochip_get_data(gc);
    int ret, val;
    ret = regmap_read(gpio.tps.regmap, TPS65912_GPIO1 + offset, &val);
    if (ret)
    return ret;
    if (val & GPIO_STS_MASK)
    return 1;
    return 0;
    }
    static int tps65912_gpio_set(struct gpio_chip *gc, unsigned int offset,
    int value)
    {
    struct tps65912_gpio *gpio = gpiochip_get_data(gc);
    return regmap_update_bits(gpio.tps.regmap, TPS65912_GPIO1 + offset,
    GPIO_SET_MASK, value ? GPIO_SET_MASK : 0);
    }
    static const struct gpio_chip template_chip = {
    .label			= "tps65912-gpio",
    .owner			= THIS_MODULE,
    .get_direction		= tps65912_gpio_get_direction,
    .direction_input	= tps65912_gpio_direction_input,
    .direction_output	= tps65912_gpio_direction_output,
    .get			= tps65912_gpio_get,
    .set			= tps65912_gpio_set,
    .base			= -1,
    .ngpio			= 5,
    .can_sleep		= true,
    };
#[no_mangle]
unsafe extern "C" fn tps65912_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int tps65912_gpio_probe(struct platform_device *pdev)
    {
    struct tps65912 *tps = dev_get_drvdata(pdev.dev.parent);
    struct tps65912_gpio *gpio;
    gpio = devm_kzalloc(&pdev.dev, sizeof(*gpio), GFP_KERNEL);
    if (!gpio)
    return -ENOMEM;
    gpio.tps = dev_get_drvdata(pdev.dev.parent);
    gpio.gpio_chip = template_chip;
    gpio.gpio_chip.parent = tps.dev;
    return devm_gpiochip_add_data(&pdev.dev, &gpio.gpio_chip, gpio);
    }
    static const struct platform_device_id tps65912_gpio_id_table[] = {
    { .name = "tps65912-gpio" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(platform, tps65912_gpio_id_table);
    static struct platform_driver tps65912_gpio_driver = {
    .driver = {
    .name = "tps65912-gpio",
    },
    .probe = tps65912_gpio_probe,
    .id_table = tps65912_gpio_id_table,
    };
    module_platform_driver(tps65912_gpio_driver);
    MODULE_AUTHOR("Andrew F. Davis <afd@ti.com>");
    MODULE_DESCRIPTION("TPS65912 GPIO driver");
    MODULE_LICENSE("GPL v2");
