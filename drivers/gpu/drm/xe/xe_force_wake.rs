//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_force_wake.h
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

extern "C" {
    pub fn xe_force_wake_put(fw: *mut xe_force_wake, fw_ref: c_uint);
}

//
// xe_force_wake_assert_held - asserts domain is awake
// @fw : xe_force_wake structure
// @domain: xe_force_wake_domains apart from XE_FORCEWAKE_ALL
//
// xe_force_wake_assert_held() is designed to confirm a particular
// forcewake domain's wakefulness; it doesn't verify the wakefulness of
// multiple domains. Make sure the caller doesn't input multiple
// domains(XE_FORCEWAKE_ALL) as a parameter.
//
// xe_force_wake_ref_has_domain - verifies if the domains are in fw_ref
// @fw_ref : the force_wake reference
// @domain : forcewake domain to verify
//
// This function confirms whether the @fw_ref includes a reference to the
// specified @domain.
//
// Return: true if domain is refcounted.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_force_wake_ref {
    pub fw: *mut xe_force_wake,
    pub domains: c_uint,
}

//
// Scoped helper for the forcewake class, using the same trick as scoped_guard()
// to bind the lifetime to the next statement/block.
//

//
// Used when xe_force_wake_constructor() has already been called by another
// function and the current function is responsible for releasing the forcewake
// reference in all possible cases and error paths.
//
