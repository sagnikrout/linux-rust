//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/panthor/panthor_gpu.h
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


// SPDX-License-Identifier: GPL-2.0 or MIT
// Copyright 2018 Marty E. Plummer <hanetzer@startmail.com>
// Copyright 2019 Collabora ltd.

extern "C" {
    pub fn panthor_gpu_init(ptdev: *mut panthor_device) -> c_int;
}
extern "C" {
    pub fn panthor_gpu_unplug(ptdev: *mut panthor_device);
}
extern "C" {
    pub fn panthor_gpu_suspend(ptdev: *mut panthor_device);
}
extern "C" {
    pub fn panthor_gpu_resume(ptdev: *mut panthor_device);
}
//
// panthor_gpu_power_on() - Power on the GPU block.
//
// Return: 0 on success, a negative error code otherwise.
//

//
// panthor_gpu_power_off() - Power off the GPU block.
//
// Return: 0 on success, a negative error code otherwise.
//

extern "C" {
    pub fn panthor_gpu_l2_power_off(ptdev: *mut panthor_device);
}
extern "C" {
    pub fn panthor_gpu_l2_power_on(ptdev: *mut panthor_device) -> c_int;
}
extern "C" {
    pub fn panthor_gpu_soft_reset(ptdev: *mut panthor_device) -> c_int;
}
extern "C" {
    pub fn panthor_gpu_power_changed_off(ptdev: *mut panthor_device);
}
extern "C" {
    pub fn panthor_gpu_power_changed_on(ptdev: *mut panthor_device) -> c_int;
}
extern "C" {
    pub fn panthor_gpu_get_timestamp(ptdev: *mut panthor_device) -> u64;
}
extern "C" {
    pub fn panthor_gpu_get_timestamp_offset(ptdev: *mut panthor_device) -> u64;
}
extern "C" {
    pub fn panthor_gpu_get_cycle_count(ptdev: *mut panthor_device) -> u64;
}
extern "C" {
    pub fn panthor_gpu_coherency_init(ptdev: *mut panthor_device) -> c_int;
}
