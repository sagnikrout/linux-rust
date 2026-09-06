//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/tps6594.h
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
// Functions to access TPS6594 Power Management IC
//
// Copyright (C) 2023 BayLibre Incorporated - https://www.baylibre.com
//

// Chip id list
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pmic_id {
    TPS6594,
    TPS6593,
    LP8764,
    TPS65224,
    TPS652G1,
}

// Macro to get page index from register address

// Registers for page 0
pub const TPS6594_REG_DEV_REV: c_uint = 0x01;
pub const TPS6594_REG_NVM_CODE_1: c_uint = 0x02;
pub const TPS6594_REG_NVM_CODE_2: c_uint = 0x03;

pub const TPS6594_REG_LDORTC_CTRL: c_uint = 0x22;

pub const TPS6594_REG_VCCA_VMON_CTRL: c_uint = 0x2b;
pub const TPS6594_REG_VCCA_PG_WINDOW: c_uint = 0x2c;
pub const TPS6594_REG_VMON1_PG_WINDOW: c_uint = 0x2d;
pub const TPS6594_REG_VMON1_PG_LEVEL: c_uint = 0x2e;
pub const TPS6594_REG_VMON2_PG_WINDOW: c_uint = 0x2f;
pub const TPS6594_REG_VMON2_PG_LEVEL: c_uint = 0x30;

pub const TPS6594_REG_NPWRON_CONF: c_uint = 0x3c;
pub const TPS6594_REG_GPIO_OUT_1: c_uint = 0x3d;
pub const TPS6594_REG_GPIO_OUT_2: c_uint = 0x3e;
pub const TPS6594_REG_GPIO_IN_1: c_uint = 0x3f;
pub const TPS6594_REG_GPIO_IN_2: c_uint = 0x40;

