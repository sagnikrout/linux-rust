//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/aw37503-regulator.c
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
// AWINIC AW37503 Regulator Driver
//
// Copyright (C) 2023 awinic. All Rights Reserved
//
// Author: <like@awinic.com>

pub const AW37503_REG_VPOS: c_uint = 0x00;
pub const AW37503_REG_VNEG: c_uint = 0x01;
pub const AW37503_REG_APPS: c_uint = 0x03;
pub const AW37503_REG_CONTROL: c_uint = 0x04;
pub const AW37503_REG_WPRTEN: c_uint = 0x21;
pub const AW37503_VOUT_MASK: c_uint = 0x1F;
pub const AW37503_VOUT_N_VOLTAGE: c_uint = 0x15;
pub const AW37503_VOUT_VMIN: c_int = 4000000;
pub const AW37503_VOUT_VMAX: c_int = 6000000;
pub const AW37503_VOUT_STEP: c_int = 100000;

pub const AW37503_REGULATOR_ID_VPOS: c_int = 0;
pub const AW37503_REGULATOR_ID_VNEG: c_int = 1;
pub const AW37503_MAX_REGULATORS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw37503_reg_pdata {
    pub en_gpiod: *mut gpio_desc,
    pub ena_gpio_state: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw37503_regulator {
    pub dev: *mut device,
    pub reg_pdata: [aw37503_reg_pdata; AW37503_MAX_REGULATORS],
}

#[no_mangle]
unsafe extern "C" fn aw37503_regulator_enable(rdev: *mut regulator_dev) -> c_int {
    static int aw37503_regulator_enable(struct regulator_dev *rdev)
    {
    struct aw37503_regulator *chip = rdev_get_drvdata(rdev);
    let mut id: c_int = rdev_get_id(rdev);
    struct aw37503_reg_pdata *rpdata = &chip.reg_pdata[id];
    int ret;
    if (!IS_ERR(rpdata.en_gpiod)) {
    gpiod_set_value_cansleep(rpdata.en_gpiod, 1);
    rpdata.ena_gpio_state = 1;
    }
// Hardware automatically enable discharge bit in enable
    if (rdev.constraints.active_discharge ==
    REGULATOR_ACTIVE_DISCHARGE_DISABLE) {
    ret = regulator_set_active_discharge_regmap(rdev, false);
    if (ret < 0) {
    dev_err(chip.dev, "Failed to disable active discharge: %d\n",
    ret);
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aw37503_regulator_disable(rdev: *mut regulator_dev) -> c_int {
    static int aw37503_regulator_disable(struct regulator_dev *rdev)
    {
    struct aw37503_regulator *chip = rdev_get_drvdata(rdev);
    let mut id: c_int = rdev_get_id(rdev);
    struct aw37503_reg_pdata *rpdata = &chip.reg_pdata[id];
    if (!IS_ERR(rpdata.en_gpiod)) {
    gpiod_set_value_cansleep(rpdata.en_gpiod, 0);
    rpdata.ena_gpio_state = 0;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aw37503_regulator_is_enabled(rdev: *mut regulator_dev) -> c_int {
    static int aw37503_regulator_is_enabled(struct regulator_dev *rdev)
    {
    struct aw37503_regulator *chip = rdev_get_drvdata(rdev);
    let mut id: c_int = rdev_get_id(rdev);
    struct aw37503_reg_pdata *rpdata = &chip.reg_pdata[id];
    if (!IS_ERR(rpdata.en_gpiod))
    return rpdata.ena_gpio_state;
    return 1;
    }
    static const struct regulator_ops aw37503_regulator_ops = {
    .enable = aw37503_regulator_enable,
    .disable = aw37503_regulator_disable,
    .is_enabled = aw37503_regulator_is_enabled,
    .list_voltage = regulator_list_voltage_linear,
    .map_voltage = regulator_map_voltage_linear,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .set_active_discharge = regulator_set_active_discharge_regmap,
    };
    static int aw37503_of_parse_cb(struct device_node *np,
    const struct regulator_desc *desc,
    struct regulator_config *config)
    {
    struct aw37503_regulator *chip = config.driver_data;
    struct aw37503_reg_pdata *rpdata = &chip.reg_pdata[desc.id];
    int ret;
    rpdata.en_gpiod = devm_fwnode_gpiod_get(chip.dev, of_fwnode_handle(np),
    "enable", GPIOD_OUT_LOW,
    "enable");
    if (IS_ERR(rpdata.en_gpiod)) {
    ret = PTR_ERR(rpdata.en_gpiod);
// Ignore the error other than probe defer
    if (ret == -EPROBE_DEFER)
    return ret;
    return 0;
    }
    return 0;
    }

    [AW37503_REGULATOR_ID_##_id] = {		\
    .name = "aw37503-"#_name,		\
    .supply_name = "vin",			\
    .id = AW37503_REGULATOR_ID_##_id,	\
    .of_match = of_match_ptr(#_name),	\
    .of_parse_cb = aw37503_of_parse_cb,	\
    .ops = &aw37503_regulator_ops,		\
    .n_voltages = AW37503_VOUT_N_VOLTAGE,	\
    .min_uV = AW37503_VOUT_VMIN,		\
    .uV_step = AW37503_VOUT_STEP,		\
    .enable_time = 500,			\
    .vsel_mask = AW37503_VOUT_MASK,	\
    .vsel_reg = AW37503_REG_##_id,		\
    .active_discharge_off = 0,			\
    .active_discharge_on = AW37503_REG_APPS_DIS_##_id, \
    .active_discharge_mask = AW37503_REG_APPS_DIS_##_id, \
    .active_discharge_reg = AW37503_REG_APPS, \
    .type = REGULATOR_VOLTAGE,		\
    .owner = THIS_MODULE,			\
    }
    static const struct regulator_desc aw_regs_desc[AW37503_MAX_REGULATORS] = {
    AW37503_REGULATOR_DESC(VPOS, outp),
    AW37503_REGULATOR_DESC(VNEG, outn),
    };
    static const struct regmap_range aw37503_no_reg_ranges[] = {
    regmap_reg_range(AW37503_REG_CONTROL + 1,
    AW37503_REG_WPRTEN - 1),
    };
    static const struct regmap_access_table aw37503_no_reg_table = {
    .no_ranges = aw37503_no_reg_ranges,
    .n_no_ranges = ARRAY_SIZE(aw37503_no_reg_ranges),
    };
    static const struct regmap_config aw37503_regmap_config = {
    .reg_bits	= 8,
    .val_bits	= 8,
    .max_register	= AW37503_REG_WPRTEN,
    .rd_table	= &aw37503_no_reg_table,
    .wr_table	= &aw37503_no_reg_table,
    };
#[no_mangle]
unsafe extern "C" fn aw37503_probe(client: *mut i2c_client) -> c_int {
    static int aw37503_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct aw37503_regulator *chip;
    struct regulator_dev *rdev;
    struct regmap *regmap;
    let mut config: regulator_config = { };
    int id;
    chip = devm_kzalloc(dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    regmap = devm_regmap_init_i2c(client, &aw37503_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(dev, PTR_ERR(regmap),
    "Failed to init regmap\n");
    i2c_set_clientdata(client, chip);
    chip.dev = dev;
    config.regmap = regmap;
    config.dev = dev;
    config.driver_data = chip;
    for (id = 0; id < AW37503_MAX_REGULATORS; ++id) {
    rdev = devm_regulator_register(dev, &aw_regs_desc[id],
    &config);
    if (IS_ERR(rdev))
    return dev_err_probe(dev, PTR_ERR(rdev),
    "Failed to register regulator %s\n",
    aw_regs_desc[id].name);
    }
    return 0;
    }
    static const struct i2c_device_id aw37503_id[] = {
    { .name = "aw37503" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, aw37503_id);
    static const struct of_device_id aw37503_of_match[] = {
    {.compatible = "awinic,aw37503",},
    { /* Sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, aw37503_of_match);
    static struct i2c_driver aw37503_i2c_driver = {
    .driver = {
    .name = "aw37503",
    .of_match_table = aw37503_of_match,
    },
    .probe = aw37503_probe,
    .id_table = aw37503_id,
    };
    module_i2c_driver(aw37503_i2c_driver);
    MODULE_DESCRIPTION("aw37503 regulator driver");
    MODULE_AUTHOR("Alec Li <like@awinic.com>");
    MODULE_LICENSE("GPL");
