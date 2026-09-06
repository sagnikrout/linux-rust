//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/regulator/mpq7920.h
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
// mpq7920.h  -  Regulator definitions for mpq7920
//
// Copyright 2019 Monolithic Power Systems, Inc
//
pub const MPQ7920_REG_CTL0: c_uint = 0x00;
pub const MPQ7920_REG_CTL1: c_uint = 0x01;
pub const MPQ7920_REG_CTL2: c_uint = 0x02;
pub const MPQ7920_BUCK1_REG_A: c_uint = 0x03;
pub const MPQ7920_BUCK1_REG_B: c_uint = 0x04;
pub const MPQ7920_BUCK1_REG_C: c_uint = 0x05;
pub const MPQ7920_BUCK1_REG_D: c_uint = 0x06;
pub const MPQ7920_BUCK2_REG_A: c_uint = 0x07;
pub const MPQ7920_BUCK2_REG_B: c_uint = 0x08;
pub const MPQ7920_BUCK2_REG_C: c_uint = 0x09;
pub const MPQ7920_BUCK2_REG_D: c_uint = 0x0a;
pub const MPQ7920_BUCK3_REG_A: c_uint = 0x0b;
pub const MPQ7920_BUCK3_REG_B: c_uint = 0x0c;
pub const MPQ7920_BUCK3_REG_C: c_uint = 0x0d;
pub const MPQ7920_BUCK3_REG_D: c_uint = 0x0e;
pub const MPQ7920_BUCK4_REG_A: c_uint = 0x0f;
pub const MPQ7920_BUCK4_REG_B: c_uint = 0x10;
pub const MPQ7920_BUCK4_REG_C: c_uint = 0x11;
pub const MPQ7920_BUCK4_REG_D: c_uint = 0x12;
pub const MPQ7920_LDO1_REG_A: c_uint = 0x13;
pub const MPQ7920_LDO1_REG_B: c_uint = 0x0;
pub const MPQ7920_LDO2_REG_A: c_uint = 0x14;
pub const MPQ7920_LDO2_REG_B: c_uint = 0x15;
pub const MPQ7920_LDO2_REG_C: c_uint = 0x16;
pub const MPQ7920_LDO3_REG_A: c_uint = 0x17;
pub const MPQ7920_LDO3_REG_B: c_uint = 0x18;
pub const MPQ7920_LDO3_REG_C: c_uint = 0x19;
pub const MPQ7920_LDO4_REG_A: c_uint = 0x1a;
pub const MPQ7920_LDO4_REG_B: c_uint = 0x1b;
pub const MPQ7920_LDO4_REG_C: c_uint = 0x1c;
pub const MPQ7920_LDO5_REG_A: c_uint = 0x1d;
pub const MPQ7920_LDO5_REG_B: c_uint = 0x1e;
pub const MPQ7920_LDO5_REG_C: c_uint = 0x1f;
pub const MPQ7920_REG_MODE: c_uint = 0x20;
pub const MPQ7920_REG_REGULATOR_EN: c_uint = 0x22;
pub const MPQ7920_MASK_VREF: c_uint = 0x7f;
pub const MPQ7920_MASK_BUCK_ILIM: c_uint = 0xc0;

pub const MPQ7920_MASK_MODE: c_uint = 0xc0;
pub const MPQ7920_MASK_SOFTSTART: c_uint = 0x0c;
pub const MPQ7920_MASK_SWITCH_FREQ: c_uint = 0x30;
pub const MPQ7920_MASK_BUCK_PHASE_DEALY: c_uint = 0x30;
pub const MPQ7920_MASK_DVS_SLEWRATE: c_uint = 0xc0;
pub const MPQ7920_MASK_OVP: c_uint = 0x40;

pub const MPQ7920_REGULATOR_EN_OFFSET: c_int = 7;
// values in mV
pub const MPQ7920_BUCK_VOLT_MIN: c_int = 400000;
pub const MPQ7920_LDO_VOLT_MIN: c_int = 650000;
pub const MPQ7920_VOLT_MAX: c_int = 3587500;
pub const MPQ7920_VOLT_STEP: c_int = 12500;
