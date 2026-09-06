//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/max77693-private.h
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
// max77693-private.h - Voltage regulator driver for the Maxim 77693
//
// Copyright (C) 2012 Samsung Electronics
// SangYoung Son <hello.son@samsung.com>
//
// This program is not provided / owned by Maxim Integrated Products.
//

// Slave addr = 0xCC: PMIC, Charger, Flash LED
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77693_pmic_reg {
    MAX77693_LED_REG_IFLASH1			= 0x00,
    MAX77693_LED_REG_IFLASH2			= 0x01,
    MAX77693_LED_REG_ITORCH				= 0x02,
    MAX77693_LED_REG_ITORCHTIMER			= 0x03,
    MAX77693_LED_REG_FLASH_TIMER			= 0x04,
    MAX77693_LED_REG_FLASH_EN			= 0x05,
    MAX77693_LED_REG_MAX_FLASH1			= 0x06,
    MAX77693_LED_REG_MAX_FLASH2			= 0x07,
    MAX77693_LED_REG_MAX_FLASH3			= 0x08,
    MAX77693_LED_REG_MAX_FLASH4			= 0x09,
    MAX77693_LED_REG_VOUT_CNTL			= 0x0A,
    MAX77693_LED_REG_VOUT_FLASH1			= 0x0B,
    MAX77693_LED_REG_VOUT_FLASH2			= 0x0C,
    MAX77693_LED_REG_FLASH_INT			= 0x0E,
    MAX77693_LED_REG_FLASH_INT_MASK			= 0x0F,
    MAX77693_LED_REG_FLASH_STATUS			= 0x10,

    MAX77693_PMIC_REG_PMIC_ID1			= 0x20,
    MAX77693_PMIC_REG_PMIC_ID2			= 0x21,
    MAX77693_PMIC_REG_INTSRC			= 0x22,
    MAX77693_PMIC_REG_INTSRC_MASK			= 0x23,
    MAX77693_PMIC_REG_TOPSYS_INT			= 0x24,
    MAX77693_PMIC_REG_TOPSYS_INT_MASK		= 0x26,
    MAX77693_PMIC_REG_TOPSYS_STAT			= 0x28,
    MAX77693_PMIC_REG_MAINCTRL1			= 0x2A,
    MAX77693_PMIC_REG_LSCNFG			= 0x2B,

    MAX77693_CHG_REG_CHG_INT			= 0xB0,
    MAX77693_CHG_REG_CHG_INT_MASK			= 0xB1,
    MAX77693_CHG_REG_CHG_INT_OK			= 0xB2,
    MAX77693_CHG_REG_CHG_DETAILS_00			= 0xB3,
    MAX77693_CHG_REG_CHG_DETAILS_01			= 0xB4,
    MAX77693_CHG_REG_CHG_DETAILS_02			= 0xB5,
    MAX77693_CHG_REG_CHG_DETAILS_03			= 0xB6,
    MAX77693_CHG_REG_CHG_CNFG_00			= 0xB7,
    MAX77693_CHG_REG_CHG_CNFG_01			= 0xB8,
    MAX77693_CHG_REG_CHG_CNFG_02			= 0xB9,
    MAX77693_CHG_REG_CHG_CNFG_03			= 0xBA,
    MAX77693_CHG_REG_CHG_CNFG_04			= 0xBB,
    MAX77693_CHG_REG_CHG_CNFG_05			= 0xBC,
    MAX77693_CHG_REG_CHG_CNFG_06			= 0xBD,
    MAX77693_CHG_REG_CHG_CNFG_07			= 0xBE,
    MAX77693_CHG_REG_CHG_CNFG_08			= 0xBF,
    MAX77693_CHG_REG_CHG_CNFG_09			= 0xC0,
    MAX77693_CHG_REG_CHG_CNFG_10			= 0xC1,
    MAX77693_CHG_REG_CHG_CNFG_11			= 0xC2,
    MAX77693_CHG_REG_CHG_CNFG_12			= 0xC3,
    MAX77693_CHG_REG_CHG_CNFG_13			= 0xC4,
    MAX77693_CHG_REG_CHG_CNFG_14			= 0xC5,
    MAX77693_CHG_REG_SAFEOUT_CTRL			= 0xC6,

    MAX77693_PMIC_REG_END,
}

