//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/chips-media/wave5/wave5-vpuconfig.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
//
// Wave5 series multi-standard codec IP - product config definitions
//
// Copyright (C) 2021-2023 CHIPS&MEDIA INC
//
pub const WAVE515_CODE: c_uint = 0x5150;
pub const WAVE517_CODE: c_uint = 0x5170;
pub const WAVE537_CODE: c_uint = 0x5370;
pub const WAVE511_CODE: c_uint = 0x5110;
pub const WAVE521_CODE: c_uint = 0x5210;
pub const WAVE521C_CODE: c_uint = 0x521c;
pub const WAVE521C_DUAL_CODE: c_uint = 0x521d  // wave521 dual core;
pub const WAVE521E1_CODE: c_uint = 0x5211;

pub const MAX_NUM_INSTANCE: c_int = 32;

// application specific configuration
pub const VPU_ENC_TIMEOUT: c_int = 60000;
pub const VPU_DEC_TIMEOUT: c_int = 60000;
pub const VPU_DEC_STOP_TIMEOUT: c_int = 300;
// for WAVE encoder
pub const USE_SRC_PRP_AXI: c_int = 0;
pub const USE_SRC_PRI_AXI: c_int = 1;

//
// VPU COMMON MEMORY
//

pub const W5_REMAP_INDEX0: c_int = 0;
pub const W5_REMAP_INDEX1: c_int = 1;

// =====4. VPU REPORT MEMORY  ======================
pub const WAVE5_UPPER_PROC_AXI_ID: c_uint = 0x0;
pub const WAVE5_PROC_AXI_ID: c_uint = 0x0;
pub const WAVE5_PRP_AXI_ID: c_uint = 0x0;
pub const WAVE5_FBD_Y_AXI_ID: c_uint = 0x0;
pub const WAVE5_FBC_Y_AXI_ID: c_uint = 0x0;
pub const WAVE5_FBD_C_AXI_ID: c_uint = 0x0;
pub const WAVE5_FBC_C_AXI_ID: c_uint = 0x0;
pub const WAVE5_SEC_AXI_ID: c_uint = 0x0;
pub const WAVE5_PRI_AXI_ID: c_uint = 0x0;
