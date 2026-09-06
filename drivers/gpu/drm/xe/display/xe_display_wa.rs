//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/xe/display/xe_display_wa.c
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
// Copyright © 2024 Intel Corporation
//

#[no_mangle]
pub unsafe extern "C" fn intel_display_needs_wa_16023588340(display: *mut intel_display) -> bool {
    bool intel_display_needs_wa_16023588340(struct intel_display *display)
    {
    struct xe_device *xe = to_xe_device(display.drm);
    struct xe_gt *wa_gt = xe_root_mmio_gt(xe);
    return wa_gt && XE_GT_WA(wa_gt, 16023588340);
    }
