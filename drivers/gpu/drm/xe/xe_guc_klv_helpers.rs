//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_guc_klv_helpers.h
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

extern "C" {
    pub fn xe_guc_klv_print_one(key: u16, len: u16, value: *const u32, p: *mut drm_printer);
}
extern "C" {
    pub fn xe_guc_klv_print(klvs: *const u32, num_dwords: u32, p: *mut drm_printer);
}
extern "C" {
    pub fn xe_guc_klv_count(klvs: *const u32, num_dwords: u32) -> c_int;
}
//
// PREP_GUC_KLV - Prepare KLV header value based on provided key and len.
// @key: KLV key
// @len: KLV length
//
// Return: value of the KLV header (u32).
//

//
// PREP_GUC_KLV_CONST - Prepare KLV header value based on const key and len.
// @key: const KLV key
// @len: const KLV length
//
// Return: value of the KLV header (u32).
//

//
// MAKE_GUC_KLV_KEY - Prepare KLV KEY name based on unique KLV definition tag.
// @TAG: unique tag of the KLV definition
//

//
// MAKE_GUC_KLV_LEN - Prepare KLV LEN name based on unique KLV definition tag.
// @TAG: unique tag of the KLV definition
//

//
// PREP_GUC_KLV_TAG - Prepare KLV header value based on unique KLV definition tag.
// @TAG: unique tag of the KLV definition
//
// Combine separate KEY and LEN definitions of the KLV identified by the TAG.
//
// Return: value of the KLV header (u32).
//

