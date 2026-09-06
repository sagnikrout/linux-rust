//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_sseu.h
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
// Copyright © 2019 Intel Corporation
//

//
// Maximum number of slices on older platforms.  Slices no longer exist
// starting on Xe_HP ("gslices," "cslices," etc. are a different concept and
// are not expressed through fusing).
//
pub const GEN_MAX_HSW_SLICES: c_int = 3;
//
// Maximum number of subslices that can exist within a HSW-style slice.  This
// is only relevant to pre-Xe_HP platforms (Xe_HP and beyond use the
// I915_MAX_SS_FUSE_BITS value below).
//
pub const GEN_MAX_SS_PER_HSW_SLICE: c_int = 8;
//
// Maximum number of 32-bit registers used by hardware to express the
// enabled/disabled subslices.
//
pub const I915_MAX_SS_FUSE_REGS: c_int = 2;

// Maximum number of EUs that can exist within a subslice or DSS.
pub const GEN_MAX_EUS_PER_SS: c_int = 16;

// The maximum number of bits needed to express each subslice/DSS independently

pub const GEN_DSS_PER_GSLICE: c_int = 4;
pub const GEN_DSS_PER_CSLICE: c_int = 8;
pub const GEN_DSS_PER_MSLICE: c_int = 8;

// Bitmap compatible with linux/bitmap.h; may exceed size of u64

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sseu_dev_info {
    pub slice_mask: u8,
    pub subslice_mask: intel_sseu_ss_mask_t,
    pub geometry_subslice_mask: intel_sseu_ss_mask_t,
    pub compute_subslice_mask: intel_sseu_ss_mask_t,
    pub hsw: [u16; GEN_MAX_HSW_SLICES][GEN_MAX_SS_PER_HSW_SLICE],
    pub xehp: [u16; I915_MAX_SS_FUSE_BITS],
    pub eu_mask: },
    pub eu_total: u16,
    pub eu_per_subslice: u8,
    pub min_eu_in_pool: u8,
// For each slice, which subslice(s) has(have) 7 EUs (bitfield)?
    pub subslice_7eu: [u8; 3],
    pub has_slice_pg:1: u8,
    pub has_subslice_pg:1: u8,
    pub has_eu_pg:1: u8,
//
// For Xe_HP and beyond, the hardware no longer has traditional slices
// so we just report the entire DSS pool under a fake "slice 0."
//
    pub has_xehp_dss:1: u8,
// Topology fields
    pub max_slices: u8,
    pub max_subslices: u8,
    pub max_eus_per_subslice: u8,
}

//
// Powergating configuration for a particular (context,engine).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_sseu {
    pub slice_mask: u8,
    pub subslice_mask: u8,
    pub min_eus_per_subslice: u8,
    pub max_eus_per_subslice: u8,
}

extern "C" {
    pub fn test_bit(_arg: subslice, _arg: sseu->subslice_mask.xehp) -> return;
}
//
// Used to obtain the index of the first DSS.  Can start searching from the
// beginning of a specific dss group (e.g., gslice, cslice, etc.) if
// groupsize and groupnum are non-zero.
//
extern "C" {
    pub fn intel_sseu_info_init(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_sseu_dump(sseu: *const sseu_dev_info, p: *mut drm_printer);
}
extern "C" {
    pub fn intel_slicemask_from_xehp_dssmask(dss_mask: intel_sseu_ss_mask_t, dss_per_slice: c_int) -> u16;
}
