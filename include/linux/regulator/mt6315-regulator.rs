//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/mt6315-regulator.h
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
// Copyright (c) 2021 MediaTek Inc.
//
pub const MT6315_RP: c_int = 3;
pub const MT6315_PP: c_int = 6;
pub const MT6315_SP: c_int = 7;
// Register
pub const MT6315_TOP2_ELR7: c_uint = 0x139;
pub const MT6315_TOP_TMA_KEY: c_uint = 0x39F;
pub const MT6315_TOP_TMA_KEY_H: c_uint = 0x3A0;
pub const MT6315_BUCK_TOP_CON0: c_uint = 0x1440;
pub const MT6315_BUCK_TOP_CON1: c_uint = 0x1443;
pub const MT6315_BUCK_TOP_ELR0: c_uint = 0x1449;
pub const MT6315_BUCK_TOP_ELR2: c_uint = 0x144B;
pub const MT6315_BUCK_TOP_ELR4: c_uint = 0x144D;
pub const MT6315_BUCK_TOP_ELR6: c_uint = 0x144F;
pub const MT6315_VBUCK1_DBG0: c_uint = 0x1499;
pub const MT6315_VBUCK1_DBG4: c_uint = 0x149D;
pub const MT6315_VBUCK2_DBG0: c_uint = 0x1519;
pub const MT6315_VBUCK2_DBG4: c_uint = 0x151D;
pub const MT6315_VBUCK3_DBG0: c_uint = 0x1599;
pub const MT6315_VBUCK3_DBG4: c_uint = 0x159D;
pub const MT6315_VBUCK4_DBG0: c_uint = 0x1619;
pub const MT6315_VBUCK4_DBG4: c_uint = 0x161D;
pub const MT6315_BUCK_TOP_4PHASE_ANA_CON42: c_uint = 0x16B1;
pub const PROTECTION_KEY_H: c_uint = 0x9C;
pub const PROTECTION_KEY: c_uint = 0xEA;
