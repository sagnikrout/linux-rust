//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/intel/mchbar_regs.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2022 Intel Corporation
//
// MCHBAR mirror.
//
// This mirrors the MCHBAR MMIO space whose location is determined by
// device 0 function 0's pci config register 0x44 or 0x48 and matches it in
// every way.  It is not accessible from the CP register read instructions.
//
// Starting from Haswell, you can't write registers using the MCHBAR mirror,
// just read. On MTL+ the mirror no longer exists.
//
pub const MCHBAR_MIRROR_BASE: c_uint = 0x10000;
pub const MCHBAR_MIRROR_END: c_uint = 0x13fff;
pub const MCHBAR_MIRROR_BASE_SNB: c_uint = 0x140000;
pub const MCHBAR_MIRROR_END_SNB: c_uint = 0x147fff;
pub const MCHBAR_MIRROR_END_ICL_RKL: c_uint = 0x14ffff;
pub const MCHBAR_MIRROR_END_TGL: c_uint = 0x15ffff;

// Pineview MCH register contains DDR3 setting

// 915-945 and GM965 MCH register controlling DRAM channel access

// 965 MCH register controlling DRAM channel configuration

// Clocking configuration register

pub const TSFS_SLOPE_MASK: c_uint = 0x0000ff00;
pub const TSFS_SLOPE_SHIFT: c_int = 8;
pub const TSFS_INTR_MASK: c_uint = 0x000000ff;
// Memory latency timer register

// the unit of memory self-refresh latency time is 0.5us

pub const BXT_D_CR_DRP0_DUNIT8: c_uint = 0x1000;
pub const BXT_D_CR_DRP0_DUNIT9: c_uint = 0x1200;
pub const BXT_D_CR_DRP0_DUNIT_START: c_int = 8;
pub const BXT_D_CR_DRP0_DUNIT_END: c_int = 11;

pub const BXT_DRAM_RANK_MASK: c_uint = 0x3;
pub const BXT_DRAM_RANK_SINGLE: c_uint = 0x1;
pub const BXT_DRAM_RANK_DUAL: c_uint = 0x3;

pub const BXT_DRAM_WIDTH_SHIFT: c_int = 4;

pub const BXT_DRAM_SIZE_SHIFT: c_int = 6;

pub const BXT_DRAM_TYPE_SHIFT: c_int = 22;

// snb MCH registers for reading the DRAM channel configuration

// DIMM sizes are in multiples of 256mb.
pub const MAD_DIMM_B_SIZE_SHIFT: c_int = 8;

pub const MAD_DIMM_A_SIZE_SHIFT: c_int = 0;

//
// *_PACKAGE_POWER_SKU - SKU power and timing parameters.
//

// snb MCH registers for priority tuning

// Memory controller frequency in MCHBAR for Haswell (possible SNB+)

//
// Please see hsw_read_dcomp() and hsw_write_dcomp() before using this register,
// since on HSW we can't write to it using intel_uncore_write.
//

