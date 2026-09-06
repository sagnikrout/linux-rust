//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/sof-of-dev.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright 2021 NXP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_of_mach {
    pub compatible: *const c_char,
    pub drv_name: *const c_char,
    pub fw_filename: *const c_char,
    pub sof_tplg_filename: *const c_char,
}

extern "C" {
    pub fn sof_of_probe(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn sof_of_remove(pdev: *mut platform_device);
}
extern "C" {
    pub fn sof_of_shutdown(pdev: *mut platform_device);
}
