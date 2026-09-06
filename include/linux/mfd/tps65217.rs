//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/tps65217.h
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
// linux/mfd/tps65217.h
//
// Functions to access TPS65217 power management chip.
//
// Copyright (C) 2011 Texas Instruments Incorporated - https://www.ti.com
//

// TPS chip id list
pub const TPS65217: c_uint = 0xF0;
// I2C ID for TPS65217 part
pub const TPS65217_I2C_ID: c_uint = 0x24;
// All register addresses

// Register field definitions
pub const TPS65217_CHIPID_CHIP_MASK: c_uint = 0xF0;
pub const TPS65217_CHIPID_REV_MASK: c_uint = 0x0F;

pub const TPS65217_PPATH_AC_CURRENT_MASK: c_uint = 0x0C;
pub const TPS65217_PPATH_USB_CURRENT_MASK: c_uint = 0x03;

pub const TPS65217_INT_SHIFT: c_int = 4;

pub const TPS65217_CHGCONFIG1_TMR_MASK: c_uint = 0xC0;

pub const TPS65217_CHGCONFIG2_VOREG_MASK: c_uint = 0x30;
pub const TPS65217_CHGCONFIG3_ICHRG_MASK: c_uint = 0xC0;
pub const TPS65217_CHGCONFIG3_DPPMTH_MASK: c_uint = 0x30;

pub const TPS65217_CHGCONFIG2_TERMIF: c_uint = 0x06;

pub const TPS65217_WLEDCTRL1_FDIM_MASK: c_uint = 0x03;
pub const TPS65217_WLEDCTRL2_DUTY_MASK: c_uint = 0x7F;
pub const TPS65217_MUXCTRL_MUX_MASK: c_uint = 0x07;

pub const TPS65217_PASSWORD_REGS_UNLOCK: c_uint = 0x7D;

pub const TPS65217_DEFPG_PGDLY_MASK: c_uint = 0x03;

pub const TPS65217_DEFDCDCX_DCDC_MASK: c_uint = 0x3F;

pub const TPS65217_DEFSLEW_SLEW_MASK: c_uint = 0x07;
pub const TPS65217_DEFLDO1_LDO1_MASK: c_uint = 0x0F;

pub const TPS65217_DEFLDO2_LDO2_MASK: c_uint = 0x3F;

pub const TPS65217_DEFLDO3_LDO3_MASK: c_uint = 0x1F;

pub const TPS65217_DEFLDO4_LDO4_MASK: c_uint = 0x1F;

pub const TPS65217_DEFUVLO_UVLO_MASK: c_uint = 0x03;
pub const TPS65217_SEQ1_DC1_SEQ_MASK: c_uint = 0xF0;
pub const TPS65217_SEQ1_DC2_SEQ_MASK: c_uint = 0x0F;
pub const TPS65217_SEQ2_DC3_SEQ_MASK: c_uint = 0xF0;
pub const TPS65217_SEQ2_LDO1_SEQ_MASK: c_uint = 0x0F;
pub const TPS65217_SEQ3_LDO2_SEQ_MASK: c_uint = 0xF0;
pub const TPS65217_SEQ3_LDO3_SEQ_MASK: c_uint = 0x0F;
pub const TPS65217_SEQ4_LDO4_SEQ_MASK: c_uint = 0xF0;
pub const TPS65217_SEQ5_DLY1_MASK: c_uint = 0xC0;
pub const TPS65217_SEQ5_DLY2_MASK: c_uint = 0x30;
pub const TPS65217_SEQ5_DLY3_MASK: c_uint = 0x0C;
pub const TPS65217_SEQ5_DLY4_MASK: c_uint = 0x03;
pub const TPS65217_SEQ6_DLY5_MASK: c_uint = 0xC0;
pub const TPS65217_SEQ6_DLY6_MASK: c_uint = 0x30;

pub const TPS65217_MAX_REGISTER: c_uint = 0x1E;
pub const TPS65217_PROTECT_NONE: c_int = 0;
pub const TPS65217_PROTECT_L1: c_int = 1;
pub const TPS65217_PROTECT_L2: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tps65217_regulator_id {
// DCDC's
    TPS65217_DCDC_1,
    TPS65217_DCDC_2,
    TPS65217_DCDC_3,
// LDOs
    TPS65217_LDO_1,
    TPS65217_LDO_2,
    TPS65217_LDO_3,
    TPS65217_LDO_4,
}

// Number of step-down converters available
pub const TPS65217_NUM_DCDC: c_int = 3;
// Number of LDO voltage regulators available
pub const TPS65217_NUM_LDO: c_int = 4;
// Number of total regulators available

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tps65217_bl_isel {
    TPS65217_BL_ISET1 = 1,
    TPS65217_BL_ISET2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tps65217_bl_fdim {
    TPS65217_BL_FDIM_100HZ,
    TPS65217_BL_FDIM_200HZ,
    TPS65217_BL_FDIM_500HZ,
    TPS65217_BL_FDIM_1000HZ,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps65217_bl_pdata {
    pub isel: tps65217_bl_isel,
    pub fdim: tps65217_bl_fdim,
    pub dft_brightness: c_int,
}

// Interrupt numbers
pub const TPS65217_IRQ_USB: c_int = 0;
pub const TPS65217_IRQ_AC: c_int = 1;
pub const TPS65217_IRQ_PB: c_int = 2;
pub const TPS65217_NUM_IRQ: c_int = 3;
//
// struct tps65217_board - packages regulator init data
// @tps65217_regulator_data: regulator initialization values
//
// Board data may be used to initialize regulator.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps65217_board {
    pub tps65217_init_data: [*mut regulator_init_data; TPS65217_NUM_REGULATOR],
    pub of_node: [*mut device_node; TPS65217_NUM_REGULATOR],
    pub bl_pdata: *mut tps65217_bl_pdata,
}

//
// struct tps65217 - tps65217 sub-driver chip access routines
//
// Device data may be used to access the TPS65217 chip
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps65217 {
    pub dev: *mut device,
    pub pdata: *mut tps65217_board,
    pub desc: [regulator_desc; TPS65217_NUM_REGULATOR],
    pub regmap: *mut regmap,
    pub strobes: *mut u8,
    pub irq_domain: *mut irq_domain,
    pub irq_lock: mutex,
    pub irq_mask: u8,
    pub irq: c_int,
}

extern "C" {
    pub fn dev_get_drvdata(_arg: dev) -> return;
}
