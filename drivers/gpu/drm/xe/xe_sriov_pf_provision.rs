//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_sriov_pf_provision.h
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
    pub fn xe_sriov_pf_provision_bulk_apply_eq(xe: *mut xe_device, eq: u32) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_provision_apply_vf_eq(xe: *mut xe_device, vfid: c_uint, eq: u32) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_provision_query_vf_eq(xe: *mut xe_device, vfid: c_uint, eq: *mut u32) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_provision_bulk_apply_pt(xe: *mut xe_device, pt: u32) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_provision_apply_vf_pt(xe: *mut xe_device, vfid: c_uint, pt: u32) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_provision_query_vf_pt(xe: *mut xe_device, vfid: c_uint, pt: *mut u32) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_provision_bulk_apply_priority(xe: *mut xe_device, prio: u32) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_provision_apply_vf_priority(xe: *mut xe_device, vfid: c_uint, prio: u32) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_provision_query_vf_priority(xe: *mut xe_device, vfid: c_uint, prio: *mut u32) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_provision_bulk_apply_vram(xe: *mut xe_device, size: u64) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_provision_apply_vf_vram(xe: *mut xe_device, vfid: c_uint, size: u64) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_provision_query_vf_vram(xe: *mut xe_device, vfid: c_uint, size: *mut u64) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_provision_vfs(xe: *mut xe_device, num_vfs: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_unprovision_vfs(xe: *mut xe_device, num_vfs: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_reprovision_default(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_sriov_pf_provision_set_mode(xe: *mut xe_device, mode: xe_sriov_provisioning_mode) -> c_int;
}
//
// xe_sriov_pf_provision_set_custom_mode() - Change VFs provision mode to custom.
// @xe: the PF &xe_device
//
// This function can only be called on PF.
//
// Return: 0 on success or a negative error code on failure.
//
extern "C" {
    pub fn xe_sriov_pf_provision_set_mode(_arg: xe, _arg: XE_SRIOV_PROVISIONING_MODE_CUSTOM) -> return;
}
