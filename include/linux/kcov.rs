//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kcov.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kcov_mode {
// Coverage collection is not enabled yet.
    KCOV_MODE_DISABLED = 0,
// KCOV was initialized, but tracing mode hasn't been chosen yet.
    KCOV_MODE_INIT = 1,
//
// Tracing coverage collection mode.
// Covered PCs are collected in a per-task buffer.
//
    KCOV_MODE_TRACE_PC = 2,
// Collecting comparison operands mode.
    KCOV_MODE_TRACE_CMP = 3,
}

extern "C" {
    pub fn kcov_task_init(t: *mut task_struct);
}
extern "C" {
    pub fn kcov_task_exit(t: *mut task_struct);
}

// See Documentation/dev-tools/kcov.rst for usage details.
extern "C" {
    pub fn kcov_remote_start(handle: u64);
}
extern "C" {
    pub fn kcov_remote_stop();
}
extern "C" {
    pub fn kcov_common_handle() -> kcov_common_handle_id;
}
//
// The softirq flavor of kcov_remote_*() functions is introduced as a temporary
// work around for kcov's lack of nested remote coverage sections support in
// task context. Adding support for nested sections is tracked in:
// https://bugzilla.kernel.org/show_bug.cgi?id=210337
//

pub type kcov_u64 = c_ulong;

pub type kcov_u64 = c_ulonglong;

extern "C" {
    pub fn __sanitizer_cov_trace_pc();
}
extern "C" {
    pub fn __sanitizer_cov_trace_cmp1(arg1: u8, arg2: u8);
}
extern "C" {
    pub fn __sanitizer_cov_trace_cmp2(arg1: u16, arg2: u16);
}
extern "C" {
    pub fn __sanitizer_cov_trace_cmp4(arg1: u32, arg2: u32);
}
extern "C" {
    pub fn __sanitizer_cov_trace_cmp8(arg1: kcov_u64, arg2: kcov_u64);
}
extern "C" {
    pub fn __sanitizer_cov_trace_const_cmp1(arg1: u8, arg2: u8);
}
extern "C" {
    pub fn __sanitizer_cov_trace_const_cmp2(arg1: u16, arg2: u16);
}
extern "C" {
    pub fn __sanitizer_cov_trace_const_cmp4(arg1: u32, arg2: u32);
}
extern "C" {
    pub fn __sanitizer_cov_trace_const_cmp8(arg1: kcov_u64, arg2: kcov_u64);
}
extern "C" {
    pub fn __sanitizer_cov_trace_switch(val: kcov_u64, cases: *mut c_void);
}

