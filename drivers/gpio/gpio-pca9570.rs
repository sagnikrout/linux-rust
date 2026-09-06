//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-pca9570.c
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
// Driver for PCA9570 I2C GPO expander
//
// Copyright (C) 2020 Sungbo Eo <mans0n@gorani.run>
//
// Based on gpio-tpic2810.c
// Copyright (C) 2015 Texas Instruments Incorporated - http://www.ti.com
// Andrew F. Davis <afd@ti.com>
//

pub const SLG7XL45106_GPO_REG: c_uint = 0xDB;
//
// struct pca9570_chip_data - GPIO platformdata
// @ngpio: no of gpios
// @command: Command to be sent
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pca9570_chip_data {
    pub ngpio: u16,
    pub command: u32,
}

//
// struct pca9570 - GPIO driver data
// @chip: GPIO controller chip
// @chip_data: GPIO controller platform data
// @lock: Protects write sequences
// @out: Buffer for device register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pca9570 {
    pub chip: gpio_chip,
    pub chip_data: *const pca9570_chip_data,
    pub lock: mutex,
    pub out: u8,
}

#[no_mangle]
unsafe extern "C" fn pca9570_read(gpio: *mut pca9570, value: *mut u8) -> c_int {
    static int pca9570_read(struct pca9570 *gpio, u8 *value)
    {
    struct i2c_client *client = to_i2c_client(gpio.chip.parent);
    int ret;
    if (gpio.chip_data.command != 0)
    ret = i2c_smbus_read_byte_data(client, gpio.chip_data.command);
    else
    ret = i2c_smbus_read_byte(client);
    if (ret < 0)
    return ret;
// value = ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pca9570_write(gpio: *mut pca9570, value: u8) -> c_int {
    static int pca9570_write(struct pca9570 *gpio, u8 value)
    {
    struct i2c_client *client = to_i2c_client(gpio.chip.parent);
    if (gpio.chip_data.command != 0)
    return i2c_smbus_write_byte_data(client, gpio.chip_data.command, value);
    return i2c_smbus_write_byte(client, value);
    }
    static int pca9570_get_direction(struct gpio_chip *chip,
    unsigned offset)
    {
// This device always output
    return GPIO_LINE_DIRECTION_OUT;
    }
#[no_mangle]
unsafe extern "C" fn pca9570_get(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int pca9570_get(struct gpio_chip *chip, unsigned offset)
    {
    struct pca9570 *gpio = gpiochip_get_data(chip);
    u8 buffer;
    int ret;
    ret = pca9570_read(gpio, &buffer);
    if (ret)
    return ret;
    return !!(buffer & BIT(offset));
    }
#[no_mangle]
unsafe extern "C" fn pca9570_set(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int pca9570_set(struct gpio_chip *chip, unsigned int offset, int value)
    {
    struct pca9570 *gpio = gpiochip_get_data(chip);
    u8 buffer;
    int ret;
    guard(mutex)(&gpio.lock);
    buffer = gpio.out;
    if (value)
    buffer |= BIT(offset);
    else
    buffer &= ~BIT(offset);
    ret = pca9570_write(gpio, buffer);
    if (ret)
    return ret;
    gpio.out = buffer;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pca9570_probe(client: *mut i2c_client) -> c_int {
    static int pca9570_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct pca9570 *gpio;
    int ret;
    gpio = devm_kzalloc(&client.dev, sizeof(*gpio), GFP_KERNEL);
    if (!gpio)
    return -ENOMEM;
    gpio.chip.label = client.name;
    gpio.chip.parent = &client.dev;
    gpio.chip.owner = THIS_MODULE;
    gpio.chip.get_direction = pca9570_get_direction;
    gpio.chip.get = pca9570_get;
    gpio.chip.set = pca9570_set;
    gpio.chip.base = -1;
    gpio.chip_data = i2c_get_match_data(client);
    gpio.chip.ngpio = gpio.chip_data.ngpio;
    gpio.chip.can_sleep = true;
    ret = devm_mutex_init(dev, &gpio.lock);
    if (ret)
    return ret;
// Read the current output level
    pca9570_read(gpio, &gpio.out);
    i2c_set_clientdata(client, gpio);
    return devm_gpiochip_add_data(&client.dev, &gpio.chip, gpio);
    }
    static const struct pca9570_chip_data pca9570_gpio = {
    .ngpio = 4,
    };
    static const struct pca9570_chip_data pca9571_gpio = {
    .ngpio = 8,
    };
    static const struct pca9570_chip_data slg7xl45106_gpio = {
    .ngpio = 8,
    .command = SLG7XL45106_GPO_REG,
    };
    static const struct i2c_device_id pca9570_id_table[] = {
    { .name = "pca9570", .driver_data = (kernel_ulong_t)&pca9570_gpio },
    { .name = "pca9571", .driver_data = (kernel_ulong_t)&pca9571_gpio },
    { .name = "slg7xl45106", .driver_data = (kernel_ulong_t)&slg7xl45106_gpio },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(i2c, pca9570_id_table);
    static const struct of_device_id pca9570_of_match_table[] = {
    { .compatible = "dlg,slg7xl45106", .data = &slg7xl45106_gpio},
    { .compatible = "nxp,pca9570", .data = &pca9570_gpio },
    { .compatible = "nxp,pca9571", .data = &pca9571_gpio },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, pca9570_of_match_table);
    static struct i2c_driver pca9570_driver = {
    .driver = {
    .name = "pca9570",
    .of_match_table = pca9570_of_match_table,
    },
    .probe = pca9570_probe,
    .id_table = pca9570_id_table,
    };
    module_i2c_driver(pca9570_driver);
    MODULE_AUTHOR("Sungbo Eo <mans0n@gorani.run>");
    MODULE_DESCRIPTION("GPIO expander driver for PCA9570");
    MODULE_LICENSE("GPL v2");
