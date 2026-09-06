//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_guc_pc_types.h
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
// struct xe_guc_pc - GuC Power Conservation (PC)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_guc_pc {
// @bo: GGTT buffer object that is shared with GuC PC
    pub bo: *mut xe_bo,
// @flush_freq_limit: 1 when max freq changes are limited by driver
    pub flush_freq_limit: core::sync::atomic::AtomicI32,
// @rp0_freq: HW RP0 frequency - The Maximum one
    pub rp0_freq: u32,
// @rpn_freq: HW RPN frequency - The Minimum one
    pub rpn_freq: u32,
// @user_requested_min: Stash the minimum requested freq by user
    pub user_requested_min: u32,
// @user_requested_max: Stash the maximum requested freq by user
    pub user_requested_max: u32,
// @stashed_min_freq: Stash the current minimum freq
    pub stashed_min_freq: u32,
// @stashed_max_freq: Stash the current maximum freq
    pub stashed_max_freq: u32,
// @freq_lock: Let's protect the frequencies
    pub freq_lock: mutex,
// @freq_ready: Only handle freq changes, if they are really ready
    pub freq_ready: bool,
// @power_profile: Base or power_saving profile
    pub power_profile: u32,
}
