//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/bcm590xx-regulator.c
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
// Broadcom BCM590xx regulator driver
//
// Copyright 2014 Linaro Limited
// Author: Matt Porter <mporter@linaro.org>
//

    enum bcm590xx_reg_type {
    BCM590XX_REG_TYPE_LDO,
    BCM590XX_REG_TYPE_GPLDO,
    BCM590XX_REG_TYPE_SR,
    BCM590XX_REG_TYPE_VBUS
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm590xx_reg_data {
    pub type: enum bcm590xx_reg_type,
    pub regmap: enum bcm590xx_regmap_type,
    pub desc: regulator_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm590xx_reg {
    pub mfd: *mut bcm590xx,
    pub n_regulators: c_uint,
    pub regs: *const bcm590xx_reg_data,
}

    static const struct regulator_ops bcm590xx_ops_ldo = {
    .is_enabled		= regulator_is_enabled_regmap,
    .enable			= regulator_enable_regmap,
    .disable		= regulator_disable_regmap,
    .get_voltage_sel	= regulator_get_voltage_sel_regmap,
    .set_voltage_sel	= regulator_set_voltage_sel_regmap,
    .list_voltage		= regulator_list_voltage_table,
    .map_voltage		= regulator_map_voltage_iterate,
    };
//
// LDO ops without voltage selection, used for MICLDO on BCM59054.
// (These are currently the same as VBUS ops, but will be different
// in the future once full PMMODE support is implemented.)
//
    static const struct regulator_ops bcm590xx_ops_ldo_novolt = {
    .is_enabled		= regulator_is_enabled_regmap,
    .enable			= regulator_enable_regmap,
    .disable		= regulator_disable_regmap,
    };
    static const struct regulator_ops bcm590xx_ops_dcdc = {
    .is_enabled		= regulator_is_enabled_regmap,
    .enable			= regulator_enable_regmap,
    .disable		= regulator_disable_regmap,
    .get_voltage_sel	= regulator_get_voltage_sel_regmap,
    .set_voltage_sel	= regulator_set_voltage_sel_regmap,
    .list_voltage		= regulator_list_voltage_linear_range,
    .map_voltage		= regulator_map_voltage_linear_range,
    };
    static const struct regulator_ops bcm590xx_ops_vbus = {
    .is_enabled		= regulator_is_enabled_regmap,
    .enable			= regulator_enable_regmap,
    .disable		= regulator_disable_regmap,
    };

    .id = _model##_REG_##_name,					\
    .name = #_name_lower,						\
    .of_match = of_match_ptr(#_name_lower),				\
    .regulators_node = of_match_ptr("regulators"),			\
    .type = REGULATOR_VOLTAGE,					\
    .owner = THIS_MODULE						\

    BCM590XX_REG_DESC(_model, _name, _name_lower),			\
    .ops = &bcm590xx_ops_ldo,					\
    .n_voltages = ARRAY_SIZE(_model_lower##_##_table),		\
    .volt_table = _model_lower##_##_table,				\
    .vsel_reg = _model##_##_name##CTRL,				\
    .vsel_mask = BCM590XX_LDO_VSEL_MASK,				\
    .enable_reg = _model##_##_name##PMCTRL1,			\
    .enable_mask = BCM590XX_REG_ENABLE,				\
    .enable_is_inverted = true

    BCM590XX_REG_DESC(_model, _name, _name_lower),			\
    .ops = &bcm590xx_ops_dcdc,					\
    .n_voltages = 64,						\
    .linear_ranges = _model_lower##_##_ranges,			\
    .n_linear_ranges = ARRAY_SIZE(_model_lower##_##_ranges),	\
    .vsel_reg = _model##_##_name##VOUT1,				\
    .vsel_mask = BCM590XX_SR_VSEL_MASK,				\
    .enable_reg = _model##_##_name##PMCTRL1,			\
    .enable_mask = BCM590XX_REG_ENABLE,				\
    .enable_is_inverted = true

    BCM590XX_REG_DESC(BCM59056, _name, _name_lower)

    BCM590XX_LDO_DESC(BCM59056, bcm59056, _name, _name_lower, _table)

    BCM590XX_SR_DESC(BCM59056, bcm59056, _name, _name_lower, _ranges)

    BCM590XX_REG_DESC(BCM59054, _name, _name_lower)

    BCM590XX_LDO_DESC(BCM59054, bcm59054, _name, _name_lower, _table)

    BCM590XX_SR_DESC(BCM59054, bcm59054, _name, _name_lower, _ranges)
// BCM59056 data
// I2C slave 0 registers
pub const BCM59056_RFLDOPMCTRL1: c_uint = 0x60;
pub const BCM59056_CAMLDO1PMCTRL1: c_uint = 0x62;
pub const BCM59056_CAMLDO2PMCTRL1: c_uint = 0x64;
pub const BCM59056_SIMLDO1PMCTRL1: c_uint = 0x66;
pub const BCM59056_SIMLDO2PMCTRL1: c_uint = 0x68;
pub const BCM59056_SDLDOPMCTRL1: c_uint = 0x6a;
pub const BCM59056_SDXLDOPMCTRL1: c_uint = 0x6c;
pub const BCM59056_MMCLDO1PMCTRL1: c_uint = 0x6e;
pub const BCM59056_MMCLDO2PMCTRL1: c_uint = 0x70;
pub const BCM59056_AUDLDOPMCTRL1: c_uint = 0x72;
pub const BCM59056_MICLDOPMCTRL1: c_uint = 0x74;
pub const BCM59056_USBLDOPMCTRL1: c_uint = 0x76;
pub const BCM59056_VIBLDOPMCTRL1: c_uint = 0x78;
pub const BCM59056_IOSR1PMCTRL1: c_uint = 0x7a;
pub const BCM59056_IOSR2PMCTRL1: c_uint = 0x7c;
pub const BCM59056_CSRPMCTRL1: c_uint = 0x7e;
pub const BCM59056_SDSR1PMCTRL1: c_uint = 0x82;
pub const BCM59056_SDSR2PMCTRL1: c_uint = 0x86;
pub const BCM59056_MSRPMCTRL1: c_uint = 0x8a;
pub const BCM59056_VSRPMCTRL1: c_uint = 0x8e;
pub const BCM59056_RFLDOCTRL: c_uint = 0x96;
pub const BCM59056_CAMLDO1CTRL: c_uint = 0x97;
pub const BCM59056_CAMLDO2CTRL: c_uint = 0x98;
pub const BCM59056_SIMLDO1CTRL: c_uint = 0x99;
pub const BCM59056_SIMLDO2CTRL: c_uint = 0x9a;
pub const BCM59056_SDLDOCTRL: c_uint = 0x9b;
pub const BCM59056_SDXLDOCTRL: c_uint = 0x9c;
pub const BCM59056_MMCLDO1CTRL: c_uint = 0x9d;
pub const BCM59056_MMCLDO2CTRL: c_uint = 0x9e;
pub const BCM59056_AUDLDOCTRL: c_uint = 0x9f;
pub const BCM59056_MICLDOCTRL: c_uint = 0xa0;
pub const BCM59056_USBLDOCTRL: c_uint = 0xa1;
pub const BCM59056_VIBLDOCTRL: c_uint = 0xa2;
pub const BCM59056_CSRVOUT1: c_uint = 0xc0;
pub const BCM59056_IOSR1VOUT1: c_uint = 0xc3;
pub const BCM59056_IOSR2VOUT1: c_uint = 0xc6;
pub const BCM59056_MSRVOUT1: c_uint = 0xc9;
pub const BCM59056_SDSR1VOUT1: c_uint = 0xcc;
pub const BCM59056_SDSR2VOUT1: c_uint = 0xcf;
pub const BCM59056_VSRVOUT1: c_uint = 0xd2;
// I2C slave 1 registers
pub const BCM59056_GPLDO5PMCTRL1: c_uint = 0x16;
pub const BCM59056_GPLDO6PMCTRL1: c_uint = 0x18;
pub const BCM59056_GPLDO1CTRL: c_uint = 0x1a;
pub const BCM59056_GPLDO2CTRL: c_uint = 0x1b;
pub const BCM59056_GPLDO3CTRL: c_uint = 0x1c;
pub const BCM59056_GPLDO4CTRL: c_uint = 0x1d;
pub const BCM59056_GPLDO5CTRL: c_uint = 0x1e;
pub const BCM59056_GPLDO6CTRL: c_uint = 0x1f;
pub const BCM59056_OTG_CTRL: c_uint = 0x40;
pub const BCM59056_GPLDO1PMCTRL1: c_uint = 0x57;
pub const BCM59056_GPLDO2PMCTRL1: c_uint = 0x59;
pub const BCM59056_GPLDO3PMCTRL1: c_uint = 0x5b;
pub const BCM59056_GPLDO4PMCTRL1: c_uint = 0x5d;
//
// RFLDO to VSR regulators are
// accessed via I2C slave 0
//
// LDO regulator IDs
pub const BCM59056_REG_RFLDO: c_int = 0;
pub const BCM59056_REG_CAMLDO1: c_int = 1;
pub const BCM59056_REG_CAMLDO2: c_int = 2;
pub const BCM59056_REG_SIMLDO1: c_int = 3;
pub const BCM59056_REG_SIMLDO2: c_int = 4;
pub const BCM59056_REG_SDLDO: c_int = 5;
pub const BCM59056_REG_SDXLDO: c_int = 6;
pub const BCM59056_REG_MMCLDO1: c_int = 7;
pub const BCM59056_REG_MMCLDO2: c_int = 8;
pub const BCM59056_REG_AUDLDO: c_int = 9;
pub const BCM59056_REG_MICLDO: c_int = 10;
pub const BCM59056_REG_USBLDO: c_int = 11;
pub const BCM59056_REG_VIBLDO: c_int = 12;
// DCDC regulator IDs
pub const BCM59056_REG_CSR: c_int = 13;
pub const BCM59056_REG_IOSR1: c_int = 14;
pub const BCM59056_REG_IOSR2: c_int = 15;
pub const BCM59056_REG_MSR: c_int = 16;
pub const BCM59056_REG_SDSR1: c_int = 17;
pub const BCM59056_REG_SDSR2: c_int = 18;
pub const BCM59056_REG_VSR: c_int = 19;
//
// GPLDO1 to VBUS regulators are
// accessed via I2C slave 1
//
pub const BCM59056_REG_GPLDO1: c_int = 20;
pub const BCM59056_REG_GPLDO2: c_int = 21;
pub const BCM59056_REG_GPLDO3: c_int = 22;
pub const BCM59056_REG_GPLDO4: c_int = 23;
pub const BCM59056_REG_GPLDO5: c_int = 24;
pub const BCM59056_REG_GPLDO6: c_int = 25;
pub const BCM59056_REG_VBUS: c_int = 26;
pub const BCM59056_NUM_REGS: c_int = 27;
// LDO group A: supported voltages in microvolts
    static const unsigned int bcm59056_ldo_a_table[] = {
    1200000, 1800000, 2500000, 2700000, 2800000,
    2900000, 3000000, 3300000,
    };
// LDO group C: supported voltages in microvolts
    static const unsigned int bcm59056_ldo_c_table[] = {
    3100000, 1800000, 2500000, 2700000, 2800000,
    2900000, 3000000, 3300000,
    };
// DCDC group CSR: supported voltages in microvolts
    static const struct linear_range bcm59056_dcdc_csr_ranges[] = {
    REGULATOR_LINEAR_RANGE(860000, 2, 50, 10000),
    REGULATOR_LINEAR_RANGE(1360000, 51, 55, 20000),
    REGULATOR_LINEAR_RANGE(900000, 56, 63, 0),
    };
// DCDC group IOSR1: supported voltages in microvolts
    static const struct linear_range bcm59056_dcdc_iosr1_ranges[] = {
    REGULATOR_LINEAR_RANGE(860000, 2, 51, 10000),
    REGULATOR_LINEAR_RANGE(1500000, 52, 52, 0),
    REGULATOR_LINEAR_RANGE(1800000, 53, 53, 0),
    REGULATOR_LINEAR_RANGE(900000, 54, 63, 0),
    };
// DCDC group SDSR1: supported voltages in microvolts
    static const struct linear_range bcm59056_dcdc_sdsr1_ranges[] = {
    REGULATOR_LINEAR_RANGE(860000, 2, 50, 10000),
    REGULATOR_LINEAR_RANGE(1340000, 51, 51, 0),
    REGULATOR_LINEAR_RANGE(900000, 52, 63, 0),
    };
    static const struct bcm590xx_reg_data bcm59056_regs[BCM59056_NUM_REGS] = {
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59056_LDO_DESC(RFLDO, rfldo, ldo_a_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59056_LDO_DESC(CAMLDO1, camldo1, ldo_c_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59056_LDO_DESC(CAMLDO2, camldo2, ldo_c_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59056_LDO_DESC(SIMLDO1, simldo1, ldo_a_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59056_LDO_DESC(SIMLDO2, simldo2, ldo_a_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59056_LDO_DESC(SDLDO, sdldo, ldo_c_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59056_LDO_DESC(SDXLDO, sdxldo, ldo_a_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59056_LDO_DESC(MMCLDO1, mmcldo1, ldo_a_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59056_LDO_DESC(MMCLDO2, mmcldo2, ldo_a_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59056_LDO_DESC(AUDLDO, audldo, ldo_a_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59056_LDO_DESC(MICLDO, micldo, ldo_a_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59056_LDO_DESC(USBLDO, usbldo, ldo_a_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59056_LDO_DESC(VIBLDO, vibldo, ldo_c_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_SR,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59056_SR_DESC(CSR, csr, dcdc_csr_ranges),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_SR,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59056_SR_DESC(IOSR1, iosr1, dcdc_iosr1_ranges),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_SR,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59056_SR_DESC(IOSR2, iosr2, dcdc_iosr1_ranges),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_SR,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59056_SR_DESC(MSR, msr, dcdc_iosr1_ranges),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_SR,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59056_SR_DESC(SDSR1, sdsr1, dcdc_sdsr1_ranges),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_SR,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59056_SR_DESC(SDSR2, sdsr2, dcdc_iosr1_ranges),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_SR,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59056_SR_DESC(VSR, vsr, dcdc_iosr1_ranges),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_GPLDO,
    .regmap = BCM590XX_REGMAP_SEC,
    .desc = {
    BCM59056_LDO_DESC(GPLDO1, gpldo1, ldo_a_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_GPLDO,
    .regmap = BCM590XX_REGMAP_SEC,
    .desc = {
    BCM59056_LDO_DESC(GPLDO2, gpldo2, ldo_a_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_GPLDO,
    .regmap = BCM590XX_REGMAP_SEC,
    .desc = {
    BCM59056_LDO_DESC(GPLDO3, gpldo3, ldo_a_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_GPLDO,
    .regmap = BCM590XX_REGMAP_SEC,
    .desc = {
    BCM59056_LDO_DESC(GPLDO4, gpldo4, ldo_a_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_GPLDO,
    .regmap = BCM590XX_REGMAP_SEC,
    .desc = {
    BCM59056_LDO_DESC(GPLDO5, gpldo5, ldo_a_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_GPLDO,
    .regmap = BCM590XX_REGMAP_SEC,
    .desc = {
    BCM59056_LDO_DESC(GPLDO6, gpldo6, ldo_a_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_VBUS,
    .regmap = BCM590XX_REGMAP_SEC,
    .desc = {
    BCM59056_REG_DESC(VBUS, vbus),
    .ops = &bcm590xx_ops_vbus,
    .n_voltages = 1,
    .fixed_uV = 5000000,
    .enable_reg = BCM59056_OTG_CTRL,
    .enable_mask = BCM590XX_VBUS_ENABLE,
    },
    },
    };
// BCM59054 data
// I2C slave 0 registers
pub const BCM59054_RFLDOPMCTRL1: c_uint = 0x60;
pub const BCM59054_CAMLDO1PMCTRL1: c_uint = 0x62;
pub const BCM59054_CAMLDO2PMCTRL1: c_uint = 0x64;
pub const BCM59054_SIMLDO1PMCTRL1: c_uint = 0x66;
pub const BCM59054_SIMLDO2PMCTRL1: c_uint = 0x68;
pub const BCM59054_SDLDOPMCTRL1: c_uint = 0x6a;
pub const BCM59054_SDXLDOPMCTRL1: c_uint = 0x6c;
pub const BCM59054_MMCLDO1PMCTRL1: c_uint = 0x6e;
pub const BCM59054_MMCLDO2PMCTRL1: c_uint = 0x70;
pub const BCM59054_AUDLDOPMCTRL1: c_uint = 0x72;
pub const BCM59054_MICLDOPMCTRL1: c_uint = 0x74;
pub const BCM59054_USBLDOPMCTRL1: c_uint = 0x76;
pub const BCM59054_VIBLDOPMCTRL1: c_uint = 0x78;
pub const BCM59054_IOSR1PMCTRL1: c_uint = 0x7a;
pub const BCM59054_IOSR2PMCTRL1: c_uint = 0x7c;
pub const BCM59054_CSRPMCTRL1: c_uint = 0x7e;
pub const BCM59054_SDSR1PMCTRL1: c_uint = 0x82;
pub const BCM59054_SDSR2PMCTRL1: c_uint = 0x86;
pub const BCM59054_MMSRPMCTRL1: c_uint = 0x8a;
pub const BCM59054_VSRPMCTRL1: c_uint = 0x8e;
pub const BCM59054_RFLDOCTRL: c_uint = 0x96;
pub const BCM59054_CAMLDO1CTRL: c_uint = 0x97;
pub const BCM59054_CAMLDO2CTRL: c_uint = 0x98;
pub const BCM59054_SIMLDO1CTRL: c_uint = 0x99;
pub const BCM59054_SIMLDO2CTRL: c_uint = 0x9a;
pub const BCM59054_SDLDOCTRL: c_uint = 0x9b;
pub const BCM59054_SDXLDOCTRL: c_uint = 0x9c;
pub const BCM59054_MMCLDO1CTRL: c_uint = 0x9d;
pub const BCM59054_MMCLDO2CTRL: c_uint = 0x9e;
pub const BCM59054_AUDLDOCTRL: c_uint = 0x9f;
pub const BCM59054_MICLDOCTRL: c_uint = 0xa0;
pub const BCM59054_USBLDOCTRL: c_uint = 0xa1;
pub const BCM59054_VIBLDOCTRL: c_uint = 0xa2;
pub const BCM59054_CSRVOUT1: c_uint = 0xc0;
pub const BCM59054_IOSR1VOUT1: c_uint = 0xc3;
pub const BCM59054_IOSR2VOUT1: c_uint = 0xc6;
pub const BCM59054_MMSRVOUT1: c_uint = 0xc9;
pub const BCM59054_SDSR1VOUT1: c_uint = 0xcc;
pub const BCM59054_SDSR2VOUT1: c_uint = 0xcf;
pub const BCM59054_VSRVOUT1: c_uint = 0xd2;
// I2C slave 1 registers
pub const BCM59054_LVLDO1PMCTRL1: c_uint = 0x16;
pub const BCM59054_LVLDO2PMCTRL1: c_uint = 0x18;
pub const BCM59054_GPLDO1CTRL: c_uint = 0x1a;
pub const BCM59054_GPLDO2CTRL: c_uint = 0x1b;
pub const BCM59054_GPLDO3CTRL: c_uint = 0x1c;
pub const BCM59054_TCXLDOCTRL: c_uint = 0x1d;
pub const BCM59054_LVLDO1CTRL: c_uint = 0x1e;
pub const BCM59054_LVLDO2CTRL: c_uint = 0x1f;
pub const BCM59054_OTG_CTRL: c_uint = 0x40;
pub const BCM59054_GPLDO1PMCTRL1: c_uint = 0x57;
pub const BCM59054_GPLDO2PMCTRL1: c_uint = 0x59;
pub const BCM59054_GPLDO3PMCTRL1: c_uint = 0x5b;
pub const BCM59054_TCXLDOPMCTRL1: c_uint = 0x5d;
//
// RFLDO to VSR regulators are
// accessed via I2C slave 0
//
// LDO regulator IDs
pub const BCM59054_REG_RFLDO: c_int = 0;
pub const BCM59054_REG_CAMLDO1: c_int = 1;
pub const BCM59054_REG_CAMLDO2: c_int = 2;
pub const BCM59054_REG_SIMLDO1: c_int = 3;
pub const BCM59054_REG_SIMLDO2: c_int = 4;
pub const BCM59054_REG_SDLDO: c_int = 5;
pub const BCM59054_REG_SDXLDO: c_int = 6;
pub const BCM59054_REG_MMCLDO1: c_int = 7;
pub const BCM59054_REG_MMCLDO2: c_int = 8;
pub const BCM59054_REG_AUDLDO: c_int = 9;
pub const BCM59054_REG_MICLDO: c_int = 10;
pub const BCM59054_REG_USBLDO: c_int = 11;
pub const BCM59054_REG_VIBLDO: c_int = 12;
// DCDC regulator IDs
pub const BCM59054_REG_CSR: c_int = 13;
pub const BCM59054_REG_IOSR1: c_int = 14;
pub const BCM59054_REG_IOSR2: c_int = 15;
pub const BCM59054_REG_MMSR: c_int = 16;
pub const BCM59054_REG_SDSR1: c_int = 17;
pub const BCM59054_REG_SDSR2: c_int = 18;
pub const BCM59054_REG_VSR: c_int = 19;
//
// GPLDO1 to VBUS regulators are
// accessed via I2C slave 1
//
pub const BCM59054_REG_GPLDO1: c_int = 20;
pub const BCM59054_REG_GPLDO2: c_int = 21;
pub const BCM59054_REG_GPLDO3: c_int = 22;
pub const BCM59054_REG_TCXLDO: c_int = 23;
pub const BCM59054_REG_LVLDO1: c_int = 24;
pub const BCM59054_REG_LVLDO2: c_int = 25;
pub const BCM59054_REG_VBUS: c_int = 26;
pub const BCM59054_NUM_REGS: c_int = 27;
// LDO group 1: supported voltages in microvolts
    static const unsigned int bcm59054_ldo_1_table[] = {
    1200000, 1800000, 2500000, 2700000, 2800000,
    2900000, 3000000, 3300000,
    };
// LDO group 2: supported voltages in microvolts
    static const unsigned int bcm59054_ldo_2_table[] = {
    3100000, 1800000, 2500000, 2700000, 2800000,
    2900000, 3000000, 3300000,
    };
// LDO group 3: supported voltages in microvolts
    static const unsigned int bcm59054_ldo_3_table[] = {
    1000000, 1107000, 1143000, 1214000, 1250000,
    1464000, 1500000, 1786000,
    };
// DCDC group SR: supported voltages in microvolts
    static const struct linear_range bcm59054_dcdc_sr_ranges[] = {
    REGULATOR_LINEAR_RANGE(0, 0, 1, 0),
    REGULATOR_LINEAR_RANGE(860000, 2, 60, 10000),
    REGULATOR_LINEAR_RANGE(1500000, 61, 61, 0),
    REGULATOR_LINEAR_RANGE(1800000, 62, 62, 0),
    REGULATOR_LINEAR_RANGE(900000, 63, 63, 0),
    };
// DCDC group VSR (BCM59054A1): supported voltages in microvolts
    static const struct linear_range bcm59054_dcdc_vsr_a1_ranges[] = {
    REGULATOR_LINEAR_RANGE(0, 0, 1, 0),
    REGULATOR_LINEAR_RANGE(860000, 2, 59, 10000),
    REGULATOR_LINEAR_RANGE(1700000, 60, 60, 0),
    REGULATOR_LINEAR_RANGE(1500000, 61, 61, 0),
    REGULATOR_LINEAR_RANGE(1800000, 62, 62, 0),
    REGULATOR_LINEAR_RANGE(1600000, 63, 63, 0),
    };
// DCDC group CSR: supported voltages in microvolts
    static const struct linear_range bcm59054_dcdc_csr_ranges[] = {
    REGULATOR_LINEAR_RANGE(700000, 0, 1, 100000),
    REGULATOR_LINEAR_RANGE(860000, 2, 60, 10000),
    REGULATOR_LINEAR_RANGE(900000, 61, 63, 0),
    };
    static const struct bcm590xx_reg_data bcm59054_regs[BCM59054_NUM_REGS] = {
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(RFLDO, rfldo, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(CAMLDO1, camldo1, ldo_2_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(CAMLDO2, camldo2, ldo_2_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(SIMLDO1, simldo1, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(SIMLDO2, simldo2, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(SDLDO, sdldo, ldo_2_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(SDXLDO, sdxldo, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(MMCLDO1, mmcldo1, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(MMCLDO2, mmcldo2, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(AUDLDO, audldo, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_REG_DESC(MICLDO, micldo),
    .ops = &bcm590xx_ops_ldo_novolt,
// MICLDO is locked at 1.8V
    .n_voltages = 1,
    .fixed_uV = 1800000,
    .enable_reg = BCM59054_MICLDOPMCTRL1,
    .enable_mask = BCM590XX_REG_ENABLE,
    .enable_is_inverted = true,
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(USBLDO, usbldo, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(VIBLDO, vibldo, ldo_2_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_SR,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_SR_DESC(CSR, csr, dcdc_csr_ranges),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_SR,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_SR_DESC(IOSR1, iosr1, dcdc_sr_ranges),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_SR,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_SR_DESC(IOSR2, iosr2, dcdc_sr_ranges),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_SR,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_SR_DESC(MMSR, mmsr, dcdc_sr_ranges),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_SR,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_SR_DESC(SDSR1, sdsr1, dcdc_sr_ranges),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_SR,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_SR_DESC(SDSR2, sdsr2, dcdc_sr_ranges),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_SR,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_SR_DESC(VSR, vsr, dcdc_sr_ranges),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_GPLDO,
    .regmap = BCM590XX_REGMAP_SEC,
    .desc = {
    BCM59054_LDO_DESC(GPLDO1, gpldo1, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_GPLDO,
    .regmap = BCM590XX_REGMAP_SEC,
    .desc = {
    BCM59054_LDO_DESC(GPLDO2, gpldo2, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_GPLDO,
    .regmap = BCM590XX_REGMAP_SEC,
    .desc = {
    BCM59054_LDO_DESC(GPLDO3, gpldo3, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_GPLDO,
    .regmap = BCM590XX_REGMAP_SEC,
    .desc = {
    BCM59054_LDO_DESC(TCXLDO, tcxldo, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_GPLDO,
    .regmap = BCM590XX_REGMAP_SEC,
    .desc = {
    BCM59054_LDO_DESC(LVLDO1, lvldo1, ldo_3_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_GPLDO,
    .regmap = BCM590XX_REGMAP_SEC,
    .desc = {
    BCM59054_LDO_DESC(LVLDO2, lvldo2, ldo_3_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_VBUS,
    .regmap = BCM590XX_REGMAP_SEC,
    .desc = {
    BCM59054_REG_DESC(VBUS, vbus),
    .ops = &bcm590xx_ops_vbus,
    .n_voltages = 1,
    .fixed_uV = 5000000,
    .enable_reg = BCM59054_OTG_CTRL,
    .enable_mask = BCM590XX_VBUS_ENABLE,
    },
    },
    };
//
// BCM59054A1 regulators; same as previous revision, but with different
// VSR voltage table.
//
    static const struct bcm590xx_reg_data bcm59054_a1_regs[BCM59054_NUM_REGS] = {
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(RFLDO, rfldo, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(CAMLDO1, camldo1, ldo_2_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(CAMLDO2, camldo2, ldo_2_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(SIMLDO1, simldo1, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(SIMLDO2, simldo2, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(SDLDO, sdldo, ldo_2_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(SDXLDO, sdxldo, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(MMCLDO1, mmcldo1, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(MMCLDO2, mmcldo2, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(AUDLDO, audldo, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_REG_DESC(MICLDO, micldo),
    .ops = &bcm590xx_ops_ldo_novolt,
// MICLDO is locked at 1.8V
    .n_voltages = 1,
    .fixed_uV = 1800000,
    .enable_reg = BCM59054_MICLDOPMCTRL1,
    .enable_mask = BCM590XX_REG_ENABLE,
    .enable_is_inverted = true,
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(USBLDO, usbldo, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_LDO,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_LDO_DESC(VIBLDO, vibldo, ldo_2_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_SR,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_SR_DESC(CSR, csr, dcdc_csr_ranges),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_SR,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_SR_DESC(IOSR1, iosr1, dcdc_sr_ranges),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_SR,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_SR_DESC(IOSR2, iosr2, dcdc_sr_ranges),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_SR,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_SR_DESC(MMSR, mmsr, dcdc_sr_ranges),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_SR,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_SR_DESC(SDSR1, sdsr1, dcdc_sr_ranges),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_SR,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_SR_DESC(SDSR2, sdsr2, dcdc_sr_ranges),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_SR,
    .regmap = BCM590XX_REGMAP_PRI,
    .desc = {
    BCM59054_SR_DESC(VSR, vsr, dcdc_vsr_a1_ranges),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_GPLDO,
    .regmap = BCM590XX_REGMAP_SEC,
    .desc = {
    BCM59054_LDO_DESC(GPLDO1, gpldo1, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_GPLDO,
    .regmap = BCM590XX_REGMAP_SEC,
    .desc = {
    BCM59054_LDO_DESC(GPLDO2, gpldo2, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_GPLDO,
    .regmap = BCM590XX_REGMAP_SEC,
    .desc = {
    BCM59054_LDO_DESC(GPLDO3, gpldo3, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_GPLDO,
    .regmap = BCM590XX_REGMAP_SEC,
    .desc = {
    BCM59054_LDO_DESC(TCXLDO, tcxldo, ldo_1_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_GPLDO,
    .regmap = BCM590XX_REGMAP_SEC,
    .desc = {
    BCM59054_LDO_DESC(LVLDO1, lvldo1, ldo_3_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_GPLDO,
    .regmap = BCM590XX_REGMAP_SEC,
    .desc = {
    BCM59054_LDO_DESC(LVLDO2, lvldo2, ldo_3_table),
    },
    },
    {
    .type = BCM590XX_REG_TYPE_VBUS,
    .regmap = BCM590XX_REGMAP_SEC,
    .desc = {
    BCM59054_REG_DESC(VBUS, vbus),
    .ops = &bcm590xx_ops_vbus,
    .n_voltages = 1,
    .fixed_uV = 5000000,
    .enable_reg = BCM59054_OTG_CTRL,
    .enable_mask = BCM590XX_VBUS_ENABLE,
    },
    },
    };
#[no_mangle]
unsafe extern "C" fn bcm590xx_probe(pdev: *mut platform_device) -> c_int {
    static int bcm590xx_probe(struct platform_device *pdev)
    {
    struct bcm590xx *bcm590xx = dev_get_drvdata(pdev.dev.parent);
    struct bcm590xx_reg *pmu;
    const struct bcm590xx_reg_data *info;
    let mut config: regulator_config = { };
    struct regulator_dev *rdev;
    unsigned int i;
    pmu = devm_kzalloc(&pdev.dev, sizeof(*pmu), GFP_KERNEL);
    if (!pmu)
    return -ENOMEM;
    pmu.mfd = bcm590xx;
    switch (pmu.mfd.pmu_id) {
    case BCM590XX_PMUID_BCM59054:
    pmu.n_regulators = BCM59054_NUM_REGS;
    if (pmu.mfd.rev_analog == BCM59054_REV_ANALOG_A1)
    pmu.regs = bcm59054_a1_regs;
    else
    pmu.regs = bcm59054_regs;
    break;
    case BCM590XX_PMUID_BCM59056:
    pmu.n_regulators = BCM59056_NUM_REGS;
    pmu.regs = bcm59056_regs;
    break;
    default:
    dev_err(bcm590xx.dev,
    "unknown device type, could not initialize\n");
    return -EINVAL;
    }
    platform_set_drvdata(pdev, pmu);
// Register the regulators
    for (i = 0; i < pmu.n_regulators; i++) {
    info = &pmu.regs[i];
    config.dev = bcm590xx.dev;
    config.driver_data = pmu;
    switch (info.regmap) {
    case BCM590XX_REGMAP_PRI:
    config.regmap = bcm590xx.regmap_pri;
    break;
    case BCM590XX_REGMAP_SEC:
    config.regmap = bcm590xx.regmap_sec;
    break;
    default:
    dev_err(bcm590xx.dev,
    "invalid regmap for %s regulator; this is a driver bug\n",
    pdev.name);
    return -EINVAL;
    }
    rdev = devm_regulator_register(&pdev.dev, &info.desc,
    &config);
    if (IS_ERR(rdev))
    return dev_err_probe(bcm590xx.dev, PTR_ERR(rdev),
    "failed to register %s regulator\n",
    pdev.name);
    }
    return 0;
    }
    static struct platform_driver bcm590xx_regulator_driver = {
    .driver = {
    .name = "bcm590xx-vregs",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe = bcm590xx_probe,
    };
    module_platform_driver(bcm590xx_regulator_driver);
    MODULE_AUTHOR("Matt Porter <mporter@linaro.org>");
    MODULE_DESCRIPTION("BCM590xx voltage regulator driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:bcm590xx-vregs");
