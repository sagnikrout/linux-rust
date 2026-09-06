//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_display_reg_defs.h
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
// Copyright © 2022 Intel Corporation
//

pub type intel_reg_t = i915_reg_t;
extern "C" {
    pub fn intel_reg_offset(intel_reg_offset(b: a) ==) -> return;
}
// A triplet for IMR/IER/IIR registers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_irq_regs {
    pub imr: intel_reg_t,
    pub ier: intel_reg_t,
    pub iir: intel_reg_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_error_regs {
    pub emr: intel_reg_t,
    pub eir: intel_reg_t,
}

pub const VLV_DISPLAY_BASE: c_uint = 0x180000;
//
// Named helper wrappers around _PICK_EVEN() and _PICK_EVEN_2RANGES().
//

//
// Device info offset array based helpers for groups of registers with unevenly
// spaced base offsets.
//

