//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_ring_types.h
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
// Copyright © 2019 Intel Corporation
//

//
// Early gen2 devices have a cacheline of just 32 bytes, using 64 is overkill,
// but keeps the logic simple. Indeed, the whole purpose of this macro is just
// to give some inclination as to some of the magic values used in the various
// workarounds!
//
pub const CACHELINE_BYTES: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_ring {
    pub ref: kref,
    pub vma: *mut i915_vma,
    pub vaddr: *mut c_void,
//
// As we have two types of rings, one global to the engine used
// by ringbuffer submission and those that are exclusive to a
// context used by execlists, we have to play safe and allow
// atomic updates to the pin_count. However, the actual pinning
// of the context is either done during initialisation for
// ringbuffer submission or serialised as part of the context
// pinning for execlists, and so we do not need a mutex ourselves
// to serialise intel_ring_pin/intel_ring_unpin.
//
    pub pin_count: core::sync::atomic::AtomicI32,
    pub /: *mut *mut u32 head; / updated during retire, loosely tracks RING_HEAD,
    pub /: *mut *mut u32 tail; / updated on submission, used for RING_TAIL,
    pub /: *mut *mut u32 emit; / updated during request construction,
    pub space: u32,
    pub size: u32,
    pub wrap: u32,
    pub effective_size: u32,
}
