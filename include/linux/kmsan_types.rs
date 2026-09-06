//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kmsan_types.h
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
//
// A minimal header declaring types added by KMSAN to existing kernel structs.
//
// Copyright (C) 2017-2022 Google LLC
// Author: Alexander Potapenko <glider@google.com>
//

// These constants are defined in the MSan LLVM instrumentation pass.
pub const KMSAN_RETVAL_SIZE: c_int = 800;
pub const KMSAN_PARAM_SIZE: c_int = 800;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmsan_context_state {
    pub param_tls: [c_char; KMSAN_PARAM_SIZE],
    pub retval_tls: [c_char; KMSAN_RETVAL_SIZE],
    pub va_arg_tls: [c_char; KMSAN_PARAM_SIZE],
    pub va_arg_origin_tls: [c_char; KMSAN_PARAM_SIZE],
    pub va_arg_overflow_size_tls: u64,
    pub param_origin_tls: [c_char; KMSAN_PARAM_SIZE],
    pub retval_origin_tls: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmsan_ctx {
    pub cstate: kmsan_context_state,
    pub kmsan_in_runtime: c_int,
    pub depth: c_uint,
}
