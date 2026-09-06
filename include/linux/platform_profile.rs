//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_profile.h
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
//
// Platform profile sysfs interface
//
// See Documentation/userspace-api/sysfs-platform_profile.rst for more
// information.
//

//
// If more options are added please update profile_names array in
// platform_profile.c and sysfs-platform_profile documentation.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum platform_profile_option {
    PLATFORM_PROFILE_LOW_POWER,
    PLATFORM_PROFILE_COOL,
    PLATFORM_PROFILE_QUIET,
    PLATFORM_PROFILE_BALANCED,
    PLATFORM_PROFILE_BALANCED_PERFORMANCE,
    PLATFORM_PROFILE_PERFORMANCE,
    PLATFORM_PROFILE_MAX_POWER,
    PLATFORM_PROFILE_CUSTOM,
    PLATFORM_PROFILE_LAST, /*must always be last */
}

//
// struct platform_profile_ops - platform profile operations
// @probe: Callback to setup choices available to the new class device. These
// choices will only be enforced when setting a new profile, not when
// getting the current one.
// @hidden_choices: Callback to setup choices that are not visible to the user
// but can be set by the driver.
// @profile_get: Callback that will be called when showing the current platform
// profile in sysfs.
// @profile_set: Callback that will be called when storing a new platform
// profile in sysfs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_profile_ops {
    pub choices): *mut *mut *mut int (probe)(void drvdata, unsigned long,
    pub choices): *mut *mut *mut int (hidden_choices)(void drvdata, unsigned long,
    pub profile): *mut *mut *mut int (profile_get)(struct device dev, enum platform_profile_option,
    pub profile): *mut *mut *mut int (profile_set)(struct device dev, enum platform_profile_option,
}

extern "C" {
    pub fn platform_profile_remove(dev: *mut device);
}
extern "C" {
    pub fn platform_profile_cycle() -> c_int;
}
extern "C" {
    pub fn platform_profile_notify(dev: *mut device);
}