// MAX77693 ITORCH register
pub const TORCH_IOUT1_SHIFT: c_int = 0;
pub const TORCH_IOUT2_SHIFT: c_int = 4;

pub const TORCH_IOUT_MIN: c_int = 15625;
pub const TORCH_IOUT_MAX: c_int = 250000;
pub const TORCH_IOUT_STEP: c_int = 15625;
// MAX77693 IFLASH1 and IFLASH2 registers
pub const FLASH_IOUT_MIN: c_int = 15625;
pub const FLASH_IOUT_MAX_1LED: c_int = 1000000;
pub const FLASH_IOUT_MAX_2LEDS: c_int = 625000;
pub const FLASH_IOUT_STEP: c_int = 15625;
// MAX77693 TORCH_TIMER register
pub const TORCH_TMR_NO_TIMER: c_uint = 0x40;
pub const TORCH_TIMEOUT_MIN: c_int = 262000;
pub const TORCH_TIMEOUT_MAX: c_int = 15728000;
// MAX77693 FLASH_TIMER register
pub const FLASH_TMR_LEVEL: c_uint = 0x80;
pub const FLASH_TIMEOUT_MIN: c_int = 62500;
pub const FLASH_TIMEOUT_MAX: c_int = 1000000;
pub const FLASH_TIMEOUT_STEP: c_int = 62500;
// MAX77693 FLASH_EN register
pub const FLASH_EN_OFF: c_uint = 0x0;
pub const FLASH_EN_FLASH: c_uint = 0x1;
pub const FLASH_EN_TORCH: c_uint = 0x2;
pub const FLASH_EN_ON: c_uint = 0x3;

// MAX77693 MAX_FLASH1 register
pub const MAX_FLASH1_MAX_FL_EN: c_uint = 0x80;
pub const MAX_FLASH1_VSYS_MIN: c_int = 2400;
pub const MAX_FLASH1_VSYS_MAX: c_int = 3400;
pub const MAX_FLASH1_VSYS_STEP: c_int = 33;
// MAX77693 VOUT_CNTL register
pub const FLASH_BOOST_FIXED: c_uint = 0x04;
pub const FLASH_BOOST_LEDNUM_2: c_uint = 0x80;
// MAX77693 VOUT_FLASH1 register
pub const FLASH_VOUT_MIN: c_int = 3300;
pub const FLASH_VOUT_MAX: c_int = 5500;
pub const FLASH_VOUT_STEP: c_int = 25;
pub const FLASH_VOUT_RMIN: c_uint = 0x0c;
// MAX77693 FLASH_STATUS register

// MAX77693 FLASH_INT register

// Fast charge timer in hours
pub const DEFAULT_FAST_CHARGE_TIMER: c_int = 4;
// microamps
pub const DEFAULT_TOP_OFF_THRESHOLD_CURRENT: c_int = 150000;
// minutes
pub const DEFAULT_TOP_OFF_TIMER: c_int = 30;
// microvolts
pub const DEFAULT_CONSTANT_VOLT: c_int = 4200000;
// microvolts
pub const DEFAULT_MIN_SYSTEM_VOLT: c_int = 3600000;
// celsius
pub const DEFAULT_THERMAL_REGULATION_TEMP: c_int = 100;
// microamps
pub const DEFAULT_BATTERY_OVERCURRENT: c_int = 3500000;
// microvolts
pub const DEFAULT_CHARGER_INPUT_THRESHOLD_VOLT: c_int = 4300000;
// MAX77693_CHG_REG_CHG_INT_OK register
pub const CHG_INT_OK_BYP_SHIFT: c_int = 0;
pub const CHG_INT_OK_BAT_SHIFT: c_int = 3;
pub const CHG_INT_OK_CHG_SHIFT: c_int = 4;
pub const CHG_INT_OK_CHGIN_SHIFT: c_int = 6;
pub const CHG_INT_OK_DETBAT_SHIFT: c_int = 7;

// MAX77693_CHG_REG_CHG_DETAILS_00 register
pub const CHG_DETAILS_00_CHGIN_SHIFT: c_int = 5;

// MAX77693_CHG_REG_CHG_DETAILS_01 register
pub const CHG_DETAILS_01_CHG_SHIFT: c_int = 0;
pub const CHG_DETAILS_01_BAT_SHIFT: c_int = 4;
pub const CHG_DETAILS_01_TREG_SHIFT: c_int = 7;

