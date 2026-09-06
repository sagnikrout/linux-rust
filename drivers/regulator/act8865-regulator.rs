//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/act8865-regulator.c
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
// act8865-regulator.c - Voltage regulation for active-semi ACT88xx PMUs
//
// http://www.active-semi.com/products/power-management-units/act88xx
//
// Copyright (C) 2013 Atmel Corporation
//

//
// ACT8600 Global Register Map.
//
pub const ACT8600_SYS_MODE: c_uint = 0x00;
pub const ACT8600_SYS_CTRL: c_uint = 0x01;
pub const ACT8600_DCDC1_VSET: c_uint = 0x10;
pub const ACT8600_DCDC1_CTRL: c_uint = 0x12;
pub const ACT8600_DCDC2_VSET: c_uint = 0x20;
pub const ACT8600_DCDC2_CTRL: c_uint = 0x22;
pub const ACT8600_DCDC3_VSET: c_uint = 0x30;
pub const ACT8600_DCDC3_CTRL: c_uint = 0x32;
pub const ACT8600_SUDCDC4_VSET: c_uint = 0x40;
pub const ACT8600_SUDCDC4_CTRL: c_uint = 0x41;
pub const ACT8600_LDO5_VSET: c_uint = 0x50;
pub const ACT8600_LDO5_CTRL: c_uint = 0x51;
pub const ACT8600_LDO6_VSET: c_uint = 0x60;
pub const ACT8600_LDO6_CTRL: c_uint = 0x61;
pub const ACT8600_LDO7_VSET: c_uint = 0x70;
pub const ACT8600_LDO7_CTRL: c_uint = 0x71;
pub const ACT8600_LDO8_VSET: c_uint = 0x80;
pub const ACT8600_LDO8_CTRL: c_uint = 0x81;
pub const ACT8600_LDO910_CTRL: c_uint = 0x91;
pub const ACT8600_APCH0: c_uint = 0xA1;
pub const ACT8600_APCH1: c_uint = 0xA8;
pub const ACT8600_APCH2: c_uint = 0xA9;
pub const ACT8600_APCH_STAT: c_uint = 0xAA;
pub const ACT8600_OTG0: c_uint = 0xB0;
pub const ACT8600_OTG1: c_uint = 0xB2;
//
// ACT8846 Global Register Map.
//
pub const ACT8846_SYS0: c_uint = 0x00;
pub const ACT8846_SYS1: c_uint = 0x01;
pub const ACT8846_REG1_VSET: c_uint = 0x10;
pub const ACT8846_REG1_CTRL: c_uint = 0x12;
pub const ACT8846_REG2_VSET0: c_uint = 0x20;
pub const ACT8846_REG2_VSET1: c_uint = 0x21;
pub const ACT8846_REG2_CTRL: c_uint = 0x22;
pub const ACT8846_REG3_VSET0: c_uint = 0x30;
pub const ACT8846_REG3_VSET1: c_uint = 0x31;
pub const ACT8846_REG3_CTRL: c_uint = 0x32;
pub const ACT8846_REG4_VSET0: c_uint = 0x40;
pub const ACT8846_REG4_VSET1: c_uint = 0x41;
pub const ACT8846_REG4_CTRL: c_uint = 0x42;
pub const ACT8846_REG5_VSET: c_uint = 0x50;
pub const ACT8846_REG5_CTRL: c_uint = 0x51;
pub const ACT8846_REG6_VSET: c_uint = 0x58;
pub const ACT8846_REG6_CTRL: c_uint = 0x59;
pub const ACT8846_REG7_VSET: c_uint = 0x60;
pub const ACT8846_REG7_CTRL: c_uint = 0x61;
pub const ACT8846_REG8_VSET: c_uint = 0x68;
pub const ACT8846_REG8_CTRL: c_uint = 0x69;
pub const ACT8846_REG9_VSET: c_uint = 0x70;
pub const ACT8846_REG9_CTRL: c_uint = 0x71;
pub const ACT8846_REG10_VSET: c_uint = 0x80;
pub const ACT8846_REG10_CTRL: c_uint = 0x81;
pub const ACT8846_REG11_VSET: c_uint = 0x90;
pub const ACT8846_REG11_CTRL: c_uint = 0x91;
pub const ACT8846_REG12_VSET: c_uint = 0xa0;
pub const ACT8846_REG12_CTRL: c_uint = 0xa1;
pub const ACT8846_REG13_CTRL: c_uint = 0xb1;
pub const ACT8846_GLB_OFF_CTRL: c_uint = 0xc3;
pub const ACT8846_OFF_SYSMASK: c_uint = 0x18;
//
// ACT8865 Global Register Map.
//
pub const ACT8865_SYS_MODE: c_uint = 0x00;
pub const ACT8865_SYS_CTRL: c_uint = 0x01;
pub const ACT8865_SYS_UNLK_REGS: c_uint = 0x0b;
pub const ACT8865_DCDC1_VSET1: c_uint = 0x20;
pub const ACT8865_DCDC1_VSET2: c_uint = 0x21;
pub const ACT8865_DCDC1_CTRL: c_uint = 0x22;
pub const ACT8865_DCDC1_SUS: c_uint = 0x24;
pub const ACT8865_DCDC2_VSET1: c_uint = 0x30;
pub const ACT8865_DCDC2_VSET2: c_uint = 0x31;
pub const ACT8865_DCDC2_CTRL: c_uint = 0x32;
pub const ACT8865_DCDC2_SUS: c_uint = 0x34;
pub const ACT8865_DCDC3_VSET1: c_uint = 0x40;
pub const ACT8865_DCDC3_VSET2: c_uint = 0x41;
pub const ACT8865_DCDC3_CTRL: c_uint = 0x42;
pub const ACT8865_DCDC3_SUS: c_uint = 0x44;
pub const ACT8865_LDO1_VSET: c_uint = 0x50;
pub const ACT8865_LDO1_CTRL: c_uint = 0x51;
pub const ACT8865_LDO1_SUS: c_uint = 0x52;
pub const ACT8865_LDO2_VSET: c_uint = 0x54;
pub const ACT8865_LDO2_CTRL: c_uint = 0x55;
pub const ACT8865_LDO2_SUS: c_uint = 0x56;
pub const ACT8865_LDO3_VSET: c_uint = 0x60;
pub const ACT8865_LDO3_CTRL: c_uint = 0x61;
pub const ACT8865_LDO3_SUS: c_uint = 0x62;
pub const ACT8865_LDO4_VSET: c_uint = 0x64;
pub const ACT8865_LDO4_CTRL: c_uint = 0x65;
pub const ACT8865_LDO4_SUS: c_uint = 0x66;
pub const ACT8865_MSTROFF: c_uint = 0x20;
//
// Field Definitions.
//
pub const ACT8865_ENA: c_uint = 0x80	/* ON - [7] */;
pub const ACT8865_DIS: c_uint = 0x40	/* DIS - [6] */;
pub const ACT8865_VSEL_MASK: c_uint = 0x3F	/* VSET - [5:0] */;
pub const ACT8600_LDO10_ENA: c_uint = 0x40	/* ON - [6] */;
pub const ACT8600_SUDCDC_VSEL_MASK: c_uint = 0xFF	/* SUDCDC VSET - [7:0] */;

