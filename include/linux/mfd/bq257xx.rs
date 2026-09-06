//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/bq257xx.h
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
// Register definitions for TI BQ257XX
// Copyright (C) 2020 Texas Instruments Incorporated - http://www.ti.com
//
pub const BQ25703_CHARGE_OPTION_0: c_uint = 0x00;
pub const BQ25703_CHARGE_CURRENT: c_uint = 0x02;
pub const BQ25703_MAX_CHARGE_VOLT: c_uint = 0x04;
pub const BQ25703_OTG_VOLT: c_uint = 0x06;
pub const BQ25703_OTG_CURRENT: c_uint = 0x08;
pub const BQ25703_INPUT_VOLTAGE: c_uint = 0x0a;
pub const BQ25703_MIN_VSYS: c_uint = 0x0c;
pub const BQ25703_IIN_HOST: c_uint = 0x0e;
pub const BQ25703_CHARGER_STATUS: c_uint = 0x20;
pub const BQ25703_PROCHOT_STATUS: c_uint = 0x22;
pub const BQ25703_IIN_DPM: c_uint = 0x24;
pub const BQ25703_ADCIBAT_CHG: c_uint = 0x28;
pub const BQ25703_ADCIINCMPIN: c_uint = 0x2a;
pub const BQ25703_ADCVSYSVBAT: c_uint = 0x2c;
pub const BQ25703_MANUFACT_DEV_ID: c_uint = 0x2e;
pub const BQ25703_CHARGE_OPTION_1: c_uint = 0x30;
pub const BQ25703_CHARGE_OPTION_2: c_uint = 0x32;
pub const BQ25703_CHARGE_OPTION_3: c_uint = 0x34;
pub const BQ25703_ADC_OPTION: c_uint = 0x3a;

pub const BQ25703_WDTMR_DISABLE: c_int = 0;
pub const BQ25703_WDTMR_5_SEC: c_int = 1;
pub const BQ25703_WDTMR_88_SEC: c_int = 2;
pub const BQ25703_WDTMR_175_SEC: c_int = 3;

pub const BQ25703_ICHG_STEP_UA: c_int = 64000;
pub const BQ25703_ICHG_MIN_UA: c_int = 64000;
pub const BQ25703_ICHG_MAX_UA: c_int = 8128000;

pub const BQ25703_VBATREG_STEP_UV: c_int = 16000;
pub const BQ25703_VBATREG_MIN_UV: c_int = 1024000;
pub const BQ25703_VBATREG_MAX_UV: c_int = 19200000;

pub const BQ25703_OTG_VOLT_STEP_UV: c_int = 64000;
pub const BQ25703_OTG_VOLT_MIN_UV: c_int = 4480000;
pub const BQ25703_OTG_VOLT_MAX_UV: c_int = 20800000;
pub const BQ25703_OTG_VOLT_NUM_VOLT: c_int = 256;

pub const BQ25703_OTG_CUR_STEP_UA: c_int = 50000;
pub const BQ25703_OTG_CUR_MAX_UA: c_int = 6350000;

pub const BQ25703_MINVSYS_STEP_UV: c_int = 256000;
pub const BQ25703_MINVSYS_MIN_UV: c_int = 1024000;
pub const BQ25703_MINVSYS_MAX_UV: c_int = 16128000;

pub const BQ25703_IINDPM_STEP_UA: c_int = 50000;
pub const BQ25703_IINDPM_MIN_UA: c_int = 50000;
pub const BQ25703_IINDPM_MAX_UA: c_int = 6400000;
pub const BQ25703_IINDPM_DEFAULT_UA: c_int = 3300000;
pub const BQ25703_IINDPM_OFFSET_UA: c_int = 50000;

pub const BQ25703_ADCIBAT_CHG_STEP_UA: c_int = 64000;
pub const BQ25703_ADCIBAT_DIS_STEP_UA: c_int = 256000;

pub const BQ25703_ADCIINCMPIN_STEP: c_int = 50000;

pub const BQ25703_ADCVSYSVBAT_OFFSET_UV: c_int = 2880000;
pub const BQ25703_ADCVSYSVBAT_STEP: c_int = 64000;

