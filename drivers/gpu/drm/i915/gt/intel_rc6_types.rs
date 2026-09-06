//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_rc6_types.h
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

// RC6 residency types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_rc6_res_type {
    INTEL_RC6_RES_RC6_LOCKED,
    INTEL_RC6_RES_RC6,
    INTEL_RC6_RES_RC6p,
    INTEL_RC6_RES_RC6pp,
    INTEL_RC6_RES_MAX,
    INTEL_RC6_RES_VLV_MEDIA = INTEL_RC6_RES_RC6p,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_rc6 {
    pub res_reg: [i915_reg_t; INTEL_RC6_RES_MAX],
    pub prev_hw_residency: [u64; INTEL_RC6_RES_MAX],
    pub cur_residency: [u64; INTEL_RC6_RES_MAX],
    pub ctl_enable: u32,
    pub bios_rc_state: u32,
    pub pctx: *mut drm_i915_gem_object,
    pub 1: bool supported :,
    pub 1: bool enabled :,
    pub 1: bool manual :,
    pub 1: bool wakeref :,
    pub 1: bool bios_state_captured :,
}
