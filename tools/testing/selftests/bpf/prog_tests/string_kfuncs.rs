//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/string_kfuncs.c
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
// Copyright (C) 2025 Red Hat, Inc.

    static const char * const test_cases[] = {
    "strcmp",
    "strcasecmp",
    "strncasecmp",
    "strchr",
    "strchrnul",
    "strnchr",
    "strrchr",
    "strlen",
    "strnlen",
    "strspn_str",
    "strspn_accept",
    "strcspn_str",
    "strcspn_reject",
    "strstr",
    "strcasestr",
    "strnstr",
    "strncasestr",
    };
#[no_mangle]
pub unsafe extern "C" fn run_too_long_tests() {
    void run_too_long_tests(void)
    {
    struct string_kfuncs_failure2 *skel;
    struct bpf_program *prog;
    char test_name[256];
    int err, i;
    skel = string_kfuncs_failure2__open_and_load();
    if (!ASSERT_OK_PTR(skel, "string_kfuncs_failure2__open_and_load"))
    return;
    memset(skel.bss.long_str, 'a', sizeof(skel.bss.long_str));
    for (i = 0; i < ARRAY_SIZE(test_cases); i++) {
    sprintf(test_name, "test_%s_too_long", test_cases[i]);
    if (!test__start_subtest(test_name))
    continue;
    prog = bpf_object__find_program_by_name(skel.obj, test_name);
    if (!ASSERT_OK_PTR(prog, "bpf_object__find_program_by_name"))
    goto cleanup;
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    err = bpf_prog_test_run_opts(bpf_program__fd(prog), &topts);
    if (!ASSERT_OK(err, "bpf_prog_test_run"))
    goto cleanup;
    ASSERT_EQ(topts.retval, -E2BIG, "reading too long string fails with -E2BIG");
    }
    cleanup:
    string_kfuncs_failure2__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_string_kfuncs() {
    void test_string_kfuncs(void)
    {
    RUN_TESTS(string_kfuncs_success);
    RUN_TESTS(string_kfuncs_failure1);
    run_too_long_tests();
    }
