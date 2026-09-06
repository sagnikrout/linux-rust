//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_gsc.h
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
// Copyright(c) 2019-2022, Intel Corporation. All rights reserved.
//

pub const INTEL_GSC_NUM_INTERFACES: c_int = 2;
//
// The HECI1 bit corresponds to bit15 and HECI2 to bit14.
// The reason for this is to allow growth for more interfaces in the future.
//

//
// struct intel_gsc - graphics security controller
//
// @intf: gsc interface
// @intf.adev: MEI aux. device for this @intf
// @intf.gem_obj: scratch memory GSC operations
// @intf.irq: IRQ for this device (%-1 for no IRQ)
// @intf.id: this interface's id number/index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gsc {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gsc_intf {
    pub adev: *mut mei_aux_device,
    pub gem_obj: *mut drm_i915_gem_object,
    pub irq: c_int,
    pub id: c_uint,
    pub intf: [}; INTEL_GSC_NUM_INTERFACES],
}

extern "C" {
    pub fn intel_gsc_init(gsc: *mut intel_gsc, i915: *mut drm_i915_private);
}
extern "C" {
    pub fn intel_gsc_fini(gsc: *mut intel_gsc);
}
extern "C" {
    pub fn intel_gsc_irq_handler(gt: *mut intel_gt, iir: u32);
}