// MAX77693_CHG_REG_CHG_DETAILS_01/CHG field
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77693_charger_charging_state {
    MAX77693_CHARGING_PREQUALIFICATION	= 0x0,
    MAX77693_CHARGING_FAST_CONST_CURRENT,
    MAX77693_CHARGING_FAST_CONST_VOLTAGE,
    MAX77693_CHARGING_TOP_OFF,
    MAX77693_CHARGING_DONE,
    MAX77693_CHARGING_HIGH_TEMP,
    MAX77693_CHARGING_TIMER_EXPIRED,
    MAX77693_CHARGING_THERMISTOR_SUSPEND,
    MAX77693_CHARGING_OFF,
    MAX77693_CHARGING_RESERVED,
    MAX77693_CHARGING_OVER_TEMP,
    MAX77693_CHARGING_WATCHDOG_EXPIRED,
}

// MAX77693_CHG_REG_CHG_DETAILS_01/BAT field
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77693_charger_battery_state {
    MAX77693_BATTERY_NOBAT			= 0x0,
// Dead-battery or low-battery prequalification
    MAX77693_BATTERY_PREQUALIFICATION,
    MAX77693_BATTERY_TIMER_EXPIRED,
    MAX77693_BATTERY_GOOD,
    MAX77693_BATTERY_LOWVOLTAGE,
    MAX77693_BATTERY_OVERVOLTAGE,
    MAX77693_BATTERY_OVERCURRENT,
    MAX77693_BATTERY_RESERVED,
}

// MAX77693_CHG_REG_CHG_DETAILS_02 register
pub const CHG_DETAILS_02_BYP_SHIFT: c_int = 0;

// MAX77693 CHG_CNFG_00 register
pub const CHG_CNFG_00_CHG_MASK: c_uint = 0x1;
pub const CHG_CNFG_00_BUCK_MASK: c_uint = 0x4;
// MAX77693_CHG_REG_CHG_CNFG_01 register
pub const CHG_CNFG_01_FCHGTIME_SHIFT: c_int = 0;
pub const CHG_CNFG_01_CHGRSTRT_SHIFT: c_int = 4;
pub const CHG_CNFG_01_PQEN_SHIFT: c_int = 7;

// MAX77693_CHG_REG_CHG_CNFG_02 register
pub const CHG_CNFG_02_CC_SHIFT: c_int = 0;
pub const CHG_CNFG_02_CC_MASK: c_uint = 0x3F;
// MAX77693_CHG_REG_CHG_CNFG_03 register
pub const CHG_CNFG_03_TOITH_SHIFT: c_int = 0;
pub const CHG_CNFG_03_TOTIME_SHIFT: c_int = 3;

// MAX77693_CHG_REG_CHG_CNFG_04 register
pub const CHG_CNFG_04_CHGCVPRM_SHIFT: c_int = 0;
pub const CHG_CNFG_04_MINVSYS_SHIFT: c_int = 5;

// MAX77693_CHG_REG_CHG_CNFG_06 register
pub const CHG_CNFG_06_CHGPROT_SHIFT: c_int = 2;

// MAX77693_CHG_REG_CHG_CNFG_07 register
pub const CHG_CNFG_07_REGTEMP_SHIFT: c_int = 5;

// MAX77693_CHG_REG_CHG_CNFG_12 register
pub const CHG_CNFG_12_B2SOVRC_SHIFT: c_int = 0;
pub const CHG_CNFG_12_VCHGINREG_SHIFT: c_int = 3;

