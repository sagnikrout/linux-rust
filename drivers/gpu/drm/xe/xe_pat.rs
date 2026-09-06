//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_pat.h
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
// Copyright © 2023 Intel Corporation
//

//
// struct xe_pat_table_entry - The pat_index encoding and other meta information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_pat_table_entry {
//
// @value: The platform specific value encoding the various memory
// attributes (this maps to some fixed pat_index). So things like
// caching, coherency, compression etc can be encoded here.
//
    pub value: u32,
//
// @coh_mode: The GPU coherency mode that @value maps to.
//
pub const XE_COH_NONE: c_int = 1;
pub const XE_COH_1WAY: c_int = 2;
pub const XE_COH_2WAY: c_int = 3;
    pub coh_mode: u16,
//
// @valid: Set to 1 if the entry is valid, 0 if it's reserved.
//
    pub valid: u16,
}

//
// xe_pat_init_early - SW initialization, setting up data based on device
// @xe: xe device
//
extern "C" {
    pub fn xe_pat_init_early(xe: *mut xe_device);
}
//
// xe_pat_init - Program HW PAT table
// @gt: GT structure
//
extern "C" {
    pub fn xe_pat_init(gt: *mut xe_gt);
}
extern "C" {
    pub fn xe_pat_dump(gt: *mut xe_gt, p: *mut drm_printer) -> c_int;
}
extern "C" {
    pub fn xe_pat_dump_sw_config(gt: *mut xe_gt, p: *mut drm_printer) -> c_int;
}
//
// xe_pat_index_get_coh_mode - Extract the coherency mode for the given
// pat_index.
// @xe: xe device
// @pat_index: The pat_index to query
//
extern "C" {
    pub fn xe_pat_index_get_coh_mode(xe: *mut xe_device, pat_index: u16) -> u16;
}
//
// xe_pat_index_get_comp_en - Extract the compression enable flag for
// the given pat_index.
// @xe: xe device
// @pat_index: The pat_index to query
//
// Return: true if compression is enabled for this pat_index, false otherwise.
//
extern "C" {
    pub fn xe_pat_index_get_comp_en(xe: *mut xe_device, pat_index: u16) -> bool;
}

//
// xe_pat_index_get_l3_policy - Extract the L3 policy for the given pat_index.
// @xe: xe device
// @pat_index: The pat_index to query
//
extern "C" {
    pub fn xe_pat_index_get_l3_policy(xe: *mut xe_device, pat_index: u16) -> u16;
}

