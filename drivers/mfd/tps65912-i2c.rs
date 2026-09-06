//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/tps65912-i2c.c
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
// I2C access driver for TI TPS65912x PMICs
//
// Copyright (C) 2015 Texas Instruments Incorporated - https://www.ti.com
// Andrew F. Davis <afd@ti.com>
//
// Based on the TPS65218 driver and the previous TPS65912 driver by
// Margarita Olaya Cabrera <magi@slimlogic.co.uk>
//

    static const struct of_device_id tps65912_i2c_of_match_table[] = {
    { .compatible = "ti,tps65912", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, tps65912_i2c_of_match_table);
#[no_mangle]
unsafe extern "C" fn tps65912_i2c_probe(client: *mut i2c_client) -> c_int {
    static int tps65912_i2c_probe(struct i2c_client *client)
    {
    struct tps65912 *tps;
    tps = devm_kzalloc(&client.dev, sizeof(*tps), GFP_KERNEL);
    if (!tps)
    return -ENOMEM;
    i2c_set_clientdata(client, tps);
    tps.dev = &client.dev;
    tps.irq = client.irq;
    tps.regmap = devm_regmap_init_i2c(client, &tps65912_regmap_config);
    if (IS_ERR(tps.regmap)) {
    dev_err(tps.dev, "Failed to initialize register map\n");
    return PTR_ERR(tps.regmap);
    }
    return tps65912_device_init(tps);
    }
    static const struct i2c_device_id tps65912_i2c_id_table[] = {
    { "tps65912" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(i2c, tps65912_i2c_id_table);
    static struct i2c_driver tps65912_i2c_driver = {
    .driver		= {
    .name	= "tps65912",
    .of_match_table = tps65912_i2c_of_match_table,
    },
    .probe		= tps65912_i2c_probe,
    .id_table       = tps65912_i2c_id_table,
    };
    module_i2c_driver(tps65912_i2c_driver);
    MODULE_AUTHOR("Andrew F. Davis <afd@ti.com>");
    MODULE_DESCRIPTION("TPS65912x I2C Interface Driver");
    MODULE_LICENSE("GPL v2");
