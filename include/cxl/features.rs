//! Automatically rewritten from C Header to Rust Module
//! Source: include/cxl/features.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright(c) 2024-2025 Intel Corporation.

// Feature UUIDs used by the kernel

// Feature commands capability supported by a device
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxl_features_capability {
    CXL_FEATURES_NONE = 0,
    CXL_FEATURES_RO,
    CXL_FEATURES_RW,
}

//
// struct cxl_features_state - The Features state for the device
// @cxlds: Pointer to CXL device state
// @entries: CXl feature entry context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_features_state {
    pub cxlds: *mut cxl_dev_state,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_feat_entries {
    pub num_features: c_int,
    pub num_user_features: c_int,
    pub __counted_by(num_features): cxl_feat_entry ent[],
    pub entries: *mut },
}

extern "C" {
    pub fn devm_cxl_setup_features(cxlds: *mut cxl_dev_state) -> c_int;
}
extern "C" {
    pub fn devm_cxl_setup_fwctl(host: *mut device, cxlmd: *mut cxl_memdev) -> c_int;
}

