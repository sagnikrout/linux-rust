//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/fan53880.c
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

    enum fan53880_regulator_ids {
    FAN53880_LDO1,
    FAN53880_LDO2,
    FAN53880_LDO3,
    FAN53880_LDO4,
    FAN53880_BUCK,
    FAN53880_BOOST,
    };
    enum fan53880_registers {
    FAN53880_PRODUCT_ID = 0x00,
    FAN53880_SILICON_REV,
    FAN53880_BUCKVOUT,
    FAN53880_BOOSTVOUT,
    FAN53880_LDO1VOUT,
    FAN53880_LDO2VOUT,
    FAN53880_LDO3VOUT,
    FAN53880_LDO4VOUT,
    FAN53880_IOUT,
    FAN53880_ENABLE,
    FAN53880_ENABLE_BOOST,
    };
pub const FAN53880_ID: c_uint = 0x01;
    static const struct regulator_ops fan53880_ops = {
    .list_voltage = regulator_list_voltage_linear_range,
    .map_voltage = regulator_map_voltage_linear_range,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    };

    [FAN53880_LDO ## _num] = {					\
    .name =		   "LDO"#_num,				\
    .of_match =	   "LDO"#_num,		\
    .regulators_node = "regulators",		\
    .type =		   REGULATOR_VOLTAGE,			\
    .owner =	   THIS_MODULE,				\
    .linear_ranges =   (struct linear_range[]) {		\
    REGULATOR_LINEAR_RANGE(_default, 0x0, 0x0, 0),	\
    REGULATOR_LINEAR_RANGE(800000, 0xf, 0x73, 25000),	\
    },							\
    .n_linear_ranges = 2,					\
    .n_voltages =	   0x74,				\
    .vsel_reg =	   FAN53880_LDO ## _num ## VOUT,	\
    .vsel_mask =	   0x7f,				\
    .enable_reg =	   FAN53880_ENABLE,			\
    .enable_mask =	   BIT(_num - 1),			\
    .enable_time =	   150,					\
    .supply_name =	   _supply,				\
    .ops =		   &fan53880_ops,			\
    }
    static const struct regulator_desc fan53880_regulators[] = {
    FAN53880_LDO(1, "VIN12", 2800000),
    FAN53880_LDO(2, "VIN12", 2800000),
    FAN53880_LDO(3, "VIN3", 1800000),
    FAN53880_LDO(4, "VIN4", 1800000),
    [FAN53880_BUCK] = {
    .name =		   "BUCK",
    .of_match =	   "BUCK",
    .regulators_node = "regulators",
    .type =		   REGULATOR_VOLTAGE,
    .owner =	   THIS_MODULE,
    .linear_ranges =   (struct linear_range[]) {
    REGULATOR_LINEAR_RANGE(1100000, 0x0, 0x0, 0),
    REGULATOR_LINEAR_RANGE(600000, 0x1f, 0xf7, 12500),
    },
    .n_linear_ranges = 2,
    .n_voltages =	   0xf8,
    .vsel_reg =	   FAN53880_BUCKVOUT,
    .vsel_mask =	   0xff,
    .enable_reg =	   FAN53880_ENABLE,
    .enable_mask =	   0x10,
    .enable_time =	   480,
    .supply_name =	   "PVIN",
    .ops =		   &fan53880_ops,
    },
    [FAN53880_BOOST] = {
    .name =		   "BOOST",
    .of_match =	   "BOOST",
    .regulators_node = "regulators",
    .type =		   REGULATOR_VOLTAGE,
    .owner =	   THIS_MODULE,
    .linear_ranges =   (struct linear_range[]) {
    REGULATOR_LINEAR_RANGE(5000000, 0x0, 0x0, 0),
    REGULATOR_LINEAR_RANGE(3000000, 0x4, 0x70, 25000),
    },
    .n_linear_ranges = 2,
    .n_voltages =	   0x71,
    .vsel_reg =	   FAN53880_BOOSTVOUT,
    .vsel_mask =	   0x7f,
    .enable_reg =	   FAN53880_ENABLE_BOOST,
    .enable_mask =	   0xff,
    .enable_time =	   580,
    .supply_name =	   "PVIN",
    .ops =		   &fan53880_ops,
    },
    };
    static const struct regmap_config fan53880_regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = FAN53880_ENABLE_BOOST,
    };
#[no_mangle]
unsafe extern "C" fn fan53880_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int fan53880_i2c_probe(struct i2c_client *i2c)
    {
    let mut config: regulator_config = { };
    struct regulator_dev *rdev;
    struct regmap *regmap;
    int i, ret;
    unsigned int data;
    regmap = devm_regmap_init_i2c(i2c, &fan53880_regmap);
    if (IS_ERR(regmap)) {
    ret = PTR_ERR(regmap);
    dev_err(&i2c.dev, "Failed to create regmap: %d\n", ret);
    return ret;
    }
    ret = regmap_read(regmap, FAN53880_PRODUCT_ID, &data);
    if (ret < 0) {
    dev_err(&i2c.dev, "Failed to read PRODUCT_ID: %d\n", ret);
    return ret;
    }
    if (data != FAN53880_ID) {
    dev_err(&i2c.dev, "Unsupported device id: 0x%x.\n", data);
    return -ENODEV;
    }
    config.dev = &i2c.dev;
    config.init_data = core::ptr::null_mut();
    for (i = 0; i < ARRAY_SIZE(fan53880_regulators); i++) {
    rdev = devm_regulator_register(&i2c.dev,
    &fan53880_regulators[i],
    &config);
    if (IS_ERR(rdev)) {
    ret = PTR_ERR(rdev);
    dev_err(&i2c.dev, "Failed to register %s: %d\n",
    fan53880_regulators[i].name, ret);
    return ret;
    }
    }
    return 0;
    }
    static const struct of_device_id fan53880_dt_ids[] = {
    { .compatible = "onnn,fan53880", },
    {}
    };
    MODULE_DEVICE_TABLE(of, fan53880_dt_ids);
    static const struct i2c_device_id fan53880_i2c_id[] = {
    { .name = "fan53880" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, fan53880_i2c_id);
    static struct i2c_driver fan53880_regulator_driver = {
    .driver = {
    .name = "fan53880",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table	= fan53880_dt_ids,
    },
    .probe = fan53880_i2c_probe,
    .id_table = fan53880_i2c_id,
    };
    module_i2c_driver(fan53880_regulator_driver);
    MODULE_DESCRIPTION("FAN53880 PMIC voltage regulator driver");
    MODULE_AUTHOR("Christoph Fritz <chf.fritz@googlemail.com>");
    MODULE_LICENSE("GPL");