pub const TPS6594_REG_RAIL_SEL_1: c_uint = 0x41;
pub const TPS6594_REG_RAIL_SEL_2: c_uint = 0x42;
pub const TPS6594_REG_RAIL_SEL_3: c_uint = 0x43;
pub const TPS6594_REG_FSM_TRIG_SEL_1: c_uint = 0x44;
pub const TPS6594_REG_FSM_TRIG_SEL_2: c_uint = 0x45;
pub const TPS6594_REG_FSM_TRIG_MASK_1: c_uint = 0x46;
pub const TPS6594_REG_FSM_TRIG_MASK_2: c_uint = 0x47;
pub const TPS6594_REG_FSM_TRIG_MASK_3: c_uint = 0x48;
pub const TPS6594_REG_MASK_BUCK1_2: c_uint = 0x49;
pub const TPS65224_REG_MASK_BUCKS: c_uint = 0x49;
pub const TPS6594_REG_MASK_BUCK3_4: c_uint = 0x4a;
pub const TPS6594_REG_MASK_BUCK5: c_uint = 0x4b;
pub const TPS6594_REG_MASK_LDO1_2: c_uint = 0x4c;
pub const TPS65224_REG_MASK_LDOS: c_uint = 0x4c;
pub const TPS6594_REG_MASK_LDO3_4: c_uint = 0x4d;
pub const TPS6594_REG_MASK_VMON: c_uint = 0x4e;
pub const TPS6594_REG_MASK_GPIO_FALL: c_uint = 0x4f;
pub const TPS6594_REG_MASK_GPIO_RISE: c_uint = 0x50;
pub const TPS6594_REG_MASK_GPIO9_11: c_uint = 0x51;
pub const TPS6594_REG_MASK_STARTUP: c_uint = 0x52;
pub const TPS6594_REG_MASK_MISC: c_uint = 0x53;
pub const TPS6594_REG_MASK_MODERATE_ERR: c_uint = 0x54;
pub const TPS6594_REG_MASK_FSM_ERR: c_uint = 0x56;
pub const TPS6594_REG_MASK_COMM_ERR: c_uint = 0x57;
pub const TPS6594_REG_MASK_READBACK_ERR: c_uint = 0x58;
pub const TPS6594_REG_MASK_ESM: c_uint = 0x59;
pub const TPS6594_REG_INT_TOP: c_uint = 0x5a;
pub const TPS6594_REG_INT_BUCK: c_uint = 0x5b;
pub const TPS6594_REG_INT_BUCK1_2: c_uint = 0x5c;
pub const TPS6594_REG_INT_BUCK3_4: c_uint = 0x5d;
pub const TPS6594_REG_INT_BUCK5: c_uint = 0x5e;
pub const TPS6594_REG_INT_LDO_VMON: c_uint = 0x5f;
pub const TPS6594_REG_INT_LDO1_2: c_uint = 0x60;
pub const TPS6594_REG_INT_LDO3_4: c_uint = 0x61;
pub const TPS6594_REG_INT_VMON: c_uint = 0x62;
pub const TPS6594_REG_INT_GPIO: c_uint = 0x63;
pub const TPS6594_REG_INT_GPIO1_8: c_uint = 0x64;
pub const TPS6594_REG_INT_STARTUP: c_uint = 0x65;
pub const TPS6594_REG_INT_MISC: c_uint = 0x66;
pub const TPS6594_REG_INT_MODERATE_ERR: c_uint = 0x67;
pub const TPS6594_REG_INT_SEVERE_ERR: c_uint = 0x68;
pub const TPS6594_REG_INT_FSM_ERR: c_uint = 0x69;
pub const TPS6594_REG_INT_COMM_ERR: c_uint = 0x6a;
pub const TPS6594_REG_INT_READBACK_ERR: c_uint = 0x6b;
pub const TPS6594_REG_INT_ESM: c_uint = 0x6c;
pub const TPS6594_REG_STAT_BUCK1_2: c_uint = 0x6d;
pub const TPS6594_REG_STAT_BUCK3_4: c_uint = 0x6e;
pub const TPS6594_REG_STAT_BUCK5: c_uint = 0x6f;
pub const TPS6594_REG_STAT_LDO1_2: c_uint = 0x70;
pub const TPS6594_REG_STAT_LDO3_4: c_uint = 0x71;
pub const TPS6594_REG_STAT_VMON: c_uint = 0x72;
pub const TPS6594_REG_STAT_STARTUP: c_uint = 0x73;
pub const TPS6594_REG_STAT_MISC: c_uint = 0x74;
pub const TPS6594_REG_STAT_MODERATE_ERR: c_uint = 0x75;
pub const TPS6594_REG_STAT_SEVERE_ERR: c_uint = 0x76;
pub const TPS6594_REG_STAT_READBACK_ERR: c_uint = 0x77;
pub const TPS6594_REG_PGOOD_SEL_1: c_uint = 0x78;
pub const TPS6594_REG_PGOOD_SEL_2: c_uint = 0x79;
pub const TPS6594_REG_PGOOD_SEL_3: c_uint = 0x7a;
pub const TPS6594_REG_PGOOD_SEL_4: c_uint = 0x7b;
pub const TPS6594_REG_PLL_CTRL: c_uint = 0x7c;
pub const TPS6594_REG_CONFIG_1: c_uint = 0x7d;
pub const TPS6594_REG_CONFIG_2: c_uint = 0x7e;
pub const TPS6594_REG_ENABLE_DRV_REG: c_uint = 0x80;
pub const TPS6594_REG_MISC_CTRL: c_uint = 0x81;
pub const TPS6594_REG_ENABLE_DRV_STAT: c_uint = 0x82;
pub const TPS6594_REG_RECOV_CNT_REG_1: c_uint = 0x83;
pub const TPS6594_REG_RECOV_CNT_REG_2: c_uint = 0x84;
pub const TPS6594_REG_FSM_I2C_TRIGGERS: c_uint = 0x85;
pub const TPS6594_REG_FSM_NSLEEP_TRIGGERS: c_uint = 0x86;
pub const TPS6594_REG_BUCK_RESET_REG: c_uint = 0x87;
pub const TPS6594_REG_SPREAD_SPECTRUM_1: c_uint = 0x88;
pub const TPS6594_REG_FREQ_SEL: c_uint = 0x8a;
pub const TPS6594_REG_FSM_STEP_SIZE: c_uint = 0x8b;
pub const TPS6594_REG_LDO_RV_TIMEOUT_REG_1: c_uint = 0x8c;
pub const TPS6594_REG_LDO_RV_TIMEOUT_REG_2: c_uint = 0x8d;
pub const TPS6594_REG_USER_SPARE_REGS: c_uint = 0x8e;
pub const TPS6594_REG_ESM_MCU_START_REG: c_uint = 0x8f;
pub const TPS6594_REG_ESM_MCU_DELAY1_REG: c_uint = 0x90;
pub const TPS6594_REG_ESM_MCU_DELAY2_REG: c_uint = 0x91;
pub const TPS6594_REG_ESM_MCU_MODE_CFG: c_uint = 0x92;
pub const TPS6594_REG_ESM_MCU_HMAX_REG: c_uint = 0x93;
pub const TPS6594_REG_ESM_MCU_HMIN_REG: c_uint = 0x94;
pub const TPS6594_REG_ESM_MCU_LMAX_REG: c_uint = 0x95;
pub const TPS6594_REG_ESM_MCU_LMIN_REG: c_uint = 0x96;
pub const TPS6594_REG_ESM_MCU_ERR_CNT_REG: c_uint = 0x97;
pub const TPS6594_REG_ESM_SOC_START_REG: c_uint = 0x98;
pub const TPS6594_REG_ESM_SOC_DELAY1_REG: c_uint = 0x99;
pub const TPS6594_REG_ESM_SOC_DELAY2_REG: c_uint = 0x9a;
pub const TPS6594_REG_ESM_SOC_MODE_CFG: c_uint = 0x9b;
pub const TPS6594_REG_ESM_SOC_HMAX_REG: c_uint = 0x9c;
pub const TPS6594_REG_ESM_SOC_HMIN_REG: c_uint = 0x9d;
pub const TPS6594_REG_ESM_SOC_LMAX_REG: c_uint = 0x9e;
pub const TPS6594_REG_ESM_SOC_LMIN_REG: c_uint = 0x9f;
pub const TPS6594_REG_ESM_SOC_ERR_CNT_REG: c_uint = 0xa0;
pub const TPS6594_REG_REGISTER_LOCK: c_uint = 0xa1;
pub const TPS65224_REG_SRAM_ACCESS_1: c_uint = 0xa2;
pub const TPS65224_REG_SRAM_ACCESS_2: c_uint = 0xa3;
pub const TPS65224_REG_SRAM_ADDR_CTRL: c_uint = 0xa4;
pub const TPS65224_REG_RECOV_CNT_PFSM_INCR: c_uint = 0xa5;
pub const TPS6594_REG_MANUFACTURING_VER: c_uint = 0xa6;
pub const TPS6594_REG_CUSTOMER_NVM_ID_REG: c_uint = 0xa7;
pub const TPS6594_REG_VMON_CONF_REG: c_uint = 0xa8;
pub const TPS6594_REG_SOFT_REBOOT_REG: c_uint = 0xab;
pub const TPS65224_REG_ADC_CTRL: c_uint = 0xac;
pub const TPS65224_REG_ADC_RESULT_REG_1: c_uint = 0xad;
pub const TPS65224_REG_ADC_RESULT_REG_2: c_uint = 0xae;
pub const TPS6594_REG_RTC_SECONDS: c_uint = 0xb5;
pub const TPS6594_REG_RTC_MINUTES: c_uint = 0xb6;
pub const TPS6594_REG_RTC_HOURS: c_uint = 0xb7;
pub const TPS6594_REG_RTC_DAYS: c_uint = 0xb8;
pub const TPS6594_REG_RTC_MONTHS: c_uint = 0xb9;
pub const TPS6594_REG_RTC_YEARS: c_uint = 0xba;
pub const TPS6594_REG_RTC_WEEKS: c_uint = 0xbb;
pub const TPS6594_REG_ALARM_SECONDS: c_uint = 0xbc;
pub const TPS6594_REG_ALARM_MINUTES: c_uint = 0xbd;
pub const TPS6594_REG_ALARM_HOURS: c_uint = 0xbe;
pub const TPS6594_REG_ALARM_DAYS: c_uint = 0xbf;
pub const TPS6594_REG_ALARM_MONTHS: c_uint = 0xc0;
pub const TPS6594_REG_ALARM_YEARS: c_uint = 0xc1;
pub const TPS6594_REG_RTC_CTRL_1: c_uint = 0xc2;
pub const TPS6594_REG_RTC_CTRL_2: c_uint = 0xc3;
pub const TPS65224_REG_STARTUP_CTRL: c_uint = 0xc3;
pub const TPS6594_REG_RTC_STATUS: c_uint = 0xc4;
pub const TPS6594_REG_RTC_INTERRUPTS: c_uint = 0xc5;
pub const TPS6594_REG_RTC_COMP_LSB: c_uint = 0xc6;
pub const TPS6594_REG_RTC_COMP_MSB: c_uint = 0xc7;
pub const TPS6594_REG_RTC_RESET_STATUS: c_uint = 0xc8;
pub const TPS6594_REG_SCRATCH_PAD_REG_1: c_uint = 0xc9;
pub const TPS6594_REG_SCRATCH_PAD_REG_2: c_uint = 0xca;
pub const TPS6594_REG_SCRATCH_PAD_REG_3: c_uint = 0xcb;
pub const TPS6594_REG_SCRATCH_PAD_REG_4: c_uint = 0xcc;
pub const TPS6594_REG_PFSM_DELAY_REG_1: c_uint = 0xcd;
pub const TPS6594_REG_PFSM_DELAY_REG_2: c_uint = 0xce;
pub const TPS6594_REG_PFSM_DELAY_REG_3: c_uint = 0xcf;
pub const TPS6594_REG_PFSM_DELAY_REG_4: c_uint = 0xd0;
pub const TPS65224_REG_ADC_GAIN_COMP_REG: c_uint = 0xd0;
pub const TPS65224_REG_CRC_CALC_CONTROL: c_uint = 0xef;
pub const TPS65224_REG_REGMAP_USER_CRC_LOW: c_uint = 0xf0;
pub const TPS65224_REG_REGMAP_USER_CRC_HIGH: c_uint = 0xf1;
// Registers for page 1
pub const TPS6594_REG_SERIAL_IF_CONFIG: c_uint = 0x11a;
pub const TPS6594_REG_I2C1_ID: c_uint = 0x122;
pub const TPS6594_REG_I2C2_ID: c_uint = 0x123;
// Registers for page 4
pub const TPS6594_REG_WD_ANSWER_REG: c_uint = 0x401;
pub const TPS6594_REG_WD_QUESTION_ANSW_CNT: c_uint = 0x402;
pub const TPS6594_REG_WD_WIN1_CFG: c_uint = 0x403;
pub const TPS6594_REG_WD_WIN2_CFG: c_uint = 0x404;
pub const TPS6594_REG_WD_LONGWIN_CFG: c_uint = 0x405;
pub const TPS6594_REG_WD_MODE_REG: c_uint = 0x406;
pub const TPS6594_REG_WD_QA_CFG: c_uint = 0x407;
pub const TPS6594_REG_WD_ERR_STATUS: c_uint = 0x408;
pub const TPS6594_REG_WD_THR_CFG: c_uint = 0x409;
pub const TPS6594_REG_DWD_FAIL_CNT_REG: c_uint = 0x40a;
// BUCKX_CTRL register field definition

