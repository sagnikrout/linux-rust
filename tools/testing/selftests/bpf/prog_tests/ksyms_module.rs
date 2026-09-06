//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/ksyms_module.c
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
// Copyright (c) 2021 Facebook

#[no_mangle]
unsafe extern "C" fn test_ksyms_module_lskel() {
    static void test_ksyms_module_lskel(void)
    {
    struct test_ksyms_module_lskel *skel;
    int err;
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    if (!env.has_testmod) {
    test__skip();
    return;
    }
    skel = test_ksyms_module_lskel__open_and_load();
    if (!ASSERT_OK_PTR(skel, "test_ksyms_module_lskel__open_and_load"))
    return;
    err = bpf_prog_test_run_opts(skel.progs.load.prog_fd, &topts);
    if (!ASSERT_OK(err, "bpf_prog_test_run"))
    goto cleanup;
    ASSERT_EQ(topts.retval, 0, "retval");
    ASSERT_EQ(skel.bss.out_bpf_testmod_ksym, 42, "bpf_testmod_ksym");
    cleanup:
    test_ksyms_module_lskel__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_ksyms_module_libbpf() {
    static void test_ksyms_module_libbpf(void)
    {
    struct test_ksyms_module *skel;
    int err;
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    if (!env.has_testmod) {
    test__skip();
    return;
    }
    skel = test_ksyms_module__open_and_load();
    if (!ASSERT_OK_PTR(skel, "test_ksyms_module__open"))
    return;
    err = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.load), &topts);
    if (!ASSERT_OK(err, "bpf_prog_test_run"))
    goto cleanup;
    ASSERT_EQ(topts.retval, 0, "retval");
    ASSERT_EQ(skel.bss.out_bpf_testmod_ksym, 42, "bpf_testmod_ksym");
    cleanup:
    test_ksyms_module__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_ksyms_module() {
    void test_ksyms_module(void)
    {
    if (test__start_subtest("lskel"))
    test_ksyms_module_lskel();
    if (test__start_subtest("libbpf"))
    test_ksyms_module_libbpf();
    }
