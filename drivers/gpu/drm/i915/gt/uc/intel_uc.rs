//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/uc/intel_uc.h
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
// Copyright © 2014-2019 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_uc_ops {
    pub uc): *mut *mut int (sanitize)(struct intel_uc,
    pub uc): *mut *mut void (init_fw)(struct intel_uc,
    pub uc): *mut *mut void (fini_fw)(struct intel_uc,
    pub uc): *mut *mut int (init)(struct intel_uc,
    pub uc): *mut *mut void (fini)(struct intel_uc,
    pub uc): *mut *mut int (init_hw)(struct intel_uc,
    pub uc): *mut *mut void (fini_hw)(struct intel_uc,
    pub uc): *mut *mut void (resume_mappings)(struct intel_uc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_uc {
    pub ops: *const intel_uc_ops,
    pub gsc: intel_gsc_uc,
    pub guc: intel_guc,
    pub huc: intel_huc,
// Snapshot of GuC log from last failed load
    pub load_err_log: *mut drm_i915_gem_object,
    pub reset_in_progress: bool,
    pub fw_table_invalid: bool,
}

extern "C" {
    pub fn intel_uc_init_early(uc: *mut intel_uc);
}
extern "C" {
    pub fn intel_uc_init_late(uc: *mut intel_uc);
}
extern "C" {
    pub fn intel_uc_driver_late_release(uc: *mut intel_uc);
}
extern "C" {
    pub fn intel_uc_driver_remove(uc: *mut intel_uc);
}
extern "C" {
    pub fn intel_uc_init_mmio(uc: *mut intel_uc);
}
extern "C" {
    pub fn intel_uc_reset_prepare(uc: *mut intel_uc);
}
extern "C" {
    pub fn intel_uc_reset(uc: *mut intel_uc, stalled: intel_engine_mask_t);
}
extern "C" {
    pub fn intel_uc_reset_finish(uc: *mut intel_uc);
}
extern "C" {
    pub fn intel_uc_cancel_requests(uc: *mut intel_uc);
}
extern "C" {
    pub fn intel_uc_suspend(uc: *mut intel_uc);
}
extern "C" {
    pub fn intel_uc_runtime_suspend(uc: *mut intel_uc);
}
extern "C" {
    pub fn intel_uc_resume(uc: *mut intel_uc) -> c_int;
}
extern "C" {
    pub fn intel_uc_runtime_resume(uc: *mut intel_uc) -> c_int;
}
//
// We need to know as early as possible if we're going to use GuC or not to
// take the correct setup paths. Additionally, once we've started loading the
// GuC, it is unsafe to keep executing without it because some parts of the HW,
// a subset of which is not cleaned on GT reset, will start expecting the GuC FW
// to be running.
// To solve both these requirements, we commit to using the microcontrollers if
// the relevant modparam is set and the blobs are found on the system. At this
// stage, the only thing that can stop us from attempting to load the blobs on
// the HW and use them is a fundamental issue (e.g. no memory for our
// structures); if we hit such a problem during driver load we're broken even
// without GuC, so there is no point in trying to fall back.
//
// Given the above, we can be in one of 4 states, with the last one implying
// we're committed to using the microcontroller:
// - Not supported: not available in HW and/or firmware not defined.
// - Supported: available in HW and firmware defined.
// - Wanted: supported + enabled in modparam.
// - In use: wanted + firmware found on the system and successfully fetched.
//

extern "C" {
    pub fn intel_guc_wait_for_idle(_arg: &uc->guc, _arg: timeout) -> return;
}

