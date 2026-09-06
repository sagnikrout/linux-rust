//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_guc.h
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

//
// GuC version number components are defined to be only 8-bit size,
// so converting to a 32bit 8.8.8 integer allows simple (and safe)
// numerical comparisons.
//

extern "C" {
    pub fn xe_guc_comm_init_early(guc: *mut xe_guc);
}
extern "C" {
    pub fn xe_guc_init_noalloc(guc: *mut xe_guc) -> c_int;
}
extern "C" {
    pub fn xe_guc_init(guc: *mut xe_guc) -> c_int;
}
extern "C" {
    pub fn xe_guc_init_post_hwconfig(guc: *mut xe_guc) -> c_int;
}
extern "C" {
    pub fn xe_guc_post_load_init(guc: *mut xe_guc) -> c_int;
}
extern "C" {
    pub fn xe_guc_reset(guc: *mut xe_guc) -> c_int;
}
extern "C" {
    pub fn xe_guc_upload(guc: *mut xe_guc) -> c_int;
}
extern "C" {
    pub fn xe_guc_min_load_for_hwconfig(guc: *mut xe_guc) -> c_int;
}
extern "C" {
    pub fn xe_guc_enable_communication(guc: *mut xe_guc) -> c_int;
}
extern "C" {
    pub fn xe_guc_opt_in_features_enable(guc: *mut xe_guc) -> c_int;
}
extern "C" {
    pub fn xe_guc_runtime_suspend(guc: *mut xe_guc);
}
extern "C" {
    pub fn xe_guc_runtime_resume(guc: *mut xe_guc);
}
extern "C" {
    pub fn xe_guc_suspend(guc: *mut xe_guc) -> c_int;
}
extern "C" {
    pub fn xe_guc_softreset(guc: *mut xe_guc) -> c_int;
}
extern "C" {
    pub fn xe_guc_notify(guc: *mut xe_guc);
}
extern "C" {
    pub fn xe_guc_auth_huc(guc: *mut xe_guc, rsa_addr: u32) -> c_int;
}
extern "C" {
    pub fn xe_guc_mmio_send(guc: *mut xe_guc, request: *const u32, len: u32) -> c_int;
}
extern "C" {
    pub fn xe_guc_self_cfg32(guc: *mut xe_guc, key: u16, val: u32) -> c_int;
}
extern "C" {
    pub fn xe_guc_self_cfg64(guc: *mut xe_guc, key: u16, val: u64) -> c_int;
}
extern "C" {
    pub fn xe_guc_irq_handler(guc: *mut xe_guc, iir: u16);
}
extern "C" {
    pub fn xe_guc_sanitize(guc: *mut xe_guc);
}
extern "C" {
    pub fn xe_guc_print_info(guc: *mut xe_guc, p: *mut drm_printer) -> c_int;
}
extern "C" {
    pub fn xe_guc_reset_prepare(guc: *mut xe_guc) -> c_int;
}
extern "C" {
    pub fn xe_guc_reset_wait(guc: *mut xe_guc);
}
extern "C" {
    pub fn xe_guc_stop_prepare(guc: *mut xe_guc);
}
extern "C" {
    pub fn xe_guc_stop(guc: *mut xe_guc);
}
extern "C" {
    pub fn xe_guc_start(guc: *mut xe_guc) -> c_int;
}
extern "C" {
    pub fn xe_guc_declare_wedged(guc: *mut xe_guc);
}
extern "C" {
    pub fn xe_guc_using_main_gamctrl_queues(guc: *mut xe_guc) -> bool;
}
extern "C" {
    pub fn xe_guc_has_paging_engine(guc: *mut xe_guc) -> bool;
}

extern "C" {
    pub fn xe_guc_g2g_test_notification(guc: *mut xe_guc, payload: *mut u32, len: u32) -> c_int;
}

extern "C" {
    pub fn xe_hwe_to_guc_class(hwe: *mut xe_hw_engine) -> u16;
}
extern "C" {
    pub fn xe_hwe_guc_logical_instance(hwe: *mut xe_hw_engine) -> u16;
}
extern "C" {
    pub fn container_of(_arg: guc, xe_gt: struct, _arg: uc.guc) -> return;
}
extern "C" {
    pub fn gt_to_xe(_arg: guc_to_gt(guc)) -> return;
}
//
// xe_guc_fw_version_at_least() - Check if GuC is at least of given version.
// @guc: the &xe_guc
// @ver: the version to check
//
// The @ver should be prepared using MAKE_GUC_VER(major, minor, patch).
//
// Return: true if loaded GuC firmware is at least of given version,
// false otherwise.
//
