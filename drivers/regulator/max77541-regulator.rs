//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/max77541-regulator.c
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
// Copyright (c) 2022 Analog Devices, Inc.
// ADI Regulator driver for the MAX77540 and MAX77541
//

    static const struct regulator_ops max77541_buck_ops = {
    .enable			= regulator_enable_regmap,
    .disable		= regulator_disable_regmap,
    .is_enabled		= regulator_is_enabled_regmap,
    .list_voltage		= regulator_list_voltage_pickable_linear_range,
    .get_voltage_sel	= regulator_get_voltage_sel_pickable_regmap,
    .set_voltage_sel	= regulator_set_voltage_sel_pickable_regmap,
    };
    static const struct linear_range max77540_buck_ranges[] = {
// Ranges when VOLT_SEL bits are 0x00
    REGULATOR_LINEAR_RANGE(500000, 0x00, 0x8B, 5000),
    REGULATOR_LINEAR_RANGE(1200000, 0x8C, 0xFF, 0),
// Ranges when VOLT_SEL bits are 0x40
    REGULATOR_LINEAR_RANGE(1200000, 0x00, 0x8B, 10000),
    REGULATOR_LINEAR_RANGE(2400000, 0x8C, 0xFF, 0),
// Ranges when VOLT_SEL bits are  0x80
    REGULATOR_LINEAR_RANGE(2000000, 0x00, 0x9F, 20000),
    REGULATOR_LINEAR_RANGE(5200000, 0xA0, 0xFF, 0),
    };
    static const struct linear_range max77541_buck_ranges[] = {
// Ranges when VOLT_SEL bits are 0x00
    REGULATOR_LINEAR_RANGE(300000, 0x00, 0xB3, 5000),
    REGULATOR_LINEAR_RANGE(1200000, 0xB4, 0xFF, 0),
// Ranges when VOLT_SEL bits are 0x40
    REGULATOR_LINEAR_RANGE(1200000, 0x00, 0x8B, 10000),
    REGULATOR_LINEAR_RANGE(2400000, 0x8C, 0xFF, 0),
// Ranges when VOLT_SEL bits are  0x80
    REGULATOR_LINEAR_RANGE(2000000, 0x00, 0x9F, 20000),
    REGULATOR_LINEAR_RANGE(5200000, 0xA0, 0xFF, 0),
    };
    static const unsigned int max77541_buck_volt_range_sel[] = {
    0x0, 0x0, 0x1, 0x1, 0x2, 0x2,
    };
    enum max77541_regulators {
    MAX77541_BUCK1 = 1,
    MAX77541_BUCK2,
    };

    {	.id = MAX77541_BUCK ## _id,				\
    .name = "buck"#_id,					\
    .of_match = "buck"#_id,					\
    .regulators_node = "regulators",			\
    .enable_reg = MAX77541_REG_EN_CTRL,			\
    .enable_mask = MAX77541_BIT_M ## _id ## _EN,		\
    .ops = &(_ops),						\
    .type = REGULATOR_VOLTAGE,				\
    .linear_ranges = max77540_buck_ranges,			\
    .n_linear_ranges = ARRAY_SIZE(max77540_buck_ranges),	\
    .vsel_reg = MAX77541_REG_M ## _id ## _VOUT,		\
    .vsel_mask = MAX77541_BITS_MX_VOUT,			\
    .vsel_range_reg = MAX77541_REG_M ## _id ## _CFG1,	\
    .vsel_range_mask = MAX77541_BITS_MX_CFG1_RNG,		\
    .linear_range_selectors_bitfield = max77541_buck_volt_range_sel, \
    .owner = THIS_MODULE,					\
    }

    {	.id = MAX77541_BUCK ## _id,				\
    .name = "buck"#_id,					\
    .of_match = "buck"#_id,					\
    .regulators_node = "regulators",			\
    .enable_reg = MAX77541_REG_EN_CTRL,			\
    .enable_mask = MAX77541_BIT_M ## _id ## _EN,		\
    .ops = &(_ops),						\
    .type = REGULATOR_VOLTAGE,				\
    .linear_ranges = max77541_buck_ranges,			\
    .n_linear_ranges = ARRAY_SIZE(max77541_buck_ranges),	\
    .vsel_reg = MAX77541_REG_M ## _id ## _VOUT,		\
    .vsel_mask = MAX77541_BITS_MX_VOUT,			\
    .vsel_range_reg = MAX77541_REG_M ## _id ## _CFG1,	\
    .vsel_range_mask = MAX77541_BITS_MX_CFG1_RNG,		\
    .linear_range_selectors_bitfield = max77541_buck_volt_range_sel, \
    .owner = THIS_MODULE,					\
    }
    static const struct regulator_desc max77540_regulators_desc[] = {
    MAX77540_BUCK(1, max77541_buck_ops),
    MAX77540_BUCK(2, max77541_buck_ops),
    };
    static const struct regulator_desc max77541_regulators_desc[] = {
    MAX77541_BUCK(1, max77541_buck_ops),
    MAX77541_BUCK(2, max77541_buck_ops),
    };
#[no_mangle]
unsafe extern "C" fn max77541_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int max77541_regulator_probe(struct platform_device *pdev)
    {
    let mut config: regulator_config = {};
    const struct regulator_desc *desc;
    struct device *dev = &pdev.dev;
    struct regulator_dev *rdev;
    struct max77541 *max77541 = dev_get_drvdata(dev.parent);
    unsigned int i;
    config.dev = dev.parent;
    switch (max77541.id) {
    case MAX77540:
    desc = max77540_regulators_desc;
    break;
    case MAX77541:
    desc = max77541_regulators_desc;
    break;
    default:
    return -EINVAL;
    }
    for (i = 0; i < MAX77541_MAX_REGULATORS; i++) {
    rdev = devm_regulator_register(dev, &desc[i], &config);
    if (IS_ERR(rdev))
    return dev_err_probe(dev, PTR_ERR(rdev),
    "Failed to register regulator\n");
    }
    return 0;
    }
    static const struct platform_device_id max77541_regulator_platform_id[] = {
    { .name = "max77540-regulator" },
    { .name = "max77541-regulator" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, max77541_regulator_platform_id);
    static struct platform_driver max77541_regulator_driver = {
    .driver = {
    .name = "max77541-regulator",
    },
    .probe = max77541_regulator_probe,
    .id_table = max77541_regulator_platform_id,
    };
    module_platform_driver(max77541_regulator_driver);
    MODULE_AUTHOR("Okan Sahin <Okan.Sahin@analog.com>");
    MODULE_DESCRIPTION("MAX77540/MAX77541 regulator driver");
    MODULE_LICENSE("GPL");
