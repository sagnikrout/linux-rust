//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_wm_types.h
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
// Copyright © 2021 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_ddb_partitioning {
    INTEL_DDB_PART_1_2,
    INTEL_DDB_PART_5_6, /* IVB+ */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ilk_wm_values {
    pub wm_pipe: [u32; 3],
    pub wm_lp: [u32; 3],
    pub wm_lp_spr: [u32; 3],
    pub enable_fbc_wm: bool,
    pub partitioning: intel_ddb_partitioning,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct g4x_pipe_wm {
    pub plane: [u16; I915_MAX_PLANES],
    pub fbc: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct g4x_sr_wm {
    pub plane: u16,
    pub cursor: u16,
    pub fbc: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlv_wm_ddl_values {
    pub plane: [u8; I915_MAX_PLANES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlv_wm_values {
    pub pipe: [g4x_pipe_wm; 3],
    pub sr: g4x_sr_wm,
    pub ddl: [vlv_wm_ddl_values; 3],
    pub level: u8,
    pub cxsr: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct g4x_wm_values {
    pub pipe: [g4x_pipe_wm; 2],
    pub sr: g4x_sr_wm,
    pub hpll: g4x_sr_wm,
    pub cxsr: bool,
    pub hpll_en: bool,
    pub fbc_en: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skl_ddb_entry {
    pub /: *mut *mut u16 start, end; / in number of blocks, 'end' is exclusive,
}
