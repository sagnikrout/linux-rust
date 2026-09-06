//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_hw_engine_class_sysfs.h
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

extern "C" {
    pub fn xe_hw_engine_class_sysfs_init(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_hw_engine_timeout_in_range(timeout: u64, min: u64, max: u64) -> bool;
}
//
// struct kobj_eclass - A eclass's kobject struct that connects the kobject and the
// eclass.
//
// When dealing with multiple eclass, this struct helps to understand which eclass
// needs to be addressed on a given sysfs call.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kobj_eclass {
// @base: The actual kobject
    pub base: kobject,
// @eclass: A pointer to the hw engine class interface
    pub eclass: *mut xe_hw_engine_class_intf,
// @xe: A pointer to the xe device
    pub xe: *mut xe_device,
}
