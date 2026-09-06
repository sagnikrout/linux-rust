//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/spacemit-p1.c
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
// Driver for regulators found in the SpacemiT P1 PMIC
//
// Copyright (C) 2025 by RISCstar Solutions Corporation.  All rights reserved.
// Derived from code from SpacemiT.
// Copyright (c) 2023, SPACEMIT Co., Ltd
//

    enum p1_regulator_id {
    P1_BUCK1,
    P1_BUCK2,
    P1_BUCK3,
    P1_BUCK4,
    P1_BUCK5,
    P1_BUCK6,
    P1_ALDO1,
    P1_ALDO2,
    P1_ALDO3,
    P1_ALDO4,
    P1_DLDO1,
    P1_DLDO2,
    P1_DLDO3,
    P1_DLDO4,
    P1_DLDO5,
    P1_DLDO6,
    P1_DLDO7,
    };
    static const struct regulator_ops p1_regulator_ops = {
    .list_voltage		= regulator_list_voltage_linear_range,
    .get_voltage_sel	= regulator_get_voltage_sel_regmap,
    .set_voltage_sel	= regulator_set_voltage_sel_regmap,
    .set_voltage_time_sel   = regulator_set_voltage_time_sel,
    .enable			= regulator_enable_regmap,
    .disable		= regulator_disable_regmap,
    .is_enabled		= regulator_is_enabled_regmap,
    };
// Selector value 255 can be used to disable the buck converter on sleep
    static const struct linear_range p1_buck_ranges[] = {
    REGULATOR_LINEAR_RANGE(500000, 0, 170, 5000),
    REGULATOR_LINEAR_RANGE(1375000, 171, 254, 25000),
    };
// Selector value 0 can be used for suspend
    static const struct linear_range p1_ldo_ranges[] = {
    REGULATOR_LINEAR_RANGE(500000, 11, 127, 25000),
    };
// These define the voltage selector field for buck and LDO regulators

    {								\
    .name			= #_type #_n,			\
    .supply_name		= _s,				\
    .of_match		= of_match_ptr(#_type #_n),	\
    .regulators_node	= of_match_ptr("regulators"),	\
    .id			= P1_ID(_TYPE, _n),		\
    .n_voltages		= _nv,				\
    .ops			= &p1_regulator_ops,		\
    .owner			= THIS_MODULE,			\
    .linear_ranges		= _ranges,			\
    .n_linear_ranges	= ARRAY_SIZE(_ranges),		\
    .vsel_reg		= P1_ENABLE_REG(_off, _n) + 1,	\
    .vsel_mask		= _mask,			\
    .enable_reg		= P1_ENABLE_REG(_off, _n),	\
    .enable_mask		= BIT(0),			\
    }

    P1_REG_DESC(BUCK, buck, _n, "vin" #_n, 0x47, BUCK_MASK, 255, p1_buck_ranges)

    P1_REG_DESC(ALDO, aldo, _n, "aldoin", 0x5b, LDO_MASK, 128, p1_ldo_ranges)

    P1_REG_DESC(DLDO, dldo, _n, "dldoin1", 0x67, LDO_MASK, 128, p1_ldo_ranges)

    P1_REG_DESC(DLDO, dldo, _n, "dldoin2", 0x67, LDO_MASK, 128, p1_ldo_ranges)
    static const struct regulator_desc p1_regulator_desc[] = {
    P1_BUCK_DESC(1),
    P1_BUCK_DESC(2),
    P1_BUCK_DESC(3),
    P1_BUCK_DESC(4),
    P1_BUCK_DESC(5),
    P1_BUCK_DESC(6),
    P1_ALDO_DESC(1),
    P1_ALDO_DESC(2),
    P1_ALDO_DESC(3),
    P1_ALDO_DESC(4),
    P1_DLDO1_DESC(1),
    P1_DLDO1_DESC(2),
    P1_DLDO1_DESC(3),
    P1_DLDO1_DESC(4),
    P1_DLDO2_DESC(5),
    P1_DLDO2_DESC(6),
    P1_DLDO2_DESC(7),
    };
#[no_mangle]
unsafe extern "C" fn p1_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int p1_regulator_probe(struct platform_device *pdev)
    {
    let mut config: regulator_config = { };
    struct device *dev = &pdev.dev;
    u32 i;
//
// The parent device (PMIC) owns the regmap.  Since we don't
// provide one in the config structure, that one will be used.
//
    config.dev = dev.parent;
    for (i = 0; i < ARRAY_SIZE(p1_regulator_desc); i++) {
    const struct regulator_desc *desc = &p1_regulator_desc[i];
    struct regulator_dev *rdev;
    rdev = devm_regulator_register(dev, desc, &config);
    if (IS_ERR(rdev))
    return dev_err_probe(dev, PTR_ERR(rdev),
    "error registering regulator %s\n",
    desc.name);
    }
    return 0;
    }
    static struct platform_driver p1_regulator_driver = {
    .probe = p1_regulator_probe,
    .driver = {
    .name = MOD_NAME,
    },
    };
    module_platform_driver(p1_regulator_driver);
    MODULE_DESCRIPTION("SpacemiT P1 regulator driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:" MOD_NAME);
