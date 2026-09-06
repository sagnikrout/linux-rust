//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/hi655x-regulator.c
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
// Device driver for regulators in Hi655x IC
//
// Copyright (c) 2016 HiSilicon Ltd.
//
// Authors:
// Chen Feng <puck.chen@hisilicon.com>
// Fei  Wang <w.f@huawei.com>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hi655x_regulator {
    pub disable_reg: c_uint,
    pub status_reg: c_uint,
    pub rdesc: regulator_desc,
}

// LDO7 & LDO10
    static const unsigned int ldo7_voltages[] = {
    1800000, 1850000, 2850000, 2900000,
    3000000, 3100000, 3200000, 3300000,
    };
    static const unsigned int ldo19_voltages[] = {
    1800000, 1850000, 1900000, 1750000,
    2800000, 2850000, 2900000, 3000000,
    };
    static const unsigned int ldo22_voltages[] = {
    900000, 1000000, 1050000, 1100000,
    1150000, 1175000, 1185000, 1200000,
    };
    enum hi655x_regulator_id {
    HI655X_LDO0,
    HI655X_LDO1,
    HI655X_LDO2,
    HI655X_LDO3,
    HI655X_LDO4,
    HI655X_LDO5,
    HI655X_LDO6,
    HI655X_LDO7,
    HI655X_LDO8,
    HI655X_LDO9,
    HI655X_LDO10,
    HI655X_LDO11,
    HI655X_LDO12,
    HI655X_LDO13,
    HI655X_LDO14,
    HI655X_LDO15,
    HI655X_LDO16,
    HI655X_LDO17,
    HI655X_LDO18,
    HI655X_LDO19,
    HI655X_LDO20,
    HI655X_LDO21,
    HI655X_LDO22,
    };
