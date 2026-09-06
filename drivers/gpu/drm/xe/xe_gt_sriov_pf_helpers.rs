//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_gt_sriov_pf_helpers.h
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
// Copyright © 2023-2024 Intel Corporation
//

//
// xe_gt_sriov_pf_assert_vfid() - warn if &id is not a supported VF number when debugging.
// @gt: the PF &xe_gt to assert on
// @vfid: the VF number to assert
//
// Assert that &gt belongs to the Physical Function (PF) device and provided &vfid
// is within a range of supported VF numbers (up to maximum number of VFs that
// driver can support, including VF0 that represents the PF itself).
//
// Note: Effective only on debug builds. See `Xe Asserts`_ for more information.
//

extern "C" {
    pub fn xe_sriov_pf_get_totalvfs(_arg: gt_to_xe(gt)) -> return;
}
extern "C" {
    pub fn xe_sriov_pf_master_mutex(_arg: gt_to_xe(gt)) -> return;
}
