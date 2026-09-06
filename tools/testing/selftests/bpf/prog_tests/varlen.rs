//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/varlen.c
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
// Copyright (c) 2020 Facebook

    CHECK((got) != (exp), "check", "got %ld != exp %ld\n", \
    (long)(got), (long)(exp))
#[no_mangle]
pub unsafe extern "C" fn test_varlen() {
    void test_varlen(void)
    {
    let mut duration: c_int = 0, err;
    struct test_varlen* skel;
    struct test_varlen__bss *bss;
    struct test_varlen__data *data;
    const char str1[] = "Hello, ";
    const char str2[] = "World!";
    const char exp_str[] = "Hello, \0World!\0";
    let mut size1: c_int = sizeof(str1);
    let mut size2: c_int = sizeof(str2);
    skel = test_varlen__open_and_load();
    if (CHECK(!skel, "skel_open", "failed to open skeleton\n"))
    return;
    bss = skel.bss;
    data = skel.data;
    err = test_varlen__attach(skel);
    if (CHECK(err, "skel_attach", "skeleton attach failed: %d\n", err))
    goto cleanup;
    bss.test_pid = getpid();
// trigger everything
    memcpy(bss.buf_in1, str1, size1);
    memcpy(bss.buf_in2, str2, size2);
    bss.capture = true;
    usleep(1);
    bss.capture = false;
    CHECK_VAL(bss.payload1_len1, size1);
    CHECK_VAL(bss.payload1_len2, size2);
    CHECK_VAL(bss.total1, size1 + size2);
    CHECK(memcmp(bss.payload1, exp_str, size1 + size2), "content_check",
    "doesn't match!\n");
    CHECK_VAL(data.payload2_len1, size1);
    CHECK_VAL(data.payload2_len2, size2);
    CHECK_VAL(data.total2, size1 + size2);
    CHECK(memcmp(data.payload2, exp_str, size1 + size2), "content_check",
    "doesn't match!\n");
    CHECK_VAL(data.payload3_len1, size1);
    CHECK_VAL(data.payload3_len2, size2);
    CHECK_VAL(data.total3, size1 + size2);
    CHECK(memcmp(data.payload3, exp_str, size1 + size2), "content_check",
    "doesn't match!\n");
    CHECK_VAL(data.payload4_len1, size1);
    CHECK_VAL(data.payload4_len2, size2);
    CHECK_VAL(data.total4, size1 + size2);
    CHECK(memcmp(data.payload4, exp_str, size1 + size2), "content_check",
    "doesn't match!\n");
    CHECK_VAL(bss.ret_bad_read, -EFAULT);
    CHECK_VAL(data.payload_bad[0], 0x42);
    CHECK_VAL(data.payload_bad[1], 0x42);
    CHECK_VAL(data.payload_bad[2], 0);
    CHECK_VAL(data.payload_bad[3], 0x42);
    CHECK_VAL(data.payload_bad[4], 0x42);
    cleanup:
    test_varlen__destroy(skel);
    }
