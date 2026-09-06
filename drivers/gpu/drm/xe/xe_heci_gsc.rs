//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_heci_gsc.h
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
// Copyright(c) 2023, Intel Corporation. All rights reserved.
//

//
// GSC HECI1 bit corresponds to bit15 and HECI2 to bit14.
// The reason for this is to allow growth for more interfaces in the future.
//

//
// CSC HECI1 bit corresponds to bit9 and HECI2 to bit10.
//

//
// struct xe_heci_gsc - graphics security controller for xe, HECI interface
//
// @adev : pointer to mei auxiliary device structure
// @irq : irq number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_heci_gsc {
    pub adev: *mut mei_aux_device,
    pub irq: c_int,
}

extern "C" {
    pub fn xe_heci_gsc_init(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_heci_gsc_irq_handler(xe: *mut xe_device, iir: u32);
}
extern "C" {
    pub fn xe_heci_csc_irq_handler(xe: *mut xe_device, iir: u32);
}
