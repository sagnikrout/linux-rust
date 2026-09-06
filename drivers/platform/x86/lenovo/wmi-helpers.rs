//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/lenovo/wmi-helpers.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2025 Derek J. Clark <derekjohn.clark@gmail.com>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_method_args_32 {
    pub arg0: u32,
    pub arg1: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lwmi_event_type {
    LWMI_GZ_GET_THERMAL_MODE = 0x01,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum thermal_mode {
    LWMI_GZ_THERMAL_MODE_NONE =	   0x00,
    LWMI_GZ_THERMAL_MODE_QUIET =	   0x01,
    LWMI_GZ_THERMAL_MODE_BALANCED =	   0x02,
    LWMI_GZ_THERMAL_MODE_PERFORMANCE = 0x03,
    LWMI_GZ_THERMAL_MODE_EXTREME =	   0xE0, /* Ver 6+ */
    LWMI_GZ_THERMAL_MODE_CUSTOM =	   0xFF,
}

extern "C" {
    pub fn lwmi_tm_register_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn lwmi_tm_unregister_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn lwmi_tm_notifier_call(mode: *mut thermal_mode) -> c_int;
}
