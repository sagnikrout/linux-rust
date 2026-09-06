//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_cmd_parser.h
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
// Copyright © 2021 Intel Corporation
//

extern "C" {
    pub fn i915_cmd_parser_get_version(dev_priv: *mut drm_i915_private) -> c_int;
}
extern "C" {
    pub fn intel_engine_init_cmd_parser(engine: *mut intel_engine_cs) -> c_int;
}
extern "C" {
    pub fn intel_engine_cleanup_cmd_parser(engine: *mut intel_engine_cs);
}
pub const I915_CMD_PARSER_TRAMPOLINE_SIZE: c_int = 8;
