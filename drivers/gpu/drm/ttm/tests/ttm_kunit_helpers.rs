//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/ttm/tests/ttm_kunit_helpers.h
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


// SPDX-License-Identifier: GPL-2.0 AND MIT
//
// Copyright © 2023 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_test_devices {
    pub drm: *mut drm_device,
    pub dev: *mut device,
    pub ttm_dev: *mut ttm_device,
}

// Building blocks for test-specific init functions
extern "C" {
    pub fn dummy_ttm_bo_destroy(bo: *mut ttm_buffer_object);
}
extern "C" {
    pub fn ttm_test_devices_put(test: *mut kunit, devs: *mut ttm_test_devices);
}
// Generic init/fini for tests that only need DRM/TTM devices
extern "C" {
    pub fn ttm_test_devices_init(test: *mut kunit) -> c_int;
}
extern "C" {
    pub fn ttm_test_devices_all_init(test: *mut kunit) -> c_int;
}
extern "C" {
    pub fn ttm_test_devices_fini(test: *mut kunit);
}
