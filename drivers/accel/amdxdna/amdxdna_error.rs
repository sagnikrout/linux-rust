//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/amdxdna/amdxdna_error.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2025, Advanced Micro Devices, Inc.
//

pub const AMDXDNA_ERR_DRV_AIE: c_int = 4;
pub const AMDXDNA_ERR_SEV_CRITICAL: c_int = 3;
pub const AMDXDNA_ERR_CLASS_AIE: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdxdna_error_num {
    AMDXDNA_ERROR_NUM_AIE_SATURATION = 3,
    AMDXDNA_ERROR_NUM_AIE_FP,
    AMDXDNA_ERROR_NUM_AIE_STREAM,
    AMDXDNA_ERROR_NUM_AIE_ACCESS,
    AMDXDNA_ERROR_NUM_AIE_BUS,
    AMDXDNA_ERROR_NUM_AIE_INSTRUCTION,
    AMDXDNA_ERROR_NUM_AIE_ECC,
    AMDXDNA_ERROR_NUM_AIE_LOCK,
    AMDXDNA_ERROR_NUM_AIE_DMA,
    AMDXDNA_ERROR_NUM_AIE_MEM_PARITY,
    AMDXDNA_ERROR_NUM_UNKNOWN = 15,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdxdna_error_module {
    AMDXDNA_ERROR_MODULE_AIE_CORE = 3,
    AMDXDNA_ERROR_MODULE_AIE_MEMORY,
    AMDXDNA_ERROR_MODULE_AIE_SHIM,
    AMDXDNA_ERROR_MODULE_AIE_NOC,
    AMDXDNA_ERROR_MODULE_AIE_PL,
    AMDXDNA_ERROR_MODULE_UNKNOWN = 8,
}

