//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_pipe_crc_regs.h
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

pub const _PIPE_CRC_CTL_A: c_uint = 0x60050;

// skl+ source selection

// ivb+ source selection

// ilk+ source selection

// embedded DP port on the north display block

// vlv source selection

// with DP port the pipe source is invalid

// gen3+ source selection

// with DP/TV port the pipe source is invalid

// gen2 doesn't have source selection bits

pub const _PIPE_CRC_EXP_GREEN_A: c_uint = 0x60054;

pub const _PIPE_CRC_EXP_BLUE_A: c_uint = 0x60058;

pub const _PIPE_CRC_EXP_RES1_A_I915: c_uint = 0x6005c /* i915+ */;

pub const _PIPE_CRC_EXP_RES2_A_G4X: c_uint = 0x60080 /* g4x+ */;

pub const _PIPE_CRC_RES_RED_A: c_uint = 0x60060;

pub const _PIPE_CRC_RES_GREEN_A: c_uint = 0x60064;

pub const _PIPE_CRC_RES_BLUE_A: c_uint = 0x60068;

pub const _PIPE_CRC_RES_RES1_A_I915: c_uint = 0x6006c /* i915+ */;

pub const _PIPE_CRC_RES_RES2_A_G4X: c_uint = 0x60080 /* g4x+ */;

// ivb
pub const _PIPE_CRC_EXP_2_A_IVB: c_uint = 0x60054;
pub const _PIPE_CRC_EXP_2_B_IVB: c_uint = 0x61054;

// ivb
pub const _PIPE_CRC_EXP_3_A_IVB: c_uint = 0x60058;
pub const _PIPE_CRC_EXP_3_B_IVB: c_uint = 0x61058;

// ivb
pub const _PIPE_CRC_EXP_4_A_IVB: c_uint = 0x6005c;
pub const _PIPE_CRC_EXP_4_B_IVB: c_uint = 0x6105c;

// ivb
pub const _PIPE_CRC_EXP_5_A_IVB: c_uint = 0x60060;
pub const _PIPE_CRC_EXP_5_B_IVB: c_uint = 0x61060;

// ivb
pub const _PIPE_CRC_RES_1_A_IVB: c_uint = 0x60064;
pub const _PIPE_CRC_RES_1_B_IVB: c_uint = 0x61064;

// ivb
pub const _PIPE_CRC_RES_2_A_IVB: c_uint = 0x60068;
pub const _PIPE_CRC_RES_2_B_IVB: c_uint = 0x61068;

// ivb
pub const _PIPE_CRC_RES_3_A_IVB: c_uint = 0x6006c;
pub const _PIPE_CRC_RES_3_B_IVB: c_uint = 0x6106c;

// ivb
pub const _PIPE_CRC_RES_4_A_IVB: c_uint = 0x60070;
pub const _PIPE_CRC_RES_4_B_IVB: c_uint = 0x61070;

// ivb
pub const _PIPE_CRC_RES_5_A_IVB: c_uint = 0x60074;
pub const _PIPE_CRC_RES_5_B_IVB: c_uint = 0x61074;

// hsw+
pub const _PIPE_CRC_EXP_A_HSW: c_uint = 0x60054;
pub const _PIPE_CRC_EXP_B_HSW: c_uint = 0x61054;

// hsw+
pub const _PIPE_CRC_RES_A_HSW: c_uint = 0x60064;
pub const _PIPE_CRC_RES_B_HSW: c_uint = 0x61064;

