//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dpp/dcn42/dcn42_dpp.h
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
// Copyright 2026 Advanced Micro Devices, Inc.

// Macro flag: #define TO_DCN42_DPP(dpp)\
// Macro flag: #define DPP_REG_LIST_SH_MASK_DCN42_COMMON(mask_sh)\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn42_dpp_registers {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn42_dpp_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn42_dpp_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn42_dpp {
    pub base: dpp,
    pub tf_regs: *const dcn42_dpp_registers,
    pub tf_shift: *const dcn42_dpp_shift,
    pub tf_mask: *const dcn42_dpp_mask,
    pub filter_v: *const u16,
    pub filter_h: *const u16,
    pub filter_v_c: *const u16,
    pub filter_h_c: *const u16,
    pub lb_pixel_depth_supported: c_int,
    pub lb_memory_size: c_int,
    pub lb_bits_per_entry: c_int,
    pub is_write_to_ram_a_safe: bool,
    pub scl_data: scaler_data,
    pub pwl_data: pwl_params,
}
