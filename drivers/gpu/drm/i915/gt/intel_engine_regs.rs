//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_engine_regs.h
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

pub const TAIL_ADDR: c_uint = 0x001FFFF8;

pub const HEAD_WRAP_COUNT: c_uint = 0xFFE00000;
pub const HEAD_WRAP_ONE: c_uint = 0x00200000;
pub const HEAD_ADDR: c_uint = 0x001FFFFC;

pub const RING_NR_PAGES: c_uint = 0x001FF000;
pub const RING_REPORT_MASK: c_uint = 0x00000006;
pub const RING_REPORT_64K: c_uint = 0x00000002;
pub const RING_REPORT_128K: c_uint = 0x00000004;
pub const RING_NO_REPORT: c_uint = 0x00000000;
pub const RING_VALID_MASK: c_uint = 0x00000001;
pub const RING_VALID: c_uint = 0x00000001;
pub const RING_INVALID: c_uint = 0x00000000;

pub const IDLE_TIME_MASK: c_uint = 0xFFFFF;

pub const GEN8_RPCS_S_CNT_SHIFT: c_int = 15;

pub const GEN11_RPCS_S_CNT_SHIFT: c_int = 12;

pub const GEN8_RPCS_SS_CNT_SHIFT: c_int = 8;

pub const GEN8_RPCS_EU_MAX_SHIFT: c_int = 4;

pub const GEN8_RPCS_EU_MIN_SHIFT: c_int = 0;

pub const GEN6_BLITTER_LOCK_SHIFT: c_int = 16;

//
// CMD_CCTL read/write fields take a MOCS value and _not_ a table index.
// The lsb of each can be considered a separate enabling bit for encryption.
// 6:0 == default MOCS value for reads  =>  6:1 == table index for reads.
// 13:7 == default MOCS value for writes => 13:8 == table index for writes.
// 15:14 == Reserved => 31:30 are set to 0.
//

pub const PP_DIR_DCLV_2G: c_uint = 0xffffffff;

pub const RING_MAX_NONPRIV_SLOTS: c_int = 12;

// There are 16 64-bit CS General Purpose Registers per-engine on Gen8+

