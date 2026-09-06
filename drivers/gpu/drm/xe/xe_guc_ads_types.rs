//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_guc_ads_types.h
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
// struct xe_guc_ads - GuC additional data structures (ADS)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_guc_ads {
// @bo: Xe BO for GuC ads blob
    pub bo: *mut xe_bo,
//
// @um_queue_bo: Dedicated BO for the HW fault ring (UM queues).
// NULL if the platform does not support USM.
//
    pub um_queue_bo: *mut xe_bo,
// @golden_lrc_size: golden LRC size
    pub golden_lrc_size: usize,
// @regset_size: size of register set passed to GuC for save/restore
    pub regset_size: u32,
// @ads_waklv_size: total waklv size supported by platform
    pub ads_waklv_size: u32,
// @capture_size: size of register set passed to GuC for capture
    pub capture_size: u32,
}
