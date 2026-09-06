//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_dp_mst.h
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
    pub fn intel_dp_mst_encoder_init(dig_port: *mut intel_digital_port, conn_id: c_int) -> c_int;
}
extern "C" {
    pub fn intel_dp_mst_encoder_cleanup(dig_port: *mut intel_digital_port);
}
extern "C" {
    pub fn intel_dp_mst_active_streams(intel_dp: *mut intel_dp) -> c_int;
}
extern "C" {
    pub fn intel_dp_mst_is_master_trans(crtc_state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_dp_mst_is_slave_trans(crtc_state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_dp_mst_source_support(intel_dp: *mut intel_dp) -> bool;
}
extern "C" {
    pub fn intel_dp_mst_prepare_probe(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_dp_mst_verify_dpcd_state(intel_dp: *mut intel_dp) -> bool;
}
