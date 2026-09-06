//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/tests/tests.h
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

extern "C" {
    pub fn int(: *mut *mut test_fnptr)(struct test_suite, _arg: c_int) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_case {
    pub name: *const c_char,
    pub desc: *const c_char,
    pub skip_reason: *const c_char,
    pub run_case: test_fnptr,
    pub exclusive: bool,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_suite {
    pub desc: *const c_char,
    pub test_cases: *mut test_case,
    pub priv: *mut c_void,
    pub suite): *mut *mut int (setup)(struct test_suite,
}

// Tests
//
// PowerPC and S390 do not support creation of instruction breakpoints using the
// perf_event interface.
//
// ARM requires explicit rounding down of the instruction pointer in Thumb mode,
// and then requires the single-step to be handled explicitly in the overflow
// handler to avoid stepping into the SIGIO handler and getting stuck on the
// breakpointed instruction.
//
// Since arm64 has the same issue with arm for the single-step handling, this
// case also gets stuck on the breakpointed instruction.
//
// Just disable the test for these architectures until these issues are
// resolved.
//

pub const BP_SIGNAL_IS_SUPPORTED: c_int = 0;

pub const BP_SIGNAL_IS_SUPPORTED: c_int = 1;

//
// Define test workloads to be used in test suites.
//
extern "C" {
    pub fn int(argc: *mut *mut workload_fnptr)(int, argv: *const c_char) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_workload {
    pub name: *const c_char,
    pub func: workload_fnptr,
}

// The list of test workloads

