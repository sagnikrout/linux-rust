//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_dram.h
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
// Copyright © 2020 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dram_info {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_dram_type {
    INTEL_DRAM_UNKNOWN,
    INTEL_DRAM_DDR2,
    INTEL_DRAM_DDR3,
    INTEL_DRAM_DDR4,
    INTEL_DRAM_LPDDR3,
    INTEL_DRAM_LPDDR4,
    INTEL_DRAM_DDR5,
    INTEL_DRAM_LPDDR5,
    INTEL_DRAM_GDDR,
    INTEL_DRAM_GDDR_ECC,
    __INTEL_DRAM_TYPE_MAX,
    } type;
    unsigned int fsb_freq;
    unsigned int mem_freq;
    u8 num_channels;
    u8 num_qgv_points;
    u8 num_psf_gv_points;
    bool ecc_impacting_de_bw; /* Only valid from Xe3p_LPD onward. */
    bool symmetric_memory;
    bool has_16gb_dimms;
}

    pub display): *mut int intel_dram_detect(struct intel_display,
    pub display): *mut unsigned int intel_fsb_freq(struct intel_display,
    pub display): *mut unsigned int intel_mem_freq(struct intel_display,
    pub display): *const *const dram_info intel_dram_info(intel_display,
    pub type): *const *const char intel_dram_type_str(enum intel_dram_type,
