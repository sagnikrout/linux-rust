//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/qcom/ubwc.h
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
// Copyright (c) 2018, The Linux Foundation
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_ubwc_cfg_data {
    pub ubwc_enc_version: u32,
//
// @highest_bank_bit: Highest Bank Bit
//
// The Highest Bank Bit value represents the bit of the highest
// DDR bank.  This should ideally use DRAM type detection.
//
    pub highest_bank_bit: c_int,
    pub flags: c_uint,

}

pub const UBWC_1_0: c_uint = 0x10000000;
pub const UBWC_2_0: c_uint = 0x20000000;
pub const UBWC_3_0: c_uint = 0x30000000;
pub const UBWC_3_1: c_uint = 0x30010000 /* UBWC 3.0 + Macrotile mode */;
pub const UBWC_4_0: c_uint = 0x40000000;
pub const UBWC_4_3: c_uint = 0x40030000;
pub const UBWC_5_0: c_uint = 0x50000000;
pub const UBWC_6_0: c_uint = 0x60000000;

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

//
// @qcom_ubwc_macrotile_mode: whether to use 4-channel or 8-channel macrotiling
//
// The 8-channel macrotiling mode was introduced in UBWC 3.1.
//
// Returns: false for the 4-channel and true for 8-channel.
//

//
// @qcom_ubwc_swizzle: Whether to enable level 1, 2 & 3 bank swizzling.
//
// UBWC 1.0 always enables all three levels.
// UBWC 2.0 removes level 1 bank swizzling, leaving levels 2 & 3.
// UBWC 4.0 adds the optional ability to disable levels 2 & 3.
//
