//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_lrc.h
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

// At the start of the context image is its per-process HWS page

// After the PPHWSP we have the logical state for the context

// Space within PPHWSP reserved to be used as scratch
pub const LRC_PPHWSP_SCRATCH: c_uint = 0x34;

extern "C" {
    pub fn lrc_init_wa_ctx(engine: *mut intel_engine_cs);
}
extern "C" {
    pub fn lrc_fini_wa_ctx(engine: *mut intel_engine_cs);
}
extern "C" {
    pub fn lrc_reset(ce: *mut intel_context);
}
extern "C" {
    pub fn lrc_fini(ce: *mut intel_context);
}
extern "C" {
    pub fn lrc_destroy(kref: *mut kref);
}
extern "C" {
    pub fn lrc_unpin(ce: *mut intel_context);
}
extern "C" {
    pub fn lrc_post_unpin(ce: *mut intel_context);
}
extern "C" {
    pub fn lrc_update_runtime(ce: *mut intel_context);
}

pub const GEN8_CTX_ADDRESSING_MODE_SHIFT: c_int = 3;

pub const GEN8_CTX_ID_SHIFT: c_int = 32;
pub const GEN8_CTX_ID_WIDTH: c_int = 21;
pub const GEN11_SW_CTX_ID_SHIFT: c_int = 37;
pub const GEN11_SW_CTX_ID_WIDTH: c_int = 11;
pub const GEN11_ENGINE_CLASS_SHIFT: c_int = 61;
pub const GEN11_ENGINE_CLASS_WIDTH: c_int = 3;
pub const GEN11_ENGINE_INSTANCE_SHIFT: c_int = 48;
pub const GEN11_ENGINE_INSTANCE_WIDTH: c_int = 6;
pub const XEHP_SW_CTX_ID_SHIFT: c_int = 39;
pub const XEHP_SW_CTX_ID_WIDTH: c_int = 16;
pub const XEHP_SW_COUNTER_SHIFT: c_int = 58;
pub const XEHP_SW_COUNTER_WIDTH: c_int = 6;
pub const GEN12_GUC_SW_CTX_ID_SHIFT: c_int = 39;
pub const GEN12_GUC_SW_CTX_ID_WIDTH: c_int = 16;

extern "C" {
    pub fn lrc_indirect_bb(ce: *const intel_context) -> u32;
}
