//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_ring.h
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

extern "C" {
    pub fn intel_ring_update_space(ring: *mut intel_ring) -> c_uint;
}
extern "C" {
    pub fn __intel_ring_pin(ring: *mut intel_ring);
}
extern "C" {
    pub fn intel_ring_pin(ring: *mut intel_ring, ww: *mut i915_gem_ww_ctx) -> c_int;
}
extern "C" {
    pub fn intel_ring_unpin(ring: *mut intel_ring);
}
extern "C" {
    pub fn intel_ring_reset(ring: *mut intel_ring, tail: u32);
}
extern "C" {
    pub fn intel_ring_free(ref: *mut kref);
}
// Dummy function.
//
// This serves as a placeholder in the code so that the reader
// can compare against the preceding intel_ring_begin() and
// check that the number of dwords emitted matches the space
// reserved for the command packet (i.e. the value passed to
// intel_ring_begin()).
//
// Don't write ring->size (equivalent to 0) as that hangs some GPUs.
extern "C" {
    pub fn intel_ring_wrap(_arg: rq->ring, _arg: offset) -> return;
}
//
// "Ring Buffer Use"
// Gen2 BSpec "1. Programming Environment" / 1.4.4.6
// Gen3 BSpec "1c Memory Interface Functions" / 2.3.4.5
// Gen4+ BSpec "1c Memory Interface and Command Stream" / 5.3.4.5
// "If the Ring Buffer Head Pointer and the Tail Pointer are on the
// same cacheline, the Head Pointer must not be greater than the Tail
// Pointer."
//
// We use ring->head as the last known location of the actual RING_HEAD,
// it may have advanced but in the worst case it is equally the same
// as ring->head and so we should never program RING_TAIL to advance
// into the same cacheline as ring->head.
//

// Whilst writes to the tail are strictly order, there is no
// serialisation between readers and the writers. The tail may be
// read by i915_request_retire() just as it is being updated
// by execlists, as although the breadcrumb is complete, the context
// switch hasn't been seen.
//
// "If the Ring Buffer Head Pointer and the Tail Pointer are on the
// same cacheline, the Head Pointer must not be greater than the Tail
// Pointer."
//
