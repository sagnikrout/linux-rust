//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/s2dos05.h
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
// s2dos05.h
//
// Copyright (c) 2016 Samsung Electronics Co., Ltd
// http://www.samsung.com
// Copyright (C) 2024 Dzmitry Sankouski <dsankouski@gmail.com>
// S2DOS05 registers
// Slave Addr : 0xC0
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum S2DOS05_reg {
    S2DOS05_REG_DEV_ID,
    S2DOS05_REG_TOPSYS_STAT,
    S2DOS05_REG_STAT,
    S2DOS05_REG_EN,
    S2DOS05_REG_LDO1_CFG,
    S2DOS05_REG_LDO2_CFG,
    S2DOS05_REG_LDO3_CFG,
    S2DOS05_REG_LDO4_CFG,
    S2DOS05_REG_BUCK_CFG,
    S2DOS05_REG_BUCK_VOUT,
    S2DOS05_REG_IRQ_MASK = 0x0D,
    S2DOS05_REG_SSD_TSD = 0x0E,
    S2DOS05_REG_OCL = 0x10,
    S2DOS05_REG_IRQ = 0x11
}

// S2DOS05 regulator ids
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum S2DOS05_regulators {
    S2DOS05_LDO1,
    S2DOS05_LDO2,
    S2DOS05_LDO3,
    S2DOS05_LDO4,
    S2DOS05_BUCK1,
    S2DOS05_REG_MAX,
}

pub const S2DOS05_BUCK_MIN1: c_int = 506250;
pub const S2DOS05_LDO_MIN1: c_int = 1500000;
pub const S2DOS05_LDO_MIN2: c_int = 2700000;
pub const S2DOS05_BUCK_STEP1: c_int = 6250;
pub const S2DOS05_LDO_STEP1: c_int = 25000;
pub const S2DOS05_LDO_VSEL_MASK: c_uint = 0x7F;
pub const S2DOS05_LDO_FD_MASK: c_uint = 0x80;
pub const S2DOS05_BUCK_VSEL_MASK: c_uint = 0xFF;
pub const S2DOS05_BUCK_FD_MASK: c_uint = 0x08;

pub const S2DOS05_RAMP_DELAY: c_int = 12000;
pub const S2DOS05_ENABLE_TIME_LDO: c_int = 50;
pub const S2DOS05_ENABLE_TIME_BUCK: c_int = 350;

