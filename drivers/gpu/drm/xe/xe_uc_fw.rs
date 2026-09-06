//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_uc_fw.h
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

extern "C" {
    pub fn xe_uc_fw_init(uc_fw: *mut xe_uc_fw) -> c_int;
}
extern "C" {
    pub fn xe_uc_fw_copy_rsa(uc_fw: *mut xe_uc_fw, dst: *mut c_void, max_len: u32) -> usize;
}
extern "C" {
    pub fn xe_uc_fw_upload(uc_fw: *mut xe_uc_fw, offset: u32, dma_flags: u32) -> c_int;
}
extern "C" {
    pub fn xe_uc_fw_check_version_requirements(uc_fw: *mut xe_uc_fw) -> c_int;
}
extern "C" {
    pub fn xe_uc_fw_print(uc_fw: *mut xe_uc_fw, p: *mut drm_printer);
}

extern "C" {
    pub fn xe_uc_fw_change_status(uc_fw: *mut xe_uc_fw, status: xe_uc_fw_status);
}

// shouldn't call this before checking hw/blob availability
//
// xe_uc_fw_get_upload_size() - Get size of firmware needed to be uploaded.
// @uc_fw: uC firmware.
//
// Get the size of the firmware and header that will be uploaded to WOPCM.
//
// Return: Upload firmware size, or zero on firmware fetch failure.
//
extern "C" {
    pub fn __xe_uc_fw_get_upload_size(_arg: uc_fw) -> return;
}

