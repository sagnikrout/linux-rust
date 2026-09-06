//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/sspl/dc_spl_isharp_filters.h
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
// Copyright 2024 Advanced Micro Devices, Inc.

pub const NUM_SHARPNESS_ADJ_LEVELS: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scale_ratio_to_sharpness_level_adj {
    pub ratio_numer: c_uint,
    pub ratio_denom: c_uint,
    pub /: *mut *mut unsigned int level_down_adj; / adjust sharpness level down,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isharp_1D_lut_pregen {
    pub sharpness_numer: c_uint,
    pub sharpness_denom: c_uint,
    pub value: [u32; ISHARP_LUT_TABLE_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum system_setup {
    SDR_NL = 0,
    SDR_L,
    HDR_NL,
    HDR_L,
    NUM_SHARPNESS_SETUPS
}

// public API
