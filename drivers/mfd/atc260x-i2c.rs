//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/atc260x-i2c.c
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
// I2C bus interface for ATC260x PMICs
//
// Copyright (C) 2019 Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>
// Copyright (C) 2020 Cristian Ciocaltea <cristian.ciocaltea@gmail.com>
//

#[no_mangle]
unsafe extern "C" fn atc260x_i2c_probe(client: *mut i2c_client) -> c_int {
    static int atc260x_i2c_probe(struct i2c_client *client)
    {
    struct atc260x *atc260x;
    struct regmap_config regmap_cfg;
    int ret;
    atc260x = devm_kzalloc(&client.dev, sizeof(*atc260x), GFP_KERNEL);
    if (!atc260x)
    return -ENOMEM;
    atc260x.dev = &client.dev;
    atc260x.irq = client.irq;
    ret = atc260x_match_device(atc260x, &regmap_cfg);
    if (ret)
    return ret;
    i2c_set_clientdata(client, atc260x);
    atc260x.regmap = devm_regmap_init_i2c(client, &regmap_cfg);
    if (IS_ERR(atc260x.regmap)) {
    ret = PTR_ERR(atc260x.regmap);
    dev_err(&client.dev, "failed to init regmap: %d\n", ret);
    return ret;
    }
    return atc260x_device_probe(atc260x);
    }
    static const struct of_device_id atc260x_i2c_of_match[] = {
    { .compatible = "actions,atc2603c", .data = (void *)ATC2603C },
    { .compatible = "actions,atc2609a", .data = (void *)ATC2609A },
    { }
    };
    MODULE_DEVICE_TABLE(of, atc260x_i2c_of_match);
    static struct i2c_driver atc260x_i2c_driver = {
    .driver = {
    .name = "atc260x",
    .of_match_table	= atc260x_i2c_of_match,
    },
    .probe = atc260x_i2c_probe,
    };
    module_i2c_driver(atc260x_i2c_driver);
    MODULE_DESCRIPTION("ATC260x PMICs I2C bus interface");
    MODULE_AUTHOR("Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>");
    MODULE_AUTHOR("Cristian Ciocaltea <cristian.ciocaltea@gmail.com>");
    MODULE_LICENSE("GPL");
