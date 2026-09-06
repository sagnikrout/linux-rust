//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firmware/trusted_foundations.h
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
// Copyright (c) 2013, NVIDIA Corporation.
//
// Support for the Trusted Foundations secure monitor.
//
// Trusted Foundation comes active on some ARM consumer devices (most
// Tegra-based devices sold on the market are concerned). Such devices can only
// perform some basic operations, like setting the CPU reset vector, through
// SMC calls to the secure monitor. The calls are completely specific to
// Trusted Foundations, and do *not* follow the SMC calling convention or the
// PSCI standard.
//

pub const TF_PM_MODE_LP0: c_int = 0;
pub const TF_PM_MODE_LP1: c_int = 1;
pub const TF_PM_MODE_LP1_NO_MC_CLK: c_int = 2;
pub const TF_PM_MODE_LP2: c_int = 3;
pub const TF_PM_MODE_LP2_NOFLUSH_L2: c_int = 4;
pub const TF_PM_MODE_NONE: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trusted_foundations_platform_data {
    pub version_major: c_uint,
    pub version_minor: c_uint,
}

extern "C" {
    pub fn register_trusted_foundations(pd: *mut trusted_foundations_platform_data);
}
extern "C" {
    pub fn of_register_trusted_foundations();
}
extern "C" {
    pub fn trusted_foundations_registered() -> bool;
}

//
// If the system requires TF and we cannot provide it, continue booting
// but disable features that cannot be provided.
//

//
// If we find the target should enable TF but does not support it,
// fail as the system won't be able to do much anyway
//

