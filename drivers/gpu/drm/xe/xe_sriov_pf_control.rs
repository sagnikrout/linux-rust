//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_sriov_pf_control.h
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
// Copyright © 2025 Intel Corporation
//
extern "C" {
    pub fn xe_sriov_pf_control_pause_vf(xe: *mut xe_device, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_control_resume_vf(xe: *mut xe_device, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_control_stop_vf(xe: *mut xe_device, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_control_reset_vf(xe: *mut xe_device, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_control_prepare_flr(xe: *mut xe_device, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_control_wait_flr(xe: *mut xe_device, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_control_sync_flr(xe: *mut xe_device, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_control_trigger_save_vf(xe: *mut xe_device, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_control_finish_save_vf(xe: *mut xe_device, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_control_trigger_restore_vf(xe: *mut xe_device, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_control_finish_restore_vf(xe: *mut xe_device, vfid: c_uint) -> c_int;
}
