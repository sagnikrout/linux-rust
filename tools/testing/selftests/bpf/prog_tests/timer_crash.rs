//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/timer_crash.c
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

    enum {
    MODE_ARRAY,
    MODE_HASH,
    };
#[no_mangle]
unsafe extern "C" fn test_timer_crash_mode(mode: c_int) {
    static void test_timer_crash_mode(int mode)
    {
    struct timer_crash *skel;
    skel = timer_crash__open_and_load();
    if (!skel && errno == EOPNOTSUPP) {
    test__skip();
    return;
    }
    if (!ASSERT_OK_PTR(skel, "timer_crash__open_and_load"))
    return;
    skel.bss.pid = getpid();
    skel.bss.crash_map = mode;
    if (!ASSERT_OK(timer_crash__attach(skel), "timer_crash__attach"))
    goto end;
    usleep(1);
    end:
    timer_crash__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_timer_crash() {
    void test_timer_crash(void)
    {
    if (test__start_subtest("array"))
    test_timer_crash_mode(MODE_ARRAY);
    if (test__start_subtest("hash"))
    test_timer_crash_mode(MODE_HASH);
    }
