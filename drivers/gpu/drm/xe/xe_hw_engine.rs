//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_hw_engine.h
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
// Copyright © 2021 Intel Corporation
//

pub const XE_HW_ENGINE_JOB_TIMEOUT_MIN: c_int = 1;

pub const XE_HW_ENGINE_TIMESLICE_MIN: c_int = 1;

pub const XE_HW_ENGINE_PREEMPT_TIMEOUT_MIN: c_int = 1;

extern "C" {
    pub fn xe_hw_engines_init_early(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_hw_engines_init(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_hw_engine_handle_irq(hwe: *mut xe_hw_engine, intr_vec: u16);
}
extern "C" {
    pub fn xe_hw_engine_enable_ring(hwe: *mut xe_hw_engine);
}
extern "C" {
    pub fn xe_hw_engine_snapshot_free(snapshot: *mut xe_hw_engine_snapshot);
}
extern "C" {
    pub fn xe_hw_engine_print(hwe: *mut xe_hw_engine, p: *mut drm_printer);
}
extern "C" {
    pub fn xe_hw_engine_setup_reg_lrc(hwe: *mut xe_hw_engine);
}
extern "C" {
    pub fn xe_hw_engine_is_reserved(hwe: *mut xe_hw_engine) -> bool;
}
extern "C" {
    pub fn xe_hw_engine_read_timestamp(hwe: *mut xe_hw_engine) -> u64;
}
extern "C" {
    pub fn xe_hw_engine_to_fw_domain(hwe: *mut xe_hw_engine) -> xe_force_wake_domains;
}
extern "C" {
    pub fn xe_hw_engine_mmio_read32(hwe: *mut xe_hw_engine, reg: xe_reg) -> u32;
}
