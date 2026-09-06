//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_gt_idle_types.h
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

// States of GT Idle
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_gt_idle_state {
    GT_IDLE_C0,
    GT_IDLE_C6,
    GT_IDLE_UNKNOWN,
}

//
// struct xe_gt_idle - A struct that contains idle properties based of gt
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_gt_idle {
// @name: name
    pub name: [c_char; 16],
// @powergate_enable: copy of powergate enable bits
    pub powergate_enable: u32,
// @residency_multiplier: residency multiplier in ns
    pub residency_multiplier: u32,
// @cur_residency: raw driver copy of idle residency
    pub cur_residency: u64,
// @prev_residency: previous residency counter
    pub prev_residency: u64,
// @lock: Lock protecting idle residency counters
    pub lock: raw_spinlock_t,
// @idle_status: get the current idle state
    pub pc): *mut *mut xe_gt_idle_state (idle_status)(struct xe_guc_pc,
// @idle_residency: get idle residency counter
    pub pc): *mut *mut u64 (idle_residency)(struct xe_guc_pc,
}
