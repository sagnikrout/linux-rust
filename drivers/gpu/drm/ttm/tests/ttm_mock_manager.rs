//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/ttm/tests/ttm_mock_manager.h
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
pub struct ttm_mock_manager {
    pub man: ttm_resource_manager,
    pub mm: gpu_buddy,
    pub default_page_size: u64,
// protects allocations of mock buffer objects
    pub lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_mock_resource {
    pub base: ttm_resource,
    pub blocks: list_head,
    pub flags: c_ulong,
}

extern "C" {
    pub fn ttm_mock_manager_init(bdev: *mut ttm_device, mem_type: u32, size: u32) -> c_int;
}
extern "C" {
    pub fn ttm_bad_manager_init(bdev: *mut ttm_device, mem_type: u32, size: u32) -> c_int;
}
extern "C" {
    pub fn ttm_busy_manager_init(bdev: *mut ttm_device, mem_type: u32, size: u32) -> c_int;
}
extern "C" {
    pub fn ttm_mock_manager_fini(bdev: *mut ttm_device, mem_type: u32);
}
extern "C" {
    pub fn ttm_bad_manager_fini(bdev: *mut ttm_device, mem_type: u32);
}
