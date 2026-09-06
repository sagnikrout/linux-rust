//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/tps65218.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// linux/mfd/tps65218.h
//
// Functions to access TPS65218 power management chip.
//
// Copyright (C) 2014 Texas Instruments Incorporated - https://www.ti.com
//

// TPS chip id list
pub const TPS65218: c_uint = 0xF0;
// I2C ID for TPS65218 part
pub const TPS65218_I2C_ID: c_uint = 0x24;
// All register addresses
pub const TPS65218_REG_CHIPID: c_uint = 0x00;
pub const TPS65218_REG_INT1: c_uint = 0x01;
pub const TPS65218_REG_INT2: c_uint = 0x02;
pub const TPS65218_REG_INT_MASK1: c_uint = 0x03;
pub const TPS65218_REG_INT_MASK2: c_uint = 0x04;
pub const TPS65218_REG_STATUS: c_uint = 0x05;
pub const TPS65218_REG_CONTROL: c_uint = 0x06;
pub const TPS65218_REG_FLAG: c_uint = 0x07;
pub const TPS65218_REG_PASSWORD: c_uint = 0x10;
pub const TPS65218_REG_ENABLE1: c_uint = 0x11;
pub const TPS65218_REG_ENABLE2: c_uint = 0x12;
pub const TPS65218_REG_CONFIG1: c_uint = 0x13;
pub const TPS65218_REG_CONFIG2: c_uint = 0x14;
pub const TPS65218_REG_CONFIG3: c_uint = 0x15;
pub const TPS65218_REG_CONTROL_DCDC1: c_uint = 0x16;
pub const TPS65218_REG_CONTROL_DCDC2: c_uint = 0x17;
pub const TPS65218_REG_CONTROL_DCDC3: c_uint = 0x18;
pub const TPS65218_REG_CONTROL_DCDC4: c_uint = 0x19;
pub const TPS65218_REG_CONTRL_SLEW_RATE: c_uint = 0x1A;
pub const TPS65218_REG_CONTROL_LDO1: c_uint = 0x1B;
pub const TPS65218_REG_SEQ1: c_uint = 0x20;
pub const TPS65218_REG_SEQ2: c_uint = 0x21;
pub const TPS65218_REG_SEQ3: c_uint = 0x22;
pub const TPS65218_REG_SEQ4: c_uint = 0x23;
pub const TPS65218_REG_SEQ5: c_uint = 0x24;
pub const TPS65218_REG_SEQ6: c_uint = 0x25;
pub const TPS65218_REG_SEQ7: c_uint = 0x26;
// Register field definitions
pub const TPS65218_CHIPID_CHIP_MASK: c_uint = 0xF8;
pub const TPS65218_CHIPID_REV_MASK: c_uint = 0x07;
pub const TPS65218_REV_1_0: c_uint = 0x0;
pub const TPS65218_REV_1_1: c_uint = 0x1;
pub const TPS65218_REV_2_0: c_uint = 0x2;
pub const TPS65218_REV_2_1: c_uint = 0x3;

pub const TPS65218_STATUS_STATE_MASK: c_uint = 0xC;
pub const TPS65218_STATUS_CC_STAT: c_uint = 0x3;

pub const TPS65218_CONFIG1_PGDLY_MASK: c_uint = 0x18;

pub const TPS65218_CONFIG1_UVLO_MASK: c_uint = 0x3;
pub const TPS65218_CONFIG1_UVLO_2750000: c_uint = 0x0;
pub const TPS65218_CONFIG1_UVLO_2950000: c_uint = 0x1;
pub const TPS65218_CONFIG1_UVLO_3250000: c_uint = 0x2;
pub const TPS65218_CONFIG1_UVLO_3350000: c_uint = 0x3;

pub const TPS65218_CONFIG2_LS3ILIM_MASK: c_uint = 0xC;
pub const TPS65218_CONFIG2_LS2ILIM_MASK: c_uint = 0x3;