// TPS6594 BUCKX_CONF register field definition

// TPS65224 BUCKX_CONF register field definition

// TPS6594 BUCKX_PG_WINDOW register field definition

// TPS65224 BUCKX_PG_WINDOW register field definition

// TPS6594 BUCKX_VOUT register field definition

// TPS65224 BUCKX_VOUT register field definition

// LDOX_CTRL register field definition

// LDORTC_CTRL register field definition

// LDOX_VOUT register field definition

// LDOX_PG_WINDOW register field definition

// LDOX_PG_WINDOW register field definition

// VCCA_VMON_CTRL register field definition

// VCCA_PG_WINDOW register field definition

// VMONX_PG_WINDOW register field definition

// VMONX_PG_WINDOW register field definition

// GPIOX_CONF register field definition

// NPWRON_CONF register field definition

// POWER_ON_CONFIG register field definition

// GPIO_OUT_X register field definition

// GPIO_IN_X register field definition

// GPIO_OUT_X register field definition

// GPIO_IN_X register field definition

// RAIL_SEL_1 register field definition

// RAIL_SEL_2 register field definition

// RAIL_SEL_3 register field definition

// FSM_TRIG_SEL_1 register field definition

// FSM_TRIG_SEL_2 register field definition

// FSM_TRIG_MASK_X register field definition

