//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_gt_mcr.h
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

extern "C" {
    pub fn xe_gt_mcr_init_early(gt: *mut xe_gt);
}
extern "C" {
    pub fn xe_gt_mcr_init(gt: *mut xe_gt);
}
extern "C" {
    pub fn xe_gt_mcr_set_implicit_defaults(gt: *mut xe_gt);
}
extern "C" {
    pub fn xe_gt_mcr_unicast_read_any(gt: *mut xe_gt, mcr_reg: xe_reg_mcr) -> u32;
}
extern "C" {
    pub fn xe_gt_mcr_check_reg(gt: *mut xe_gt, reg: xe_reg) -> bool;
}
extern "C" {
    pub fn xe_gt_mcr_steering_dump(gt: *mut xe_gt, p: *mut drm_printer);
}
//
// Loop over each DSS and determine the group and instance IDs that
// should be used to steer MCR accesses toward this DSS.
// @dss: DSS ID to obtain steering for
// @gt: GT structure
// @group: steering group ID, data type: u16
// @instance: steering instance ID, data type: u16
//

//
// Loop over each DSS available for geometry and determine the group and
// instance IDs that should be used to steer MCR accesses toward this DSS.
// @dss: DSS ID to obtain steering for
// @gt: GT structure
// @group: steering group ID, data type: u16
// @instance: steering instance ID, data type: u16
//

//
// Loop over each DSS available for compute and determine the group and
// instance IDs that should be used to steer MCR accesses toward this DSS.
// @dss: DSS ID to obtain steering for
// @gt: GT structure
// @group: steering group ID, data type: u16
// @instance: steering instance ID, data type: u16
//

