//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/static_linked.c
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
// Copyright (c) 2019 Facebook

#[no_mangle]
pub unsafe extern "C" fn test_static_linked() {
    void test_static_linked(void)
    {
    int err;
    struct test_static_linked* skel;
    skel = test_static_linked__open();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    skel.rodata.rovar1 = 1;
    skel.rodata.rovar2 = 4;
    err = test_static_linked__load(skel);
    if (!ASSERT_OK(err, "skel_load"))
    goto cleanup;
    err = test_static_linked__attach(skel);
    if (!ASSERT_OK(err, "skel_attach"))
    goto cleanup;
// trigger
    usleep(1);
    ASSERT_EQ(skel.data.var1, 1 * 2 + 2 + 3, "var1");
    ASSERT_EQ(skel.data.var2, 4 * 3 + 5 + 6, "var2");
    cleanup:
    test_static_linked__destroy(skel);
    }
