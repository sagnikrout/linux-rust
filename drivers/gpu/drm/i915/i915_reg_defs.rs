//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_reg_defs.h
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

//
// These macros can be used on either i915_reg_t or i915_mcr_reg_t since they're
// simply operations on the register's offset and don't care about the MCR vs
// non-MCR nature of the register.
//

// A triplet for IMR/IER/IIR registers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_irq_regs {
    pub imr: i915_reg_t,
    pub ier: i915_reg_t,
    pub iir: i915_reg_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_error_regs {
    pub emr: i915_reg_t,
    pub eir: i915_reg_t,
}

