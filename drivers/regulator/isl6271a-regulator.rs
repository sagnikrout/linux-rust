//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/isl6271a-regulator.c
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
// isl6271a-regulator.c
//
// Support for Intersil ISL6271A voltage regulator
//
// Copyright (C) 2010 Marek Vasut <marek.vasut@gmail.com>
//

pub const ISL6271A_VOLTAGE_MIN: c_int = 850000;
pub const ISL6271A_VOLTAGE_MAX: c_int = 1600000;
pub const ISL6271A_VOLTAGE_STEP: c_int = 50000;
// PMIC details
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isl_pmic {
    pub client: *mut i2c_client,
    pub mtx: mutex,
}

#[no_mangle]
unsafe extern "C" fn isl6271a_get_voltage_sel(dev: *mut regulator_dev) -> c_int {
    static int isl6271a_get_voltage_sel(struct regulator_dev *dev)
    {
    struct isl_pmic *pmic = rdev_get_drvdata(dev);
    int idx;
    mutex_lock(&pmic.mtx);
    idx = i2c_smbus_read_byte(pmic.client);
    if (idx < 0)
    dev_err(&pmic.client.dev, "Error getting voltage\n");
    mutex_unlock(&pmic.mtx);
    return idx;
    }
    static int isl6271a_set_voltage_sel(struct regulator_dev *dev,
    unsigned selector)
    {
    struct isl_pmic *pmic = rdev_get_drvdata(dev);
    int err;
    mutex_lock(&pmic.mtx);
    err = i2c_smbus_write_byte(pmic.client, selector);
    if (err < 0)
    dev_err(&pmic.client.dev, "Error setting voltage\n");
    mutex_unlock(&pmic.mtx);
    return err;
    }
    static const struct regulator_ops isl_core_ops = {
    .get_voltage_sel = isl6271a_get_voltage_sel,
    .set_voltage_sel = isl6271a_set_voltage_sel,
    .list_voltage	= regulator_list_voltage_linear,
    .map_voltage	= regulator_map_voltage_linear,
    };
    static const struct regulator_ops isl_fixed_ops = {
    .list_voltage	= regulator_list_voltage_linear,
    };
    static const struct regulator_desc isl_rd[] = {
    {
    .name		= "Core Buck",
    .id		= 0,
    .n_voltages	= 16,
    .ops		= &isl_core_ops,
    .type		= REGULATOR_VOLTAGE,
    .owner		= THIS_MODULE,
    .min_uV		= ISL6271A_VOLTAGE_MIN,
    .uV_step	= ISL6271A_VOLTAGE_STEP,
    }, {
    .name		= "LDO1",
    .id		= 1,
    .n_voltages	= 1,
    .ops		= &isl_fixed_ops,
    .type		= REGULATOR_VOLTAGE,
    .owner		= THIS_MODULE,
    .min_uV		= 1100000,
    }, {
    .name		= "LDO2",
    .id		= 2,
    .n_voltages	= 1,
    .ops		= &isl_fixed_ops,
    .type		= REGULATOR_VOLTAGE,
    .owner		= THIS_MODULE,
    .min_uV		= 1300000,
    },
    };
#[no_mangle]
unsafe extern "C" fn isl6271a_probe(i2c: *mut i2c_client) -> c_int {
    static int isl6271a_probe(struct i2c_client *i2c)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(i2c);
    struct regulator_dev *rdev;
    let mut config: regulator_config = { };
    struct regulator_init_data *init_data	= dev_get_platdata(&i2c.dev);
    struct isl_pmic *pmic;
    int i;
    if (!i2c_check_functionality(i2c.adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -EIO;
    pmic = devm_kzalloc(&i2c.dev, sizeof(struct isl_pmic), GFP_KERNEL);
    if (!pmic)
    return -ENOMEM;
    pmic.client = i2c;
    mutex_init(&pmic.mtx);
    for (i = 0; i < 3; i++) {
    config.dev = &i2c.dev;
    if (i == 0)
    config.init_data = init_data;
    else
    config.init_data = core::ptr::null_mut();
    config.driver_data = pmic;
    rdev = devm_regulator_register(&i2c.dev, &isl_rd[i], &config);
    if (IS_ERR(rdev)) {
    dev_err(&i2c.dev, "failed to register %s\n", id.name);
    return PTR_ERR(rdev);
    }
    }
    i2c_set_clientdata(i2c, pmic);
    return 0;
    }
    static const struct i2c_device_id isl6271a_id[] = {
    { .name = "isl6271a" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, isl6271a_id);
    static struct i2c_driver isl6271a_i2c_driver = {
    .driver = {
    .name = "isl6271a",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe = isl6271a_probe,
    .id_table = isl6271a_id,
    };
#[no_mangle]
unsafe extern "C" fn isl6271a_init() -> int __init {
    static int __init isl6271a_init(void)
    {
    return i2c_add_driver(&isl6271a_i2c_driver);
    }
#[no_mangle]
unsafe extern "C" fn isl6271a_cleanup() -> void __exit {
    static void __exit isl6271a_cleanup(void)
    {
    i2c_del_driver(&isl6271a_i2c_driver);
    }
    subsys_initcall(isl6271a_init);
    module_exit(isl6271a_cleanup);
    MODULE_AUTHOR("Marek Vasut <marek.vasut@gmail.com>");
    MODULE_DESCRIPTION("Intersil ISL6271A voltage regulator driver");
    MODULE_LICENSE("GPL v2");