#[no_mangle]
unsafe extern "C" fn hi655x_is_enabled(rdev: *mut regulator_dev) -> c_int {
    static int hi655x_is_enabled(struct regulator_dev *rdev)
    {
    let mut value: c_uint = 0;
    const struct hi655x_regulator *regulator = rdev_get_drvdata(rdev);
    regmap_read(rdev.regmap, regulator.status_reg, &value);
    return (value & rdev.desc.enable_mask);
    }
#[no_mangle]
unsafe extern "C" fn hi655x_disable(rdev: *mut regulator_dev) -> c_int {
    static int hi655x_disable(struct regulator_dev *rdev)
    {
    const struct hi655x_regulator *regulator = rdev_get_drvdata(rdev);
    return regmap_write(rdev.regmap, regulator.disable_reg,
    rdev.desc.enable_mask);
    }
    static const struct regulator_ops hi655x_regulator_ops = {
    .enable = regulator_enable_regmap,
    .disable = hi655x_disable,
    .is_enabled = hi655x_is_enabled,
    .list_voltage = regulator_list_voltage_table,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    };
    static const struct regulator_ops hi655x_ldo_linear_ops = {
    .enable = regulator_enable_regmap,
    .disable = hi655x_disable,
    .is_enabled = hi655x_is_enabled,
    .list_voltage = regulator_list_voltage_linear,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    };

    sreg, cmask, vtable) {                        \
    .rdesc = {                                               \
    .name            = #_ID,                         \
    .of_match        = of_match_ptr(#_ID),           \
    .ops             = &hi655x_regulator_ops,        \
    .regulators_node = of_match_ptr("regulators"),   \
    .type            = REGULATOR_VOLTAGE,            \
    .id              = HI655X_##_ID,                 \
    .owner           = THIS_MODULE,                  \
    .n_voltages      = ARRAY_SIZE(vtable),           \
    .volt_table      = vtable,                       \
    .vsel_reg        = HI655X_BUS_ADDR(vreg),        \
    .vsel_mask       = vmask,                        \
    .enable_reg      = HI655X_BUS_ADDR(ereg),        \
    .enable_mask     = BIT(cmask),                   \
    },                                                       \
    .disable_reg = HI655X_BUS_ADDR(dreg),                    \
    .status_reg = HI655X_BUS_ADDR(sreg),                     \
    }

    sreg, cmask, minv, nvolt, vstep) {     \
    .rdesc = {                                               \
    .name            = #_ID,                         \
    .of_match        = of_match_ptr(#_ID),           \
    .ops             = &hi655x_ldo_linear_ops,       \
    .regulators_node = of_match_ptr("regulators"),   \
    .type            = REGULATOR_VOLTAGE,            \
    .id              = HI655X_##_ID,                 \
    .owner           = THIS_MODULE,                  \
    .min_uV          = minv,                         \
    .n_voltages      = nvolt,                        \
    .uV_step         = vstep,                        \
    .vsel_reg        = HI655X_BUS_ADDR(vreg),        \
    .vsel_mask       = vmask,                        \
    .enable_reg      = HI655X_BUS_ADDR(ereg),        \
    .enable_mask     = BIT(cmask),                   \
    },                                                       \
    .disable_reg = HI655X_BUS_ADDR(dreg),                    \
    .status_reg = HI655X_BUS_ADDR(sreg),                     \
    }
    static const struct hi655x_regulator regulators[] = {
    HI655X_LDO_LINEAR(LDO2, 0x72, 0x07, 0x29, 0x2a, 0x2b, 0x01,
    2500000, 8, 100000),
    HI655X_LDO(LDO7, 0x78, 0x07, 0x29, 0x2a, 0x2b, 0x06, ldo7_voltages),
    HI655X_LDO(LDO10, 0x78, 0x07, 0x29, 0x2a, 0x2b, 0x01, ldo7_voltages),
    HI655X_LDO_LINEAR(LDO13, 0x7e, 0x07, 0x2c, 0x2d, 0x2e, 0x04,
    1600000, 8, 50000),
    HI655X_LDO_LINEAR(LDO14, 0x7f, 0x07, 0x2c, 0x2d, 0x2e, 0x05,
    2500000, 8, 100000),
    HI655X_LDO_LINEAR(LDO15, 0x80, 0x07, 0x2c, 0x2d, 0x2e, 0x06,
    1600000, 8, 50000),
    HI655X_LDO_LINEAR(LDO17, 0x82, 0x07, 0x2f, 0x30, 0x31, 0x00,
    2500000, 8, 100000),
    HI655X_LDO(LDO19, 0x84, 0x07, 0x2f, 0x30, 0x31, 0x02, ldo19_voltages),
    HI655X_LDO_LINEAR(LDO21, 0x86, 0x07, 0x2f, 0x30, 0x31, 0x04,
    1650000, 8, 50000),
    HI655X_LDO(LDO22, 0x87, 0x07, 0x2f, 0x30, 0x31, 0x05, ldo22_voltages),
    };
#[no_mangle]
unsafe extern "C" fn hi655x_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int hi655x_regulator_probe(struct platform_device *pdev)
    {
    unsigned int i;
    struct hi655x_pmic *pmic;
    let mut config: regulator_config = { };
    struct regulator_dev *rdev;
    pmic = dev_get_drvdata(pdev.dev.parent);
    if (!pmic) {
    dev_err(&pdev.dev, "no pmic in the regulator parent node\n");
    return -ENODEV;
    }
    config.dev = pdev.dev.parent;
    config.regmap = pmic.regmap;
    for (i = 0; i < ARRAY_SIZE(regulators); i++) {
    config.driver_data = (void *) &regulators[i];
    rdev = devm_regulator_register(&pdev.dev,
    &regulators[i].rdesc,
    &config);
    if (IS_ERR(rdev)) {
    dev_err(&pdev.dev, "failed to register regulator %s\n",
    regulators[i].rdesc.name);
    return PTR_ERR(rdev);
    }
    }
    return 0;
    }
    static const struct platform_device_id hi655x_regulator_table[] = {
    { .name = "hi655x-regulator" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, hi655x_regulator_table);
    static struct platform_driver hi655x_regulator_driver = {
    .id_table = hi655x_regulator_table,
    .driver = {
    .name	= "hi655x-regulator",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe	= hi655x_regulator_probe,
    };
    module_platform_driver(hi655x_regulator_driver);
    MODULE_AUTHOR("Chen Feng <puck.chen@hisilicon.com>");
    MODULE_DESCRIPTION("Hisilicon Hi655x regulator driver");
    MODULE_LICENSE("GPL v2");
