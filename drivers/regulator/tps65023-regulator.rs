//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/tps65023-regulator.c
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
// tps65023-regulator.c
//
// Supports TPS65023 Regulator
//
// Copyright (C) 2009 Texas Instrument Incorporated - https://www.ti.com
//

// Register definitions
pub const TPS65023_REG_VERSION: c_int = 0;
pub const TPS65023_REG_PGOODZ: c_int = 1;
pub const TPS65023_REG_MASK: c_int = 2;
pub const TPS65023_REG_REG_CTRL: c_int = 3;
pub const TPS65023_REG_CON_CTRL: c_int = 4;
pub const TPS65023_REG_CON_CTRL2: c_int = 5;
pub const TPS65023_REG_DEF_CORE: c_int = 6;
pub const TPS65023_REG_DEFSLEW: c_int = 7;
pub const TPS65023_REG_LDO_CTRL: c_int = 8;
// PGOODZ bitfields

// MASK bitfields

// REG_CTRL bitfields

// REG_CTRL2 bitfields

// Number of step-down converters available
pub const TPS65023_NUM_DCDC: c_int = 3;
// Number of LDO voltage regulators  available
pub const TPS65023_NUM_LDO: c_int = 2;
// Number of total regulators available

// DCDCs
pub const TPS65023_DCDC_1: c_int = 0;
pub const TPS65023_DCDC_2: c_int = 1;
pub const TPS65023_DCDC_3: c_int = 2;
// LDOs
pub const TPS65023_LDO_1: c_int = 3;
pub const TPS65023_LDO_2: c_int = 4;

    {							\
    .name		= "VDCDC"#_num,			\
    .of_match	= of_match_ptr("VDCDC"#_num),	\
    .regulators_node = of_match_ptr("regulators"),	\
    .id		= TPS65023_DCDC_##_num,		\
    .n_voltages     = ARRAY_SIZE(_t),		\
    .ops		= &tps65023_dcdc_ops,		\
    .type		= REGULATOR_VOLTAGE,		\
    .owner		= THIS_MODULE,			\
    .volt_table	= _t,				\
    .vsel_reg	= TPS65023_REG_DEF_CORE,	\
    .vsel_mask	= ARRAY_SIZE(_t) - 1,		\
    .enable_mask	= _em,				\
    .enable_reg	= TPS65023_REG_REG_CTRL,	\
    .apply_reg	= TPS65023_REG_CON_CTRL2,	\
    .apply_bit	= TPS65023_REG_CTRL2_GO,	\
    }							\

    {							\
    .name		= "LDO"#_num,			\
    .of_match	= of_match_ptr("LDO"#_num),	\
    .regulators_node = of_match_ptr("regulators"),	\
    .id		= TPS65023_LDO_##_num,		\
    .n_voltages     = ARRAY_SIZE(_t),		\
    .ops		= &tps65023_ldo_ops,		\
    .type		= REGULATOR_VOLTAGE,		\
    .owner		= THIS_MODULE,			\
    .volt_table	= _t,				\
    .vsel_reg	= TPS65023_REG_LDO_CTRL,	\
    .vsel_mask	= _vm,				\
    .enable_mask	= 1 << (_num),			\
    .enable_reg	= TPS65023_REG_REG_CTRL,	\
    }							\
// Supported voltage values for regulators
    static const unsigned int VCORE_VSEL_table[] = {
    800000, 825000, 850000, 875000,
    900000, 925000, 950000, 975000,
    1000000, 1025000, 1050000, 1075000,
    1100000, 1125000, 1150000, 1175000,
    1200000, 1225000, 1250000, 1275000,
    1300000, 1325000, 1350000, 1375000,
    1400000, 1425000, 1450000, 1475000,
    1500000, 1525000, 1550000, 1600000,
    };
    static const unsigned int DCDC_FIXED_3300000_VSEL_table[] = {
    3300000,
    };
    static const unsigned int DCDC_FIXED_1800000_VSEL_table[] = {
    1800000,
    };
// Supported voltage values for LDO regulators for tps65020
    static const unsigned int TPS65020_LDO_VSEL_table[] = {
    1000000, 1050000, 1100000, 1300000,
    1800000, 2500000, 3000000, 3300000,
    };
// Supported voltage values for LDO regulators
// for tps65021 and tps65023
    static const unsigned int TPS65023_LDO1_VSEL_table[] = {
    1000000, 1100000, 1300000, 1800000,
    2200000, 2600000, 2800000, 3150000,
    };
    static const unsigned int TPS65023_LDO2_VSEL_table[] = {
    1050000, 1200000, 1300000, 1800000,
    2500000, 2800000, 3000000, 3300000,
    };
// PMIC details
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps_pmic {
    pub rdev: [*mut regulator_dev; TPS65023_NUM_REGULATOR],
    pub driver_data: *const tps_driver_data,
    pub regmap: *mut regmap,
}

// Struct passed as driver data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps_driver_data {
    pub desc: *const regulator_desc,
    pub core_regulator: u8,
}

#[no_mangle]
unsafe extern "C" fn tps65023_dcdc_get_voltage_sel(dev: *mut regulator_dev) -> c_int {
    static int tps65023_dcdc_get_voltage_sel(struct regulator_dev *dev)
    {
    struct tps_pmic *tps = rdev_get_drvdata(dev);
    let mut dcdc: c_int = rdev_get_id(dev);
    if (dcdc < TPS65023_DCDC_1 || dcdc > TPS65023_DCDC_3)
    return -EINVAL;
    if (dcdc != tps.driver_data.core_regulator)
    return 0;
    return regulator_get_voltage_sel_regmap(dev);
    }
    static int tps65023_dcdc_set_voltage_sel(struct regulator_dev *dev,
    unsigned selector)
    {
    struct tps_pmic *tps = rdev_get_drvdata(dev);
    let mut dcdc: c_int = rdev_get_id(dev);
    if (dcdc != tps.driver_data.core_regulator)
    return -EINVAL;
    return regulator_set_voltage_sel_regmap(dev, selector);
    }
// Operations permitted on VDCDCx
    static const struct regulator_ops tps65023_dcdc_ops = {
    .is_enabled = regulator_is_enabled_regmap,
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .get_voltage_sel = tps65023_dcdc_get_voltage_sel,
    .set_voltage_sel = tps65023_dcdc_set_voltage_sel,
    .list_voltage = regulator_list_voltage_table,
    .map_voltage = regulator_map_voltage_ascend,
    };
// Operations permitted on LDOx
    static const struct regulator_ops tps65023_ldo_ops = {
    .is_enabled = regulator_is_enabled_regmap,
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .list_voltage = regulator_list_voltage_table,
    .map_voltage = regulator_map_voltage_ascend,
    };
    static const struct regmap_config tps65023_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    };
    static const struct regulator_desc tps65020_regulators[] = {
    TPS65023_REGULATOR_DCDC(1, DCDC_FIXED_3300000_VSEL_table, 0x20),
    TPS65023_REGULATOR_DCDC(2, DCDC_FIXED_1800000_VSEL_table, 0x10),
    TPS65023_REGULATOR_DCDC(3, VCORE_VSEL_table, 0x08),
    TPS65023_REGULATOR_LDO(1, TPS65020_LDO_VSEL_table, 0x07),
    TPS65023_REGULATOR_LDO(2, TPS65020_LDO_VSEL_table, 0x70),
    };
    static const struct regulator_desc tps65021_regulators[] = {
    TPS65023_REGULATOR_DCDC(1, DCDC_FIXED_3300000_VSEL_table, 0x20),
    TPS65023_REGULATOR_DCDC(2, DCDC_FIXED_1800000_VSEL_table, 0x10),
    TPS65023_REGULATOR_DCDC(3, VCORE_VSEL_table, 0x08),
    TPS65023_REGULATOR_LDO(1, TPS65023_LDO1_VSEL_table, 0x07),
    TPS65023_REGULATOR_LDO(2, TPS65023_LDO2_VSEL_table, 0x70),
    };
    static const struct regulator_desc tps65023_regulators[] = {
    TPS65023_REGULATOR_DCDC(1, VCORE_VSEL_table, 0x20),
    TPS65023_REGULATOR_DCDC(2, DCDC_FIXED_3300000_VSEL_table, 0x10),
    TPS65023_REGULATOR_DCDC(3, DCDC_FIXED_1800000_VSEL_table, 0x08),
    TPS65023_REGULATOR_LDO(1, TPS65023_LDO1_VSEL_table, 0x07),
    TPS65023_REGULATOR_LDO(2, TPS65023_LDO2_VSEL_table, 0x70),
    };
    static const struct tps_driver_data tps65020_drv_data = {
    .desc = tps65020_regulators,
    .core_regulator = TPS65023_DCDC_3,
    };
    static const struct tps_driver_data tps65021_drv_data = {
    .desc = tps65021_regulators,
    .core_regulator = TPS65023_DCDC_3,
    };
    static const struct tps_driver_data tps65023_drv_data = {
    .desc = tps65023_regulators,
    .core_regulator = TPS65023_DCDC_1,
    };
#[no_mangle]
unsafe extern "C" fn tps_65023_probe(client: *mut i2c_client) -> c_int {
    static int tps_65023_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct regulator_init_data *init_data = dev_get_platdata(&client.dev);
    let mut config: regulator_config = { };
    struct tps_pmic *tps;
    int i;
    int error;
    tps = devm_kzalloc(&client.dev, sizeof(*tps), GFP_KERNEL);
    if (!tps)
    return -ENOMEM;
    tps.driver_data = (struct tps_driver_data *)id.driver_data;
    tps.regmap = devm_regmap_init_i2c(client, &tps65023_regmap_config);
    if (IS_ERR(tps.regmap)) {
    error = PTR_ERR(tps.regmap);
    dev_err(&client.dev, "Failed to allocate register map: %d\n",
    error);
    return error;
    }
// common for all regulators
    config.dev = &client.dev;
    config.driver_data = tps;
    config.regmap = tps.regmap;
    for (i = 0; i < TPS65023_NUM_REGULATOR; i++) {
    if (init_data)
    config.init_data = &init_data[i];
// Register the regulators
    tps.rdev[i] = devm_regulator_register(&client.dev,
    &tps.driver_data.desc[i], &config);
    if (IS_ERR(tps.rdev[i])) {
    dev_err(&client.dev, "failed to register %s\n",
    id.name);
    return PTR_ERR(tps.rdev[i]);
    }
    }
    i2c_set_clientdata(client, tps);
// Enable setting output voltage by I2C
    regmap_update_bits(tps.regmap, TPS65023_REG_CON_CTRL2,
    TPS65023_REG_CTRL2_CORE_ADJ, 0);
    return 0;
    }
    static const struct of_device_id __maybe_unused tps65023_of_match[] = {
    { .compatible = "ti,tps65020", .data = &tps65020_drv_data},
    { .compatible = "ti,tps65021", .data = &tps65021_drv_data},
    { .compatible = "ti,tps65023", .data = &tps65023_drv_data},
    {},
    };
    MODULE_DEVICE_TABLE(of, tps65023_of_match);
    static const struct i2c_device_id tps_65023_id[] = {
    {
    .name = "tps65023",
    .driver_data = (kernel_ulong_t)&tps65023_drv_data,
    }, {
    .name = "tps65021",
    .driver_data = (kernel_ulong_t)&tps65021_drv_data,
    }, {
    .name = "tps65020",
    .driver_data = (kernel_ulong_t)&tps65020_drv_data,
    },
    { },
    };
    MODULE_DEVICE_TABLE(i2c, tps_65023_id);
    static struct i2c_driver tps_65023_i2c_driver = {
    .driver = {
    .name = "tps65023",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = of_match_ptr(tps65023_of_match),
    },
    .probe = tps_65023_probe,
    .id_table = tps_65023_id,
    };
#[no_mangle]
unsafe extern "C" fn tps_65023_init() -> int __init {
    static int __init tps_65023_init(void)
    {
    return i2c_add_driver(&tps_65023_i2c_driver);
    }
    subsys_initcall(tps_65023_init);
#[no_mangle]
unsafe extern "C" fn tps_65023_cleanup() -> void __exit {
    static void __exit tps_65023_cleanup(void)
    {
    i2c_del_driver(&tps_65023_i2c_driver);
    }
    module_exit(tps_65023_cleanup);
    MODULE_AUTHOR("Texas Instruments");
    MODULE_DESCRIPTION("TPS65023 voltage regulator driver");
    MODULE_LICENSE("GPL v2");
