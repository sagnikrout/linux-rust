//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/regulator/pv88080-regulator.h
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
// pv88080-regulator.h - Regulator definitions for PV88080
// Copyright (C) 2016 Powerventure Semiconductor Ltd.
//
// System Control and Event Registers
pub const PV88080_REG_EVENT_A: c_uint = 0x04;
pub const PV88080_REG_MASK_A: c_uint = 0x09;
pub const PV88080_REG_MASK_B: c_uint = 0x0A;
pub const PV88080_REG_MASK_C: c_uint = 0x0B;
// Regulator Registers - rev. AA
pub const PV88080AA_REG_HVBUCK_CONF1: c_uint = 0x2D;
pub const PV88080AA_REG_HVBUCK_CONF2: c_uint = 0x2E;
pub const PV88080AA_REG_BUCK1_CONF0: c_uint = 0x27;
pub const PV88080AA_REG_BUCK1_CONF1: c_uint = 0x28;
pub const PV88080AA_REG_BUCK1_CONF2: c_uint = 0x59;
pub const PV88080AA_REG_BUCK1_CONF5: c_uint = 0x5C;
pub const PV88080AA_REG_BUCK2_CONF0: c_uint = 0x29;
pub const PV88080AA_REG_BUCK2_CONF1: c_uint = 0x2A;
pub const PV88080AA_REG_BUCK2_CONF2: c_uint = 0x61;
pub const PV88080AA_REG_BUCK2_CONF5: c_uint = 0x64;
pub const PV88080AA_REG_BUCK3_CONF0: c_uint = 0x2B;
pub const PV88080AA_REG_BUCK3_CONF1: c_uint = 0x2C;
pub const PV88080AA_REG_BUCK3_CONF2: c_uint = 0x69;
pub const PV88080AA_REG_BUCK3_CONF5: c_uint = 0x6C;
// Regulator Registers - rev. BA
pub const PV88080BA_REG_HVBUCK_CONF1: c_uint = 0x33;
pub const PV88080BA_REG_HVBUCK_CONF2: c_uint = 0x34;
pub const PV88080BA_REG_BUCK1_CONF0: c_uint = 0x2A;
pub const PV88080BA_REG_BUCK1_CONF1: c_uint = 0x2C;
pub const PV88080BA_REG_BUCK1_CONF2: c_uint = 0x5A;
pub const PV88080BA_REG_BUCK1_CONF5: c_uint = 0x5D;
pub const PV88080BA_REG_BUCK2_CONF0: c_uint = 0x2D;
pub const PV88080BA_REG_BUCK2_CONF1: c_uint = 0x2F;
pub const PV88080BA_REG_BUCK2_CONF2: c_uint = 0x63;
pub const PV88080BA_REG_BUCK2_CONF5: c_uint = 0x66;
pub const PV88080BA_REG_BUCK3_CONF0: c_uint = 0x30;
pub const PV88080BA_REG_BUCK3_CONF1: c_uint = 0x32;
pub const PV88080BA_REG_BUCK3_CONF2: c_uint = 0x6C;
pub const PV88080BA_REG_BUCK3_CONF5: c_uint = 0x6F;
// PV88080_REG_EVENT_A (addr=0x04)
pub const PV88080_E_VDD_FLT: c_uint = 0x01;
pub const PV88080_E_OVER_TEMP: c_uint = 0x02;
// PV88080_REG_MASK_A (addr=0x09)
pub const PV88080_M_VDD_FLT: c_uint = 0x01;
pub const PV88080_M_OVER_TEMP: c_uint = 0x02;
// PV88080_REG_BUCK1_CONF0 (addr=0x27|0x2A)
pub const PV88080_BUCK1_EN: c_uint = 0x80;
pub const PV88080_VBUCK1_MASK: c_uint = 0x7F;
// PV88080_REG_BUCK2_CONF0 (addr=0x29|0x2D)
pub const PV88080_BUCK2_EN: c_uint = 0x80;
pub const PV88080_VBUCK2_MASK: c_uint = 0x7F;
// PV88080_REG_BUCK3_CONF0 (addr=0x2B|0x30)
pub const PV88080_BUCK3_EN: c_uint = 0x80;
pub const PV88080_VBUCK3_MASK: c_uint = 0x7F;
// PV88080_REG_BUCK1_CONF1 (addr=0x28|0x2C)
pub const PV88080_BUCK1_ILIM_SHIFT: c_int = 2;
pub const PV88080_BUCK1_ILIM_MASK: c_uint = 0x0C;
pub const PV88080_BUCK1_MODE_MASK: c_uint = 0x03;
// PV88080_REG_BUCK2_CONF1 (addr=0x2A|0x2F)
pub const PV88080_BUCK2_ILIM_SHIFT: c_int = 2;
pub const PV88080_BUCK2_ILIM_MASK: c_uint = 0x0C;
pub const PV88080_BUCK2_MODE_MASK: c_uint = 0x03;
// PV88080_REG_BUCK3_CONF1 (addr=0x2C|0x32)
pub const PV88080_BUCK3_ILIM_SHIFT: c_int = 2;
pub const PV88080_BUCK3_ILIM_MASK: c_uint = 0x0C;
pub const PV88080_BUCK3_MODE_MASK: c_uint = 0x03;
pub const PV88080_BUCK_MODE_SLEEP: c_uint = 0x00;
pub const PV88080_BUCK_MODE_AUTO: c_uint = 0x01;
pub const PV88080_BUCK_MODE_SYNC: c_uint = 0x02;
// PV88080_REG_HVBUCK_CONF1 (addr=0x2D|0x33)
pub const PV88080_VHVBUCK_MASK: c_uint = 0xFF;
// PV88080_REG_HVBUCK_CONF1 (addr=0x2E|0x34)
pub const PV88080_HVBUCK_EN: c_uint = 0x01;
// PV88080_REG_BUCK2_CONF2 (addr=0x61|0x63)
// PV88080_REG_BUCK3_CONF2 (addr=0x69|0x6C)
pub const PV88080_BUCK_VDAC_RANGE_SHIFT: c_int = 7;
pub const PV88080_BUCK_VDAC_RANGE_MASK: c_uint = 0x01;
pub const PV88080_BUCK_VDAC_RANGE_1: c_uint = 0x00;
pub const PV88080_BUCK_VDAC_RANGE_2: c_uint = 0x01;
// PV88080_REG_BUCK2_CONF5 (addr=0x64|0x66)
// PV88080_REG_BUCK3_CONF5 (addr=0x6C|0x6F)
pub const PV88080_BUCK_VRANGE_GAIN_SHIFT: c_int = 0;
pub const PV88080_BUCK_VRANGE_GAIN_MASK: c_uint = 0x01;
pub const PV88080_BUCK_VRANGE_GAIN_1: c_uint = 0x00;
pub const PV88080_BUCK_VRANGE_GAIN_2: c_uint = 0x01;