// MASK_BUCKX register field definition

// MASK_LDOX register field definition

// MASK_VMON register field definition

// MASK_BUCK Register field definition

// MASK_LDO_VMON register field definition

// MASK_GPIOX register field definition

// MASK_GPIOX register field definition

// MASK_STARTUP register field definition

// MASK_MISC register field definition

// MASK_MODERATE_ERR register field definition

// MASK_FSM_ERR register field definition

// MASK_COMM_ERR register field definition

// MASK_READBACK_ERR register field definition

// MASK_ESM register field definition

// INT_TOP register field definition

// INT_BUCK register field definition

// INT_BUCK register field definition

// INT_BUCKX register field definition

// INT_LDO_VMON register field definition

// INT_LDO_VMON register field definition

// INT_LDOX register field definition

// INT_VMON register field definition

// INT_GPIO register field definition

// INT_GPIOX register field definition

// INT_GPIO register field definition

// INT_STARTUP register field definition

// INT_MISC register field definition

// INT_MODERATE_ERR register field definition

// INT_SEVERE_ERR register field definition

// INT_FSM_ERR register field definition

// INT_COMM_ERR register field definition

// INT_READBACK_ERR register field definition

// INT_ESM register field definition

// STAT_BUCKX register field definition

// STAT_LDOX register field definition

