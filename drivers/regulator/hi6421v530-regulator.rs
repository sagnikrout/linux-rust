//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/hi6421v530-regulator.c
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
// Device driver for regulators in Hi6421V530 IC
//
// Copyright (c) <2017> HiSilicon Technologies Co., Ltd.
// http://www.hisilicon.com
// Copyright (c) <2017> Linaro Ltd.
// https://www.linaro.org
//
// Author: Wang Xiaoyin <hw.wangxiaoyin@hisilicon.com>
// Guodong Xu <guodong.xu@linaro.org>

//
// struct hi6421v530_regulator_info - hi6421v530 regulator information
// @desc: regulator description
// @mode_mask: ECO mode bitmask of LDOs; for BUCKs, this masks sleep
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hi6421v530_regulator_info {
    pub rdesc: regulator_desc,
    pub mode_mask: u8,
}

// HI6421v530 regulators
    enum hi6421v530_regulator_id {
    HI6421V530_LDO3,
    HI6421V530_LDO9,
    HI6421V530_LDO11,
    HI6421V530_LDO15,
    HI6421V530_LDO16,
    };
    static const unsigned int ldo_3_voltages[] = {
    1800000, 1825000, 1850000, 1875000,
    1900000, 1925000, 1950000, 1975000,
    2000000, 2025000, 2050000, 2075000,
    2100000, 2125000, 2150000, 2200000,
    };
    static const unsigned int ldo_9_11_voltages[] = {
    1750000, 1800000, 1825000, 2800000,
    2850000, 2950000, 3000000, 3300000,
    };
    static const unsigned int ldo_15_16_voltages[] = {
    1750000, 1800000, 2400000, 2600000,
    2700000, 2850000, 2950000, 3000000,
    };
    static const struct regulator_ops hi6421v530_ldo_ops;

//
// _id - LDO id name string
// v_table - voltage table
// vreg - voltage select register
// vmask - voltage select mask
// ereg - enable register
// emask - enable mask
// odelay - off/on delay time in uS
// ecomask - eco mode mask
//

    odelay, ecomask) {					\
    .rdesc = {							\
    .name		 = #_ID,				\
    .of_match        = of_match_ptr(#_ID),			\
    .regulators_node = of_match_ptr("regulators"),		\
    .ops		 = &hi6421v530_ldo_ops,			\
    .type		 = REGULATOR_VOLTAGE,			\
    .id		 = HI6421V530_##_ID,			\
    .owner		 = THIS_MODULE,				\
    .n_voltages	 = ARRAY_SIZE(v_table),			\
    .volt_table	 = v_table,				\
    .vsel_reg	 = HI6421_REG_TO_BUS_ADDR(vreg),	\
    .vsel_mask	 = vmask,				\
    .enable_reg	 = HI6421_REG_TO_BUS_ADDR(ereg),	\
    .enable_mask	 = emask,				\
    .enable_time	 = HI6421V530_LDO_ENABLE_TIME,		\
    .off_on_delay	 = odelay,				\
    },								\
    .mode_mask	= ecomask,					\
    }
// HI6421V530 regulator information
    static const struct hi6421v530_regulator_info hi6421v530_regulator_info[] = {
    HI6421V530_LDO(LDO3, ldo_3_voltages, 0x061, 0xf, 0x060, 0x2,
    20000, 0x6),
    HI6421V530_LDO(LDO9, ldo_9_11_voltages, 0x06b, 0x7, 0x06a, 0x2,
    40000, 0x6),
    HI6421V530_LDO(LDO11, ldo_9_11_voltages, 0x06f, 0x7, 0x06e, 0x2,
    40000, 0x6),
    HI6421V530_LDO(LDO15, ldo_15_16_voltages, 0x077, 0x7, 0x076, 0x2,
    40000, 0x6),
    HI6421V530_LDO(LDO16, ldo_15_16_voltages, 0x079, 0x7, 0x078, 0x2,
    40000, 0x6),
    };
    static unsigned int hi6421v530_regulator_ldo_get_mode(
    struct regulator_dev *rdev)
    {
    const struct hi6421v530_regulator_info *info;
    unsigned int reg_val;
    info = container_of_const(rdev.desc, struct hi6421v530_regulator_info, rdesc);
    regmap_read(rdev.regmap, rdev.desc.enable_reg, &reg_val);
    if (reg_val & (info.mode_mask))
    return REGULATOR_MODE_IDLE;
    return REGULATOR_MODE_NORMAL;
    }
    static int hi6421v530_regulator_ldo_set_mode(struct regulator_dev *rdev,
    unsigned int mode)
    {
    const struct hi6421v530_regulator_info *info;
    unsigned int new_mode;
    info = container_of_const(rdev.desc, struct hi6421v530_regulator_info, rdesc);
    switch (mode) {
    case REGULATOR_MODE_NORMAL:
    new_mode = 0;
    break;
    case REGULATOR_MODE_IDLE:
    new_mode = info.mode_mask;
    break;
    default:
    return -EINVAL;
    }
    regmap_update_bits(rdev.regmap, rdev.desc.enable_reg,
    info.mode_mask, new_mode);
    return 0;
    }
    static const struct regulator_ops hi6421v530_ldo_ops = {
    .is_enabled = regulator_is_enabled_regmap,
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .list_voltage = regulator_list_voltage_table,
    .map_voltage = regulator_map_voltage_ascend,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .get_mode = hi6421v530_regulator_ldo_get_mode,
    .set_mode = hi6421v530_regulator_ldo_set_mode,
    };
#[no_mangle]
unsafe extern "C" fn hi6421v530_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int hi6421v530_regulator_probe(struct platform_device *pdev)
    {
    struct hi6421_pmic *pmic;
    struct regulator_dev *rdev;
    let mut config: regulator_config = { };
    unsigned int i;
    pmic = dev_get_drvdata(pdev.dev.parent);
    if (!pmic) {
    dev_err(&pdev.dev, "no pmic in the regulator parent node\n");
    return -ENODEV;
    }
    for (i = 0; i < ARRAY_SIZE(hi6421v530_regulator_info); i++) {
    config.dev = pdev.dev.parent;
    config.regmap = pmic.regmap;
    rdev = devm_regulator_register(&pdev.dev,
    &hi6421v530_regulator_info[i].rdesc,
    &config);
    if (IS_ERR(rdev)) {
    dev_err(&pdev.dev, "failed to register regulator %s\n",
    hi6421v530_regulator_info[i].rdesc.name);
    return PTR_ERR(rdev);
    }
    }
    return 0;
    }
    static const struct platform_device_id hi6421v530_regulator_table[] = {
    { .name = "hi6421v530-regulator" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, hi6421v530_regulator_table);
    static struct platform_driver hi6421v530_regulator_driver = {
    .id_table = hi6421v530_regulator_table,
    .driver = {
    .name	= "hi6421v530-regulator",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe	= hi6421v530_regulator_probe,
    };
    module_platform_driver(hi6421v530_regulator_driver);
    MODULE_AUTHOR("Wang Xiaoyin <hw.wangxiaoyin@hisilicon.com>");
    MODULE_DESCRIPTION("Hi6421v530 regulator driver");
    MODULE_LICENSE("GPL v2");
