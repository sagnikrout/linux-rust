//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/mc13xxx-i2c.c
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
// Copyright 2009-2010 Creative Product Design
// Marc Reilly marc@cpdesign.com.au
//

    static const struct i2c_device_id mc13xxx_i2c_device_id[] = {
    {
    .name = "mc13892",
    .driver_data = (kernel_ulong_t)&mc13xxx_variant_mc13892,
    }, {
    .name = "mc34708",
    .driver_data = (kernel_ulong_t)&mc13xxx_variant_mc34708,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(i2c, mc13xxx_i2c_device_id);
    static const struct of_device_id mc13xxx_dt_ids[] = {
    {
    .compatible = "fsl,mc13892",
    .data = &mc13xxx_variant_mc13892,
    }, {
    .compatible = "fsl,mc34708",
    .data = &mc13xxx_variant_mc34708,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, mc13xxx_dt_ids);
    static const struct regmap_config mc13xxx_regmap_i2c_config = {
    .reg_bits = 8,
    .val_bits = 24,
    .max_register = MC13XXX_NUMREGS,
    .cache_type = REGCACHE_NONE,
    };
#[no_mangle]
unsafe extern "C" fn mc13xxx_i2c_probe(client: *mut i2c_client) -> c_int {
    static int mc13xxx_i2c_probe(struct i2c_client *client)
    {
    struct mc13xxx *mc13xxx;
    int ret;
    mc13xxx = devm_kzalloc(&client.dev, sizeof(*mc13xxx), GFP_KERNEL);
    if (!mc13xxx)
    return -ENOMEM;
    dev_set_drvdata(&client.dev, mc13xxx);
    mc13xxx.irq = client.irq;
    mc13xxx.regmap = devm_regmap_init_i2c(client,
    &mc13xxx_regmap_i2c_config);
    if (IS_ERR(mc13xxx.regmap)) {
    ret = PTR_ERR(mc13xxx.regmap);
    dev_err(&client.dev, "Failed to initialize regmap: %d\n", ret);
    return ret;
    }
    mc13xxx.variant = i2c_get_match_data(client);
    return mc13xxx_common_init(&client.dev);
    }
#[no_mangle]
unsafe extern "C" fn mc13xxx_i2c_remove(client: *mut i2c_client) {
    static void mc13xxx_i2c_remove(struct i2c_client *client)
    {
    mc13xxx_common_exit(&client.dev);
    }
    static struct i2c_driver mc13xxx_i2c_driver = {
    .id_table = mc13xxx_i2c_device_id,
    .driver = {
    .name = "mc13xxx",
    .of_match_table = mc13xxx_dt_ids,
    },
    .probe = mc13xxx_i2c_probe,
    .remove = mc13xxx_i2c_remove,
    };
#[no_mangle]
unsafe extern "C" fn mc13xxx_i2c_init() -> int __init {
    static int __init mc13xxx_i2c_init(void)
    {
    return i2c_add_driver(&mc13xxx_i2c_driver);
    }
    subsys_initcall(mc13xxx_i2c_init);
#[no_mangle]
unsafe extern "C" fn mc13xxx_i2c_exit() -> void __exit {
    static void __exit mc13xxx_i2c_exit(void)
    {
    i2c_del_driver(&mc13xxx_i2c_driver);
    }
    module_exit(mc13xxx_i2c_exit);
    MODULE_DESCRIPTION("i2c driver for Freescale MC13XXX PMIC");
    MODULE_AUTHOR("Marc Reilly <marc@cpdesign.com.au");
    MODULE_LICENSE("GPL v2");
