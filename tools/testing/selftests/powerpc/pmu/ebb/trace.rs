//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/powerpc/pmu/ebb/trace.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2014, Michael Ellerman, IBM Corp.
//

pub const TRACE_TYPE_REG: c_int = 1;
pub const TRACE_TYPE_COUNTER: c_int = 2;
pub const TRACE_TYPE_STRING: c_int = 3;
pub const TRACE_TYPE_INDENT: c_int = 4;
pub const TRACE_TYPE_OUTDENT: c_int = 5;
extern "C" {
    pub fn trace_log_reg(tb: *mut trace_buffer, reg: u64, value: u64) -> c_int;
}
extern "C" {
    pub fn trace_log_counter(tb: *mut trace_buffer, value: u64) -> c_int;
}
extern "C" {
    pub fn trace_log_string(tb: *mut trace_buffer, str: *mut c_char) -> c_int;
}
extern "C" {
    pub fn trace_log_indent(tb: *mut trace_buffer) -> c_int;
}
extern "C" {
    pub fn trace_log_outdent(tb: *mut trace_buffer) -> c_int;
}
extern "C" {
    pub fn trace_buffer_print(tb: *mut trace_buffer);
}
extern "C" {
    pub fn trace_print_location(tb: *mut trace_buffer);
}
