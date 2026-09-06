//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/isl9305.c
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
// isl9305 - Intersil ISL9305 DCDC regulator
//
// Copyright 2014 Linaro Ltd
//
// Author: Mark Brown <broonie@kernel.org>
//

//
// Registers
//
pub const ISL9305_DCD1OUT: c_uint = 0x0;
pub const ISL9305_DCD2OUT: c_uint = 0x1;
pub const ISL9305_LDO1OUT: c_uint = 0x2;
pub const ISL9305_LDO2OUT: c_uint = 0x3;
pub const ISL9305_DCD_PARAMETER: c_uint = 0x4;
pub const ISL9305_SYSTEM_PARAMETER: c_uint = 0x5;
pub const ISL9305_DCD_SRCTL: c_uint = 0x6;

//
// DCD_PARAMETER
//
pub const ISL9305_DCD_PHASE: c_uint = 0x40;
pub const ISL9305_DCD2_ULTRA: c_uint = 0x20;
pub const ISL9305_DCD1_ULTRA: c_uint = 0x10;
pub const ISL9305_DCD2_BLD: c_uint = 0x08;
pub const ISL9305_DCD1_BLD: c_uint = 0x04;
pub const ISL9305_DCD2_MODE: c_uint = 0x02;
pub const ISL9305_DCD1_MODE: c_uint = 0x01;
//
// SYSTEM_PARAMETER
//
pub const ISL9305_I2C_EN: c_uint = 0x40;
pub const ISL9305_DCDPOR_MASK: c_uint = 0x30;
pub const ISL9305_LDO2_EN: c_uint = 0x08;
pub const ISL9305_LDO1_EN: c_uint = 0x04;
pub const ISL9305_DCD2_EN: c_uint = 0x02;
pub const ISL9305_DCD1_EN: c_uint = 0x01;
//
// DCD_SRCTL
//
pub const ISL9305_DCD2SR_MASK: c_uint = 0xc0;
pub const ISL9305_DCD1SR_MASK: c_uint = 0x07;
    static const struct regulator_ops isl9305_ops = {
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .list_voltage = regulator_list_voltage_linear,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    };
    static const struct regulator_desc isl9305_regulators[] = {
    [ISL9305_DCD1] = {
    .name =		"DCD1",
    .of_match =	of_match_ptr("dcd1"),
    .regulators_node = of_match_ptr("regulators"),
    .n_voltages =	0x70,
    .min_uV =	825000,
    .uV_step =	25000,
    .vsel_reg =	ISL9305_DCD1OUT,
    .vsel_mask =	0x7f,
    .enable_reg =	ISL9305_SYSTEM_PARAMETER,
    .enable_mask =	ISL9305_DCD1_EN,
    .supply_name =	"VINDCD1",
    .ops =		&isl9305_ops,
    .owner =	THIS_MODULE,
    },
    [ISL9305_DCD2] = {
    .name =		"DCD2",
    .of_match =	of_match_ptr("dcd2"),
    .regulators_node = of_match_ptr("regulators"),
    .n_voltages =	0x70,
    .min_uV =	825000,
    .uV_step =	25000,
    .vsel_reg =	ISL9305_DCD2OUT,
    .vsel_mask =	0x7f,
    .enable_reg =	ISL9305_SYSTEM_PARAMETER,
    .enable_mask =	ISL9305_DCD2_EN,
    .supply_name =	"VINDCD2",
    .ops =		&isl9305_ops,
    .owner =	THIS_MODULE,
    },
    [ISL9305_LDO1] = {
    .name =		"LDO1",
    .of_match =	of_match_ptr("ldo1"),
    .regulators_node = of_match_ptr("regulators"),
    .n_voltages =	0x37,
    .min_uV =	900000,
    .uV_step =	50000,
    .vsel_reg =	ISL9305_LDO1OUT,
    .vsel_mask =	0x3f,
    .enable_reg =	ISL9305_SYSTEM_PARAMETER,
    .enable_mask =	ISL9305_LDO1_EN,
    .supply_name =	"VINLDO1",
    .ops =		&isl9305_ops,
    .owner =	THIS_MODULE,
    },
    [ISL9305_LDO2] = {
    .name =		"LDO2",
    .of_match =	of_match_ptr("ldo2"),
    .regulators_node = of_match_ptr("regulators"),
    .n_voltages =	0x37,
    .min_uV =	900000,
    .uV_step =	50000,
    .vsel_reg =	ISL9305_LDO2OUT,
    .vsel_mask =	0x3f,
    .enable_reg =	ISL9305_SYSTEM_PARAMETER,
    .enable_mask =	ISL9305_LDO2_EN,
    .supply_name =	"VINLDO2",
    .ops =		&isl9305_ops,
    .owner =	THIS_MODULE,
    },
    };
    static const struct regmap_config isl9305_regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = ISL9305_MAX_REG,
    .cache_type = REGCACHE_MAPLE,
    };
#[no_mangle]
unsafe extern "C" fn isl9305_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int isl9305_i2c_probe(struct i2c_client *i2c)
    {
    let mut config: regulator_config = { };
    struct isl9305_pdata *pdata = i2c.dev.platform_data;
    struct regulator_dev *rdev;
    struct regmap *regmap;
    int i, ret;
    regmap = devm_regmap_init_i2c(i2c, &isl9305_regmap);
    if (IS_ERR(regmap)) {
    ret = PTR_ERR(regmap);
    dev_err(&i2c.dev, "Failed to create regmap: %d\n", ret);
    return ret;
    }
    config.dev = &i2c.dev;
    for (i = 0; i < ARRAY_SIZE(isl9305_regulators); i++) {
    if (pdata)
    config.init_data = pdata.init_data[i];
    else
    config.init_data = core::ptr::null_mut();
    rdev = devm_regulator_register(&i2c.dev,
    &isl9305_regulators[i],
    &config);
    if (IS_ERR(rdev)) {
    ret = PTR_ERR(rdev);
    dev_err(&i2c.dev, "Failed to register %s: %d\n",
    isl9305_regulators[i].name, ret);
    return ret;
    }
    }
    return 0;
    }

    static const struct of_device_id isl9305_dt_ids[] = {
    { .compatible = "isl,isl9305" }, /* for backward compat., don't use */
    { .compatible = "isil,isl9305" },
    { .compatible = "isl,isl9305h" }, /* for backward compat., don't use */
    { .compatible = "isil,isl9305h" },
    {},
    };
    MODULE_DEVICE_TABLE(of, isl9305_dt_ids);

    static const struct i2c_device_id isl9305_i2c_id[] = {
    { .name = "isl9305" },
    { .name = "isl9305h" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, isl9305_i2c_id);
    static struct i2c_driver isl9305_regulator_driver = {
    .driver = {
    .name = "isl9305",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table	= of_match_ptr(isl9305_dt_ids),
    },
    .probe = isl9305_i2c_probe,
    .id_table = isl9305_i2c_id,
    };
    module_i2c_driver(isl9305_regulator_driver);
    MODULE_AUTHOR("Mark Brown");
    MODULE_DESCRIPTION("Intersil ISL9305 DCDC regulator");
    MODULE_LICENSE("GPL");
