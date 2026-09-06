//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/tps65219.h
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
// Functions to access TPS65215/TPS65219 Power Management Integrated Chips
//
// Copyright (C) 2022 BayLibre Incorporated - https://www.baylibre.com
// Copyright (C) 2024 Texas Instruments Incorporated - https://www.ti.com
//

// Chip id list
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pmic_id {
    TPS65214,
    TPS65215,
    TPS65219,
}

// I2C ID for TPS65219 part
pub const TPS65219_I2C_ID: c_uint = 0x24;
// All register addresses
pub const TPS65219_REG_TI_DEV_ID: c_uint = 0x00;
pub const TPS65219_REG_NVM_ID: c_uint = 0x01;
pub const TPS65219_REG_ENABLE_CTRL: c_uint = 0x02;
pub const TPS65219_REG_BUCKS_CONFIG: c_uint = 0x03;
pub const TPS65214_REG_LOCK: c_uint = 0x03;
pub const TPS65219_REG_LDO4_VOUT: c_uint = 0x04;
pub const TPS65214_REG_LDO1_VOUT_STBY: c_uint = 0x04;
pub const TPS65219_REG_LDO3_VOUT: c_uint = 0x05;
pub const TPS65215_REG_LDO2_VOUT: c_uint = 0x05;
pub const TPS65214_REG_LDO1_VOUT: c_uint = 0x05;
pub const TPS65219_REG_LDO2_VOUT: c_uint = 0x06;
pub const TPS65214_REG_LDO2_VOUT: c_uint = 0x06;
pub const TPS65219_REG_LDO1_VOUT: c_uint = 0x07;
pub const TPS65214_REG_LDO2_VOUT_STBY: c_uint = 0x07;
pub const TPS65219_REG_BUCK3_VOUT: c_uint = 0x8;
pub const TPS65219_REG_BUCK2_VOUT: c_uint = 0x9;
pub const TPS65219_REG_BUCK1_VOUT: c_uint = 0xA;
pub const TPS65219_REG_LDO4_SEQUENCE_SLOT: c_uint = 0xB;
pub const TPS65219_REG_LDO3_SEQUENCE_SLOT: c_uint = 0xC;
pub const TPS65215_REG_LDO2_SEQUENCE_SLOT: c_uint = 0xC;
pub const TPS65214_REG_LDO1_SEQUENCE_SLOT: c_uint = 0xC;
pub const TPS65219_REG_LDO2_SEQUENCE_SLOT: c_uint = 0xD;
pub const TPS65219_REG_LDO1_SEQUENCE_SLOT: c_uint = 0xE;
pub const TPS65219_REG_BUCK3_SEQUENCE_SLOT: c_uint = 0xF;
pub const TPS65219_REG_BUCK2_SEQUENCE_SLOT: c_uint = 0x10;
pub const TPS65219_REG_BUCK1_SEQUENCE_SLOT: c_uint = 0x11;
pub const TPS65219_REG_nRST_SEQUENCE_SLOT: c_uint = 0x12;
pub const TPS65219_REG_GPIO_SEQUENCE_SLOT: c_uint = 0x13;
pub const TPS65219_REG_GPO2_SEQUENCE_SLOT: c_uint = 0x14;
pub const TPS65214_REG_GPIO_GPI_SEQUENCE_SLOT: c_uint = 0x14;
pub const TPS65219_REG_GPO1_SEQUENCE_SLOT: c_uint = 0x15;
pub const TPS65214_REG_GPO_SEQUENCE_SLOT: c_uint = 0x15;
pub const TPS65219_REG_POWER_UP_SLOT_DURATION_1: c_uint = 0x16;
pub const TPS65219_REG_POWER_UP_SLOT_DURATION_2: c_uint = 0x17;
// _SLOT_DURATION_3 doesn't apply to TPS65215
pub const TPS65219_REG_POWER_UP_SLOT_DURATION_3: c_uint = 0x18;
pub const TPS65219_REG_POWER_UP_SLOT_DURATION_4: c_uint = 0x19;
pub const TPS65214_REG_BUCK3_VOUT_STBY: c_uint = 0x19;
pub const TPS65219_REG_POWER_DOWN_SLOT_DURATION_1: c_uint = 0x1A;
pub const TPS65219_REG_POWER_DOWN_SLOT_DURATION_2: c_uint = 0x1B;
pub const TPS65219_REG_POWER_DOWN_SLOT_DURATION_3: c_uint = 0x1C;
pub const TPS65214_REG_BUCK2_VOUT_STBY: c_uint = 0x1C;
pub const TPS65219_REG_POWER_DOWN_SLOT_DURATION_4: c_uint = 0x1D;
pub const TPS65214_REG_BUCK1_VOUT_STBY: c_uint = 0x1D;
pub const TPS65219_REG_GENERAL_CONFIG: c_uint = 0x1E;
pub const TPS65219_REG_MFP_1_CONFIG: c_uint = 0x1F;
pub const TPS65219_REG_MFP_2_CONFIG: c_uint = 0x20;
pub const TPS65219_REG_STBY_1_CONFIG: c_uint = 0x21;
pub const TPS65219_REG_STBY_2_CONFIG: c_uint = 0x22;
pub const TPS65219_REG_OC_DEGL_CONFIG: c_uint = 0x23;
// 'sub irq' MASK registers
pub const TPS65219_REG_INT_MASK_UV: c_uint = 0x24;
pub const TPS65219_REG_MASK_CONFIG: c_uint = 0x25;
pub const TPS65219_REG_I2C_ADDRESS_REG: c_uint = 0x26;
pub const TPS65219_REG_USER_GENERAL_NVM_STORAGE: c_uint = 0x27;
pub const TPS65219_REG_MANUFACTURING_VER: c_uint = 0x28;
pub const TPS65219_REG_MFP_CTRL: c_uint = 0x29;
pub const TPS65219_REG_DISCHARGE_CONFIG: c_uint = 0x2A;
// main irq registers
pub const TPS65219_REG_INT_SOURCE: c_uint = 0x2B;
// TPS65219 'sub irq' registers
pub const TPS65219_REG_INT_LDO_3_4: c_uint = 0x2C;
pub const TPS65219_REG_INT_LDO_1_2: c_uint = 0x2D;
// TPS65215 specific 'sub irq' registers
pub const TPS65215_REG_INT_LDO_2: c_uint = 0x2C;
pub const TPS65215_REG_INT_LDO_1: c_uint = 0x2D;
// TPS65214 specific 'sub irq' register
pub const TPS65214_REG_INT_LDO_1_2: c_uint = 0x2D;
// Common TPS65215 & TPS65219 'sub irq' registers
pub const TPS65219_REG_INT_BUCK_3: c_uint = 0x2E;
pub const TPS65219_REG_INT_BUCK_1_2: c_uint = 0x2F;
pub const TPS65219_REG_INT_SYSTEM: c_uint = 0x30;
pub const TPS65219_REG_INT_RV: c_uint = 0x31;
pub const TPS65219_REG_INT_TIMEOUT_RV_SD: c_uint = 0x32;
pub const TPS65219_REG_INT_PB: c_uint = 0x33;
pub const TPS65219_REG_INT_LDO_3_4_POS: c_int = 0;
pub const TPS65219_REG_INT_LDO_1_2_POS: c_int = 1;
pub const TPS65219_REG_INT_BUCK_3_POS: c_int = 2;
pub const TPS65219_REG_INT_BUCK_1_2_POS: c_int = 3;
pub const TPS65219_REG_INT_SYS_POS: c_int = 4;
pub const TPS65219_REG_INT_RV_POS: c_int = 5;
pub const TPS65219_REG_INT_TO_RV_POS: c_int = 6;
pub const TPS65219_REG_INT_PB_POS: c_int = 7;
pub const TPS65215_REG_INT_LDO_2_POS: c_int = 0;
pub const TPS65215_REG_INT_LDO_1_POS: c_int = 1;
pub const TPS65214_REG_INT_LDO_1_2_POS: c_int = 0;
pub const TPS65214_REG_INT_BUCK_3_POS: c_int = 1;
pub const TPS65214_REG_INT_BUCK_1_2_POS: c_int = 2;
pub const TPS65214_REG_INT_SYS_POS: c_int = 3;
pub const TPS65214_REG_INT_RV_POS: c_int = 4;
pub const TPS65214_REG_INT_TO_RV_POS: c_int = 5;
pub const TPS65214_REG_INT_PB_POS: c_int = 6;
pub const TPS65219_REG_USER_NVM_CMD: c_uint = 0x34;
pub const TPS65219_REG_POWER_UP_STATUS: c_uint = 0x35;
pub const TPS65219_REG_SPARE_2: c_uint = 0x36;
pub const TPS65219_REG_SPARE_3: c_uint = 0x37;
pub const TPS65219_REG_FACTORY_CONFIG_2: c_uint = 0x41;
// Register field definitions

