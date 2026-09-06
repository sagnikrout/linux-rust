//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/gen8_engine_cs.h
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
// Copyright © 2014 Intel Corporation
//

extern "C" {
    pub fn gen8_emit_flush_rcs(rq: *mut i915_request, mode: u32) -> c_int;
}
extern "C" {
    pub fn gen11_emit_flush_rcs(rq: *mut i915_request, mode: u32) -> c_int;
}
extern "C" {
    pub fn gen12_emit_flush_rcs(rq: *mut i915_request, mode: u32) -> c_int;
}
extern "C" {
    pub fn gen8_emit_flush_xcs(rq: *mut i915_request, mode: u32) -> c_int;
}
extern "C" {
    pub fn gen12_emit_flush_xcs(rq: *mut i915_request, mode: u32) -> c_int;
}
extern "C" {
    pub fn gen8_emit_init_breadcrumb(rq: *mut i915_request) -> c_int;
}
extern "C" {
    pub fn __gen8_emit_pipe_control(_arg: batch, _arg: 0, _arg: bit_group_1, _arg: offset) -> return;
}
// cs++ = GFX_OP_PIPE_CONTROL(6) | flags0;
// cs++ = flags1 | PIPE_CONTROL_QW_WRITE;
// cs++ = offset;
// cs++ = 0;
// cs++ = value;
// cs++ = 0; /* We're thrashing one extra dword.
// We're using qword write, offset should be aligned to 8 bytes.
// cs++ = (MI_FLUSH_DW + 1) | flags;
// cs++ = gtt_offset;
// cs++ = 0;
// cs++ = value;
// w/a: bit 5 needs to be zero for MI_FLUSH_DW address.
// Offset should be aligned to 8 bytes for both (QW/DW) write types
