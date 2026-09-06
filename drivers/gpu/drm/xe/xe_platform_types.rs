//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_platform_types.h
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
// Keep this in graphics version based order and chronological order within a
// version
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_platform {
    XE_PLATFORM_UNINITIALIZED = 0,
    XE_TIGERLAKE,
    XE_ROCKETLAKE,
    XE_ALDERLAKE_S,
    XE_ALDERLAKE_P,
    XE_ALDERLAKE_N,
    XE_DG1,
    XE_DG2,
    XE_PVC,
    XE_METEORLAKE,
    XE_LUNARLAKE,
    XE_BATTLEMAGE,
    XE_PANTHERLAKE,
    XE_NOVALAKE_S,
    XE_CRESCENTISLAND,
    XE_NOVALAKE_P,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_subplatform {
    XE_SUBPLATFORM_UNINITIALIZED = 0,
    XE_SUBPLATFORM_NONE,
    XE_SUBPLATFORM_ALDERLAKE_P_RPLU,
    XE_SUBPLATFORM_ALDERLAKE_S_RPLS,
    XE_SUBPLATFORM_DG2_G10,
    XE_SUBPLATFORM_DG2_G11,
    XE_SUBPLATFORM_DG2_G12,
    XE_SUBPLATFORM_BATTLEMAGE_G21,
}
