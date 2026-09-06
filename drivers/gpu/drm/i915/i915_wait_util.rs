//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_wait_util.h
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
// Copyright © 2025 Intel Corporation

//
// __wait_for - magic wait macro
//
// Macro to help avoid open coding check/wait/timeout patterns. Note that it's
// important that we check the condition again after having timed out, since the
// timeout could be due to preemption or similar and we've never had a chance to
// check the condition before the timeout.
//

// Guarantee COND check prior to timeout */		\

//
// If CONFIG_PREEMPT_COUNT is disabled, in_atomic() always reports false.
// On PREEMPT_RT the context isn't becoming atomic because it is used in an
// interrupt handler or because a spinlock_t is acquired. This leads to
// warnings which don't occur otherwise and therefore the check is disabled.
//

// Guarantee COND check prior to timeout */ \

