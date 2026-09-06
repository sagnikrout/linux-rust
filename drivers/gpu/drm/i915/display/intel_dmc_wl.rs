//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_dmc_wl.h
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
// Copyright (C) 2024 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dmc_wl {
    pub /: *mut *mut spinlock_t lock; / protects enabled, taken, dc_state and refcount,
    pub enabled: bool,
    pub taken: bool,
    pub refcount: refcount_t,
//
// We are keeping a copy of the enabled DC state because
// intel_display.power.domains is protected by a mutex and we do
// not want call mutex_lock() in atomic context, where some of
// the tracked MMIO operations happen.
//
    pub dc_state: u32,
    pub work: delayed_work,
}

extern "C" {
    pub fn intel_dmc_wl_init(display: *mut intel_display);
}
extern "C" {
    pub fn intel_dmc_wl_enable(display: *mut intel_display, dc_state: u32);
}
extern "C" {
    pub fn intel_dmc_wl_disable(display: *mut intel_display);
}
extern "C" {
    pub fn intel_dmc_wl_flush_release_work(display: *mut intel_display);
}
extern "C" {
    pub fn intel_dmc_wl_get(display: *mut intel_display, reg: intel_reg_t);
}
extern "C" {
    pub fn intel_dmc_wl_put(display: *mut intel_display, reg: intel_reg_t);
}
extern "C" {
    pub fn intel_dmc_wl_get_noreg(display: *mut intel_display);
}
extern "C" {
    pub fn intel_dmc_wl_put_noreg(display: *mut intel_display);
}