pub const BQ25792_REG00_MIN_SYS_VOLTAGE: c_uint = 0x00;
pub const BQ25792_REG01_CHARGE_VOLTAGE_LIMIT: c_uint = 0x01;
pub const BQ25792_REG03_CHARGE_CURRENT_LIMIT: c_uint = 0x03;
pub const BQ25792_REG05_INPUT_VOLTAGE_LIMIT: c_uint = 0x05;
pub const BQ25792_REG06_INPUT_CURRENT_LIMIT: c_uint = 0x06;
pub const BQ25792_REG08_PRECHARGE_CONTROL: c_uint = 0x08;
pub const BQ25792_REG09_TERMINATION_CONTROL: c_uint = 0x09;
pub const BQ25792_REG0A_RECHARGE_CONTROL: c_uint = 0x0a;
pub const BQ25792_REG0B_VOTG_REGULATION: c_uint = 0x0b;
pub const BQ25792_REG0D_IOTG_REGULATION: c_uint = 0x0d;
pub const BQ25792_REG0E_TIMER_CONTROL: c_uint = 0x0e;
pub const BQ25792_REG0F_CHARGER_CONTROL_0: c_uint = 0x0f;
pub const BQ25792_REG10_CHARGER_CONTROL_1: c_uint = 0x10;
pub const BQ25792_REG11_CHARGER_CONTROL_2: c_uint = 0x11;
pub const BQ25792_REG12_CHARGER_CONTROL_3: c_uint = 0x12;
pub const BQ25792_REG13_CHARGER_CONTROL_4: c_uint = 0x13;
pub const BQ25792_REG14_CHARGER_CONTROL_5: c_uint = 0x14;
// REG15 reserved
pub const BQ25792_REG16_TEMPERATURE_CONTROL: c_uint = 0x16;
pub const BQ25792_REG17_NTC_CONTROL_0: c_uint = 0x17;
pub const BQ25792_REG18_NTC_CONTROL_1: c_uint = 0x18;
pub const BQ25792_REG19_ICO_CURRENT_LIMIT: c_uint = 0x19;
pub const BQ25792_REG1B_CHARGER_STATUS_0: c_uint = 0x1b;
pub const BQ25792_REG1C_CHARGER_STATUS_1: c_uint = 0x1c;
pub const BQ25792_REG1D_CHARGER_STATUS_2: c_uint = 0x1d;
pub const BQ25792_REG1E_CHARGER_STATUS_3: c_uint = 0x1e;
pub const BQ25792_REG1F_CHARGER_STATUS_4: c_uint = 0x1f;
pub const BQ25792_REG20_FAULT_STATUS_0: c_uint = 0x20;
pub const BQ25792_REG21_FAULT_STATUS_1: c_uint = 0x21;
pub const BQ25792_REG22_CHARGER_FLAG_0: c_uint = 0x22;
pub const BQ25792_REG23_CHARGER_FLAG_1: c_uint = 0x23;
pub const BQ25792_REG24_CHARGER_FLAG_2: c_uint = 0x24;
pub const BQ25792_REG25_CHARGER_FLAG_3: c_uint = 0x25;
pub const BQ25792_REG26_FAULT_FLAG_0: c_uint = 0x26;
pub const BQ25792_REG27_FAULT_FLAG_1: c_uint = 0x27;
pub const BQ25792_REG28_CHARGER_MASK_0: c_uint = 0x28;
pub const BQ25792_REG29_CHARGER_MASK_1: c_uint = 0x29;
pub const BQ25792_REG2A_CHARGER_MASK_2: c_uint = 0x2a;
pub const BQ25792_REG2B_CHARGER_MASK_3: c_uint = 0x2b;
pub const BQ25792_REG2C_FAULT_MASK_0: c_uint = 0x2c;
pub const BQ25792_REG2D_FAULT_MASK_1: c_uint = 0x2d;
pub const BQ25792_REG2E_ADC_CONTROL: c_uint = 0x2e;
pub const BQ25792_REG2F_ADC_FUNCTION_DISABLE_0: c_uint = 0x2f;
pub const BQ25792_REG30_ADC_FUNCTION_DISABLE_1: c_uint = 0x30;
pub const BQ25792_REG31_IBUS_ADC: c_uint = 0x31;
pub const BQ25792_REG33_IBAT_ADC: c_uint = 0x33;
pub const BQ25792_REG35_VBUS_ADC: c_uint = 0x35;
pub const BQ25792_REG37_VAC1_ADC: c_uint = 0x37;
pub const BQ25792_REG39_VAC2_ADC: c_uint = 0x39;
pub const BQ25792_REG3B_VBAT_ADC: c_uint = 0x3b;
pub const BQ25792_REG3D_VSYS_ADC: c_uint = 0x3d;
pub const BQ25792_REG3F_TS_ADC: c_uint = 0x3f;
pub const BQ25792_REG41_TDIE_ADC: c_uint = 0x41;
pub const BQ25792_REG43_DP_ADC: c_uint = 0x43;
pub const BQ25792_REG45_DM_ADC: c_uint = 0x45;
pub const BQ25792_REG47_DPDM_DRIVER: c_uint = 0x47;
pub const BQ25792_REG48_PART_INFORMATION: c_uint = 0x48;
// Minimal System Voltage

