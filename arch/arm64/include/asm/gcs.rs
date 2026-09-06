//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/gcs.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2023 ARM Ltd.
//

extern "C" {
    pub fn volatile("memory": ".inst 0xd503227f" : : :) -> asm;
}
// GCSSTTR x1, [x0]

extern "C" {
    pub fn gcs_set_el0_mode(task: *mut task_struct);
}
extern "C" {
    pub fn gcs_free(task: *mut task_struct);
}
extern "C" {
    pub fn gcs_preserve_current_state();
}
// GCSSTTR x1, [x0]
// err = -EFAULT;
// err = ret;
//
// Unlike put/push_user_gcs() above, get/pop_user_gsc() doesn't
// validate the GCS permission is set on the page being read.  This
// differs from how the hardware works when it consumes data stored at
// GCSPR. Callers should ensure this is acceptable.
//
// Ensure previous GCS operation are visible before we read the page
// err = ret;

// err = -EFAULT;

