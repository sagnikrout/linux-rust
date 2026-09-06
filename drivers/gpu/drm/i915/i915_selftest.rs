//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_selftest.h
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


//
// Copyright © 2016 Intel Corporation
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

pub const STACK_MAGIC: c_uint = 0xdeadbeef;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_selftest {
    pub timeout_jiffies: c_ulong,
    pub timeout_ms: c_uint,
    pub random_seed: c_uint,
    pub userspace_pid: c_uint,
    pub filter: *mut c_char,
    pub mock: c_int,
    pub live: c_int,
    pub perf: c_int,
}

extern "C" {
    pub fn i915_mock_selftests() -> c_int;
}
extern "C" {
    pub fn i915_live_selftests(pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn i915_perf_selftests(pdev: *mut pci_dev) -> c_int;
}
// We extract the function declarations from i915_mock_selftests.h and
// i915_live_selftests.h Add your unit test declarations there!
//
// Mock unit tests are run very early upon module load, before the driver
// is probed. All hardware interactions, as well as other subsystems, must
// be "mocked".
//
// Live unit tests are run after the driver is loaded - all hardware
// interactions are real.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_subtest {
    pub data): *mut *mut int (func)(void,
    pub name: *const c_char,
}

extern "C" {
    pub fn __i915_nop_setup(data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn __i915_nop_teardown(err: c_int, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn __i915_live_setup(data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn __i915_live_teardown(err: c_int, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn __intel_gt_live_setup(data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn __intel_gt_live_teardown(err: c_int, data: *mut c_void) -> c_int;
}

// Macro flag: #define I915_SELFTEST_EXPORT

// Macro flag: #define I915_SELFTEST_DECLARE(x)
pub const I915_SELFTEST_ONLY(x): c_int = 0;

// Using the i915_selftest_ prefix becomes a little unwieldy with the helpers.
// Instead we use the igt_ shorthand, in reference to the intel-gpu-tools
// suite of uabi test cases (which includes a test runner for our selftests).
//

extern "C" {
    pub fn __igt_timeout(timeout: c_ulong, fmt: *const c_char, ...) -> bool;
}

extern "C" {
    pub fn igt_hexdump(buf: *const c_void, len: usize);
}
