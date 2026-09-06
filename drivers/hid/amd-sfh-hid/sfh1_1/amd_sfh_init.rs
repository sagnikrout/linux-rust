//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/amd-sfh-hid/sfh1_1/amd_sfh_init.h
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
// AMD MP2 1.1 initialization structures
//
// Copyright (c) 2022, Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// Author: Basavaraj Natikar <Basavaraj.Natikar@amd.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_sfh1_1_ops {
    pub mp2): *mut *mut int (init)(struct amd_mp2_dev,
    pub enable): *mut *mut *mut void (toggle_hpd)(struct amd_mp2_dev mp2, bool,
}

extern "C" {
    pub fn amd_sfh1_1_init(mp2: *mut amd_mp2_dev) -> c_int;
}
extern "C" {
    pub fn amd_sfh_toggle_hpd(mp2: *mut amd_mp2_dev, enabled: bool);
}
