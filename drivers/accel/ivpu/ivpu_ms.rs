//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/ivpu/ivpu_ms.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Copyright (C) 2020-2024 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivpu_ms_instance {
    pub bo: *mut ivpu_bo,
    pub ms_instance_node: list_head,
    pub mask: u64,
    pub buff_size: u64,
    pub active_buff_vpu_addr: u64,
    pub inactive_buff_vpu_addr: u64,
    pub active_buff_ptr: *mut c_void,
    pub inactive_buff_ptr: *mut c_void,
    pub leftover_bytes: u64,
    pub leftover_addr: *mut c_void,
}

extern "C" {
    pub fn ivpu_ms_start_ioctl(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn ivpu_ms_stop_ioctl(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn ivpu_ms_get_data_ioctl(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn ivpu_ms_get_info_ioctl(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn ivpu_ms_cleanup(file_priv: *mut ivpu_file_priv);
}
extern "C" {
    pub fn ivpu_ms_cleanup_all(vdev: *mut ivpu_device);
}