// STAT_VMON register field definition

// STAT_LDO_VMON register field definition

// STAT_STARTUP register field definition

// STAT_MISC register field definition

// STAT_MODERATE_ERR register field definition

// STAT_SEVERE_ERR register field definition

// STAT_READBACK_ERR register field definition

// PGOOD_SEL_1 register field definition

// PGOOD_SEL_2 register field definition

// PGOOD_SEL_3 register field definition

// PGOOD_SEL_4 register field definition

// PLL_CTRL register field definition

// CONFIG_1 register field definition

// CONFIG_2 register field definition

// ENABLE_DRV_REG register field definition

// MISC_CTRL register field definition

// ENABLE_DRV_STAT register field definition

// RECOV_CNT_REG_1 register field definition

// RECOV_CNT_REG_2 register field definition

// FSM_I2C_TRIGGERS register field definition

// FSM_NSLEEP_TRIGGERS register field definition

// BUCK_RESET_REG register field definition

// SPREAD_SPECTRUM_1 register field definition

// FREQ_SEL register field definition

// FSM_STEP_SIZE register field definition

// LDO_RV_TIMEOUT_REG_1 register field definition

// LDO_RV_TIMEOUT_REG_2 register field definition

// USER_SPARE_REGS register field definition

// ESM_MCU_START_REG register field definition

// ESM_MCU_MODE_CFG register field definition

// ESM_MCU_ERR_CNT_REG register field definition

