//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_trace.h
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


// SPDX-License-Identifier: GPL-2.0

// object tracking

// NB: the blocking information is racy since mutex_is_locked
// doesn't check that the current thread holds the lock. The only
// other option would be to pass the boolean information of whether
// or not the class was blocking down through the stack which is
// less desirable.
//
// DOC: i915_ppgtt_create and i915_ppgtt_release tracepoints
//
// With full ppgtt enabled each process using drm will allocate at least one
// translation table. With these traces it is possible to keep track of the
// allocation and of the lifetime of the tables; this can be used during
// testing/debug to verify that we are not leaking ppgtts.
// These traces identify the ppgtt through the vm pointer, which is also printed
// by the i915_vma_bind and i915_vma_unbind tracepoints.
//
// DOC: i915_context_create and i915_context_free tracepoints
//
// These tracepoints are used to track creation and deletion of contexts.
// If full ppgtt is enabled, they also print the address of the vm assigned to
// the context.
//

// This part must be outside protection

