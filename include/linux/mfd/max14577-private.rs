//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/max14577-private.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// max14577-private.h - Common API for the Maxim 14577/77836 internal sub chip
//
// Copyright (C) 2014 Samsung Electronics
// Chanwoo Choi <cw00.choi@samsung.com>
// Krzysztof Kozlowski <krzk@kernel.org>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum maxim_device_type {
    MAXIM_DEVICE_TYPE_UNKNOWN	= 0,
    MAXIM_DEVICE_TYPE_MAX14577,
    MAXIM_DEVICE_TYPE_MAX77836,

    MAXIM_DEVICE_TYPE_NUM,
}

// Slave addr = 0x4A: MUIC and Charger
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max14577_reg {
    MAX14577_REG_DEVICEID		= 0x00,
    MAX14577_REG_INT1		= 0x01,
    MAX14577_REG_INT2		= 0x02,
    MAX14577_REG_INT3		= 0x03,
    MAX14577_REG_STATUS1		= 0x04,
    MAX14577_REG_STATUS2		= 0x05,
    MAX14577_REG_STATUS3		= 0x06,
    MAX14577_REG_INTMASK1		= 0x07,
    MAX14577_REG_INTMASK2		= 0x08,
    MAX14577_REG_INTMASK3		= 0x09,
    MAX14577_REG_CDETCTRL1		= 0x0A,
    MAX14577_REG_RFU		= 0x0B,
    MAX14577_REG_CONTROL1		= 0x0C,
    MAX14577_REG_CONTROL2		= 0x0D,
    MAX14577_REG_CONTROL3		= 0x0E,
    MAX14577_REG_CHGCTRL1		= 0x0F,
    MAX14577_REG_CHGCTRL2		= 0x10,
    MAX14577_REG_CHGCTRL3		= 0x11,
    MAX14577_REG_CHGCTRL4		= 0x12,
    MAX14577_REG_CHGCTRL5		= 0x13,
    MAX14577_REG_CHGCTRL6		= 0x14,
    MAX14577_REG_CHGCTRL7		= 0x15,

    MAX14577_REG_END,
}

// Slave addr = 0x4A: MUIC
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max14577_muic_reg {
    MAX14577_MUIC_REG_STATUS1	= 0x04,
    MAX14577_MUIC_REG_STATUS2	= 0x05,
    MAX14577_MUIC_REG_CONTROL1	= 0x0C,
    MAX14577_MUIC_REG_CONTROL3	= 0x0E,

    MAX14577_MUIC_REG_END,
}

//
// Combined charger types for max14577 and max77836.
//
// On max14577 three lower bits map to STATUS2/CHGTYP field.
// However the max77836 has different two last values of STATUS2/CHGTYP.
// To indicate the difference enum has two additional values for max77836.
// These values are just a register value bitwise OR with 0x8.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max14577_muic_charger_type {
    MAX14577_CHARGER_TYPE_NONE		= 0x0,
    MAX14577_CHARGER_TYPE_USB		= 0x1,
    MAX14577_CHARGER_TYPE_DOWNSTREAM_PORT	= 0x2,
    MAX14577_CHARGER_TYPE_DEDICATED_CHG	= 0x3,
    MAX14577_CHARGER_TYPE_SPECIAL_500MA	= 0x4,
// Special 1A or 2A charger
    MAX14577_CHARGER_TYPE_SPECIAL_1A	= 0x5,
// max14577: reserved, used on max77836
    MAX14577_CHARGER_TYPE_RESERVED		= 0x6,
// max14577: dead-battery charing with maximum current 100mA
    MAX14577_CHARGER_TYPE_DEAD_BATTERY	= 0x7,
//
// max77836: special charger (bias on D+/D-),
// matches register value of 0x6
//
    MAX77836_CHARGER_TYPE_SPECIAL_BIAS	= 0xe,
// max77836: reserved, register value 0x7
    MAX77836_CHARGER_TYPE_RESERVED		= 0xf,
}

// MAX14577 interrupts

// MAX14577 DEVICE ID register
pub const DEVID_VENDORID_SHIFT: c_int = 0;
pub const DEVID_DEVICEID_SHIFT: c_int = 3;

