//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/panthor/panthor_hw.h
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
// Copyright 2025 ARM Limited. All rights reserved.

//
// struct panthor_hw_ops - HW operations that are specific to a GPU
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_hw_ops {
// @soft_reset: Soft reset function pointer
    pub ptdev): *mut *mut int (soft_reset)(struct panthor_device,
// @l2_power_off: L2 power off function pointer
    pub ptdev): *mut *mut void (l2_power_off)(struct panthor_device,
// @l2_power_on: L2 power on function pointer
    pub ptdev): *mut *mut int (l2_power_on)(struct panthor_device,
// @power_changed_on: Start listening to power change IRQs
    pub ptdev): *mut *mut int (power_changed_on)(struct panthor_device,
// @power_changed_off: Stop listening to power change IRQs
    pub ptdev): *mut *mut void (power_changed_off)(struct panthor_device,
}

//
// struct panthor_hw - GPU specific register mapping and functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_hw {
// @features: Bitmap containing panthor_hw_feature
// @ops: Panthor HW specific operations
    pub ops: panthor_hw_ops,
}

extern "C" {
    pub fn panthor_hw_init(ptdev: *mut panthor_device) -> c_int;
}
extern "C" {
    pub fn panthor_hw_power_status_register() -> c_int;
}
extern "C" {
    pub fn panthor_hw_power_status_unregister();
}
