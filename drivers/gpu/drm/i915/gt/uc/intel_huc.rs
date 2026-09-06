//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/uc/intel_huc.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_huc_delayed_load_status {
    INTEL_HUC_WAITING_ON_GSC = 0,
    INTEL_HUC_WAITING_ON_PXP,
    INTEL_HUC_DELAYED_LOAD_ERROR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_huc_authentication_type {
    INTEL_HUC_AUTH_BY_GUC = 0,
    INTEL_HUC_AUTH_BY_GSC,
    INTEL_HUC_AUTH_MAX_MODES
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_huc {
// Generic uC firmware management
    pub fw: intel_uc_fw,
// HuC-specific additions
    pub reg: i915_reg_t,
    pub mask: u32,
    pub value: u32,
    pub status: [}; INTEL_HUC_AUTH_MAX_MODES],
    pub fence: i915_sw_fence,
    pub timer: hrtimer,
    pub nb: notifier_block,
    pub status: intel_huc_delayed_load_status,
    pub delayed_load: },
// for load via GSCCS
    pub heci_pkt: *mut i915_vma,
    pub loaded_via_gsc: bool,
}

extern "C" {
    pub fn intel_huc_sanitize(huc: *mut intel_huc) -> c_int;
}
extern "C" {
    pub fn intel_huc_init_early(huc: *mut intel_huc);
}
extern "C" {
    pub fn intel_huc_fini_late(huc: *mut intel_huc);
}
extern "C" {
    pub fn intel_huc_init(huc: *mut intel_huc) -> c_int;
}
extern "C" {
    pub fn intel_huc_fini(huc: *mut intel_huc);
}
extern "C" {
    pub fn intel_huc_auth(huc: *mut intel_huc, type: intel_huc_authentication_type) -> c_int;
}
extern "C" {
    pub fn intel_huc_check_status(huc: *mut intel_huc) -> c_int;
}
extern "C" {
    pub fn intel_huc_update_auth_status(huc: *mut intel_huc);
}
extern "C" {
    pub fn intel_huc_register_gsc_notifier(huc: *mut intel_huc, bus: *const bus_type);
}
extern "C" {
    pub fn intel_huc_unregister_gsc_notifier(huc: *mut intel_huc, bus: *const bus_type);
}
extern "C" {
    pub fn intel_uc_fw_is_supported(_arg: &huc->fw) -> return;
}
extern "C" {
    pub fn intel_uc_fw_is_enabled(_arg: &huc->fw) -> return;
}
extern "C" {
    pub fn intel_uc_fw_is_available(_arg: &huc->fw) -> return;
}
extern "C" {
    pub fn intel_huc_load_status(huc: *mut intel_huc, p: *mut drm_printer);
}
