//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/uc/intel_guc_slpc_types.h
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

pub const SLPC_RESET_TIMEOUT_MS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_guc_slpc {
    pub vma: *mut i915_vma,
    pub vaddr: *mut slpc_shared_data,
    pub supported: bool,
    pub selected: bool,
// Indicates this is a server part
    pub min_is_rpmax: bool,
// platform frequency limits
    pub min_freq: u32,
    pub rp0_freq: u32,
    pub rp1_freq: u32,
    pub boost_freq: u32,
// frequency softlimits
    pub min_freq_softlimit: u32,
    pub max_freq_softlimit: u32,
    pub ignore_eff_freq: bool,
// Base or power saving
    pub power_profile: u32,
// cached media ratio mode
    pub media_ratio_mode: u32,
// Protects set/reset of boost freq
// and value of num_waiters
//
    pub lock: mutex,
    pub boost_work: work_struct,
    pub num_waiters: core::sync::atomic::AtomicI32,
    pub num_boosts: u32,
}
