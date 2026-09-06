//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/kernel_flag.c
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
// Copyright (c) 2025 Microsoft

#[no_mangle]
pub unsafe extern "C" fn test_kernel_flag() {
    void test_kernel_flag(void)
    {
    struct test_kernel_flag *lsm_skel;
    struct kfunc_call_test *skel = core::ptr::null_mut();
    struct kfunc_call_test_lskel *lskel = core::ptr::null_mut();
    int ret;
    lsm_skel = test_kernel_flag__open_and_load();
    if (!ASSERT_OK_PTR(lsm_skel, "lsm_skel"))
    return;
    lsm_skel.bss.monitored_tid = sys_gettid();
    ret = test_kernel_flag__attach(lsm_skel);
    if (!ASSERT_OK(ret, "test_kernel_flag__attach"))
    goto close_prog;
// Test with skel. This should pass the gatekeeper
    skel = kfunc_call_test__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel"))
    goto close_prog;
// Test with lskel. This should fail due to blocking kernel-based bpf() invocations
    lskel = kfunc_call_test_lskel__open_and_load();
    if (!ASSERT_ERR_PTR(lskel, "lskel"))
    goto close_prog;
    close_prog:
    if (skel)
    kfunc_call_test__destroy(skel);
    if (lskel)
    kfunc_call_test_lskel__destroy(lskel);
    lsm_skel.bss.monitored_tid = 0;
    test_kernel_flag__destroy(lsm_skel);
    }
