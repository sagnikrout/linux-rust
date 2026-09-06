//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_guc_capture.h
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
// Copyright © 2021-2024 Intel Corporation
//

extern "C" {
    pub fn xe_guc_class_to_capture_class(_arg: xe_hwe_to_guc_class(hwe)) -> return;
}
extern "C" {
    pub fn xe_guc_capture_process(guc: *mut xe_guc);
}
extern "C" {
    pub fn xe_guc_capture_getnullheader(guc: *mut xe_guc, outptr: *mut c_void, size: *mut usize) -> c_int;
}
extern "C" {
    pub fn xe_guc_capture_ads_input_worst_size(guc: *mut xe_guc) -> usize;
}
extern "C" {
    pub fn xe_engine_manual_capture(hwe: *mut xe_hw_engine, snapshot: *mut xe_hw_engine_snapshot);
}
extern "C" {
    pub fn xe_engine_snapshot_print(snapshot: *mut xe_hw_engine_snapshot, p: *mut drm_printer);
}
extern "C" {
    pub fn xe_engine_snapshot_capture_for_queue(q: *mut xe_exec_queue);
}
extern "C" {
    pub fn xe_guc_capture_steered_list_init(guc: *mut xe_guc);
}
extern "C" {
    pub fn xe_guc_capture_put_matched_nodes(guc: *mut xe_guc);
}
extern "C" {
    pub fn xe_guc_capture_init(guc: *mut xe_guc) -> c_int;
}
