//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_gt_topology.h
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
// Loop over each DSS with the bit is 1 in geometry or compute mask
// @dss: iterated DSS bit from the DSS mask
// @gt: GT structure
//

extern "C" {
    pub fn xe_gt_topology_init(gt: *mut xe_gt);
}
extern "C" {
    pub fn xe_gt_topology_dump(gt: *mut xe_gt, p: *mut drm_printer) -> c_int;
}
//
// xe_gt_topology_mask_last_dss() - Returns the index of the last DSS in a mask.
// @mask: Input DSS mask
//
// Return: Index of the last DSS in the input DSS mask,
// XE_MAX_DSS_FUSE_BITS if DSS mask is empty.
//
extern "C" {
    pub fn find_last_bit(_arg: mask, _arg: XE_MAX_DSS_FUSE_BITS) -> return;
}
extern "C" {
    pub fn xe_gt_has_geometry_dss(gt: *mut xe_gt, dss: c_uint) -> bool;
}
extern "C" {
    pub fn xe_gt_has_compute_dss(gt: *mut xe_gt, dss: c_uint) -> bool;
}
extern "C" {
    pub fn xe_gt_has_discontiguous_dss_groups(gt: *const xe_gt) -> bool;
}
extern "C" {
    pub fn xe_gt_topology_report_l3(gt: *mut xe_gt) -> bool;
}
