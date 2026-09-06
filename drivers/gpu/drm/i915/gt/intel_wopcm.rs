//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_wopcm.h
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
// Copyright © 2017-2018 Intel Corporation
//

//
// struct intel_wopcm - Overall WOPCM info and WOPCM regions.
// @size: Size of overall WOPCM.
// @guc: GuC WOPCM Region info.
// @guc.base: GuC WOPCM base which is offset from WOPCM base.
// @guc.size: Size of the GuC WOPCM region.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_wopcm {
    pub size: u32,
    pub base: u32,
    pub size: u32,
    pub guc: },
}

//
// intel_wopcm_guc_base()
// @wopcm:	intel_wopcm structure
//
// Returns the base of the WOPCM shadowed region.
//
// Returns:
// 0 if GuC is not present or not in use.
// Otherwise, the GuC WOPCM base.
//
// intel_wopcm_guc_size()
// @wopcm:	intel_wopcm structure
//
// Returns size of the WOPCM shadowed region.
//
// Returns:
// 0 if GuC is not present or not in use.
// Otherwise, the GuC WOPCM size.
//
extern "C" {
    pub fn intel_wopcm_init_early(wopcm: *mut intel_wopcm);
}
extern "C" {
    pub fn intel_wopcm_init(wopcm: *mut intel_wopcm);
}
