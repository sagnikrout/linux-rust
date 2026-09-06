//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_guc_klv_thresholds_set.h
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

//
// MAKE_GUC_KLV_VF_CFG_THRESHOLD_KEY - Prepare the name of the KLV key constant.
// @TAG: unique tag of the GuC threshold KLV key.
//

//
// MAKE_GUC_KLV_VF_CFG_THRESHOLD_LEN - Prepare the name of the KLV length constant.
// @TAG: unique tag of the GuC threshold KLV key.
//

//
// xe_guc_klv_threshold_key_to_index - Find index of the tracked GuC threshold.
// @key: GuC threshold KLV key.
//
// This translation is automatically generated using &MAKE_XE_GUC_KLV_THRESHOLDS_SET.
// Return: index of the GuC threshold KLV or -1 if not found.
//

extern "C" {
    pub fn MAKE_XE_GUC_KLV_THRESHOLD_INDEX(_arg: TAG) -> return;
}
// private: auto-generated case statements

//
// xe_guc_klv_threshold_index_to_key - Get tracked GuC threshold KLV key.
// @index: GuC threshold KLV index.
//
// This translation is automatically generated using &MAKE_XE_GUC_KLV_THRESHOLDS_SET.
// Return: key of the GuC threshold KLV or 0 on malformed index.
//

extern "C" {
    pub fn MAKE_GUC_KLV_VF_CFG_THRESHOLD_KEY(_arg: TAG) -> return;
}
// private: auto-generated case statements

