//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_pch.h
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
// Copyright 2025 Intel Corporation.
//
// Sorted by south display engine compatibility.
// If the new PCH comes with a south display engine that is not
// inherited from the latest item, please do not add it to the
// end. Instead, add it right after its "parent" PCH.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_pch {
    PCH_NOP = -1,	/* PCH without south display */
    PCH_NONE = 0,	/* No PCH present */
    PCH_IBX,	/* Ibexpeak PCH */
    PCH_CPT,	/* Cougarpoint/Pantherpoint PCH */
    PCH_LPT_H,	/* Lynxpoint/Wildcatpoint H PCH */
    PCH_LPT_LP,	/* Lynxpoint/Wildcatpoint LP PCH */
    PCH_SPT,        /* Sunrisepoint/Kaby Lake PCH */
    PCH_CNP,        /* Cannon/Comet Lake PCH */
    PCH_ICP,	/* Ice Lake/Jasper Lake PCH */
    PCH_TGP,	/* Tiger Lake/Mule Creek Canyon PCH */
    PCH_ADP,	/* Alder Lake PCH */

// Fake PCHs, functionality handled on the same PCI dev
    PCH_DG1 = 1024,
    PCH_DG2,
    PCH_MTL,
    PCH_LNL,
}

extern "C" {
    pub fn intel_pch_detect(display: *mut intel_display);
}
extern "C" {
    pub fn intel_pch_init_clock_gating(display: *mut intel_display);
}
