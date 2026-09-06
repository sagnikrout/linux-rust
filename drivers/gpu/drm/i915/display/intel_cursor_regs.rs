//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_cursor_regs.h
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
// Copyright © 2024 Intel Corporation
//

pub const _CURACNTR: c_uint = 0x70080;

// Old style CUR*CNTR flags (desktop 8xx)

// New style CUR*CNTR flags

pub const MCURSOR_MODE_MASK: c_uint = 0x27;
pub const MCURSOR_MODE_DISABLE: c_uint = 0x00;
pub const MCURSOR_MODE_128_32B_AX: c_uint = 0x02;
pub const MCURSOR_MODE_256_32B_AX: c_uint = 0x03;
pub const MCURSOR_MODE_64_2B: c_uint = 0x04;
pub const MCURSOR_MODE_64_32B_AX: c_uint = 0x07;

pub const _CURABASE: c_uint = 0x70084;

pub const _CURAPOS: c_uint = 0x70088;

pub const _CURAPOS_ERLY_TPT: c_uint = 0x7008c;

pub const _CURASIZE: c_uint = 0x700a0 /* 845/865 */;

pub const _CUR_FBC_CTL_A: c_uint = 0x700a0 /* ivb+ */;

pub const _CUR_CHICKEN_A: c_uint = 0x700a4 /* mtl+ */;

pub const _CURASURFLIVE: c_uint = 0x700ac /* g4x+ */;

// skl+
pub const _CUR_WM_A_0: c_uint = 0x70140;
pub const _CUR_WM_B_0: c_uint = 0x71140;

pub const _CUR_WM_SAGV_A: c_uint = 0x70158;
pub const _CUR_WM_SAGV_B: c_uint = 0x71158;

pub const _CUR_WM_SAGV_TRANS_A: c_uint = 0x7015C;
pub const _CUR_WM_SAGV_TRANS_B: c_uint = 0x7115C;

pub const _CUR_WM_TRANS_A: c_uint = 0x70168;
pub const _CUR_WM_TRANS_B: c_uint = 0x71168;

pub const _CUR_BUF_CFG_A: c_uint = 0x7017c;
pub const _CUR_BUF_CFG_B: c_uint = 0x7117c;

// skl+: 10 bits, icl+ 11 bits, adlp+ 12 bits

// tgl+
pub const _SEL_FETCH_CUR_CTL_A: c_uint = 0x70880;
pub const _SEL_FETCH_CUR_CTL_B: c_uint = 0x71880;

