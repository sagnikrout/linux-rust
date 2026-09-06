//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/regs/xe_gt_regs.h
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

//
// The GSI register range [0x0 - 0x40000) is replicated at a higher offset
// for the media GT.  xe_mmio and xe_gt_mcr functions will automatically
// translate offsets by MEDIA_GT_GSI_OFFSET when operating on the media GT.
//
pub const MEDIA_GT_GSI_OFFSET: c_uint = 0x380000;
pub const MEDIA_GT_GSI_LENGTH: c_uint = 0x40000;
// MTL workpoint reg to get core C state and actual freq of 3D, SAMedia

pub const MTL_CRST: c_uint = 0xf;
// RPM unit config (Gen8+)

pub const RPM_CONFIG0_CRYSTAL_CLOCK_FREQ_24_MHZ: c_int = 0;
pub const RPM_CONFIG0_CRYSTAL_CLOCK_FREQ_19_2_MHZ: c_int = 1;
pub const RPM_CONFIG0_CRYSTAL_CLOCK_FREQ_38_4_MHZ: c_int = 2;
pub const RPM_CONFIG0_CRYSTAL_CLOCK_FREQ_25_MHZ: c_int = 3;

//
// Spec defines these bits as "Reserved", but then make them assume some
// meaning that depends on the ARCH. To avoid any confusion, call them
// SUBIP_FLAG_MASK.
//

pub const XE2_GAMWALK_CTRL: c_uint = 0x47e4;

// Fuse readout registers for GT

pub const L3BANK_PAIR_COUNT: c_int = 4;

// on Xe_HP the same fuses indicates mslices instead of L3 banks
pub const MAX_MSLICES: c_int = 4;

pub const LNCFCMOCS_REG_COUNT: c_int = 32;

//
// Total of 4 cslices, where each cslice is in the form:
// [0-3]     CCS ID
// [4-6]     RSVD
// [7]       Disabled
//

pub const CCS_MODE_CSLICE_MASK: c_uint = 0x7 /* CCS0-3 + rsvd */;

// Applicable for all FORCEWAKE_DOMAIN and FORCEWAKE_ACK_DOMAIN regs
pub const FORCEWAKE_KERNEL: c_int = 0;

pub const GT_C0: c_int = 0;
pub const GT_C6: c_int = 3;

// Common performance limit reason bits - available on all platforms
pub const GT0_PERF_LIMIT_REASONS_MASK: c_uint = 0xde3;

// Platform-specific performance limit reason bits - for Crescent Island
pub const CRI_PERF_LIMIT_REASONS_MASK: c_uint = 0xfdff;

