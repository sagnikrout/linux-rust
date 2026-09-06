//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_strncmp.c
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
// Copyright (C) 2021. Huawei Technologies Co., Ltd

#[no_mangle]
unsafe extern "C" fn trigger_strncmp(skel: *const strncmp_test) -> c_int {
    static int trigger_strncmp(const struct strncmp_test *skel)
    {
    int cmp;
    usleep(1);
    cmp = skel.bss.cmp_ret;
    if (cmp > 0)
    return 1;
    if (cmp < 0)
    return -1;
    return 0;
    }
//
// Compare str and target after making str[i] != target[i].
// When exp is -1, make str[i] < target[i] and delta = -1.
//
    static void strncmp_full_str_cmp(struct strncmp_test *skel, const char *name,
    int exp)
    {
    let mut nr: usize = sizeof(skel.bss.str);
    char *str = skel.bss.str;
    let mut delta: c_int = exp;
    int got;
    size_t i;
    memcpy(str, skel.rodata.target, nr);
    for (i = 0; i < nr - 1; i++) {
    str[i] += delta;
    got = trigger_strncmp(skel);
    ASSERT_EQ(got, exp, name);
    str[i] -= delta;
    }
    }
#[no_mangle]
unsafe extern "C" fn test_strncmp_ret() {
    static void test_strncmp_ret(void)
    {
    struct strncmp_test *skel;
    int err, got;
    skel = strncmp_test__open();
    if (!ASSERT_OK_PTR(skel, "strncmp_test open"))
    return;
    bpf_program__set_autoload(skel.progs.do_strncmp, true);
    err = strncmp_test__load(skel);
    if (!ASSERT_EQ(err, 0, "strncmp_test load"))
    goto out;
    err = strncmp_test__attach(skel);
    if (!ASSERT_EQ(err, 0, "strncmp_test attach"))
    goto out;
    skel.bss.target_pid = getpid();
// Empty str
    skel.bss.str[0] = '\0';
    got = trigger_strncmp(skel);
    ASSERT_EQ(got, -1, "strncmp: empty str");
// Same string
    memcpy(skel.bss.str, skel.rodata.target, sizeof(skel.bss.str));
    got = trigger_strncmp(skel);
    ASSERT_EQ(got, 0, "strncmp: same str");
// Not-null-terminated string
    memcpy(skel.bss.str, skel.rodata.target, sizeof(skel.bss.str));
    skel.bss.str[sizeof(skel.bss.str) - 1] = 'A';
    got = trigger_strncmp(skel);
    ASSERT_EQ(got, 1, "strncmp: not-null-term str");
    strncmp_full_str_cmp(skel, "strncmp: less than", -1);
    strncmp_full_str_cmp(skel, "strncmp: greater than", 1);
    out:
    strncmp_test__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_strncmp_bad_not_const_str_size() {
    static void test_strncmp_bad_not_const_str_size(void)
    {
    struct strncmp_test *skel;
    int err;
    skel = strncmp_test__open();
    if (!ASSERT_OK_PTR(skel, "strncmp_test open"))
    return;
    bpf_program__set_autoload(skel.progs.strncmp_bad_not_const_str_size, true);
    err = strncmp_test__load(skel);
    ASSERT_ERR(err, "strncmp_test load bad_not_const_str_size");
    strncmp_test__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_strncmp_bad_writable_target() {
    static void test_strncmp_bad_writable_target(void)
    {
    struct strncmp_test *skel;
    int err;
    skel = strncmp_test__open();
    if (!ASSERT_OK_PTR(skel, "strncmp_test open"))
    return;
    bpf_program__set_autoload(skel.progs.strncmp_bad_writable_target, true);
    err = strncmp_test__load(skel);
    ASSERT_ERR(err, "strncmp_test load bad_writable_target");
    strncmp_test__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_strncmp_bad_not_null_term_target() {
    static void test_strncmp_bad_not_null_term_target(void)
    {
    struct strncmp_test *skel;
    int err;
    skel = strncmp_test__open();
    if (!ASSERT_OK_PTR(skel, "strncmp_test open"))
    return;
    bpf_program__set_autoload(skel.progs.strncmp_bad_not_null_term_target, true);
    err = strncmp_test__load(skel);
    ASSERT_ERR(err, "strncmp_test load bad_not_null_term_target");
    strncmp_test__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_test_strncmp() {
    void test_test_strncmp(void)
    {
    if (test__start_subtest("strncmp_ret"))
    test_strncmp_ret();
    if (test__start_subtest("strncmp_bad_not_const_str_size"))
    test_strncmp_bad_not_const_str_size();
    if (test__start_subtest("strncmp_bad_writable_target"))
    test_strncmp_bad_writable_target();
    if (test__start_subtest("strncmp_bad_not_null_term_target"))
    test_strncmp_bad_not_null_term_target();
    }