// ESM_SOC_START_REG register field definition

// ESM_MCU_START_REG register field definition

// ESM_SOC_MODE_CFG register field definition

// ESM_MCU_MODE_CFG register field definition

// ESM_SOC_ERR_CNT_REG register field definition

// ESM_MCU_ERR_CNT_REG register field definition

// REGISTER_LOCK register field definition

// VMON_CONF register field definition

// SRAM_ACCESS_1 Register field definition

// SRAM_ACCESS_2 Register field definition

// SRAM_ADDR_CTRL Register field definition

// RECOV_CNT_PFSM_INCR Register field definition

// MANUFACTURING_VER Register field definition

// CUSTOMER_NVM_ID_REG Register field definition

// SOFT_REBOOT_REG register field definition

// RTC_SECONDS & ALARM_SECONDS register field definition

// RTC_MINUTES & ALARM_MINUTES register field definition

// RTC_HOURS & ALARM_HOURS register field definition

// RTC_DAYS & ALARM_DAYS register field definition

// RTC_MONTHS & ALARM_MONTHS register field definition

// RTC_YEARS & ALARM_YEARS register field definition

// RTC_WEEKS register field definition

// RTC_CTRL_1 register field definition

// RTC_CTRL_2 register field definition

// RTC_STATUS register field definition

// RTC_INTERRUPTS register field definition

// RTC_RESET_STATUS register field definition

// SERIAL_IF_CONFIG register field definition

// ADC_CTRL Register field definition

// ADC_RESULT_REG_1 Register field definition

// ADC_RESULT_REG_2 Register field definition

// STARTUP_CTRL Register field definition

// SCRATCH_PAD_REG_1 Register field definition

// SCRATCH_PAD_REG_2 Register field definition

// SCRATCH_PAD_REG_3 Register field definition

// SCRATCH_PAD_REG_4 Register field definition

// PFSM_DELAY_REG_1 Register field definition

// PFSM_DELAY_REG_2 Register field definition

// PFSM_DELAY_REG_3 Register field definition

// PFSM_DELAY_REG_4 Register field definition

// CRC_CALC_CONTROL Register field definition

// ADC_GAIN_COMP_REG Register field definition

// REGMAP_USER_CRC_LOW Register field definition

// REGMAP_USER_CRC_HIGH Register field definition

// WD_ANSWER_REG Register field definition

// WD_QUESTION_ANSW_CNT register field definition

// WD WIN1_CFG register field definition

// WD WIN2_CFG register field definition

// WD LongWin register field definition

// WD_MODE_REG register field definition

// WD_QA_CFG register field definition

// WD_ERR_STATUS register field definition

// WD_THR_CFG register field definition

// WD_FAIL_CNT_REG register field definition

// CRC8 polynomial for I2C & SPI protocols
pub const TPS6594_CRC8_POLYNOMIAL: c_uint = 0x07;
// IRQs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tps6594_irqs {
// INT_BUCK1_2 register
    TPS6594_IRQ_BUCK1_OV,
    TPS6594_IRQ_BUCK1_UV,
    TPS6594_IRQ_BUCK1_SC,
    TPS6594_IRQ_BUCK1_ILIM,
    TPS6594_IRQ_BUCK2_OV,
    TPS6594_IRQ_BUCK2_UV,
    TPS6594_IRQ_BUCK2_SC,
    TPS6594_IRQ_BUCK2_ILIM,
// INT_BUCK3_4 register
    TPS6594_IRQ_BUCK3_OV,
    TPS6594_IRQ_BUCK3_UV,
    TPS6594_IRQ_BUCK3_SC,
    TPS6594_IRQ_BUCK3_ILIM,
    TPS6594_IRQ_BUCK4_OV,
    TPS6594_IRQ_BUCK4_UV,
    TPS6594_IRQ_BUCK4_SC,
    TPS6594_IRQ_BUCK4_ILIM,
