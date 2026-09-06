//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/regulator/pv88090-regulator.h
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
// pv88090-regulator.h - Regulator definitions for PV88090
// Copyright (C) 2015 Powerventure Semiconductor Ltd.
//
// System Control and Event Registers
pub const PV88090_REG_EVENT_A: c_uint = 0x03;
pub const PV88090_REG_MASK_A: c_uint = 0x06;
pub const PV88090_REG_MASK_B: c_uint = 0x07;
// Regulator Registers
pub const PV88090_REG_BUCK1_CONF0: c_uint = 0x18;
pub const PV88090_REG_BUCK1_CONF1: c_uint = 0x19;
pub const PV88090_REG_BUCK1_CONF2: c_uint = 0x1a;
pub const PV88090_REG_BUCK2_CONF0: c_uint = 0x1b;
pub const PV88090_REG_BUCK2_CONF1: c_uint = 0x1c;
pub const PV88090_REG_BUCK2_CONF2: c_uint = 0x58;
pub const PV88090_REG_BUCK3_CONF0: c_uint = 0x1d;
pub const PV88090_REG_BUCK3_CONF1: c_uint = 0x1e;
pub const PV88090_REG_BUCK3_CONF2: c_uint = 0x5c;
pub const PV88090_REG_LDO1_CONT: c_uint = 0x1f;
pub const PV88090_REG_LDO2_CONT: c_uint = 0x20;
pub const PV88090_REG_LDO3_CONT: c_uint = 0x21;
pub const PV88090_REG_BUCK_FOLD_RANGE: c_uint = 0x61;
// PV88090_REG_EVENT_A (addr=0x03)
pub const PV88090_E_VDD_FLT: c_uint = 0x01;
pub const PV88090_E_OVER_TEMP: c_uint = 0x02;
// PV88090_REG_MASK_A (addr=0x06)
pub const PV88090_M_VDD_FLT: c_uint = 0x01;
pub const PV88090_M_OVER_TEMP: c_uint = 0x02;
// PV88090_REG_BUCK1_CONF0 (addr=0x18)
pub const PV88090_BUCK1_EN: c_uint = 0x80;
pub const PV88090_VBUCK1_MASK: c_uint = 0x7F;
// PV88090_REG_BUCK2_CONF0 (addr=0x1b)
pub const PV88090_BUCK2_EN: c_uint = 0x80;
pub const PV88090_VBUCK2_MASK: c_uint = 0x7F;
// PV88090_REG_BUCK3_CONF0 (addr=0x1d)
pub const PV88090_BUCK3_EN: c_uint = 0x80;
pub const PV88090_VBUCK3_MASK: c_uint = 0x7F;
// PV88090_REG_LDO1_CONT (addr=0x1f)
pub const PV88090_LDO1_EN: c_uint = 0x40;
pub const PV88090_VLDO1_MASK: c_uint = 0x3F;
// PV88090_REG_LDO2_CONT (addr=0x20)
pub const PV88090_LDO2_EN: c_uint = 0x40;
pub const PV88090_VLDO2_MASK: c_uint = 0x3F;
// PV88090_REG_BUCK1_CONF1 (addr=0x19)
pub const PV88090_BUCK1_ILIM_SHIFT: c_int = 2;
pub const PV88090_BUCK1_ILIM_MASK: c_uint = 0x7C;
pub const PV88090_BUCK1_MODE_MASK: c_uint = 0x03;
// PV88090_REG_BUCK2_CONF1 (addr=0x1c)
pub const PV88090_BUCK2_ILIM_SHIFT: c_int = 2;
pub const PV88090_BUCK2_ILIM_MASK: c_uint = 0x0C;
pub const PV88090_BUCK2_MODE_MASK: c_uint = 0x03;
// PV88090_REG_BUCK3_CONF1 (addr=0x1e)
pub const PV88090_BUCK3_ILIM_SHIFT: c_int = 2;
pub const PV88090_BUCK3_ILIM_MASK: c_uint = 0x0C;
pub const PV88090_BUCK3_MODE_MASK: c_uint = 0x03;
pub const PV88090_BUCK_MODE_SLEEP: c_uint = 0x00;
pub const PV88090_BUCK_MODE_AUTO: c_uint = 0x01;
pub const PV88090_BUCK_MODE_SYNC: c_uint = 0x02;
// PV88090_REG_BUCK2_CONF2 (addr=0x58)
// PV88090_REG_BUCK3_CONF2 (addr=0x5c)
pub const PV88090_BUCK_VDAC_RANGE_SHIFT: c_int = 7;
pub const PV88090_BUCK_VDAC_RANGE_MASK: c_uint = 0x01;
pub const PV88090_BUCK_VDAC_RANGE_1: c_uint = 0x00;
pub const PV88090_BUCK_VDAC_RANGE_2: c_uint = 0x01;
// PV88090_REG_BUCK_FOLD_RANGE (addr=0x61)
pub const PV88090_BUCK_VRANGE_GAIN_SHIFT: c_int = 3;
pub const PV88090_BUCK_VRANGE_GAIN_MASK: c_uint = 0x01;
pub const PV88090_BUCK_VRANGE_GAIN_1: c_uint = 0x00;
pub const PV88090_BUCK_VRANGE_GAIN_2: c_uint = 0x01;
