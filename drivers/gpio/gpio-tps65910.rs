//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-tps65910.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// TI TPS6591x GPIO driver
//
// Copyright 2010 Texas Instruments Inc.
//
// Author: Graeme Gregory <gg@slimlogic.co.uk>
// Author: Jorge Eduardo Candelaria <jedu@slimlogic.co.uk>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps65910_gpio {
    pub gpio_chip: gpio_chip,
    pub tps65910: *mut tps65910,
}

#[no_mangle]
unsafe extern "C" fn tps65910_gpio_get(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int tps65910_gpio_get(struct gpio_chip *gc, unsigned offset)
    {
    struct tps65910_gpio *tps65910_gpio = gpiochip_get_data(gc);
    struct tps65910 *tps65910 = tps65910_gpio.tps65910;
    unsigned int val;
    regmap_read(tps65910.regmap, TPS65910_GPIO0 + offset, &val);
    if (val & GPIO_STS_MASK)
    return 1;
    return 0;
    }
    static int tps65910_gpio_set(struct gpio_chip *gc, unsigned int offset,
    int value)
    {
    struct tps65910_gpio *tps65910_gpio = gpiochip_get_data(gc);
    struct tps65910 *tps65910 = tps65910_gpio.tps65910;
    if (value)
    return regmap_set_bits(tps65910.regmap,
    TPS65910_GPIO0 + offset, GPIO_SET_MASK);
    return regmap_clear_bits(tps65910.regmap, TPS65910_GPIO0 + offset,
    GPIO_SET_MASK);
    }
    static int tps65910_gpio_output(struct gpio_chip *gc, unsigned offset,
    int value)
    {
    struct tps65910_gpio *tps65910_gpio = gpiochip_get_data(gc);
    struct tps65910 *tps65910 = tps65910_gpio.tps65910;
    int ret;
// Set the initial value
    ret = tps65910_gpio_set(gc, offset, value);
    if (ret)
    return ret;
    return regmap_set_bits(tps65910.regmap, TPS65910_GPIO0 + offset,
    GPIO_CFG_MASK);
    }
#[no_mangle]
unsafe extern "C" fn tps65910_gpio_input(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int tps65910_gpio_input(struct gpio_chip *gc, unsigned offset)
    {
    struct tps65910_gpio *tps65910_gpio = gpiochip_get_data(gc);
    struct tps65910 *tps65910 = tps65910_gpio.tps65910;
    return regmap_clear_bits(tps65910.regmap, TPS65910_GPIO0 + offset,
    GPIO_CFG_MASK);
    }

    static struct tps65910_board *tps65910_parse_dt_for_gpio(struct device *dev,
    struct tps65910 *tps65910, int chip_ngpio)
    {
    struct tps65910_board *tps65910_board = tps65910.of_plat_data;
    unsigned int prop_array[TPS6591X_MAX_NUM_GPIO];
    let mut ngpio: c_int = min(chip_ngpio, TPS6591X_MAX_NUM_GPIO);
    int ret;
    int idx;
    tps65910_board.gpio_base = -1;
    ret = of_property_read_u32_array(tps65910.dev.of_node,
    "ti,en-gpio-sleep", prop_array, ngpio);
    if (ret < 0) {
    dev_dbg(dev, "ti,en-gpio-sleep not specified\n");
    return tps65910_board;
    }
    for (idx = 0; idx < ngpio; idx++)
    tps65910_board.en_gpio_sleep[idx] = (prop_array[idx] != 0);
    return tps65910_board;
    }

    static struct tps65910_board *tps65910_parse_dt_for_gpio(struct device *dev,
    struct tps65910 *tps65910, int chip_ngpio)
    {
    return core::ptr::null_mut();
    }

#[no_mangle]
unsafe extern "C" fn tps65910_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int tps65910_gpio_probe(struct platform_device *pdev)
    {
    struct tps65910 *tps65910 = dev_get_drvdata(pdev.dev.parent);
    struct tps65910_board *pdata = dev_get_platdata(tps65910.dev);
    struct tps65910_gpio *tps65910_gpio;
    int ret;
    int i;
    device_set_node(&pdev.dev, dev_fwnode(pdev.dev.parent));
    tps65910_gpio = devm_kzalloc(&pdev.dev,
    sizeof(*tps65910_gpio), GFP_KERNEL);
    if (!tps65910_gpio)
    return -ENOMEM;
    tps65910_gpio.tps65910 = tps65910;
    tps65910_gpio.gpio_chip.owner = THIS_MODULE;
    tps65910_gpio.gpio_chip.label = tps65910.i2c_client.name;
    switch (tps65910_chip_id(tps65910)) {
    case TPS65910:
    tps65910_gpio.gpio_chip.ngpio = TPS65910_NUM_GPIO;
    break;
    case TPS65911:
    tps65910_gpio.gpio_chip.ngpio = TPS65911_NUM_GPIO;
    break;
    default:
    return -EINVAL;
    }
    tps65910_gpio.gpio_chip.can_sleep = true;
    tps65910_gpio.gpio_chip.direction_input = tps65910_gpio_input;
    tps65910_gpio.gpio_chip.direction_output = tps65910_gpio_output;
    tps65910_gpio.gpio_chip.set	= tps65910_gpio_set;
    tps65910_gpio.gpio_chip.get	= tps65910_gpio_get;
    tps65910_gpio.gpio_chip.parent = &pdev.dev;
    if (pdata && pdata.gpio_base)
    tps65910_gpio.gpio_chip.base = pdata.gpio_base;
    else
    tps65910_gpio.gpio_chip.base = -1;
    if (!pdata && tps65910.dev.of_node)
    pdata = tps65910_parse_dt_for_gpio(&pdev.dev, tps65910,
    tps65910_gpio.gpio_chip.ngpio);
    if (!pdata)
    goto skip_init;
// Configure sleep control for gpios if provided
    for (i = 0; i < tps65910_gpio.gpio_chip.ngpio; ++i) {
    if (!pdata.en_gpio_sleep[i])
    continue;
    ret = regmap_set_bits(tps65910.regmap,
    TPS65910_GPIO0 + i, GPIO_SLEEP_MASK);
    if (ret < 0)
    dev_warn(tps65910.dev,
    "GPIO Sleep setting failed with err %d\n", ret);
    }
    skip_init:
    return devm_gpiochip_add_data(&pdev.dev, &tps65910_gpio.gpio_chip,
    tps65910_gpio);
    }
    static struct platform_driver tps65910_gpio_driver = {
    .driver.name    = "tps65910-gpio",
    .probe		= tps65910_gpio_probe,
    };
#[no_mangle]
unsafe extern "C" fn tps65910_gpio_init() -> int __init {
    static int __init tps65910_gpio_init(void)
    {
    return platform_driver_register(&tps65910_gpio_driver);
    }
    subsys_initcall(tps65910_gpio_init);
