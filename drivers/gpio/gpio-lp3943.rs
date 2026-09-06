//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-lp3943.c
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
// TI/National Semiconductor LP3943 GPIO driver
//
// Copyright 2013 Texas Instruments
//
// Author: Milo Kim <milo.kim@ti.com>
//

    enum lp3943_gpios {
    LP3943_GPIO1,
    LP3943_GPIO2,
    LP3943_GPIO3,
    LP3943_GPIO4,
    LP3943_GPIO5,
    LP3943_GPIO6,
    LP3943_GPIO7,
    LP3943_GPIO8,
    LP3943_GPIO9,
    LP3943_GPIO10,
    LP3943_GPIO11,
    LP3943_GPIO12,
    LP3943_GPIO13,
    LP3943_GPIO14,
    LP3943_GPIO15,
    LP3943_GPIO16,
    LP3943_MAX_GPIO,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp3943_gpio {
    pub chip: gpio_chip,
    pub lp3943: *mut lp3943,
    pub /: *mut *mut u16 input_mask; / 1 = GPIO is input direction, 0 = output,
}

#[no_mangle]
unsafe extern "C" fn lp3943_gpio_request(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int lp3943_gpio_request(struct gpio_chip *chip, unsigned int offset)
    {
    struct lp3943_gpio *lp3943_gpio = gpiochip_get_data(chip);
    struct lp3943 *lp3943 = lp3943_gpio.lp3943;
// Return an error if the pin is already assigned
    if (test_and_set_bit(offset, &lp3943.pin_used))
    return -EBUSY;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lp3943_gpio_free(chip: *mut gpio_chip, offset: c_uint) {
    static void lp3943_gpio_free(struct gpio_chip *chip, unsigned int offset)
    {
    struct lp3943_gpio *lp3943_gpio = gpiochip_get_data(chip);
    struct lp3943 *lp3943 = lp3943_gpio.lp3943;
    clear_bit(offset, &lp3943.pin_used);
    }
    static int lp3943_gpio_set_mode(struct lp3943_gpio *lp3943_gpio, u8 offset,
    u8 val)
    {
    struct lp3943 *lp3943 = lp3943_gpio.lp3943;
    const struct lp3943_reg_cfg *mux = lp3943.mux_cfg;
    return lp3943_update_bits(lp3943, mux[offset].reg, mux[offset].mask,
    val << mux[offset].shift);
    }
#[no_mangle]
unsafe extern "C" fn lp3943_gpio_direction_input(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int lp3943_gpio_direction_input(struct gpio_chip *chip, unsigned int offset)
    {
    struct lp3943_gpio *lp3943_gpio = gpiochip_get_data(chip);
    lp3943_gpio.input_mask |= BIT(offset);
    return lp3943_gpio_set_mode(lp3943_gpio, offset, LP3943_GPIO_IN);
    }
    static int lp3943_get_gpio_in_status(struct lp3943_gpio *lp3943_gpio,
    struct gpio_chip *chip, unsigned int offset)
    {
    u8 addr, read;
    int err;
    switch (offset) {
    case LP3943_GPIO1 ... LP3943_GPIO8:
    addr = LP3943_REG_GPIO_A;
    break;
    case LP3943_GPIO9 ... LP3943_GPIO16:
    addr = LP3943_REG_GPIO_B;
    offset = offset - 8;
    break;
    default:
    return -EINVAL;
    }
    err = lp3943_read_byte(lp3943_gpio.lp3943, addr, &read);
    if (err)
    return err;
    return !!(read & BIT(offset));
    }
    static int lp3943_get_gpio_out_status(struct lp3943_gpio *lp3943_gpio,
    struct gpio_chip *chip, unsigned int offset)
    {
    struct lp3943 *lp3943 = lp3943_gpio.lp3943;
    const struct lp3943_reg_cfg *mux = lp3943.mux_cfg;
    u8 read;
    int err;
    err = lp3943_read_byte(lp3943, mux[offset].reg, &read);
    if (err)
    return err;
    read = (read & mux[offset].mask) >> mux[offset].shift;
    if (read == LP3943_GPIO_OUT_HIGH)
    return 1;
#[no_mangle]
pub unsafe extern "C" fn if(LP3943_GPIO_OUT_LOW: read ==) -> else {
    else if (read == LP3943_GPIO_OUT_LOW)
    return 0;
    else
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn lp3943_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int lp3943_gpio_get(struct gpio_chip *chip, unsigned int offset)
    {
    struct lp3943_gpio *lp3943_gpio = gpiochip_get_data(chip);
//
// Limitation:
// LP3943 doesn't have the GPIO direction register. It provides
// only input and output status registers.
// So, direction info is required to handle the 'get' operation.
// This variable is updated whenever the direction is changed and
// it is used here.
//
    if (lp3943_gpio.input_mask & BIT(offset))
    return lp3943_get_gpio_in_status(lp3943_gpio, chip, offset);
    else
    return lp3943_get_gpio_out_status(lp3943_gpio, chip, offset);
    }
    static int lp3943_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct lp3943_gpio *lp3943_gpio = gpiochip_get_data(chip);
    u8 data;
    if (value)
    data = LP3943_GPIO_OUT_HIGH;
    else
    data = LP3943_GPIO_OUT_LOW;
    return lp3943_gpio_set_mode(lp3943_gpio, offset, data);
    }
    static int lp3943_gpio_direction_output(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct lp3943_gpio *lp3943_gpio = gpiochip_get_data(chip);
    int ret;
    ret = lp3943_gpio_set(chip, offset, value);
    if (ret)
    return ret;
    lp3943_gpio.input_mask &= ~BIT(offset);
    return 0;
    }
    static const struct gpio_chip lp3943_gpio_chip = {
    .label			= "lp3943",
    .owner			= THIS_MODULE,
    .request		= lp3943_gpio_request,
    .free			= lp3943_gpio_free,
    .direction_input	= lp3943_gpio_direction_input,
    .get			= lp3943_gpio_get,
    .direction_output	= lp3943_gpio_direction_output,
    .set			= lp3943_gpio_set,
    .base			= -1,
    .ngpio			= LP3943_MAX_GPIO,
    .can_sleep		= 1,
    };
#[no_mangle]
unsafe extern "C" fn lp3943_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int lp3943_gpio_probe(struct platform_device *pdev)
    {
    struct lp3943 *lp3943 = dev_get_drvdata(pdev.dev.parent);
    struct lp3943_gpio *lp3943_gpio;
    lp3943_gpio = devm_kzalloc(&pdev.dev, sizeof(*lp3943_gpio),
    GFP_KERNEL);
    if (!lp3943_gpio)
    return -ENOMEM;
    lp3943_gpio.lp3943 = lp3943;
    lp3943_gpio.chip = lp3943_gpio_chip;
    lp3943_gpio.chip.parent = &pdev.dev;
    return devm_gpiochip_add_data(&pdev.dev, &lp3943_gpio.chip,
    lp3943_gpio);
    }
    static const struct of_device_id lp3943_gpio_of_match[] = {
    { .compatible = "ti,lp3943-gpio", },
    { }
    };
    MODULE_DEVICE_TABLE(of, lp3943_gpio_of_match);
    static struct platform_driver lp3943_gpio_driver = {
    .probe = lp3943_gpio_probe,
    .driver = {
    .name = "lp3943-gpio",
    .of_match_table = lp3943_gpio_of_match,
    },
    };
    module_platform_driver(lp3943_gpio_driver);
    MODULE_DESCRIPTION("LP3943 GPIO driver");
    MODULE_ALIAS("platform:lp3943-gpio");
    MODULE_AUTHOR("Milo Kim");
    MODULE_LICENSE("GPL");
