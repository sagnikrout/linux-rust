//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_priolist_types.h
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


//
// SPDX-License-Identifier: MIT
//
// Copyright © 2018 Intel Corporation
//

// A preemptive pulse used to monitor the health of each engine
// Interactive workload, scheduled for immediate pageflipping
// Smallest priority value that cannot be bumped.

//
// Requests containing performance queries must not be preempted by
// another context. They get scheduled with their default priority and
// once they reach the execlist ports we ensure that they stick on the
// HW until finished by pretending that they have maximum priority,
// i.e. nothing can have higher priority and force us to usurp the
// active request.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_priolist {
    pub requests: list_head,
    pub node: rb_node,
    pub priority: c_int,
}