// MAX14577 STATUS1 register
pub const STATUS1_ADC_SHIFT: c_int = 0;
pub const STATUS1_ADCLOW_SHIFT: c_int = 5;
pub const STATUS1_ADCERR_SHIFT: c_int = 6;
pub const MAX77836_STATUS1_ADC1K_SHIFT: c_int = 7;

// MAX14577 STATUS2 register
pub const STATUS2_CHGTYP_SHIFT: c_int = 0;
pub const STATUS2_CHGDETRUN_SHIFT: c_int = 3;
pub const STATUS2_DCDTMR_SHIFT: c_int = 4;
pub const MAX14577_STATUS2_DBCHG_SHIFT: c_int = 5;
pub const MAX77836_STATUS2_DXOVP_SHIFT: c_int = 5;
pub const STATUS2_VBVOLT_SHIFT: c_int = 6;
pub const MAX77836_STATUS2_VIDRM_SHIFT: c_int = 7;

// MAX14577 CONTROL1 register
pub const COMN1SW_SHIFT: c_int = 0;
pub const COMP2SW_SHIFT: c_int = 3;
pub const MICEN_SHIFT: c_int = 6;
pub const IDBEN_SHIFT: c_int = 7;

// MAX14577 CONTROL2 register

// MAX14577 CONTROL3 register
pub const CTRL3_JIGSET_SHIFT: c_int = 0;
pub const CTRL3_BOOTSET_SHIFT: c_int = 2;
pub const CTRL3_ADCDBSET_SHIFT: c_int = 4;
pub const CTRL3_WBTH_SHIFT: c_int = 6;

// Slave addr = 0x4A: Charger
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max14577_charger_reg {
    MAX14577_CHG_REG_STATUS3	= 0x06,
    MAX14577_CHG_REG_CHG_CTRL1	= 0x0F,
    MAX14577_CHG_REG_CHG_CTRL2	= 0x10,
    MAX14577_CHG_REG_CHG_CTRL3	= 0x11,
    MAX14577_CHG_REG_CHG_CTRL4	= 0x12,
    MAX14577_CHG_REG_CHG_CTRL5	= 0x13,
    MAX14577_CHG_REG_CHG_CTRL6	= 0x14,
    MAX14577_CHG_REG_CHG_CTRL7	= 0x15,

    MAX14577_CHG_REG_END,
}

// MAX14577 STATUS3 register
pub const STATUS3_EOC_SHIFT: c_int = 0;
pub const STATUS3_CGMBC_SHIFT: c_int = 1;
pub const STATUS3_OVP_SHIFT: c_int = 2;
pub const STATUS3_MBCCHGERR_SHIFT: c_int = 3;

// MAX14577 CDETCTRL1 register
pub const CDETCTRL1_CHGDETEN_SHIFT: c_int = 0;
pub const CDETCTRL1_CHGTYPMAN_SHIFT: c_int = 1;
pub const CDETCTRL1_DCDEN_SHIFT: c_int = 2;
pub const CDETCTRL1_DCD2SCT_SHIFT: c_int = 3;
pub const MAX14577_CDETCTRL1_DCHKTM_SHIFT: c_int = 4;
pub const MAX77836_CDETCTRL1_CDLY_SHIFT: c_int = 4;
pub const MAX14577_CDETCTRL1_DBEXIT_SHIFT: c_int = 5;
pub const MAX77836_CDETCTRL1_DCDCPL_SHIFT: c_int = 5;
pub const CDETCTRL1_DBIDLE_SHIFT: c_int = 6;
pub const CDETCTRL1_CDPDET_SHIFT: c_int = 7;

// MAX14577 CHGCTRL1 register
pub const CHGCTRL1_TCHW_SHIFT: c_int = 4;

// MAX14577 CHGCTRL2 register
pub const CHGCTRL2_MBCHOSTEN_SHIFT: c_int = 6;

pub const CHGCTRL2_VCHGR_RC_SHIFT: c_int = 7;

// MAX14577 CHGCTRL3 register
pub const CHGCTRL3_MBCCVWRC_SHIFT: c_int = 0;

