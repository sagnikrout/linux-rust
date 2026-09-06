//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/uc/intel_gsc_uc.h
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
// Copyright © 2022 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gsc_uc {
// Generic uC firmware management
    pub fw: intel_uc_fw,
// GSC-specific additions
//
// The GSC has 3 version numbers:
// - Release version (incremented with each build)
// - Security version (incremented on security fix)
// - Compatibility version (incremented on interface change)
//
// The one we care about to use the binary is the last one, so that's
// the one we save inside the intel_uc_fw structure. The other two
// versions are only used for debug/info purposes, so we save them here.
//
// Note that the release and security versions are available in the
// binary header, while the compatibility version must be queried after
// loading the binary.
//
    pub release: intel_uc_fw_ver,
    pub security_version: u32,
    pub /: *mut *mut *mut i915_vma local; / private memory for GSC usage,
    pub /: *mut *mut *mut void __iomem local_vaddr; / pointer to access the private memory,
    pub /: *mut *mut *mut intel_context ce; / for submission to GSC FW via GSC engine,
// for delayed load and proxy handling
    pub wq: *mut workqueue_struct,
    pub work: work_struct,
    pub /: *mut *mut u32 gsc_work_actions; / protected by gt->irq_lock,

    pub component: *mut i915_gsc_proxy_component,
    pub component_added: bool,
    pub vma: *mut i915_vma,
    pub to_gsc: *mut c_void,
    pub to_csme: *mut c_void,
    pub /: *mut *mut mutex mutex; / protects the tee channel binding,
    pub proxy: },
}

extern "C" {
    pub fn intel_gsc_uc_init_early(gsc: *mut intel_gsc_uc);
}
extern "C" {
    pub fn intel_gsc_uc_init(gsc: *mut intel_gsc_uc) -> c_int;
}
extern "C" {
    pub fn intel_gsc_uc_fini(gsc: *mut intel_gsc_uc);
}
extern "C" {
    pub fn intel_gsc_uc_suspend(gsc: *mut intel_gsc_uc);
}
extern "C" {
    pub fn intel_gsc_uc_resume(gsc: *mut intel_gsc_uc);
}
extern "C" {
    pub fn intel_gsc_uc_flush_work(gsc: *mut intel_gsc_uc);
}
extern "C" {
    pub fn intel_gsc_uc_load_start(gsc: *mut intel_gsc_uc);
}
extern "C" {
    pub fn intel_gsc_uc_load_status(gsc: *mut intel_gsc_uc, p: *mut drm_printer);
}
extern "C" {
    pub fn intel_uc_fw_is_supported(_arg: &gsc->fw) -> return;
}
extern "C" {
    pub fn intel_uc_fw_is_enabled(_arg: &gsc->fw) -> return;
}
extern "C" {
    pub fn intel_uc_fw_is_available(_arg: &gsc->fw) -> return;
}