pub const TPS65218_CONTROL_DCDC1_MASK: c_uint = 0x7F;

pub const TPS65218_CONTROL_DCDC2_MASK: c_uint = 0x3F;

pub const TPS65218_CONTROL_DCDC3_MASK: c_uint = 0x3F;

pub const TPS65218_CONTROL_DCDC4_MASK: c_uint = 0x3F;

pub const TPS65218_SLEW_RATE_SLEW_MASK: c_uint = 0x7;
pub const TPS65218_CONTROL_LDO1_MASK: c_uint = 0x3F;

pub const TPS65218_SEQ3_DC2_SEQ_MASK: c_uint = 0xF0;
pub const TPS65218_SEQ3_DC1_SEQ_MASK: c_uint = 0xF;
pub const TPS65218_SEQ4_DC4_SEQ_MASK: c_uint = 0xF0;
pub const TPS65218_SEQ4_DC3_SEQ_MASK: c_uint = 0xF;
pub const TPS65218_SEQ5_DC6_SEQ_MASK: c_uint = 0xF0;
pub const TPS65218_SEQ5_DC5_SEQ_MASK: c_uint = 0xF;
pub const TPS65218_SEQ6_LS1_SEQ_MASK: c_uint = 0xF0;
pub const TPS65218_SEQ6_LDO1_SEQ_MASK: c_uint = 0xF;
pub const TPS65218_SEQ7_GPO3_SEQ_MASK: c_uint = 0xF0;
pub const TPS65218_SEQ7_GPO1_SEQ_MASK: c_uint = 0xF;
pub const TPS65218_PROTECT_NONE: c_int = 0;
pub const TPS65218_PROTECT_L1: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tps65218_regulator_id {
// DCDC's
    TPS65218_DCDC_1,
    TPS65218_DCDC_2,
    TPS65218_DCDC_3,
    TPS65218_DCDC_4,
    TPS65218_DCDC_5,
    TPS65218_DCDC_6,
// LDOs
    TPS65218_LDO_1,
// LS's
    TPS65218_LS_2,
    TPS65218_LS_3,
}

// Number of step-down converters available
pub const TPS65218_NUM_DCDC: c_int = 6;
// Number of LDO voltage regulators available
pub const TPS65218_NUM_LDO: c_int = 1;
// Number of total LS current regulators available
pub const TPS65218_NUM_LS: c_int = 2;
// Number of total regulators available

// Define the TPS65218 IRQ numbers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tps65218_irqs {
// INT1 registers
    TPS65218_PRGC_IRQ,
    TPS65218_CC_AQC_IRQ,
    TPS65218_HOT_IRQ,
    TPS65218_PB_IRQ,
    TPS65218_AC_IRQ,
    TPS65218_VPRG_IRQ,
    TPS65218_INVALID1_IRQ,
    TPS65218_INVALID2_IRQ,
// INT2 registers
    TPS65218_LS1_I_IRQ,
    TPS65218_LS2_I_IRQ,
    TPS65218_LS3_I_IRQ,
    TPS65218_LS1_F_IRQ,
    TPS65218_LS2_F_IRQ,
    TPS65218_LS3_F_IRQ,
    TPS65218_INVALID3_IRQ,
    TPS65218_INVALID4_IRQ,
}

//
// struct tps65218 - tps65218 sub-driver chip access routines
//
// Device data may be used to access the TPS65218 chip
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps65218 {
    pub dev: *mut device,
    pub id: c_uint,
    pub rev: u8,
    pub /: *mut *mut mutex tps_lock; / lock guarding the data structure,
// IRQ Data
    pub irq: c_int,
    pub irq_mask: u32,
    pub irq_data: *mut regmap_irq_chip_data,
    pub desc: [regulator_desc; TPS65218_NUM_REGULATOR],
    pub regmap: *mut regmap,
    pub strobes: *mut u8,
}