// INT_BUCK5 register
    TPS6594_IRQ_BUCK5_OV,
    TPS6594_IRQ_BUCK5_UV,
    TPS6594_IRQ_BUCK5_SC,
    TPS6594_IRQ_BUCK5_ILIM,
// INT_LDO1_2 register
    TPS6594_IRQ_LDO1_OV,
    TPS6594_IRQ_LDO1_UV,
    TPS6594_IRQ_LDO1_SC,
    TPS6594_IRQ_LDO1_ILIM,
    TPS6594_IRQ_LDO2_OV,
    TPS6594_IRQ_LDO2_UV,
    TPS6594_IRQ_LDO2_SC,
    TPS6594_IRQ_LDO2_ILIM,
// INT_LDO3_4 register
    TPS6594_IRQ_LDO3_OV,
    TPS6594_IRQ_LDO3_UV,
    TPS6594_IRQ_LDO3_SC,
    TPS6594_IRQ_LDO3_ILIM,
    TPS6594_IRQ_LDO4_OV,
    TPS6594_IRQ_LDO4_UV,
    TPS6594_IRQ_LDO4_SC,
    TPS6594_IRQ_LDO4_ILIM,
// INT_VMON register
    TPS6594_IRQ_VCCA_OV,
    TPS6594_IRQ_VCCA_UV,
    TPS6594_IRQ_VMON1_OV,
    TPS6594_IRQ_VMON1_UV,
    TPS6594_IRQ_VMON1_RV,
    TPS6594_IRQ_VMON2_OV,
    TPS6594_IRQ_VMON2_UV,
    TPS6594_IRQ_VMON2_RV,
// INT_GPIO register
    TPS6594_IRQ_GPIO9,
    TPS6594_IRQ_GPIO10,
    TPS6594_IRQ_GPIO11,
// INT_GPIO1_8 register
    TPS6594_IRQ_GPIO1,
    TPS6594_IRQ_GPIO2,
    TPS6594_IRQ_GPIO3,
    TPS6594_IRQ_GPIO4,
    TPS6594_IRQ_GPIO5,
    TPS6594_IRQ_GPIO6,
    TPS6594_IRQ_GPIO7,
    TPS6594_IRQ_GPIO8,
// INT_STARTUP register
    TPS6594_IRQ_NPWRON_START,
    TPS6594_IRQ_ENABLE,
    TPS6594_IRQ_FSD,
    TPS6594_IRQ_SOFT_REBOOT,
// INT_MISC register
    TPS6594_IRQ_BIST_PASS,
    TPS6594_IRQ_EXT_CLK,
    TPS6594_IRQ_TWARN,
// INT_MODERATE_ERR register
    TPS6594_IRQ_TSD_ORD,
    TPS6594_IRQ_BIST_FAIL,
    TPS6594_IRQ_REG_CRC_ERR,
    TPS6594_IRQ_RECOV_CNT,
    TPS6594_IRQ_SPMI_ERR,
    TPS6594_IRQ_NPWRON_LONG,
    TPS6594_IRQ_NINT_READBACK,
    TPS6594_IRQ_NRSTOUT_READBACK,
// INT_SEVERE_ERR register
    TPS6594_IRQ_TSD_IMM,
    TPS6594_IRQ_VCCA_OVP,
    TPS6594_IRQ_PFSM_ERR,
// INT_FSM_ERR register
    TPS6594_IRQ_IMM_SHUTDOWN,
    TPS6594_IRQ_ORD_SHUTDOWN,
    TPS6594_IRQ_MCU_PWR_ERR,
    TPS6594_IRQ_SOC_PWR_ERR,
// INT_COMM_ERR register
    TPS6594_IRQ_COMM_FRM_ERR,
    TPS6594_IRQ_COMM_CRC_ERR,
    TPS6594_IRQ_COMM_ADR_ERR,
    TPS6594_IRQ_I2C2_CRC_ERR,
    TPS6594_IRQ_I2C2_ADR_ERR,
