//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_dmc.h
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

extern "C" {
    pub fn intel_dmc_init(display: *mut intel_display);
}
extern "C" {
    pub fn intel_dmc_load_program(display: *mut intel_display);
}
extern "C" {
    pub fn intel_dmc_wait_fw_load(display: *mut intel_display);
}
extern "C" {
    pub fn intel_dmc_disable_program(display: *mut intel_display);
}
extern "C" {
    pub fn intel_dmc_enable_pipe(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_dmc_disable_pipe(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_dmc_fini(display: *mut intel_display);
}
extern "C" {
    pub fn intel_dmc_suspend(display: *mut intel_display);
}
extern "C" {
    pub fn intel_dmc_resume(display: *mut intel_display);
}
extern "C" {
    pub fn intel_dmc_has_payload(display: *mut intel_display) -> bool;
}
extern "C" {
    pub fn intel_dmc_debugfs_register(display: *mut intel_display);
}
extern "C" {
    pub fn intel_dmc_snapshot_print(snapshot: *const intel_dmc_snapshot, p: *mut drm_printer);
}
extern "C" {
    pub fn intel_dmc_update_dc6_allowed_count(display: *mut intel_display, start_tracking: bool);
}
extern "C" {
    pub fn assert_main_dmc_loaded(display: *mut intel_display);
}
extern "C" {
    pub fn intel_pipedmc_irq_handler(display: *mut intel_display, pipe: pipe);
}
extern "C" {
    pub fn intel_pipedmc_dcb_enable(dsb: *mut intel_dsb, crtc: *mut intel_crtc);
}
extern "C" {
    pub fn intel_pipedmc_dcb_disable(dsb: *mut intel_dsb, crtc: *mut intel_crtc);
}
extern "C" {
    pub fn intel_pipedmc_start_mmioaddr(crtc: *mut intel_crtc) -> u32;
}
extern "C" {
    pub fn intel_pipedmc_irq_handler(display: *mut intel_display, pipe: pipe);
}
