//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/ixp4xx/cpu.h
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
// IXP4XX cpu type detection
//
// Copyright (C) 2007 MontaVista Software, Inc.
//

// Processor id value in CP15 Register 0
pub const IXP42X_PROCESSOR_ID_VALUE: c_uint = 0x690541c0 /* including unused 0x690541Ex */;
pub const IXP42X_PROCESSOR_ID_MASK: c_uint = 0xffffffc0;
pub const IXP43X_PROCESSOR_ID_VALUE: c_uint = 0x69054040;
pub const IXP43X_PROCESSOR_ID_MASK: c_uint = 0xfffffff0;
pub const IXP46X_PROCESSOR_ID_VALUE: c_uint = 0x69054200 /* including IXP455 */;
pub const IXP46X_PROCESSOR_ID_MASK: c_uint = 0xfffffff0;
// Feature register in the expansion bus controller
pub const IXP4XX_EXP_CNFG2: c_uint = 0x2c;
// "fuse" bits of IXP_EXP_CFG2
// All IXP4xx CPUs

// IXP43x/46x CPUs

// IXP46x CPU (including IXP455) only

// For some reason this register is inverted

pub const cpu_is_ixp42x_rev_a0(): c_int = 0;
pub const cpu_is_ixp42x(): c_int = 0;
pub const cpu_is_ixp43x(): c_int = 0;
pub const cpu_is_ixp46x(): c_int = 0;

