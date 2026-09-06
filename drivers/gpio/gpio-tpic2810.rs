//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-tpic2810.c
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
// Copyright (C) 2015-2023 Texas Instruments Incorporated - https://www.ti.com
// Andrew Davis <afd@ti.com>
//

pub const TPIC2810_WS_COMMAND: c_uint = 0x44;
//
// struct tpic2810 - GPIO driver data
// @chip: GPIO controller chip
// @client: I2C device pointer
// @buffer: Buffer for device register
// @lock: Protects write sequences
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpic2810 {
    pub chip: gpio_chip,
    pub client: *mut i2c_client,
    pub buffer: u8,
    pub lock: mutex,
}

    static int tpic2810_set(struct gpio_chip *chip, unsigned int offset, int value);
    static int tpic2810_get_direction(struct gpio_chip *chip,
    unsigned offset)
    {
// This device always output
    return GPIO_LINE_DIRECTION_OUT;
    }
    static int tpic2810_direction_output(struct gpio_chip *chip,
    unsigned offset, int value)
    {
// This device always output
    return tpic2810_set(chip, offset, value);
    }
#[no_mangle]
unsafe extern "C" fn tpic2810_set_mask_bits(chip: *mut gpio_chip, mask: u8, bits: u8) {
    static void tpic2810_set_mask_bits(struct gpio_chip *chip, u8 mask, u8 bits)
    {
    struct tpic2810 *gpio = gpiochip_get_data(chip);
    u8 buffer;
    int err;
    mutex_lock(&gpio.lock);
    buffer = gpio.buffer & ~mask;
    buffer |= (mask & bits);
    err = i2c_smbus_write_byte_data(gpio.client, TPIC2810_WS_COMMAND,
    buffer);
    if (!err)
    gpio.buffer = buffer;
    mutex_unlock(&gpio.lock);
    }
#[no_mangle]
unsafe extern "C" fn tpic2810_set(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int tpic2810_set(struct gpio_chip *chip, unsigned int offset, int value)
    {
    tpic2810_set_mask_bits(chip, BIT(offset), value ? BIT(offset) : 0);
    return 0;
    }
    static int tpic2810_set_multiple(struct gpio_chip *chip, unsigned long *mask,
    unsigned long *bits)
    {
    tpic2810_set_mask_bits(chip, *mask, *bits);
    return 0;
    }
    static const struct gpio_chip template_chip = {
    .label			= "tpic2810",
    .owner			= THIS_MODULE,
    .get_direction		= tpic2810_get_direction,
    .direction_output	= tpic2810_direction_output,
    .set			= tpic2810_set,
    .set_multiple		= tpic2810_set_multiple,
    .base			= -1,
    .ngpio			= 8,
    .can_sleep		= true,
    };
    static const struct of_device_id tpic2810_of_match_table[] = {
    { .compatible = "ti,tpic2810" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, tpic2810_of_match_table);
#[no_mangle]
unsafe extern "C" fn tpic2810_probe(client: *mut i2c_client) -> c_int {
    static int tpic2810_probe(struct i2c_client *client)
    {
    struct tpic2810 *gpio;
    gpio = devm_kzalloc(&client.dev, sizeof(*gpio), GFP_KERNEL);
    if (!gpio)
    return -ENOMEM;
    gpio.chip = template_chip;
    gpio.chip.parent = &client.dev;
    gpio.client = client;
    mutex_init(&gpio.lock);
    return devm_gpiochip_add_data(&client.dev, &gpio.chip, gpio);
    }
    static const struct i2c_device_id tpic2810_id_table[] = {
    { .name = "tpic2810" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(i2c, tpic2810_id_table);
    static struct i2c_driver tpic2810_driver = {
    .driver = {
    .name = "tpic2810",
    .of_match_table = tpic2810_of_match_table,
    },
    .probe = tpic2810_probe,
    .id_table = tpic2810_id_table,
    };
    module_i2c_driver(tpic2810_driver);
    MODULE_AUTHOR("Andrew Davis <afd@ti.com>");
    MODULE_DESCRIPTION("TPIC2810 8-Bit LED Driver GPIO Driver");
    MODULE_LICENSE("GPL v2");