// MAX77693 CHG_CNFG_09 Register
pub const CHG_CNFG_09_CHGIN_ILIM_SHIFT: c_int = 0;
pub const CHG_CNFG_09_CHGIN_ILIM_MASK: c_uint = 0x7F;
// MAX77693 CHG_CTRL Register
pub const SAFEOUT_CTRL_SAFEOUT1_MASK: c_uint = 0x3;
pub const SAFEOUT_CTRL_SAFEOUT2_MASK: c_uint = 0xC;
pub const SAFEOUT_CTRL_ENSAFEOUT1_MASK: c_uint = 0x40;
pub const SAFEOUT_CTRL_ENSAFEOUT2_MASK: c_uint = 0x80;
// Slave addr = 0x4A: MUIC
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77693_muic_reg {
    MAX77693_MUIC_REG_ID		= 0x00,
    MAX77693_MUIC_REG_INT1		= 0x01,
    MAX77693_MUIC_REG_INT2		= 0x02,
    MAX77693_MUIC_REG_INT3		= 0x03,
    MAX77693_MUIC_REG_STATUS1	= 0x04,
    MAX77693_MUIC_REG_STATUS2	= 0x05,
    MAX77693_MUIC_REG_STATUS3	= 0x06,
    MAX77693_MUIC_REG_INTMASK1	= 0x07,
    MAX77693_MUIC_REG_INTMASK2	= 0x08,
    MAX77693_MUIC_REG_INTMASK3	= 0x09,
    MAX77693_MUIC_REG_CDETCTRL1	= 0x0A,
    MAX77693_MUIC_REG_CDETCTRL2	= 0x0B,
    MAX77693_MUIC_REG_CTRL1		= 0x0C,
    MAX77693_MUIC_REG_CTRL2		= 0x0D,
    MAX77693_MUIC_REG_CTRL3		= 0x0E,

    MAX77693_MUIC_REG_END,
}

// MAX77693 INTMASK1~2 Register
pub const INTMASK1_ADC1K_SHIFT: c_int = 3;
pub const INTMASK1_ADCERR_SHIFT: c_int = 2;
pub const INTMASK1_ADCLOW_SHIFT: c_int = 1;
pub const INTMASK1_ADC_SHIFT: c_int = 0;

pub const INTMASK2_VIDRM_SHIFT: c_int = 5;
pub const INTMASK2_VBVOLT_SHIFT: c_int = 4;
pub const INTMASK2_DXOVP_SHIFT: c_int = 3;
pub const INTMASK2_DCDTMR_SHIFT: c_int = 2;
pub const INTMASK2_CHGDETRUN_SHIFT: c_int = 1;
pub const INTMASK2_CHGTYP_SHIFT: c_int = 0;

// MAX77693 MUIC - STATUS1~3 Register
pub const MAX77693_STATUS1_ADC_SHIFT: c_int = 0;
pub const MAX77693_STATUS1_ADCLOW_SHIFT: c_int = 5;
pub const MAX77693_STATUS1_ADCERR_SHIFT: c_int = 6;
pub const MAX77693_STATUS1_ADC1K_SHIFT: c_int = 7;

pub const MAX77693_STATUS2_CHGTYP_SHIFT: c_int = 0;
pub const MAX77693_STATUS2_CHGDETRUN_SHIFT: c_int = 3;
pub const MAX77693_STATUS2_DCDTMR_SHIFT: c_int = 4;
pub const MAX77693_STATUS2_DXOVP_SHIFT: c_int = 5;
pub const MAX77693_STATUS2_VBVOLT_SHIFT: c_int = 6;
pub const MAX77693_STATUS2_VIDRM_SHIFT: c_int = 7;

pub const MAX77693_STATUS3_OVP_SHIFT: c_int = 2;

// MAX77693 CDETCTRL1~2 register

// MAX77693 MUIC - CONTROL1~3 register

pub const MAX77693_CONTROL2_LOWPWR_SHIFT: c_int = 0;
pub const MAX77693_CONTROL2_ADCEN_SHIFT: c_int = 1;
pub const MAX77693_CONTROL2_CPEN_SHIFT: c_int = 2;
pub const MAX77693_CONTROL2_SFOUTASRT_SHIFT: c_int = 3;
pub const MAX77693_CONTROL2_SFOUTORD_SHIFT: c_int = 4;
pub const MAX77693_CONTROL2_ACCDET_SHIFT: c_int = 5;
pub const MAX77693_CONTROL2_USBCPINT_SHIFT: c_int = 6;
pub const MAX77693_CONTROL2_RCPS_SHIFT: c_int = 7;

pub const MAX77693_CONTROL3_JIGSET_SHIFT: c_int = 0;
pub const MAX77693_CONTROL3_BTLDSET_SHIFT: c_int = 2;
pub const MAX77693_CONTROL3_ADCDBSET_SHIFT: c_int = 4;

