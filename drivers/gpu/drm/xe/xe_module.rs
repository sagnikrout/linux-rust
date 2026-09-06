//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_module.h
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

// Module modprobe variables
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_modparam {
    pub probe_display: bool,
    pub force_vram_bar_size: c_int,
    pub guc_log_level: c_int,
    pub guc_firmware_path: *mut c_char,
    pub huc_firmware_path: *mut c_char,
    pub gsc_firmware_path: *mut c_char,
    pub force_probe: *mut c_char,

    pub max_vfs: c_uint,

    pub wedged_mode: c_uint,
    pub svm_notifier_size: u32,
}

extern "C" {
    pub fn xe_destroy_wq_queue(work: *mut work_struct) -> bool;
}
extern "C" {
    pub fn xe_destroy_wq_flush();
}
