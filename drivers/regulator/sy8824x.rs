//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/sy8824x.c
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
// SY8824C/SY8824E regulator driver
//
// Copyright (C) 2019 Synaptics Incorporated
//
// Author: Jisheng Zhang <jszhang@kernel.org>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sy8824_config {
// registers
    pub vol_reg: c_uint,
    pub mode_reg: c_uint,
    pub enable_reg: c_uint,
// Voltage range and step(linear)
    pub vsel_min: c_uint,
    pub vsel_step: c_uint,
    pub vsel_count: c_uint,
    pub config: *const regmap_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sy8824_device_info {
    pub dev: *mut device,
    pub desc: regulator_desc,
    pub regulator: *mut regulator_init_data,
    pub cfg: *const sy8824_config,
}

#[no_mangle]
unsafe extern "C" fn sy8824_set_mode(rdev: *mut regulator_dev, mode: c_uint) -> c_int {
    static int sy8824_set_mode(struct regulator_dev *rdev, unsigned int mode)
    {
    struct sy8824_device_info *di = rdev_get_drvdata(rdev);
    const struct sy8824_config *cfg = di.cfg;
    switch (mode) {
    case REGULATOR_MODE_FAST:
    regmap_update_bits(rdev.regmap, cfg.mode_reg,
    SY8824C_MODE, SY8824C_MODE);
    break;
    case REGULATOR_MODE_NORMAL:
    regmap_update_bits(rdev.regmap, cfg.mode_reg,
    SY8824C_MODE, 0);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sy8824_get_mode(rdev: *mut regulator_dev) -> c_uint {
    static unsigned int sy8824_get_mode(struct regulator_dev *rdev)
    {
    struct sy8824_device_info *di = rdev_get_drvdata(rdev);
    const struct sy8824_config *cfg = di.cfg;
    u32 val;
    let mut ret: c_int = 0;
    ret = regmap_read(rdev.regmap, cfg.mode_reg, &val);
    if (ret < 0)
    return ret;
    if (val & SY8824C_MODE)
    return REGULATOR_MODE_FAST;
    else
    return REGULATOR_MODE_NORMAL;
    }
    static const struct regulator_ops sy8824_regulator_ops = {
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_voltage_time_sel = regulator_set_voltage_time_sel,
    .map_voltage = regulator_map_voltage_linear,
    .list_voltage = regulator_list_voltage_linear,
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .set_mode = sy8824_set_mode,
    .get_mode = sy8824_get_mode,
    };
    static int sy8824_regulator_register(struct sy8824_device_info *di,
    struct regulator_config *config)
    {
    struct regulator_desc *rdesc = &di.desc;
    const struct sy8824_config *cfg = di.cfg;
    struct regulator_dev *rdev;
    rdesc.name = "sy8824-reg";
    rdesc.supply_name = "vin";
    rdesc.ops = &sy8824_regulator_ops;
    rdesc.type = REGULATOR_VOLTAGE;
    rdesc.n_voltages = cfg.vsel_count;
    rdesc.enable_reg = cfg.enable_reg;
    rdesc.enable_mask = SY8824C_BUCK_EN;
    rdesc.min_uV = cfg.vsel_min;
    rdesc.uV_step = cfg.vsel_step;
    rdesc.vsel_reg = cfg.vol_reg;
    rdesc.vsel_mask = cfg.vsel_count - 1;
    rdesc.owner = THIS_MODULE;
    rdev = devm_regulator_register(di.dev, &di.desc, config);
    return PTR_ERR_OR_ZERO(rdev);
    }
    static const struct regmap_config sy8824_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .num_reg_defaults_raw = 1,
    .cache_type = REGCACHE_FLAT,
    };
    static const struct regmap_config sy20276_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .num_reg_defaults_raw = 2,
    .cache_type = REGCACHE_FLAT,
    };
#[no_mangle]
unsafe extern "C" fn sy8824_i2c_probe(client: *mut i2c_client) -> c_int {
    static int sy8824_i2c_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct device_node *np = dev.of_node;
    struct sy8824_device_info *di;
    let mut config: regulator_config = { };
    struct regmap *regmap;
    int ret;
    di = devm_kzalloc(dev, sizeof(struct sy8824_device_info), GFP_KERNEL);
    if (!di)
    return -ENOMEM;
    di.regulator = of_get_regulator_init_data(dev, np, &di.desc);
    if (!di.regulator) {
    dev_err(dev, "Platform data not found!\n");
    return -EINVAL;
    }
    di.dev = dev;
    di.cfg = i2c_get_match_data(client);
    regmap = devm_regmap_init_i2c(client, di.cfg.config);
    if (IS_ERR(regmap)) {
    dev_err(dev, "Failed to allocate regmap!\n");
    return PTR_ERR(regmap);
    }
    i2c_set_clientdata(client, di);
    config.dev = di.dev;
    config.init_data = di.regulator;
    config.regmap = regmap;
    config.driver_data = di;
    config.of_node = np;
    ret = sy8824_regulator_register(di, &config);
    if (ret < 0)
    dev_err(dev, "Failed to register regulator!\n");
    return ret;
    }
    static const struct sy8824_config sy8824c_cfg = {
    .vol_reg = 0x00,
    .mode_reg = 0x00,
    .enable_reg = 0x00,
    .vsel_min = 762500,
    .vsel_step = 12500,
    .vsel_count = 64,
    .config = &sy8824_regmap_config,
    };
    static const struct sy8824_config sy8824e_cfg = {
    .vol_reg = 0x00,
    .mode_reg = 0x00,
    .enable_reg = 0x00,
    .vsel_min = 700000,
    .vsel_step = 12500,
    .vsel_count = 64,
    .config = &sy8824_regmap_config,
    };
    static const struct sy8824_config sy20276_cfg = {
    .vol_reg = 0x00,
    .mode_reg = 0x01,
    .enable_reg = 0x01,
    .vsel_min = 600000,
    .vsel_step = 10000,
    .vsel_count = 128,
    .config = &sy20276_regmap_config,
    };
    static const struct sy8824_config sy20278_cfg = {
    .vol_reg = 0x00,
    .mode_reg = 0x01,
    .enable_reg = 0x01,
    .vsel_min = 762500,
    .vsel_step = 12500,
    .vsel_count = 64,
    .config = &sy20276_regmap_config,
    };
    static const struct of_device_id sy8824_dt_ids[] = {
    { .compatible = "silergy,sy8824c", .data = &sy8824c_cfg },
    { .compatible = "silergy,sy8824e", .data = &sy8824e_cfg },
    { .compatible = "silergy,sy20276", .data = &sy20276_cfg },
    { .compatible = "silergy,sy20278", .data = &sy20278_cfg },
    { }
    };
    MODULE_DEVICE_TABLE(of, sy8824_dt_ids);
    static const struct i2c_device_id sy8824_id[] = {
    { .name = "sy8824c", .driver_data = (kernel_ulong_t)&sy8824c_cfg },
    { .name = "sy8824e", .driver_data = (kernel_ulong_t)&sy8824e_cfg },
    { .name = "sy20276", .driver_data = (kernel_ulong_t)&sy20276_cfg },
    { .name = "sy20278", .driver_data = (kernel_ulong_t)&sy20278_cfg },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, sy8824_id);
    static struct i2c_driver sy8824_regulator_driver = {
    .driver = {
    .name = "sy8824-regulator",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = sy8824_dt_ids,
    },
    .probe = sy8824_i2c_probe,
    .id_table = sy8824_id,
    };
    module_i2c_driver(sy8824_regulator_driver);
    MODULE_AUTHOR("Jisheng Zhang <jszhang@kernel.org>");
    MODULE_DESCRIPTION("SY8824C/SY8824E regulator driver");
    MODULE_LICENSE("GPL v2");
