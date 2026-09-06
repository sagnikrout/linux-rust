//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/88pm800-regulator.c
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
// Regulators driver for Marvell 88PM800
//
// Copyright (C) 2012 Marvell International Ltd.
// Joseph(Yossi) Hanin <yhanin@marvell.com>
// Yi Zhang <yizhang@marvell.com>
//

// LDO1 with DVC[0..3]

// BUCK1 with DVC[0..3]

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm800_regulator_info {
    pub desc: regulator_desc,
    pub max_ua: c_int,
}

//
// vreg - the buck regs string.
// ereg - the string for the enable register.
// ebit - the bit number in the enable register.
// amax - the current
// Buck has 2 kinds of voltage steps. It is easy to find voltage by ranges,
// not the constant voltage table.
// n_volt - Number of available selectors
//

    {									\
    .desc	= {							\
    .name			= #vreg,			\
    .of_match		= of_match_ptr(#match),		\
    .regulators_node	= of_match_ptr("regulators"),	\
    .ops			= &pm800_volt_range_ops,	\
    .type			= REGULATOR_VOLTAGE,		\
    .id			= PM800_ID_##vreg,		\
    .owner			= THIS_MODULE,			\
    .n_voltages		= n_volt,			\
    .linear_ranges		= volt_ranges,			\
    .n_linear_ranges	= ARRAY_SIZE(volt_ranges),	\
    .vsel_reg		= PM800_##vreg,			\
    .vsel_mask		= 0x7f,				\
    .enable_reg		= PM800_##ereg,			\
    .enable_mask		= 1 << (ebit),			\
    },								\
    .max_ua	= (amax),						\
    }
//
// vreg - the LDO regs string
// ereg -  the string for the enable register.
// ebit - the bit number in the enable register.
// amax - the current
// volt_table - the LDO voltage table
// For all the LDOes, there are too many ranges. Using volt_table will be
// simpler and faster.
//

    {									\
    .desc	= {							\
    .name			= #vreg,			\
    .of_match		= of_match_ptr(#match),		\
    .regulators_node	= of_match_ptr("regulators"),	\
    .ops			= &pm800_volt_table_ops,	\
    .type			= REGULATOR_VOLTAGE,		\
    .id			= PM800_ID_##vreg,		\
    .owner			= THIS_MODULE,			\
    .n_voltages		= ARRAY_SIZE(ldo_volt_table),	\
    .vsel_reg		= PM800_##vreg##_VOUT,		\
    .vsel_mask		= 0xf,				\
    .enable_reg		= PM800_##ereg,			\
    .enable_mask		= 1 << (ebit),			\
    .volt_table		= ldo_volt_table,		\
    },								\
    .max_ua	= (amax),						\
    }
// Ranges are sorted in ascending order.
    static const struct linear_range buck1_volt_range[] = {
    REGULATOR_LINEAR_RANGE(600000, 0, 0x4f, 12500),
    REGULATOR_LINEAR_RANGE(1600000, 0x50, 0x54, 50000),
    };
// BUCK 2~5 have same ranges.
    static const struct linear_range buck2_5_volt_range[] = {
    REGULATOR_LINEAR_RANGE(600000, 0, 0x4f, 12500),
    REGULATOR_LINEAR_RANGE(1600000, 0x50, 0x72, 50000),
    };
    static const unsigned int ldo1_volt_table[] = {
    600000,  650000,  700000,  750000,  800000,  850000,  900000,  950000,
    1000000, 1050000, 1100000, 1150000, 1200000, 1300000, 1400000, 1500000,
    };
    static const unsigned int ldo2_volt_table[] = {
    1700000, 1800000, 1900000, 2000000, 2100000, 2500000, 2700000, 2800000,
    };
// LDO 3~17 have same voltage table.
    static const unsigned int ldo3_17_volt_table[] = {
    1200000, 1250000, 1700000, 1800000, 1850000, 1900000, 2500000, 2600000,
    2700000, 2750000, 2800000, 2850000, 2900000, 3000000, 3100000, 3300000,
    };
// LDO 18~19 have same voltage table.
    static const unsigned int ldo18_19_volt_table[] = {
    1700000, 1800000, 1900000, 2500000, 2800000, 2900000, 3100000, 3300000,
    };
#[no_mangle]
unsafe extern "C" fn pm800_get_current_limit(rdev: *mut regulator_dev) -> c_int {
    static int pm800_get_current_limit(struct regulator_dev *rdev)
    {
    struct pm800_regulator_info *info = rdev_get_drvdata(rdev);
    return info.max_ua;
    }
    static const struct regulator_ops pm800_volt_range_ops = {
    .list_voltage		= regulator_list_voltage_linear_range,
    .map_voltage		= regulator_map_voltage_linear_range,
    .set_voltage_sel	= regulator_set_voltage_sel_regmap,
    .get_voltage_sel	= regulator_get_voltage_sel_regmap,
    .enable			= regulator_enable_regmap,
    .disable		= regulator_disable_regmap,
    .is_enabled		= regulator_is_enabled_regmap,
    .get_current_limit	= pm800_get_current_limit,
    };
    static const struct regulator_ops pm800_volt_table_ops = {
    .list_voltage		= regulator_list_voltage_table,
    .map_voltage		= regulator_map_voltage_iterate,
    .set_voltage_sel	= regulator_set_voltage_sel_regmap,
    .get_voltage_sel	= regulator_get_voltage_sel_regmap,
    .enable			= regulator_enable_regmap,
    .disable		= regulator_disable_regmap,
    .is_enabled		= regulator_is_enabled_regmap,
    .get_current_limit	= pm800_get_current_limit,
    };
// The array is indexed by id(PM800_ID_XXX)
    static struct pm800_regulator_info pm800_regulator_info[] = {
    PM800_BUCK(buck1, BUCK1, BUCK_ENA, 0, 3000000, buck1_volt_range, 0x55),
    PM800_BUCK(buck2, BUCK2, BUCK_ENA, 1, 1200000, buck2_5_volt_range, 0x73),
    PM800_BUCK(buck3, BUCK3, BUCK_ENA, 2, 1200000, buck2_5_volt_range, 0x73),
    PM800_BUCK(buck4, BUCK4, BUCK_ENA, 3, 1200000, buck2_5_volt_range, 0x73),
    PM800_BUCK(buck5, BUCK5, BUCK_ENA, 4, 1200000, buck2_5_volt_range, 0x73),
    PM800_LDO(ldo1, LDO1, LDO_ENA1_1, 0, 200000, ldo1_volt_table),
    PM800_LDO(ldo2, LDO2, LDO_ENA1_1, 1, 10000, ldo2_volt_table),
    PM800_LDO(ldo3, LDO3, LDO_ENA1_1, 2, 300000, ldo3_17_volt_table),
    PM800_LDO(ldo4, LDO4, LDO_ENA1_1, 3, 300000, ldo3_17_volt_table),
    PM800_LDO(ldo5, LDO5, LDO_ENA1_1, 4, 300000, ldo3_17_volt_table),
    PM800_LDO(ldo6, LDO6, LDO_ENA1_1, 5, 300000, ldo3_17_volt_table),
    PM800_LDO(ldo7, LDO7, LDO_ENA1_1, 6, 300000, ldo3_17_volt_table),
    PM800_LDO(ldo8, LDO8, LDO_ENA1_1, 7, 300000, ldo3_17_volt_table),
    PM800_LDO(ldo9, LDO9, LDO_ENA1_2, 0, 300000, ldo3_17_volt_table),
    PM800_LDO(ldo10, LDO10, LDO_ENA1_2, 1, 300000, ldo3_17_volt_table),
    PM800_LDO(ldo11, LDO11, LDO_ENA1_2, 2, 300000, ldo3_17_volt_table),
    PM800_LDO(ldo12, LDO12, LDO_ENA1_2, 3, 300000, ldo3_17_volt_table),
    PM800_LDO(ldo13, LDO13, LDO_ENA1_2, 4, 300000, ldo3_17_volt_table),
    PM800_LDO(ldo14, LDO14, LDO_ENA1_2, 5, 300000, ldo3_17_volt_table),
    PM800_LDO(ldo15, LDO15, LDO_ENA1_2, 6, 300000, ldo3_17_volt_table),
    PM800_LDO(ldo16, LDO16, LDO_ENA1_2, 7, 300000, ldo3_17_volt_table),
    PM800_LDO(ldo17, LDO17, LDO_ENA1_3, 0, 300000, ldo3_17_volt_table),
    PM800_LDO(ldo18, LDO18, LDO_ENA1_3, 1, 200000, ldo18_19_volt_table),
    PM800_LDO(ldo19, LDO19, LDO_ENA1_3, 2, 200000, ldo18_19_volt_table),
    };
#[no_mangle]
unsafe extern "C" fn pm800_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int pm800_regulator_probe(struct platform_device *pdev)
    {
    struct pm80x_chip *chip = dev_get_drvdata(pdev.dev.parent);
    struct pm80x_platform_data *pdata = dev_get_platdata(pdev.dev.parent);
    let mut config: regulator_config = { };
    struct regulator_init_data *init_data;
    int i, ret;
    if (pdata && pdata.num_regulators) {
    let mut count: c_uint = 0;
// Check whether num_regulator is valid.
    for (i = 0; i < ARRAY_SIZE(pdata.regulators); i++) {
    if (pdata.regulators[i])
    count++;
    }
    if (count != pdata.num_regulators)
    return -EINVAL;
    }
    config.dev = chip.dev;
    config.regmap = chip.subchip.regmap_power;
    for (i = 0; i < PM800_ID_RG_MAX; i++) {
    struct regulator_dev *regulator;
    if (pdata && pdata.num_regulators) {
    init_data = pdata.regulators[i];
    if (!init_data)
    continue;
    config.init_data = init_data;
    }
    config.driver_data = &pm800_regulator_info[i];
    regulator = devm_regulator_register(&pdev.dev,
    &pm800_regulator_info[i].desc, &config);
    if (IS_ERR(regulator)) {
    ret = PTR_ERR(regulator);
    dev_err(&pdev.dev, "Failed to register %s\n",
    pm800_regulator_info[i].desc.name);
    return ret;
    }
    }
    return 0;
    }
    static struct platform_driver pm800_regulator_driver = {
    .driver		= {
    .name	= "88pm80x-regulator",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe		= pm800_regulator_probe,
    };
    module_platform_driver(pm800_regulator_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Joseph(Yossi) Hanin <yhanin@marvell.com>");
    MODULE_DESCRIPTION("Regulator Driver for Marvell 88PM800 PMIC");
    MODULE_ALIAS("platform:88pm800-regulator");