pub const BQ25792_MINVSYS_MIN_UV: c_int = 2500000;
pub const BQ25792_MINVSYS_STEP_UV: c_int = 250000;
pub const BQ25792_MINVSYS_MAX_UV: c_int = 16000000;
// Charge Voltage Limit

pub const BQ25792_VBATREG_MIN_UV: c_int = 3000000;
pub const BQ25792_VBATREG_STEP_UV: c_int = 10000;
pub const BQ25792_VBATREG_MAX_UV: c_int = 18800000;
// Charge Current Limit

pub const BQ25792_ICHG_MIN_UA: c_int = 50000;
pub const BQ25792_ICHG_STEP_UA: c_int = 10000;
pub const BQ25792_ICHG_MAX_UA: c_int = 5000000;
// Input Voltage Limit

// Input Current Limit

pub const BQ25792_IINDPM_DEFAULT_UA: c_int = 3000000;
pub const BQ25792_IINDPM_STEP_UA: c_int = 10000;
pub const BQ25792_IINDPM_MIN_UA: c_int = 100000;
pub const BQ25792_IINDPM_MAX_UA: c_int = 3300000;
// Precharge Control

// Termination Control

// Re-charge Control

pub const BQ25792_CELL_1S: c_int = 0;
pub const BQ25792_CELL_2S: c_int = 1;
pub const BQ25792_CELL_3S: c_int = 2;
pub const BQ25792_CELL_4S: c_int = 3;
pub const BQ25792_TRECHG_64MS: c_int = 0;
pub const BQ25792_TRECHG_256MS: c_int = 1;
pub const BQ25792_TRECHG_1024MS: c_int = 2;
pub const BQ25792_TRECHG_2048MS: c_int = 3;
pub const BQ25792_VRECHG_MIN_UV: c_int = 50000;
pub const BQ25792_VRECHG_STEP_UV: c_int = 50000;
pub const BQ25792_VRECHG_MAX_UV: c_int = 800000;
// VOTG regulation

pub const BQ25792_OTG_VOLT_MIN_UV: c_int = 2800000;
pub const BQ25792_OTG_VOLT_STEP_UV: c_int = 10000;
pub const BQ25792_OTG_VOLT_MAX_UV: c_int = 22000000;

// IOTG regulation

pub const BQ25792_OTG_CUR_MIN_UA: c_int = 120000;
pub const BQ25792_OTG_CUR_STEP_UA: c_int = 40000;
pub const BQ25792_OTG_CUR_MAX_UA: c_int = 3320000;
// Timer Control

// Charger Control 0

// bit0 reserved
// Charger Control 1

// Charger Control 2

// Charger Control 3

// Charger Control 4

// Charger Control 5

// bit6 reserved

// Temperature Control

// NTC Control 0

// NTC Control 1

// ICO Current Limit

// Charger Status 0

// Charger Status 1

// Charger Status 2

// Charger Status 3

// Charger Status 4

// FAULT Status 0

// FAULT Status 1

// Charger Flag 0

// Charger Flag 1

// Charger Flag 2

// Charger Flag 3

// FAULT Flag 0

// FAULT Flag 1

// Charger Mask 0

// Charger Mask 1

// Charger Mask 2

// Charger Mask 3

// FAULT Mask 0

// FAULT Mask 1

// ADC Control

// ADC Function Disable 0

// ADC Function Disable 1

// 0x31-0x45: ADC result registers (16-bit, RO): single full-width field
pub const BQ25792_ADCVSYSVBAT_STEP_UV: c_int = 1000;
pub const BQ25792_ADCIBAT_STEP_UA: c_int = 1000;
// DPDM Driver

// Part Information

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bq257xx_type {
    BQ25703A = 1,
    BQ25792,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bq257xx_device {
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub type: bq257xx_type,
}
