//! Automatically rewritten from C to Rust
//! Source: drivers/net/dsa/microchip/ksz9477_i2c.c
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
// Microchip KSZ9477 series register access through I2C
//
// Copyright (C) 2018-2024 Microchip Technology Inc.
//

    KSZ_REGMAP_TABLE(ksz9477, not_used, 16, 0, 0);
#[no_mangle]
unsafe extern "C" fn ksz9477_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int ksz9477_i2c_probe(struct i2c_client *i2c)
    {
    const struct ksz_chip_data *chip;
    struct device *ddev = &i2c.dev;
    struct regmap_config rc;
    struct ksz_device *dev;
    int i, ret;
    chip = device_get_match_data(ddev);
    if (!chip)
    return -EINVAL;
    dev = ksz_switch_alloc(&i2c.dev, chip, i2c);
    if (!dev)
    return -ENOMEM;
// Save chip id to do special initialization when probing.
    dev.chip_id = chip.chip_id;
    for (i = 0; i < __KSZ_NUM_REGMAPS; i++) {
    rc = ksz9477_regmap_config[i];
    rc.lock_arg = &dev.regmap_mutex;
    dev.regmap[i] = devm_regmap_init_i2c(i2c, &rc);
    if (IS_ERR(dev.regmap[i])) {
    return dev_err_probe(&i2c.dev, PTR_ERR(dev.regmap[i]),
    "Failed to initialize regmap%i\n",
    ksz9477_regmap_config[i].val_bits);
    }
    }
    if (i2c.dev.platform_data)
    dev.pdata = i2c.dev.platform_data;
    dev.irq = i2c.irq;
    ret = ksz_switch_register(dev);
// Main DSA driver may not be started yet.
    if (ret)
    return ret;
    i2c_set_clientdata(i2c, dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ksz9477_i2c_remove(i2c: *mut i2c_client) {
    static void ksz9477_i2c_remove(struct i2c_client *i2c)
    {
    struct ksz_device *dev = i2c_get_clientdata(i2c);
    if (dev)
    ksz_switch_remove(dev);
    }
#[no_mangle]
unsafe extern "C" fn ksz9477_i2c_shutdown(i2c: *mut i2c_client) {
    static void ksz9477_i2c_shutdown(struct i2c_client *i2c)
    {
    struct ksz_device *dev = i2c_get_clientdata(i2c);
    if (!dev)
    return;
    ksz_switch_shutdown(dev);
    i2c_set_clientdata(i2c, core::ptr::null_mut());
    }
    static const struct i2c_device_id ksz9477_i2c_id[] = {
    { .name = "ksz9477-switch" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ksz9477_i2c_id);
    static const struct of_device_id ksz9477_dt_ids[] = {
    {
    .compatible = "microchip,ksz9477",
    .data = &ksz_switch_chips[KSZ9477]
    },
    {
    .compatible = "microchip,ksz9896",
    .data = &ksz_switch_chips[KSZ9896]
    },
    {
    .compatible = "microchip,ksz9897",
    .data = &ksz_switch_chips[KSZ9897]
    },
    {
    .compatible = "microchip,ksz9893",
    .data = &ksz_switch_chips[KSZ9893]
    },
    {
    .compatible = "microchip,ksz9563",
    .data = &ksz_switch_chips[KSZ9563]
    },
    {
    .compatible = "microchip,ksz8563",
    .data = &ksz_switch_chips[KSZ8563]
    },
    {
    .compatible = "microchip,ksz8567",
    .data = &ksz_switch_chips[KSZ8567]
    },
    {
    .compatible = "microchip,ksz9567",
    .data = &ksz_switch_chips[KSZ9567]
    },
    {
    .compatible = "microchip,lan9646",
    .data = &ksz_switch_chips[LAN9646]
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, ksz9477_dt_ids);
    static DEFINE_SIMPLE_DEV_PM_OPS(ksz_i2c_pm_ops,
    ksz_switch_suspend, ksz_switch_resume);
    static struct i2c_driver ksz9477_i2c_driver = {
    .driver = {
    .name	= "ksz9477-switch",
    .of_match_table = ksz9477_dt_ids,
    .pm = &ksz_i2c_pm_ops,
    },
    .probe = ksz9477_i2c_probe,
    .remove	= ksz9477_i2c_remove,
    .shutdown = ksz9477_i2c_shutdown,
    .id_table = ksz9477_i2c_id,
    };
    module_i2c_driver(ksz9477_i2c_driver);
    MODULE_AUTHOR("Tristram Ha <Tristram.Ha@microchip.com>");
    MODULE_DESCRIPTION("Microchip KSZ9477 Series Switch I2C access Driver");
    MODULE_LICENSE("GPL v2");