// Slave addr = 0x90: Haptic
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77693_haptic_reg {
    MAX77693_HAPTIC_REG_STATUS		= 0x00,
    MAX77693_HAPTIC_REG_CONFIG1		= 0x01,
    MAX77693_HAPTIC_REG_CONFIG2		= 0x02,
    MAX77693_HAPTIC_REG_CONFIG_CHNL		= 0x03,
    MAX77693_HAPTIC_REG_CONFG_CYC1		= 0x04,
    MAX77693_HAPTIC_REG_CONFG_CYC2		= 0x05,
    MAX77693_HAPTIC_REG_CONFIG_PER1		= 0x06,
    MAX77693_HAPTIC_REG_CONFIG_PER2		= 0x07,
    MAX77693_HAPTIC_REG_CONFIG_PER3		= 0x08,
    MAX77693_HAPTIC_REG_CONFIG_PER4		= 0x09,
    MAX77693_HAPTIC_REG_CONFIG_DUTY1	= 0x0A,
    MAX77693_HAPTIC_REG_CONFIG_DUTY2	= 0x0B,
    MAX77693_HAPTIC_REG_CONFIG_PWM1		= 0x0C,
    MAX77693_HAPTIC_REG_CONFIG_PWM2		= 0x0D,
    MAX77693_HAPTIC_REG_CONFIG_PWM3		= 0x0E,
    MAX77693_HAPTIC_REG_CONFIG_PWM4		= 0x0F,
    MAX77693_HAPTIC_REG_REV			= 0x10,

    MAX77693_HAPTIC_REG_END,
}

// max77693-pmic LSCNFG configuration register
pub const MAX77693_PMIC_LOW_SYS_MASK: c_uint = 0x80;
pub const MAX77693_PMIC_LOW_SYS_SHIFT: c_int = 7;
// max77693-haptic configuration register
pub const MAX77693_CONFIG2_MODE: c_int = 7;
pub const MAX77693_CONFIG2_MEN: c_int = 6;
pub const MAX77693_CONFIG2_HTYP: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77693_irq {
// PMIC - FLASH
    MAX77693_LED_IRQ_FLED2_OPEN,
    MAX77693_LED_IRQ_FLED2_SHORT,
    MAX77693_LED_IRQ_FLED1_OPEN,
    MAX77693_LED_IRQ_FLED1_SHORT,
    MAX77693_LED_IRQ_MAX_FLASH,

// PMIC - TOPSYS
    MAX77693_TOPSYS_IRQ_T120C_INT,
    MAX77693_TOPSYS_IRQ_T140C_INT,
    MAX77693_TOPSYS_IRQ_LOWSYS_INT,

// PMIC - Charger
    MAX77693_CHG_IRQ_BYP_I,
    MAX77693_CHG_IRQ_THM_I,
    MAX77693_CHG_IRQ_BAT_I,
    MAX77693_CHG_IRQ_CHG_I,
    MAX77693_CHG_IRQ_CHGIN_I,

    MAX77693_IRQ_NR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77693_irq_muic {
// MUIC INT1
    MAX77693_MUIC_IRQ_INT1_ADC,
    MAX77693_MUIC_IRQ_INT1_ADC_LOW,
    MAX77693_MUIC_IRQ_INT1_ADC_ERR,
    MAX77693_MUIC_IRQ_INT1_ADC1K,

// MUIC INT2
    MAX77693_MUIC_IRQ_INT2_CHGTYP,
    MAX77693_MUIC_IRQ_INT2_CHGDETREUN,
    MAX77693_MUIC_IRQ_INT2_DCDTMR,
    MAX77693_MUIC_IRQ_INT2_DXOVP,
    MAX77693_MUIC_IRQ_INT2_VBVOLT,
    MAX77693_MUIC_IRQ_INT2_VIDRM,

// MUIC INT3
    MAX77693_MUIC_IRQ_INT3_EOC,
    MAX77693_MUIC_IRQ_INT3_CGMBC,
    MAX77693_MUIC_IRQ_INT3_OVP,
    MAX77693_MUIC_IRQ_INT3_MBCCHG_ERR,
    MAX77693_MUIC_IRQ_INT3_CHG_ENABLED,
    MAX77693_MUIC_IRQ_INT3_BAT_DET,

    MAX77693_MUIC_IRQ_NR,
}
