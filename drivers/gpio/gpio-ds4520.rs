//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-ds4520.c
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
// Copyright (c) 2023 Analog Devices, Inc.
// Driver for the DS4520 I/O Expander
//

pub const DS4520_PULLUP0: c_uint = 0xF0;
pub const DS4520_IO_CONTROL0: c_uint = 0xF2;
pub const DS4520_IO_STATUS0: c_uint = 0xF8;
    static const struct regmap_config ds4520_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    };
#[no_mangle]
unsafe extern "C" fn ds4520_gpio_probe(client: *mut i2c_client) -> c_int {
    static int ds4520_gpio_probe(struct i2c_client *client)
    {
    let mut config: gpio_regmap_config = { };
    struct device *dev = &client.dev;
    struct regmap *regmap;
    u32 base;
    int ret;
    ret = device_property_read_u32(dev, "reg", &base);
    if (ret)
    return dev_err_probe(dev, ret, "Missing 'reg' property.\n");
    regmap = devm_regmap_init_i2c(client, &ds4520_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(dev, PTR_ERR(regmap),
    "Failed to allocate register map\n");
    config.regmap = regmap;
    config.parent = dev;
    config.reg_dat_base = base + DS4520_IO_STATUS0;
    config.reg_set_base = base + DS4520_PULLUP0;
    config.reg_dir_out_base = base + DS4520_IO_CONTROL0;
    return PTR_ERR_OR_ZERO(devm_gpio_regmap_register(dev, &config));
    }
    static const struct of_device_id ds4520_gpio_of_match_table[] = {
    { .compatible = "adi,ds4520-gpio" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ds4520_gpio_of_match_table);
    static const struct i2c_device_id ds4520_gpio_id_table[] = {
    { .name = "ds4520-gpio" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ds4520_gpio_id_table);
    static struct i2c_driver ds4520_gpio_driver = {
    .driver = {
    .name = "ds4520-gpio",
    .of_match_table = ds4520_gpio_of_match_table,
    },
    .probe = ds4520_gpio_probe,
    .id_table = ds4520_gpio_id_table,
    };
    module_i2c_driver(ds4520_gpio_driver);
    MODULE_DESCRIPTION("DS4520 I/O Expander");
    MODULE_AUTHOR("Okan Sahin <okan.sahin@analog.com>");
    MODULE_LICENSE("GPL");
