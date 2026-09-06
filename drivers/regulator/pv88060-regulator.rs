//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/regulator/pv88060-regulator.h
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
// pv88060-regulator.h - Regulator definitions for PV88060
// Copyright (C) 2015 Powerventure Semiconductor Ltd.
//
// System Control and Event Registers
pub const PV88060_REG_EVENT_A: c_uint = 0x04;
pub const PV88060_REG_MASK_A: c_uint = 0x08;
pub const PV88060_REG_MASK_B: c_uint = 0x09;
pub const PV88060_REG_MASK_C: c_uint = 0x0A;
// Regulator Registers
pub const PV88060_REG_BUCK1_CONF0: c_uint = 0x1B;
pub const PV88060_REG_BUCK1_CONF1: c_uint = 0x1C;
pub const PV88060_REG_LDO1_CONF: c_uint = 0x1D;
pub const PV88060_REG_LDO2_CONF: c_uint = 0x1E;
pub const PV88060_REG_LDO3_CONF: c_uint = 0x1F;
pub const PV88060_REG_LDO4_CONF: c_uint = 0x20;
pub const PV88060_REG_LDO5_CONF: c_uint = 0x21;
pub const PV88060_REG_LDO6_CONF: c_uint = 0x22;
pub const PV88060_REG_LDO7_CONF: c_uint = 0x23;
pub const PV88060_REG_SW1_CONF: c_uint = 0x3B;
pub const PV88060_REG_SW2_CONF: c_uint = 0x3C;
pub const PV88060_REG_SW3_CONF: c_uint = 0x3D;
pub const PV88060_REG_SW4_CONF: c_uint = 0x3E;
pub const PV88060_REG_SW5_CONF: c_uint = 0x3F;
pub const PV88060_REG_SW6_CONF: c_uint = 0x40;
// PV88060_REG_EVENT_A (addr=0x04)
pub const PV88060_E_VDD_FLT: c_uint = 0x01;
pub const PV88060_E_OVER_TEMP: c_uint = 0x02;
// PV88060_REG_MASK_A (addr=0x08)
pub const PV88060_M_VDD_FLT: c_uint = 0x01;
pub const PV88060_M_OVER_TEMP: c_uint = 0x02;
// PV88060_REG_BUCK1_CONF0 (addr=0x1B)
pub const PV88060_BUCK_EN: c_uint = 0x80;
pub const PV88060_VBUCK_MASK: c_uint = 0x7F;
// PV88060_REG_LDO1/2/3/4/5/6/7_CONT
pub const PV88060_LDO_EN: c_uint = 0x40;
pub const PV88060_VLDO_MASK: c_uint = 0x3F;
// PV88060_REG_SW1/2/3/4/5_CONF
pub const PV88060_SW_EN: c_uint = 0x80;
// PV88060_REG_BUCK1_CONF1 (addr=0x1C)
pub const PV88060_BUCK_ILIM_SHIFT: c_int = 2;
pub const PV88060_BUCK_ILIM_MASK: c_uint = 0x0C;
pub const PV88060_BUCK_MODE_SHIFT: c_int = 0;
pub const PV88060_BUCK_MODE_MASK: c_uint = 0x03;
pub const PV88060_BUCK_MODE_SLEEP: c_uint = 0x00;
pub const PV88060_BUCK_MODE_AUTO: c_uint = 0x01;
pub const PV88060_BUCK_MODE_SYNC: c_uint = 0x02;