// MAX14577 CHGCTRL4 register
pub const CHGCTRL4_MBCICHWRCH_SHIFT: c_int = 0;

pub const CHGCTRL4_MBCICHWRCL_SHIFT: c_int = 4;

// MAX14577 CHGCTRL5 register
pub const CHGCTRL5_EOCS_SHIFT: c_int = 0;

// MAX14577 CHGCTRL6 register
pub const CHGCTRL6_AUTOSTOP_SHIFT: c_int = 5;

// MAX14577 CHGCTRL7 register
pub const CHGCTRL7_OTPCGHCVS_SHIFT: c_int = 0;

// MAX14577 charger current limits (as in CHGCTRL4 register), uA

// MAX77836 charger current limits (as in CHGCTRL4 register), uA

//
// MAX14577 charger End-Of-Charge current limits
// (as in CHGCTRL5 register), uA
//

//
// MAX14577/MAX77836 Battery Constant Voltage
// (as in CHGCTRL3 register), uV
//

// Default value for fast charge timer, in hours
pub const MAXIM_CHARGER_FAST_CHARGE_TIMER_DEFAULT: c_int = 5;
// MAX14577 regulator SFOUT LDO voltage, fixed, uV
pub const MAX14577_REGULATOR_SAFEOUT_VOLTAGE: c_int = 4900000;
// MAX77836 regulator LDOx voltage, uV
pub const MAX77836_REGULATOR_LDO_VOLTAGE_MIN: c_int = 800000;
pub const MAX77836_REGULATOR_LDO_VOLTAGE_MAX: c_int = 3950000;
pub const MAX77836_REGULATOR_LDO_VOLTAGE_STEP: c_int = 50000;
pub const MAX77836_REGULATOR_LDO_VOLTAGE_STEPS_NUM: c_int = 64;
// Slave addr = 0x46: PMIC
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77836_pmic_reg {
    MAX77836_PMIC_REG_PMIC_ID		= 0x20,
    MAX77836_PMIC_REG_PMIC_REV		= 0x21,
    MAX77836_PMIC_REG_INTSRC		= 0x22,
    MAX77836_PMIC_REG_INTSRC_MASK		= 0x23,
    MAX77836_PMIC_REG_TOPSYS_INT		= 0x24,
    MAX77836_PMIC_REG_TOPSYS_INT_MASK	= 0x26,
    MAX77836_PMIC_REG_TOPSYS_STAT		= 0x28,
    MAX77836_PMIC_REG_MRSTB_CNTL		= 0x2A,
    MAX77836_PMIC_REG_LSCNFG		= 0x2B,

    MAX77836_LDO_REG_CNFG1_LDO1		= 0x51,
    MAX77836_LDO_REG_CNFG2_LDO1		= 0x52,
    MAX77836_LDO_REG_CNFG1_LDO2		= 0x53,
    MAX77836_LDO_REG_CNFG2_LDO2		= 0x54,
    MAX77836_LDO_REG_CNFG_LDO_BIAS		= 0x55,

    MAX77836_COMP_REG_COMP1			= 0x60,

    MAX77836_PMIC_REG_END,
}

pub const MAX77836_INTSRC_MASK_TOP_INT_SHIFT: c_int = 1;
pub const MAX77836_INTSRC_MASK_MUIC_CHG_INT_SHIFT: c_int = 3;

// MAX77836 PMIC interrupts
pub const MAX77836_TOPSYS_INT_T120C_SHIFT: c_int = 0;
pub const MAX77836_TOPSYS_INT_T140C_SHIFT: c_int = 1;

// LDO1/LDO2 CONFIG1 register
pub const MAX77836_CNFG1_LDO_PWRMD_SHIFT: c_int = 6;
pub const MAX77836_CNFG1_LDO_TV_SHIFT: c_int = 0;

// LDO1/LDO2 CONFIG2 register
pub const MAX77836_CNFG2_LDO_OVCLMPEN_SHIFT: c_int = 7;
pub const MAX77836_CNFG2_LDO_ALPMEN_SHIFT: c_int = 6;
pub const MAX77836_CNFG2_LDO_COMP_SHIFT: c_int = 4;
pub const MAX77836_CNFG2_LDO_POK_SHIFT: c_int = 3;
pub const MAX77836_CNFG2_LDO_ADE_SHIFT: c_int = 1;
pub const MAX77836_CNFG2_LDO_SS_SHIFT: c_int = 0;

