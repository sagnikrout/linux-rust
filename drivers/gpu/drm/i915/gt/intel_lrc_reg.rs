//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_lrc_reg.h
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
// Copyright © 2014-2018 Intel Corporation
//

// GEN8 to GEN12 Reg State Context

pub const GEN9_CTX_RING_MI_MODE: c_uint = 0x54;

pub const GEN8_CTX_RCS_INDIRECT_CTX_OFFSET_DEFAULT: c_uint = 0x17;
pub const GEN9_CTX_RCS_INDIRECT_CTX_OFFSET_DEFAULT: c_uint = 0x26;
pub const GEN10_CTX_RCS_INDIRECT_CTX_OFFSET_DEFAULT: c_uint = 0x19;
pub const GEN11_CTX_RCS_INDIRECT_CTX_OFFSET_DEFAULT: c_uint = 0x1A;
pub const GEN12_CTX_RCS_INDIRECT_CTX_OFFSET_DEFAULT: c_uint = 0xD;
pub const GEN8_EXECLISTS_STATUS_BUF: c_uint = 0x370;
pub const GEN11_EXECLISTS_STATUS_BUF2: c_uint = 0x3c0;
//
// The docs specify that the write pointer wraps around after 5h, "After status
// is written out to the last available status QW at offset 5h, this pointer
// wraps to 0."
//
// Therefore, one must infer than even though there are 3 bits available, 6 and
// 7 appear to be * reserved.
//
pub const GEN8_CSB_ENTRIES: c_int = 6;
pub const GEN8_CSB_PTR_MASK: c_uint = 0x7;

pub const GEN11_CSB_ENTRIES: c_int = 12;
pub const GEN11_CSB_PTR_MASK: c_uint = 0xf;

// in Gen12 ID 0x7FF is reserved to indicate idle

// in Xe_HP ID 0xFFFF is reserved to indicate "invalid context"
pub const XEHP_MAX_CONTEXT_HW_ID: c_uint = 0xFFFF;
