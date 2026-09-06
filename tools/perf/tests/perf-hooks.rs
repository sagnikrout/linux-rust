//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/perf-hooks.c
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

#[no_mangle]
unsafe extern "C" fn sigsegv_handler(__maybe_unused: int sig) {
    static void sigsegv_handler(int sig __maybe_unused)
    {
    pr_debug("SIGSEGV is observed as expected, try to recover.\n");
    perf_hooks__recover();
    signal(SIGSEGV, SIG_DFL);
    raise(SIGSEGV);
    exit(-1);
    }
#[no_mangle]
unsafe extern "C" fn the_hook(_hook_flags: *mut c_void) {
    static void the_hook(void *_hook_flags)
    {
    int *hook_flags = _hook_flags;
// hook_flags = 1234;
// Generate a segfault, test perf_hooks__recover
    raise(SIGSEGV);
    }
#[no_mangle]
unsafe extern "C" fn test__perf_hooks(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__perf_hooks(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    let mut hook_flags: c_int = 0;
    signal(SIGSEGV, sigsegv_handler);
    perf_hooks__set_hook("test", the_hook, &hook_flags);
    perf_hooks__invoke_test();
// hook is triggered?
    if (hook_flags != 1234) {
    pr_debug("Setting failed: %d (%p)\n", hook_flags, &hook_flags);
    return TEST_FAIL;
    }
// the buggy hook is removed?
    if (perf_hooks__get_hook("test"))
    return TEST_FAIL;
    return TEST_OK;
    }
    DEFINE_SUITE("perf hooks", perf_hooks);