// Slave addr = 0x6C: Fuel-Gauge/Battery
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77836_fg_reg {
    MAX77836_FG_REG_VCELL_MSB	= 0x02,
    MAX77836_FG_REG_VCELL_LSB	= 0x03,
    MAX77836_FG_REG_SOC_MSB		= 0x04,
    MAX77836_FG_REG_SOC_LSB		= 0x05,
    MAX77836_FG_REG_MODE_H		= 0x06,
    MAX77836_FG_REG_MODE_L		= 0x07,
    MAX77836_FG_REG_VERSION_MSB	= 0x08,
    MAX77836_FG_REG_VERSION_LSB	= 0x09,
    MAX77836_FG_REG_HIBRT_H		= 0x0A,
    MAX77836_FG_REG_HIBRT_L		= 0x0B,
    MAX77836_FG_REG_CONFIG_H	= 0x0C,
    MAX77836_FG_REG_CONFIG_L	= 0x0D,
    MAX77836_FG_REG_VALRT_MIN	= 0x14,
    MAX77836_FG_REG_VALRT_MAX	= 0x15,
    MAX77836_FG_REG_CRATE_MSB	= 0x16,
    MAX77836_FG_REG_CRATE_LSB	= 0x17,
    MAX77836_FG_REG_VRESET		= 0x18,
    MAX77836_FG_REG_FGID		= 0x19,
    MAX77836_FG_REG_STATUS_H	= 0x1A,
    MAX77836_FG_REG_STATUS_L	= 0x1B,
//
// TODO: TABLE registers
// TODO: CMD register
//

    MAX77836_FG_REG_END,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max14577_irq {
// INT1
    MAX14577_IRQ_INT1_ADC,
    MAX14577_IRQ_INT1_ADCLOW,
    MAX14577_IRQ_INT1_ADCERR,
    MAX77836_IRQ_INT1_ADC1K,

// INT2
    MAX14577_IRQ_INT2_CHGTYP,
    MAX14577_IRQ_INT2_CHGDETRUN,
    MAX14577_IRQ_INT2_DCDTMR,
    MAX14577_IRQ_INT2_DBCHG,
    MAX14577_IRQ_INT2_VBVOLT,
    MAX77836_IRQ_INT2_VIDRM,

// INT3
    MAX14577_IRQ_INT3_EOC,
    MAX14577_IRQ_INT3_CGMBC,
    MAX14577_IRQ_INT3_OVP,
    MAX14577_IRQ_INT3_MBCCHGERR,

// TOPSYS_INT, only MAX77836
    MAX77836_IRQ_TOPSYS_T140C,
    MAX77836_IRQ_TOPSYS_T120C,

    MAX14577_IRQ_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max14577 {
    pub dev: *mut device,
    pub /: *mut *mut *mut i2c_client i2c; / Slave addr = 0x4A,
    pub /: *mut *mut *mut i2c_client i2c_pmic; / Slave addr = 0x46,
    pub dev_type: maxim_device_type,
    pub /: *mut *mut *mut regmap regmap; / For MUIC and Charger,
    pub regmap_pmic: *mut regmap,
    pub /: *mut *mut *mut regmap_irq_chip_data irq_data; / For MUIC and Charger,
    pub irq_data_pmic: *mut regmap_irq_chip_data,
    pub irq: c_int,
}

// MAX14577 shared regmap API function
// dest = val;
extern "C" {
    pub fn regmap_bulk_read(_arg: map, _arg: reg, _arg: buf, _arg: count) -> return;
}
extern "C" {
    pub fn regmap_write(_arg: map, _arg: reg, _arg: value) -> return;
}
extern "C" {
    pub fn regmap_bulk_write(_arg: map, _arg: reg, _arg: buf, _arg: count) -> return;
}
extern "C" {
    pub fn regmap_update_bits(_arg: map, _arg: reg, _arg: mask, _arg: val) -> return;
}
