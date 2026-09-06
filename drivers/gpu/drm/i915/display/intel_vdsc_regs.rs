//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_vdsc_regs.h
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

// Display Stream Splitter Control

pub const MAX_DL_BUFFER_TARGET_DEPTH: c_uint = 0x5a0;

pub const _ICL_PIPE_DSS_CTL1_PB: c_uint = 0x78200;
pub const _ICL_PIPE_DSS_CTL1_PC: c_uint = 0x78400;

pub const _ICL_PIPE_DSS_CTL2_PB: c_uint = 0x78204;
pub const _ICL_PIPE_DSS_CTL2_PC: c_uint = 0x78404;

// Icelake Display Stream Compression Registers

pub const _DSCA_PPS_0: c_uint = 0x6B200;
pub const _DSCC_PPS_0: c_uint = 0x6BA00;

pub const _ICL_DSC0_PICTURE_PARAMETER_SET_0_PB: c_uint = 0x78270;
pub const _ICL_DSC1_PICTURE_PARAMETER_SET_0_PB: c_uint = 0x78370;
pub const _BMG_DSC2_PICTURE_PARAMETER_SET_0_PB: c_uint = 0x78970;
pub const _ICL_DSC0_PICTURE_PARAMETER_SET_0_PC: c_uint = 0x78470;
pub const _ICL_DSC1_PICTURE_PARAMETER_SET_0_PC: c_uint = 0x78570;
pub const _BMG_DSC2_PICTURE_PARAMETER_SET_0_PC: c_uint = 0x78A70;

// PPS 0

// PPS 1

// PPS 2

// PPS 3

// PPS 4

// PPS 5

// PPS 6

// PPS 7

// PPS 8

// PPS 9

// PPS 10

// PPS 16

// PPS 17 (MTL+)

// PPS 18 (MTL+)

pub const _LNL_DSC0_SU_PARAMETER_SET_0_PA: c_uint = 0x78064;
pub const _LNL_DSC1_SU_PARAMETER_SET_0_PA: c_uint = 0x78164;
pub const _LNL_DSC0_SU_PARAMETER_SET_0_PB: c_uint = 0x78264;
pub const _LNL_DSC1_SU_PARAMETER_SET_0_PB: c_uint = 0x78364;

// Icelake Rate Control Buffer Threshold Registers

// Icelake DSC Rate Control Range Parameter Registers

pub const RC_BPG_OFFSET_SHIFT: c_int = 10;
pub const RC_MAX_QP_SHIFT: c_int = 5;
pub const RC_MIN_QP_SHIFT: c_int = 0;