//
// ACT8865 voltage number
//
pub const ACT8865_VOLTAGE_NUM: c_int = 64;
pub const ACT8600_SUDCDC_VOLTAGE_NUM: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct act8865 {
    pub regmap: *mut regmap,
    pub off_reg: c_int,
    pub off_mask: c_int,
}

    static const struct regmap_range act8600_reg_ranges[] = {
    regmap_reg_range(0x00, 0x01),
    regmap_reg_range(0x10, 0x10),
    regmap_reg_range(0x12, 0x12),
    regmap_reg_range(0x20, 0x20),
    regmap_reg_range(0x22, 0x22),
    regmap_reg_range(0x30, 0x30),
    regmap_reg_range(0x32, 0x32),
    regmap_reg_range(0x40, 0x41),
    regmap_reg_range(0x50, 0x51),
    regmap_reg_range(0x60, 0x61),
    regmap_reg_range(0x70, 0x71),
    regmap_reg_range(0x80, 0x81),
    regmap_reg_range(0x91, 0x91),
    regmap_reg_range(0xA1, 0xA1),
    regmap_reg_range(0xA8, 0xAA),
    regmap_reg_range(0xB0, 0xB0),
    regmap_reg_range(0xB2, 0xB2),
    regmap_reg_range(0xC1, 0xC1),
    };
    static const struct regmap_range act8600_reg_ro_ranges[] = {
    regmap_reg_range(0xAA, 0xAA),
    regmap_reg_range(0xC1, 0xC1),
    };
    static const struct regmap_range act8600_reg_volatile_ranges[] = {
    regmap_reg_range(0x00, 0x01),
    regmap_reg_range(0x12, 0x12),
    regmap_reg_range(0x22, 0x22),
    regmap_reg_range(0x32, 0x32),
    regmap_reg_range(0x41, 0x41),
    regmap_reg_range(0x51, 0x51),
    regmap_reg_range(0x61, 0x61),
    regmap_reg_range(0x71, 0x71),
    regmap_reg_range(0x81, 0x81),
    regmap_reg_range(0xA8, 0xA8),
    regmap_reg_range(0xAA, 0xAA),
    regmap_reg_range(0xB0, 0xB0),
    regmap_reg_range(0xC1, 0xC1),
    };
    static const struct regmap_access_table act8600_write_ranges_table = {
    .yes_ranges	= act8600_reg_ranges,
    .n_yes_ranges	= ARRAY_SIZE(act8600_reg_ranges),
    .no_ranges	= act8600_reg_ro_ranges,
    .n_no_ranges	= ARRAY_SIZE(act8600_reg_ro_ranges),
    };
    static const struct regmap_access_table act8600_read_ranges_table = {
    .yes_ranges	= act8600_reg_ranges,
    .n_yes_ranges	= ARRAY_SIZE(act8600_reg_ranges),
    };
    static const struct regmap_access_table act8600_volatile_ranges_table = {
    .yes_ranges	= act8600_reg_volatile_ranges,
    .n_yes_ranges	= ARRAY_SIZE(act8600_reg_volatile_ranges),
    };
    static const struct regmap_config act8600_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0xFF,
    .wr_table = &act8600_write_ranges_table,
    .rd_table = &act8600_read_ranges_table,
    .volatile_table = &act8600_volatile_ranges_table,
    };
    static const struct regmap_config act8865_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    };
    static const struct linear_range act8865_voltage_ranges[] = {
    REGULATOR_LINEAR_RANGE(600000, 0, 23, 25000),
    REGULATOR_LINEAR_RANGE(1200000, 24, 47, 50000),
    REGULATOR_LINEAR_RANGE(2400000, 48, 63, 100000),
    };
    static const struct linear_range act8600_sudcdc_voltage_ranges[] = {
    REGULATOR_LINEAR_RANGE(3000000, 0, 63, 0),
    REGULATOR_LINEAR_RANGE(3000000, 64, 159, 100000),
    REGULATOR_LINEAR_RANGE(12600000, 160, 191, 200000),
    REGULATOR_LINEAR_RANGE(19000000, 192, 247, 400000),
    REGULATOR_LINEAR_RANGE(41400000, 248, 255, 0),
    };
