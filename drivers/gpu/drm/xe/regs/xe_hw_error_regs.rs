//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/regs/xe_hw_error_regs.h
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
// Copyright © 2025 Intel Corporation
//

pub const ERR_STAT_GT_COR: c_uint = 0x100160;

pub const ERR_STAT_GT_NONFATAL: c_uint = 0x100164;
pub const ERR_STAT_GT_FATAL: c_uint = 0x100168;

pub const DEV_ERR_STAT_NONFATAL: c_uint = 0x100178;
pub const DEV_ERR_STAT_CORRECTABLE: c_uint = 0x10017c;

pub const XE_CSC_ERROR: c_int = 17;
pub const XE_SOC_ERROR: c_int = 16;
pub const XE_GT_ERROR: c_int = 0;
pub const ERR_STAT_GT_FATAL_VECTOR_0: c_uint = 0x100260;
pub const ERR_STAT_GT_FATAL_VECTOR_1: c_uint = 0x100264;

pub const ERR_STAT_GT_COR_VECTOR_0: c_uint = 0x1002a0;
pub const ERR_STAT_GT_COR_VECTOR_1: c_uint = 0x1002a4;

pub const SOC_PVC_MASTER_BASE: c_uint = 0x282000;
pub const SOC_PVC_SLAVE_BASE: c_uint = 0x283000;
pub const SOC_GCOERRSTS: c_uint = 0x200;
pub const SOC_GNFERRSTS: c_uint = 0x210;

pub const SOC_GSYSEVTCTL: c_uint = 0x264;

pub const SOC_LERRUNCSTS: c_uint = 0x280;
pub const SOC_LERRCORSTS: c_uint = 0x294;

