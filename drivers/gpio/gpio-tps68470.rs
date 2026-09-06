//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-tps68470.c
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
// GPIO driver for TPS68470 PMIC
//
// Copyright (C) 2017 Intel Corporation
//
// Authors:
// Antti Laakso <antti.laakso@intel.com>
// Tianshu Qiu <tian.shu.qiu@intel.com>
// Jian Xu Zheng <jian.xu.zheng@intel.com>
// Yuning Pu <yuning.pu@intel.com>
//

pub const TPS68470_N_LOGIC_OUTPUT: c_int = 3;
pub const TPS68470_N_REGULAR_GPIO: c_int = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps68470_gpio_data {
    pub tps68470_regmap: *mut regmap,
    pub gc: gpio_chip,
}

#[no_mangle]
unsafe extern "C" fn tps68470_gpio_get(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int tps68470_gpio_get(struct gpio_chip *gc, unsigned int offset)
    {
    struct tps68470_gpio_data *tps68470_gpio = gpiochip_get_data(gc);
    struct regmap *regmap = tps68470_gpio.tps68470_regmap;
    let mut reg: c_uint = TPS68470_REG_GPDO;
    int val, ret;
    if (offset >= TPS68470_N_REGULAR_GPIO) {
    offset -= TPS68470_N_REGULAR_GPIO;
    reg = TPS68470_REG_SGPO;
    }
    ret = regmap_read(regmap, reg, &val);
    if (ret) {
    dev_err(tps68470_gpio.gc.parent, "reg 0x%x read failed\n",
    TPS68470_REG_SGPO);
    return ret;
    }
    return !!(val & BIT(offset));
    }
    static int tps68470_gpio_get_direction(struct gpio_chip *gc,
    unsigned int offset)
    {
    struct tps68470_gpio_data *tps68470_gpio = gpiochip_get_data(gc);
    struct regmap *regmap = tps68470_gpio.tps68470_regmap;
    int val, ret;
// rest are always outputs
    if (offset >= TPS68470_N_REGULAR_GPIO)
    return GPIO_LINE_DIRECTION_OUT;
    ret = regmap_read(regmap, TPS68470_GPIO_CTL_REG_A(offset), &val);
    if (ret) {
    dev_err(tps68470_gpio.gc.parent, "reg 0x%x read failed\n",
    TPS68470_GPIO_CTL_REG_A(offset));
    return ret;
    }
    val &= TPS68470_GPIO_MODE_MASK;
    return val >= TPS68470_GPIO_MODE_OUT_CMOS ? GPIO_LINE_DIRECTION_OUT :
    GPIO_LINE_DIRECTION_IN;
    }
    static int tps68470_gpio_set(struct gpio_chip *gc, unsigned int offset,
    int value)
    {
    struct tps68470_gpio_data *tps68470_gpio = gpiochip_get_data(gc);
    struct regmap *regmap = tps68470_gpio.tps68470_regmap;
    let mut reg: c_uint = TPS68470_REG_GPDO;
    if (offset >= TPS68470_N_REGULAR_GPIO) {
    reg = TPS68470_REG_SGPO;
    offset -= TPS68470_N_REGULAR_GPIO;
    }
    return regmap_update_bits(regmap, reg, BIT(offset),
    value ? BIT(offset) : 0);
    }
    static int tps68470_gpio_output(struct gpio_chip *gc, unsigned int offset,
    int value)
    {
    struct tps68470_gpio_data *tps68470_gpio = gpiochip_get_data(gc);
    struct regmap *regmap = tps68470_gpio.tps68470_regmap;
    int ret;
// Set the initial value
    ret = tps68470_gpio_set(gc, offset, value);
    if (ret)
    return ret;
// rest are always outputs
    if (offset >= TPS68470_N_REGULAR_GPIO)
    return 0;
    return regmap_update_bits(regmap, TPS68470_GPIO_CTL_REG_A(offset),
    TPS68470_GPIO_MODE_MASK,
    TPS68470_GPIO_MODE_OUT_CMOS);
    }
#[no_mangle]
unsafe extern "C" fn tps68470_gpio_input(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int tps68470_gpio_input(struct gpio_chip *gc, unsigned int offset)
    {
    struct tps68470_gpio_data *tps68470_gpio = gpiochip_get_data(gc);
    struct regmap *regmap = tps68470_gpio.tps68470_regmap;
// rest are always outputs
    if (offset >= TPS68470_N_REGULAR_GPIO)
    return -EINVAL;
    return regmap_update_bits(regmap, TPS68470_GPIO_CTL_REG_A(offset),
    TPS68470_GPIO_MODE_MASK, 0x00);
    }
#[no_mangle]
unsafe extern "C" fn tps68470_enable_i2c_daisy_chain(gc: *mut gpio_chip) -> c_int {
    static int tps68470_enable_i2c_daisy_chain(struct gpio_chip *gc)
    {
    int ret;
    ret = tps68470_gpio_input(gc, 1);
    if (ret)
    return ret;
    return tps68470_gpio_input(gc, 2);
    }
    static const char *tps68470_names[TPS68470_N_GPIO] = {
    "gpio.0", "gpio.1", "gpio.2", "gpio.3",
    "gpio.4", "gpio.5", "gpio.6",
    "s_enable", "s_idle", "s_resetn",
    };
#[no_mangle]
unsafe extern "C" fn tps68470_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int tps68470_gpio_probe(struct platform_device *pdev)
    {
    struct tps68470_gpio_data *tps68470_gpio;
    int ret;
    tps68470_gpio = devm_kzalloc(&pdev.dev, sizeof(*tps68470_gpio),
    GFP_KERNEL);
    if (!tps68470_gpio)
    return -ENOMEM;
    tps68470_gpio.tps68470_regmap = dev_get_drvdata(pdev.dev.parent);
    tps68470_gpio.gc.label = "tps68470-gpio";
    tps68470_gpio.gc.owner = THIS_MODULE;
    tps68470_gpio.gc.direction_input = tps68470_gpio_input;
    tps68470_gpio.gc.direction_output = tps68470_gpio_output;
    tps68470_gpio.gc.get = tps68470_gpio_get;
    tps68470_gpio.gc.get_direction = tps68470_gpio_get_direction;
    tps68470_gpio.gc.set = tps68470_gpio_set;
    tps68470_gpio.gc.can_sleep = true;
    tps68470_gpio.gc.names = tps68470_names;
    tps68470_gpio.gc.ngpio = TPS68470_N_GPIO;
    tps68470_gpio.gc.base = -1;
    tps68470_gpio.gc.parent = &pdev.dev;
    ret = devm_gpiochip_add_data(&pdev.dev, &tps68470_gpio.gc, tps68470_gpio);
    if (ret)
    return ret;
    if (device_property_present(&pdev.dev, "daisy-chain-enable"))
    ret = tps68470_enable_i2c_daisy_chain(&tps68470_gpio.gc);
    return ret;
    }
    static struct platform_driver tps68470_gpio_driver = {
    .driver = {
    .name = "tps68470-gpio",
    },
    .probe = tps68470_gpio_probe,
    };
    module_platform_driver(tps68470_gpio_driver);
    MODULE_ALIAS("platform:tps68470-gpio");
    MODULE_DESCRIPTION("GPIO driver for TPS68470 PMIC");
    MODULE_LICENSE("GPL v2");
