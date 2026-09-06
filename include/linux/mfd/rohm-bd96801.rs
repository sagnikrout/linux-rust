//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/rohm-bd96801.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2024 ROHM Semiconductors
pub const BD96801_REG_SSCG_CTRL: c_uint = 0x09;
pub const BD96801_REG_SHD_INTB: c_uint = 0x20;
pub const BD96801_LDO5_VOL_LVL_REG: c_uint = 0x2c;
pub const BD96801_LDO6_VOL_LVL_REG: c_uint = 0x2d;
pub const BD96801_LDO7_VOL_LVL_REG: c_uint = 0x2e;
pub const BD96801_REG_BUCK_OVP: c_uint = 0x30;
pub const BD96801_REG_BUCK_OVD: c_uint = 0x35;
pub const BD96801_REG_LDO_OVP: c_uint = 0x31;
pub const BD96801_REG_LDO_OVD: c_uint = 0x36;
pub const BD96801_REG_BOOT_OVERTIME: c_uint = 0x3a;
pub const BD96801_REG_WD_TMO: c_uint = 0x40;
pub const BD96801_REG_WD_CONF: c_uint = 0x41;
pub const BD96801_REG_WD_FEED: c_uint = 0x42;
pub const BD96801_REG_WD_FAILCOUNT: c_uint = 0x43;
pub const BD96801_REG_WD_ASK: c_uint = 0x46;
pub const BD96801_REG_WD_STATUS: c_uint = 0x4a;
pub const BD96801_REG_PMIC_STATE: c_uint = 0x4f;
pub const BD96801_REG_EXT_STATE: c_uint = 0x50;
pub const BD96801_STATE_STBY: c_uint = 0x09;
pub const BD96801_LOCK_REG: c_uint = 0x04;
pub const BD96801_UNLOCK: c_uint = 0x9d;
pub const BD96801_LOCK: c_uint = 0x00;
// IRQ register area
pub const BD96801_REG_INT_MAIN: c_uint = 0x51;
//
// The BD96801 has two physical IRQ lines, INTB and ERRB.
//
// The 'main status register' is located at 0x51.
// The ERRB status registers are located at 0x52 ... 0x5B
// INTB status registers are at range 0x5c ... 0x63
//
pub const BD96801_REG_INT_SYS_ERRB1: c_uint = 0x52;
pub const BD96801_REG_INT_BUCK2_ERRB: c_uint = 0x56;
pub const BD96801_REG_INT_SYS_INTB: c_uint = 0x5c;
pub const BD96801_REG_INT_BUCK2_INTB: c_uint = 0x5e;
pub const BD96801_REG_INT_LDO7_INTB: c_uint = 0x63;
// MASK registers
pub const BD96801_REG_MASK_SYS_INTB: c_uint = 0x73;
pub const BD96801_REG_MASK_SYS_ERRB: c_uint = 0x69;
pub const BD96801_MAX_REGISTER: c_uint = 0x7a;

// ERRB IRQs
// Reg 0x52, 0x53, 0x54 - ERRB system IRQs
// Reg 0x55 BUCK1 ERR IRQs
// Reg 0x56 BUCK2 ERR IRQs
// Reg 0x57 BUCK3 ERR IRQs
// Reg 0x58 BUCK4 ERR IRQs
// Reg 0x59 LDO5 ERR IRQs
// Reg 0x5a LDO6 ERR IRQs
// Reg 0x5b LDO7 ERR IRQs
// INTB IRQs
// Reg 0x5c (System INTB)
// Reg 0x5d (BUCK1 INTB)
// Reg 0x5e (BUCK2 INTB)
// Reg 0x5f (BUCK3 INTB)
// Reg 0x60 (BUCK4 INTB)
// Reg 0x61 (LDO5 INTB)
// Reg 0x62 (LDO6 INTB)
// Reg 0x63 (LDO7 INTB)
// IRQ MASKs