pub const LDO_BYP_SHIFT: c_int = 6;

// Regulators enable control

// Register Unlock
pub const TPS65214_LOCK_ACCESS_CMD: c_uint = 0x5a;
// power ON-OFF sequence slot

// TODO: Not needed, same mapping as TPS65219_ENABLE_REGNAME_EN, factorize

// STBY_2 config

// MFP Control

// MFP_1 Config

// MFP_2 Config

pub const TPS65219_MFP_2_EN: c_int = 0;

// MASK_UV Config

// MASK Config
// SENSOR_N_WARM_MASK already defined in Thermal

// UnderVoltage - Short to GND - OverCurrent
// LDO3-4: only for TPS65219

// LDO1-2: TPS65214 & TPS65219

// TPS65215 LDO1-2

// BUCK3

// BUCK1-2

// Thermal Sensor: TPS65219/TPS65215

// Thermal Sensor: TPS65219/TPS65215/TPS65214

// Residual Voltage

// Residual Voltage ShutDown

// Power Button

pub const TPS65219_PB_POS: c_int = 7;
pub const TPS65219_TO_RV_POS: c_int = 6;
pub const TPS65219_RV_POS: c_int = 5;
pub const TPS65219_SYS_POS: c_int = 4;
pub const TPS65219_BUCK_1_2_POS: c_int = 3;
pub const TPS65219_BUCK_3_POS: c_int = 2;
pub const TPS65219_LDO_1_2_POS: c_int = 1;
pub const TPS65219_LDO_3_4_POS: c_int = 0;
// IRQs
// LDO3-4 register IRQs
// TPS65215 LDO1
// TPS65215 LDO2
// LDO1-2: TPS65219/TPS65214
// BUCK3
// BUCK1-2
// Thermal Sensor
// Residual Voltage
// Residual Voltage ShutDown
// Power Button
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tps65214_regulator_id {
//
// DCDC's same as TPS65219
// LDO1 maps to TPS65219's LDO3
// LDO2 is the same as TPS65219
//
    TPS65214_LDO_1 = 3,
    TPS65214_LDO_2 = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tps65215_regulator_id {
// DCDC's same as TPS65219
// LDO1 is the same as TPS65219
    TPS65215_LDO_2 = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tps65219_regulator_id {
// DCDC's
    TPS65219_BUCK_1,
    TPS65219_BUCK_2,
    TPS65219_BUCK_3,
// LDOs
    TPS65219_LDO_1,
    TPS65219_LDO_2,
    TPS65219_LDO_3,
    TPS65219_LDO_4,
}

// Number of step-down converters available
pub const TPS6521X_NUM_BUCKS: c_int = 3;
// Number of LDO voltage regulators available
pub const TPS65219_NUM_LDO: c_int = 4;
pub const TPS65215_NUM_LDO: c_int = 2;
pub const TPS65214_NUM_LDO: c_int = 2;
// Number of total regulators available

// Define the TPS65214 IRQ numbers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tps65214_irqs {
// INT source registers
    TPS65214_TO_RV_SD_SET_IRQ,
    TPS65214_RV_SET_IRQ,
    TPS65214_SYS_SET_IRQ,
    TPS65214_BUCK_1_2_SET_IRQ,
    TPS65214_BUCK_3_SET_IRQ,
    TPS65214_LDO_1_2_SET_IRQ,
    TPS65214_PB_SET_IRQ = 7,
}

// Define the TPS65215 IRQ numbers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tps65215_irqs {
// INT source registers
    TPS65215_TO_RV_SD_SET_IRQ,
    TPS65215_RV_SET_IRQ,
    TPS65215_SYS_SET_IRQ,
    TPS65215_BUCK_1_2_SET_IRQ,
    TPS65215_BUCK_3_SET_IRQ,
    TPS65215_LDO_1_SET_IRQ,
    TPS65215_LDO_2_SET_IRQ,
    TPS65215_PB_SET_IRQ,
}

// Define the TPS65219 IRQ numbers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tps65219_irqs {
// INT source registers
    TPS65219_TO_RV_SD_SET_IRQ,
    TPS65219_RV_SET_IRQ,
    TPS65219_SYS_SET_IRQ,
    TPS65219_BUCK_1_2_SET_IRQ,
    TPS65219_BUCK_3_SET_IRQ,
    TPS65219_LDO_1_2_SET_IRQ,
    TPS65219_LDO_3_4_SET_IRQ,
    TPS65219_PB_SET_IRQ,
}

//
// struct tps65219 - tps65219 sub-driver chip access routines
//
// Device data may be used to access the TPS65219 chip
//
// @dev: MFD device
// @regmap: Regmap for accessing the device registers
// @irq_data: Regmap irq data used for the irq chip
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps65219 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub irq_data: *mut regmap_irq_chip_data,
}
