//! Automatically rewritten from C Header to Rust Module
//! Source: scripts/gcc-plugins/gcc-generate-gimple-pass.h
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
// Generator for GIMPLE pass related boilerplate code/data
//
// Supports gcc 4.5-6
//
// Usage:
//
// 1. before inclusion define PASS_NAME
// 2. before inclusion define NO_* for unimplemented callbacks
// NO_GATE
// NO_EXECUTE
// 3. before inclusion define PROPERTIES_* and TODO_FLAGS_* to override
// the default 0 values
// 4. for convenience, all the above will be undefined after inclusion!
// 5. the only exported name is make_PASS_NAME_pass() to register with gcc
//

pub const PROPERTIES_REQUIRED: c_int = 0;

pub const PROPERTIES_PROVIDED: c_int = 0;

pub const PROPERTIES_DESTROYED: c_int = 0;

pub const TODO_FLAGS_START: c_int = 0;

pub const TODO_FLAGS_FINISH: c_int = 0;

extern "C" {
    pub fn _PASS_NAME_PASS() -> return new;
}

// clean up user provided defines

// clean up generated defines

