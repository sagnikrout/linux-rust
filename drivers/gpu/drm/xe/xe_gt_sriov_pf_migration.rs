//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_gt_sriov_pf_migration.h
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
// Copyright © 2024 Intel Corporation
//

// TODO: get this information by querying GuC in the future

extern "C" {
    pub fn xe_gt_sriov_pf_migration_init(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_migration_guc_save(gt: *mut xe_gt, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_migration_ggtt_save(gt: *mut xe_gt, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_migration_mmio_save(gt: *mut xe_gt, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_migration_vram_save(gt: *mut xe_gt, vfid: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_gt_sriov_pf_migration_size(gt: *mut xe_gt, vfid: c_uint) -> isize;
}
extern "C" {
    pub fn xe_gt_sriov_pf_migration_ring_empty(gt: *mut xe_gt, vfid: c_uint) -> bool;
}
extern "C" {
    pub fn xe_gt_sriov_pf_migration_ring_full(gt: *mut xe_gt, vfid: c_uint) -> bool;
}
extern "C" {
    pub fn xe_gt_sriov_pf_migration_ring_free(gt: *mut xe_gt, vfid: c_uint);
}
extern "C" {
    pub fn xe_gt_sriov_pf_migration_save_init(gt: *mut xe_gt, vfid: c_uint);
}
