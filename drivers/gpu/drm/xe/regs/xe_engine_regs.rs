//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/regs/xe_engine_regs.h
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
// Copyright © 2023 Intel Corporation
//

//
// These *_BASE values represent the MMIO offset where each hardware engine's
// registers start.  The other definitions in this header are parameterized
// macros that will take one of these values as a parameter.
//
pub const RENDER_RING_BASE: c_uint = 0x02000;
pub const BSD_RING_BASE: c_uint = 0x1c0000;
pub const BSD2_RING_BASE: c_uint = 0x1c4000;
pub const BSD3_RING_BASE: c_uint = 0x1d0000;
pub const BSD4_RING_BASE: c_uint = 0x1d4000;
pub const XEHP_BSD5_RING_BASE: c_uint = 0x1e0000;
pub const XEHP_BSD6_RING_BASE: c_uint = 0x1e4000;
pub const XEHP_BSD7_RING_BASE: c_uint = 0x1f0000;
pub const XEHP_BSD8_RING_BASE: c_uint = 0x1f4000;
pub const VEBOX_RING_BASE: c_uint = 0x1c8000;
pub const VEBOX2_RING_BASE: c_uint = 0x1d8000;
pub const XEHP_VEBOX3_RING_BASE: c_uint = 0x1e8000;
pub const XEHP_VEBOX4_RING_BASE: c_uint = 0x1f8000;
pub const COMPUTE0_RING_BASE: c_uint = 0x1a000;
pub const COMPUTE1_RING_BASE: c_uint = 0x1c000;
pub const COMPUTE2_RING_BASE: c_uint = 0x1e000;
pub const COMPUTE3_RING_BASE: c_uint = 0x26000;
pub const BLT_RING_BASE: c_uint = 0x22000;
pub const XEHPC_BCS1_RING_BASE: c_uint = 0x3e0000;
pub const XEHPC_BCS2_RING_BASE: c_uint = 0x3e2000;
pub const XEHPC_BCS3_RING_BASE: c_uint = 0x3e4000;
pub const XEHPC_BCS4_RING_BASE: c_uint = 0x3e6000;
pub const XEHPC_BCS5_RING_BASE: c_uint = 0x3e8000;
pub const XEHPC_BCS6_RING_BASE: c_uint = 0x3ea000;
pub const XEHPC_BCS7_RING_BASE: c_uint = 0x3ec000;
pub const XEHPC_BCS8_RING_BASE: c_uint = 0x3ee000;
pub const GSCCS_RING_BASE: c_uint = 0x11a000;

//
// CMD_CCTL read/write fields take a MOCS value and _not_ a table index.
// The lsb of each can be considered a separate enabling bit for encryption.
// 6:0 == default MOCS value for reads  =>  6:1 == table index for reads.
// 13:7 == default MOCS value for writes => 13:8 == table index for writes.
// 15:14 == Reserved => 31:30 are set to 0.
//

pub const CTR_START: c_int = 0;
pub const CTR_STOP: c_int = 1;

// Handling MOCS value in BLIT_CCTL like it was done CMD_CCTL

pub const RING_VALID_MASK: c_uint = 0x00000001;
pub const RING_VALID: c_uint = 0x00000001;

pub const RING_MAX_NONPRIV_SLOTS: c_int = 12;