// INT_READBACK_ERR register
    TPS6594_IRQ_EN_DRV_READBACK,
    TPS6594_IRQ_NRSTOUT_SOC_READBACK,
// INT_ESM register
    TPS6594_IRQ_ESM_SOC_PIN,
    TPS6594_IRQ_ESM_SOC_FAIL,
    TPS6594_IRQ_ESM_SOC_RST,
// RTC_STATUS register
    TPS6594_IRQ_TIMER,
    TPS6594_IRQ_ALARM,
    TPS6594_IRQ_POWER_UP,
}

// IRQs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tps65224_irqs {
// INT_BUCK register
    TPS65224_IRQ_BUCK1_UVOV,
    TPS65224_IRQ_BUCK2_UVOV,
    TPS65224_IRQ_BUCK3_UVOV,
    TPS65224_IRQ_BUCK4_UVOV,
// INT_LDO_VMON register
    TPS65224_IRQ_LDO1_UVOV,
    TPS65224_IRQ_LDO2_UVOV,
    TPS65224_IRQ_LDO3_UVOV,
    TPS65224_IRQ_VCCA_UVOV,
    TPS65224_IRQ_VMON1_UVOV,
    TPS65224_IRQ_VMON2_UVOV,
// INT_GPIO register
    TPS65224_IRQ_GPIO1,
    TPS65224_IRQ_GPIO2,
    TPS65224_IRQ_GPIO3,
    TPS65224_IRQ_GPIO4,
    TPS65224_IRQ_GPIO5,
    TPS65224_IRQ_GPIO6,
// INT_STARTUP register
    TPS65224_IRQ_VSENSE,
    TPS65224_IRQ_ENABLE,
    TPS65224_IRQ_PB_SHORT,
    TPS65224_IRQ_FSD,
    TPS65224_IRQ_SOFT_REBOOT,
// INT_MISC register
    TPS65224_IRQ_BIST_PASS,
    TPS65224_IRQ_EXT_CLK,
    TPS65224_IRQ_REG_UNLOCK,
    TPS65224_IRQ_TWARN,
    TPS65224_IRQ_PB_LONG,
    TPS65224_IRQ_PB_FALL,
    TPS65224_IRQ_PB_RISE,
    TPS65224_IRQ_ADC_CONV_READY,
// INT_MODERATE_ERR register
    TPS65224_IRQ_TSD_ORD,
    TPS65224_IRQ_BIST_FAIL,
    TPS65224_IRQ_REG_CRC_ERR,
    TPS65224_IRQ_RECOV_CNT,
// INT_SEVERE_ERR register
    TPS65224_IRQ_TSD_IMM,
    TPS65224_IRQ_VCCA_OVP,
    TPS65224_IRQ_PFSM_ERR,
    TPS65224_IRQ_BG_XMON,
// INT_FSM_ERR register
    TPS65224_IRQ_IMM_SHUTDOWN,
    TPS65224_IRQ_ORD_SHUTDOWN,
    TPS65224_IRQ_MCU_PWR_ERR,
    TPS65224_IRQ_SOC_PWR_ERR,
    TPS65224_IRQ_COMM_ERR,
    TPS65224_IRQ_I2C2_ERR,
}

//
// struct tps6594 - device private data structure
//
// @dev:      MFD parent device
// @chip_id:  chip ID
// @reg:      I2C slave address or SPI chip select number
// @use_crc:  if true, use CRC for I2C and SPI interface protocols
// @regmap:   regmap for accessing the device registers
// @irq:      irq generated by the device
// @irq_data: regmap irq data used for the irq chip
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps6594 {
    pub dev: *mut device,
    pub chip_id: c_ulong,
    pub reg: c_ushort,
    pub use_crc: bool,
    pub regmap: *mut regmap,
    pub irq: c_int,
    pub irq_data: *mut regmap_irq_chip_data,
}

extern "C" {
    pub fn tps6594_device_init(tps: *mut tps6594, enable_crc: bool) -> c_int;
}