#[no_mangle]
unsafe extern "C" fn act8865_set_suspend_state(rdev: *mut regulator_dev, enable: bool) -> c_int {
    static int act8865_set_suspend_state(struct regulator_dev *rdev, bool enable)
    {
    struct regmap *regmap = rdev.regmap;
    let mut id: c_int = rdev.desc.id, reg, val;
    switch (id) {
    case ACT8865_ID_DCDC1:
    reg = ACT8865_DCDC1_SUS;
    val = 0xa8;
    break;
    case ACT8865_ID_DCDC2:
    reg = ACT8865_DCDC2_SUS;
    val = 0xa8;
    break;
    case ACT8865_ID_DCDC3:
    reg = ACT8865_DCDC3_SUS;
    val = 0xa8;
    break;
    case ACT8865_ID_LDO1:
    reg = ACT8865_LDO1_SUS;
    val = 0xe8;
    break;
    case ACT8865_ID_LDO2:
    reg = ACT8865_LDO2_SUS;
    val = 0xe8;
    break;
    case ACT8865_ID_LDO3:
    reg = ACT8865_LDO3_SUS;
    val = 0xe8;
    break;
    case ACT8865_ID_LDO4:
    reg = ACT8865_LDO4_SUS;
    val = 0xe8;
    break;
    default:
    return -EINVAL;
    }
    if (enable)
    val |= BIT(4);
//
// Ask the PMIC to enable/disable this output when entering hibernate
// mode.
//
    return regmap_write(regmap, reg, val);
    }
#[no_mangle]
unsafe extern "C" fn act8865_set_suspend_enable(rdev: *mut regulator_dev) -> c_int {
    static int act8865_set_suspend_enable(struct regulator_dev *rdev)
    {
    return act8865_set_suspend_state(rdev, true);
    }
#[no_mangle]
unsafe extern "C" fn act8865_set_suspend_disable(rdev: *mut regulator_dev) -> c_int {
    static int act8865_set_suspend_disable(struct regulator_dev *rdev)
    {
    return act8865_set_suspend_state(rdev, false);
    }
#[no_mangle]
unsafe extern "C" fn act8865_of_map_mode(mode: c_uint) -> c_uint {
    static unsigned int act8865_of_map_mode(unsigned int mode)
    {
    switch (mode) {
    case ACT8865_REGULATOR_MODE_FIXED:
    return REGULATOR_MODE_FAST;
    case ACT8865_REGULATOR_MODE_NORMAL:
    return REGULATOR_MODE_NORMAL;
    case ACT8865_REGULATOR_MODE_LOWPOWER:
    return REGULATOR_MODE_STANDBY;
    default:
    return REGULATOR_MODE_INVALID;
    }
    }
#[no_mangle]
unsafe extern "C" fn act8865_set_mode(rdev: *mut regulator_dev, mode: c_uint) -> c_int {
    static int act8865_set_mode(struct regulator_dev *rdev, unsigned int mode)
    {
    struct regmap *regmap = rdev.regmap;
    let mut id: c_int = rdev_get_id(rdev);
    int reg, val = 0;
    switch (id) {
    case ACT8865_ID_DCDC1:
    reg = ACT8865_DCDC1_CTRL;
    break;
    case ACT8865_ID_DCDC2:
    reg = ACT8865_DCDC2_CTRL;
    break;
    case ACT8865_ID_DCDC3:
    reg = ACT8865_DCDC3_CTRL;
    break;
    case ACT8865_ID_LDO1:
    reg = ACT8865_LDO1_CTRL;
    break;
    case ACT8865_ID_LDO2:
    reg = ACT8865_LDO2_CTRL;
    break;
    case ACT8865_ID_LDO3:
    reg = ACT8865_LDO3_CTRL;
    break;
    case ACT8865_ID_LDO4:
    reg = ACT8865_LDO4_CTRL;
    break;
    default:
    return -EINVAL;
    }
    switch (mode) {
    case REGULATOR_MODE_FAST:
    case REGULATOR_MODE_NORMAL:
    if (id <= ACT8865_ID_DCDC3)
    val = BIT(5);
    break;
    case REGULATOR_MODE_STANDBY:
    if (id > ACT8865_ID_DCDC3)
    val = BIT(5);
    break;
    default:
    return -EINVAL;
    }
    return regmap_update_bits(regmap, reg, BIT(5), val);
    }
#[no_mangle]
unsafe extern "C" fn act8865_get_mode(rdev: *mut regulator_dev) -> c_uint {
    static unsigned int act8865_get_mode(struct regulator_dev *rdev)
    {
    struct regmap *regmap = rdev.regmap;
    let mut id: c_int = rdev_get_id(rdev);
    int reg, ret, val = 0;
    switch (id) {
    case ACT8865_ID_DCDC1:
    reg = ACT8865_DCDC1_CTRL;
    break;
    case ACT8865_ID_DCDC2:
    reg = ACT8865_DCDC2_CTRL;
    break;
    case ACT8865_ID_DCDC3:
    reg = ACT8865_DCDC3_CTRL;
    break;
    case ACT8865_ID_LDO1:
    reg = ACT8865_LDO1_CTRL;
    break;
    case ACT8865_ID_LDO2:
    reg = ACT8865_LDO2_CTRL;
    break;
    case ACT8865_ID_LDO3:
    reg = ACT8865_LDO3_CTRL;
    break;
    case ACT8865_ID_LDO4:
    reg = ACT8865_LDO4_CTRL;
    break;
    default:
    return -EINVAL;
    }
    ret = regmap_read(regmap, reg, &val);
    if (ret)
    return ret;
    if (id <= ACT8865_ID_DCDC3 && (val & BIT(5)))
    return REGULATOR_MODE_FAST;
#[no_mangle]
pub unsafe extern "C" fn if(BIT(5)): id > ACT8865_ID_DCDC3 && !(val &) -> else {
    else if	(id > ACT8865_ID_DCDC3 && !(val & BIT(5)))
    return REGULATOR_MODE_NORMAL;
    else
    return REGULATOR_MODE_STANDBY;
    }
    static const struct regulator_ops act8865_ops = {
    .list_voltage		= regulator_list_voltage_linear_range,
    .map_voltage		= regulator_map_voltage_linear_range,
    .get_voltage_sel	= regulator_get_voltage_sel_regmap,
    .set_voltage_sel	= regulator_set_voltage_sel_regmap,
    .enable			= regulator_enable_regmap,
    .disable		= regulator_disable_regmap,
    .set_mode		= act8865_set_mode,
    .get_mode		= act8865_get_mode,
    .is_enabled		= regulator_is_enabled_regmap,
    .set_suspend_enable	= act8865_set_suspend_enable,
    .set_suspend_disable	= act8865_set_suspend_disable,
    };
    static const struct regulator_ops act8865_ldo_ops = {
    .list_voltage		= regulator_list_voltage_linear_range,
    .map_voltage		= regulator_map_voltage_linear_range,
    .get_voltage_sel	= regulator_get_voltage_sel_regmap,
    .set_voltage_sel	= regulator_set_voltage_sel_regmap,
    .enable			= regulator_enable_regmap,
    .disable		= regulator_disable_regmap,
    .set_mode		= act8865_set_mode,
    .get_mode		= act8865_get_mode,
    .is_enabled		= regulator_is_enabled_regmap,
    .set_suspend_enable	= act8865_set_suspend_enable,
    .set_suspend_disable	= act8865_set_suspend_disable,
    .set_pull_down		= regulator_set_pull_down_regmap,
    };
    static const struct regulator_ops act8865_fixed_ldo_ops = {
    .enable			= regulator_enable_regmap,
    .disable		= regulator_disable_regmap,
    .is_enabled		= regulator_is_enabled_regmap,
    };

    [_family##_ID_##_id] = {					\
    .name			= _name,			\
    .of_match		= of_match_ptr(_name),		\
    .of_map_mode		= act8865_of_map_mode,		\
    .regulators_node	= of_match_ptr("regulators"),	\
    .supply_name		= _supply,			\
    .id			= _family##_ID_##_id,		\
    .type			= REGULATOR_VOLTAGE,		\
    .ops			= _ops,				\
    .n_voltages		= ACT8865_VOLTAGE_NUM,		\
    .linear_ranges		= act8865_voltage_ranges,	\
    .n_linear_ranges	= ARRAY_SIZE(act8865_voltage_ranges), \
    .vsel_reg		= _family##_##_id##_##_vsel_reg, \
    .vsel_mask		= ACT8865_VSEL_MASK,		\
    .enable_reg		= _family##_##_id##_CTRL,	\
    .enable_mask		= ACT8865_ENA,			\
    .pull_down_reg		= _family##_##_id##_CTRL,	\
    .pull_down_mask		= ACT8865_DIS,			\
    .owner			= THIS_MODULE,			\
    }

    ACT88xx_REG_(_name, _family, _id, _vsel_reg, _supply, &act8865_ops)

    ACT88xx_REG_(_name, _family, _id, _vsel_reg, _supply, &act8865_ldo_ops)
    static const struct regulator_desc act8600_regulators[] = {
    ACT88xx_REG("DCDC1", ACT8600, DCDC1, VSET, "vp1"),
    ACT88xx_REG("DCDC2", ACT8600, DCDC2, VSET, "vp2"),
    ACT88xx_REG("DCDC3", ACT8600, DCDC3, VSET, "vp3"),
    {
    .name = "SUDCDC_REG4",
    .of_match = of_match_ptr("SUDCDC_REG4"),
    .regulators_node = of_match_ptr("regulators"),
    .id = ACT8600_ID_SUDCDC4,
    .ops = &act8865_ops,
    .type = REGULATOR_VOLTAGE,
    .n_voltages = ACT8600_SUDCDC_VOLTAGE_NUM,
    .linear_ranges = act8600_sudcdc_voltage_ranges,
    .n_linear_ranges = ARRAY_SIZE(act8600_sudcdc_voltage_ranges),
    .vsel_reg = ACT8600_SUDCDC4_VSET,
    .vsel_mask = ACT8600_SUDCDC_VSEL_MASK,
    .enable_reg = ACT8600_SUDCDC4_CTRL,
    .enable_mask = ACT8865_ENA,
    .owner = THIS_MODULE,
    },
    ACT88xx_REG("LDO5", ACT8600, LDO5, VSET, "inl"),
    ACT88xx_REG("LDO6", ACT8600, LDO6, VSET, "inl"),
    ACT88xx_REG("LDO7", ACT8600, LDO7, VSET, "inl"),
    ACT88xx_REG("LDO8", ACT8600, LDO8, VSET, "inl"),
    {
    .name = "LDO_REG9",
    .of_match = of_match_ptr("LDO_REG9"),
    .regulators_node = of_match_ptr("regulators"),
    .id = ACT8600_ID_LDO9,
    .ops = &act8865_fixed_ldo_ops,
    .type = REGULATOR_VOLTAGE,
    .n_voltages = 1,
    .fixed_uV = 3300000,
    .enable_reg = ACT8600_LDO910_CTRL,
    .enable_mask = ACT8865_ENA,
    .owner = THIS_MODULE,
    },
    {
    .name = "LDO_REG10",
    .of_match = of_match_ptr("LDO_REG10"),
    .regulators_node = of_match_ptr("regulators"),
    .id = ACT8600_ID_LDO10,
    .ops = &act8865_fixed_ldo_ops,
    .type = REGULATOR_VOLTAGE,
    .n_voltages = 1,
    .fixed_uV = 1200000,
    .enable_reg = ACT8600_LDO910_CTRL,
    .enable_mask = ACT8600_LDO10_ENA,
    .owner = THIS_MODULE,
    },
    };
    static const struct regulator_desc act8846_regulators[] = {
    ACT88xx_REG("REG1", ACT8846, REG1, VSET, "vp1"),
    ACT88xx_REG("REG2", ACT8846, REG2, VSET0, "vp2"),
    ACT88xx_REG("REG3", ACT8846, REG3, VSET0, "vp3"),
    ACT88xx_REG("REG4", ACT8846, REG4, VSET0, "vp4"),
    ACT88xx_REG("REG5", ACT8846, REG5, VSET, "inl1"),
    ACT88xx_REG("REG6", ACT8846, REG6, VSET, "inl1"),
    ACT88xx_REG("REG7", ACT8846, REG7, VSET, "inl1"),
    ACT88xx_REG("REG8", ACT8846, REG8, VSET, "inl2"),
    ACT88xx_REG("REG9", ACT8846, REG9, VSET, "inl2"),
    ACT88xx_REG("REG10", ACT8846, REG10, VSET, "inl3"),
    ACT88xx_REG("REG11", ACT8846, REG11, VSET, "inl3"),
    ACT88xx_REG("REG12", ACT8846, REG12, VSET, "inl3"),
    };
    static const struct regulator_desc act8865_regulators[] = {
    ACT88xx_REG("DCDC_REG1", ACT8865, DCDC1, VSET1, "vp1"),
    ACT88xx_REG("DCDC_REG2", ACT8865, DCDC2, VSET1, "vp2"),
    ACT88xx_REG("DCDC_REG3", ACT8865, DCDC3, VSET1, "vp3"),
    ACT88xx_LDO("LDO_REG1", ACT8865, LDO1, VSET, "inl45"),
    ACT88xx_LDO("LDO_REG2", ACT8865, LDO2, VSET, "inl45"),
    ACT88xx_LDO("LDO_REG3", ACT8865, LDO3, VSET, "inl67"),
    ACT88xx_LDO("LDO_REG4", ACT8865, LDO4, VSET, "inl67"),
    };
    static const struct regulator_desc act8865_alt_regulators[] = {
    ACT88xx_REG("DCDC_REG1", ACT8865, DCDC1, VSET2, "vp1"),
    ACT88xx_REG("DCDC_REG2", ACT8865, DCDC2, VSET2, "vp2"),
    ACT88xx_REG("DCDC_REG3", ACT8865, DCDC3, VSET2, "vp3"),
    ACT88xx_LDO("LDO_REG1", ACT8865, LDO1, VSET, "inl45"),
    ACT88xx_LDO("LDO_REG2", ACT8865, LDO2, VSET, "inl45"),
    ACT88xx_LDO("LDO_REG3", ACT8865, LDO3, VSET, "inl67"),
    ACT88xx_LDO("LDO_REG4", ACT8865, LDO4, VSET, "inl67"),
    };

    static const struct of_device_id act8865_dt_ids[] = {
    { .compatible = "active-semi,act8600", .data = (void *)ACT8600 },
    { .compatible = "active-semi,act8846", .data = (void *)ACT8846 },
    { .compatible = "active-semi,act8865", .data = (void *)ACT8865 },
    { }
    };
    MODULE_DEVICE_TABLE(of, act8865_dt_ids);

    static struct act8865_regulator_data *act8865_get_regulator_data(
    int id, struct act8865_platform_data *pdata)
    {
    int i;
    for (i = 0; i < pdata.num_regulators; i++) {
    if (pdata.regulators[i].id == id)
    return &pdata.regulators[i];
    }
    return core::ptr::null_mut();
    }
    static struct i2c_client *act8865_i2c_client;
#[no_mangle]
unsafe extern "C" fn act8865_power_off() {
    static void act8865_power_off(void)
    {
    struct act8865 *act8865;
    act8865 = i2c_get_clientdata(act8865_i2c_client);
    regmap_write(act8865.regmap, act8865.off_reg, act8865.off_mask);
    while (1);
    }
#[no_mangle]
unsafe extern "C" fn act8600_charger_get_status(map: *mut regmap) -> c_int {
    static int act8600_charger_get_status(struct regmap *map)
    {
    unsigned int val;
    int ret;
    u8 state0, state1;
    ret = regmap_read(map, ACT8600_APCH_STAT, &val);
    if (ret < 0)
    return ret;
    state0 = val & ACT8600_APCH_CSTATE0;
    state1 = val & ACT8600_APCH_CSTATE1;
    if (state0 && !state1)
    return POWER_SUPPLY_STATUS_CHARGING;
    if (!state0 && state1)
    return POWER_SUPPLY_STATUS_NOT_CHARGING;
    if (!state0 && !state1)
    return POWER_SUPPLY_STATUS_DISCHARGING;
    return POWER_SUPPLY_STATUS_UNKNOWN;
    }
    static int act8600_charger_get_property(struct power_supply *psy,
    enum power_supply_property psp, union power_supply_propval *val)
    {
    struct regmap *map = power_supply_get_drvdata(psy);
    int ret;
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    ret = act8600_charger_get_status(map);
    if (ret < 0)
    return ret;
    val.intval = ret;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static enum power_supply_property act8600_charger_properties[] = {
    POWER_SUPPLY_PROP_STATUS,
    };
    static const struct power_supply_desc act8600_charger_desc = {
    .name = "act8600-charger",
    .type = POWER_SUPPLY_TYPE_BATTERY,
    .properties = act8600_charger_properties,
    .num_properties = ARRAY_SIZE(act8600_charger_properties),
    .get_property = act8600_charger_get_property,
    };
#[no_mangle]
unsafe extern "C" fn act8600_charger_probe(dev: *mut device, regmap: *mut regmap) -> c_int {
    static int act8600_charger_probe(struct device *dev, struct regmap *regmap)
    {
    struct power_supply *charger;
    struct power_supply_config cfg = {
    .drv_data = regmap,
    .fwnode = dev_fwnode(dev),
    };
    charger = devm_power_supply_register(dev, &act8600_charger_desc, &cfg);
    return PTR_ERR_OR_ZERO(charger);
    }
#[no_mangle]
unsafe extern "C" fn act8865_pmic_probe(client: *mut i2c_client) -> c_int {
    static int act8865_pmic_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *i2c_id = i2c_client_get_device_id(client);
    const struct regulator_desc *regulators;
    struct act8865_platform_data *pdata = core::ptr::null_mut();
    struct device *dev = &client.dev;
    int i, ret, num_regulators;
    struct act8865 *act8865;
    const struct regmap_config *regmap_config;
    unsigned long type;
    int off_reg, off_mask;
    let mut voltage_select: c_int = 0;
    if (dev.of_node) {
    const struct of_device_id *id;
    id = of_match_device(of_match_ptr(act8865_dt_ids), dev);
    if (!id)
    return -ENODEV;
    type = (unsigned long) id.data;
    voltage_select = of_property_read_bool(dev.of_node, "active-semi,vsel-high");
    } else {
    type = i2c_id.driver_data;
    pdata = dev_get_platdata(dev);
    }
    switch (type) {
    case ACT8600:
    regulators = act8600_regulators;
    num_regulators = ARRAY_SIZE(act8600_regulators);
    regmap_config = &act8600_regmap_config;
    off_reg = -1;
    off_mask = -1;
    break;
    case ACT8846:
    regulators = act8846_regulators;
    num_regulators = ARRAY_SIZE(act8846_regulators);
    regmap_config = &act8865_regmap_config;
    off_reg = ACT8846_GLB_OFF_CTRL;
    off_mask = ACT8846_OFF_SYSMASK;
    break;
    case ACT8865:
    if (voltage_select) {
    regulators = act8865_alt_regulators;
    num_regulators = ARRAY_SIZE(act8865_alt_regulators);
    } else {
    regulators = act8865_regulators;
    num_regulators = ARRAY_SIZE(act8865_regulators);
    }
    regmap_config = &act8865_regmap_config;
    off_reg = ACT8865_SYS_CTRL;
    off_mask = ACT8865_MSTROFF;
    break;
    default:
    dev_err(dev, "invalid device id %lu\n", type);
    return -EINVAL;
    }
    act8865 = devm_kzalloc(dev, sizeof(struct act8865), GFP_KERNEL);
    if (!act8865)
    return -ENOMEM;
    act8865.regmap = devm_regmap_init_i2c(client, regmap_config);
    if (IS_ERR(act8865.regmap)) {
    ret = PTR_ERR(act8865.regmap);
    dev_err(dev, "Failed to allocate register map: %d\n", ret);
    return ret;
    }
    if (of_device_is_system_power_controller(dev.of_node)) {
    if (!pm_power_off && (off_reg > 0)) {
    act8865_i2c_client = client;
    act8865.off_reg = off_reg;
    act8865.off_mask = off_mask;
    pm_power_off = act8865_power_off;
    } else {
    dev_err(dev, "Failed to set poweroff capability, already defined\n");
    }
    }
// Finally register devices
    for (i = 0; i < num_regulators; i++) {
    const struct regulator_desc *desc = &regulators[i];
    let mut config: regulator_config = { };
    struct regulator_dev *rdev;
    config.dev = dev;
    config.driver_data = act8865;
    config.regmap = act8865.regmap;
    if (pdata) {
    struct act8865_regulator_data *rdata;
    rdata = act8865_get_regulator_data(desc.id, pdata);
    if (rdata) {
    config.init_data = rdata.init_data;
    config.of_node = rdata.of_node;
    }
    }
    rdev = devm_regulator_register(dev, desc, &config);
    if (IS_ERR(rdev)) {
    dev_err(dev, "failed to register %s\n", desc.name);
    return PTR_ERR(rdev);
    }
    }
    if (type == ACT8600) {
    ret = act8600_charger_probe(dev, act8865.regmap);
    if (ret < 0) {
    if (ret != -EPROBE_DEFER)
    dev_err(dev, "Failed to probe charger");
    return ret;
    }
    }
    i2c_set_clientdata(client, act8865);
// Unlock expert registers for ACT8865.
    return type != ACT8865 ? 0 : regmap_write(act8865.regmap,
    ACT8865_SYS_UNLK_REGS, 0xef);
    }
    static const struct i2c_device_id act8865_ids[] = {
    { .name = "act8600", .driver_data = ACT8600 },
    { .name = "act8846", .driver_data = ACT8846 },
    { .name = "act8865", .driver_data = ACT8865 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, act8865_ids);
    static struct i2c_driver act8865_pmic_driver = {
    .driver	= {
    .name	= "act8865",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe		= act8865_pmic_probe,
    .id_table	= act8865_ids,
    };
    module_i2c_driver(act8865_pmic_driver);
    MODULE_DESCRIPTION("active-semi act88xx voltage regulator driver");
    MODULE_AUTHOR("Wenyou Yang <wenyou.yang@atmel.com>");
    MODULE_LICENSE("GPL v2");
