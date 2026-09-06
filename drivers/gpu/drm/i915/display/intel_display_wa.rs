//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_display_wa.h
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

extern "C" {
    pub fn intel_display_wa_apply(display: *mut intel_display);
}

extern "C" {
    pub fn intel_display_needs_wa_16023588340(display: *mut intel_display) -> bool;
}

//
// This enum lists display workarounds; each entry here must have a
// corresponding case in __intel_display_wa().  Keep both sorted by lineage
// number.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_display_wa {
    INTEL_DISPLAY_WA_1409120013,
    INTEL_DISPLAY_WA_1409767108,
    INTEL_DISPLAY_WA_13012396614,
    INTEL_DISPLAY_WA_14010477008,
    INTEL_DISPLAY_WA_14010480278,
    INTEL_DISPLAY_WA_14010547955,
    INTEL_DISPLAY_WA_14010685332,
    INTEL_DISPLAY_WA_14011294188,
    INTEL_DISPLAY_WA_14011503030,
    INTEL_DISPLAY_WA_14011503117,
    INTEL_DISPLAY_WA_14011508470,
    INTEL_DISPLAY_WA_14011765242,
    INTEL_DISPLAY_WA_14014143976,
    INTEL_DISPLAY_WA_14016740474,
    INTEL_DISPLAY_WA_14020863754,
    INTEL_DISPLAY_WA_14025769978,
    INTEL_DISPLAY_WA_14026643300,
    INTEL_DISPLAY_WA_15013987218,
    INTEL_DISPLAY_WA_15018326506,
    INTEL_DISPLAY_WA_16011181250,
    INTEL_DISPLAY_WA_16011303918,
    INTEL_DISPLAY_WA_16011342517,
    INTEL_DISPLAY_WA_16011863758,
    INTEL_DISPLAY_WA_16023588340,
    INTEL_DISPLAY_WA_16025573575,
    INTEL_DISPLAY_WA_16025596647,
    INTEL_DISPLAY_WA_16029024088,
    INTEL_DISPLAY_WA_16030862157,
    INTEL_DISPLAY_WA_18034343758,
    INTEL_DISPLAY_WA_22010178259,
    INTEL_DISPLAY_WA_22010947358,
    INTEL_DISPLAY_WA_22011320316,
    INTEL_DISPLAY_WA_22012278275,
    INTEL_DISPLAY_WA_22012358565,
    INTEL_DISPLAY_WA_22014263786,
    INTEL_DISPLAY_WA_22021048059,
}

extern "C" {
    pub fn __intel_display_wa(display: *mut intel_display, wa: intel_display_wa, name: *const c_char) -> bool;
}

