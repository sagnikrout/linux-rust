//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/dummy_st_ops.c
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

// Need to keep consistent with definition in include/linux/bpf.h
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_dummy_ops_state {
    pub val: c_int,
}

#[no_mangle]
unsafe extern "C" fn test_dummy_st_ops_attach() {
    static void test_dummy_st_ops_attach(void)
    {
    struct dummy_st_ops_success *skel;
    struct bpf_link *link;
    skel = dummy_st_ops_success__open_and_load();
    if (!ASSERT_OK_PTR(skel, "dummy_st_ops_load"))
    return;
    link = bpf_map__attach_struct_ops(skel.maps.dummy_1);
    ASSERT_EQ(libbpf_get_error(link), -EOPNOTSUPP, "dummy_st_ops_attach");
    dummy_st_ops_success__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_dummy_init_ret_value() {
    static void test_dummy_init_ret_value(void)
    {
    __u64 args[1] = {0};
    LIBBPF_OPTS(bpf_test_run_opts, attr,
    .ctx_in = args,
    .ctx_size_in = sizeof(args),
    );
    struct dummy_st_ops_success *skel;
    int fd, err;
    skel = dummy_st_ops_success__open_and_load();
    if (!ASSERT_OK_PTR(skel, "dummy_st_ops_load"))
    return;
    fd = bpf_program__fd(skel.progs.test_1);
    err = bpf_prog_test_run_opts(fd, &attr);
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(attr.retval, 0xf2f3f4f5, "test_ret");
    dummy_st_ops_success__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_dummy_init_ptr_arg() {
    static void test_dummy_init_ptr_arg(void)
    {
    let mut exp_retval: c_int = 0xbeef;
    struct bpf_dummy_ops_state in_state = {
    .val = exp_retval,
    };
    __u64 args[1] = {(unsigned long)&in_state};
    LIBBPF_OPTS(bpf_test_run_opts, attr,
    .ctx_in = args,
    .ctx_size_in = sizeof(args),
    );
    struct trace_dummy_st_ops *trace_skel;
    struct dummy_st_ops_success *skel;
    int fd, err;
    skel = dummy_st_ops_success__open_and_load();
    if (!ASSERT_OK_PTR(skel, "dummy_st_ops_load"))
    return;
    fd = bpf_program__fd(skel.progs.test_1);
    trace_skel = trace_dummy_st_ops__open();
    if (!ASSERT_OK_PTR(trace_skel, "trace_dummy_st_ops__open"))
    goto done;
    err = bpf_program__set_attach_target(trace_skel.progs.fentry_test_1,
    fd, "test_1");
    if (!ASSERT_OK(err, "set_attach_target(fentry_test_1)"))
    goto done;
    err = trace_dummy_st_ops__load(trace_skel);
    if (!ASSERT_OK(err, "load(trace_skel)"))
    goto done;
    err = trace_dummy_st_ops__attach(trace_skel);
    if (!ASSERT_OK(err, "attach(trace_skel)"))
    goto done;
    err = bpf_prog_test_run_opts(fd, &attr);
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(in_state.val, 0x5a, "test_ptr_ret");
    ASSERT_EQ(attr.retval, exp_retval, "test_ret");
    ASSERT_EQ(trace_skel.bss.val, exp_retval, "fentry_val");
    done:
    dummy_st_ops_success__destroy(skel);
    trace_dummy_st_ops__destroy(trace_skel);
    }
#[no_mangle]
unsafe extern "C" fn test_dummy_multiple_args() {
    static void test_dummy_multiple_args(void)
    {
    let mut st: bpf_dummy_ops_state = { 7 };
    __u64 args[5] = {(__u64)&st, -100, 0x8a5f, 'c', 0x1234567887654321ULL};
    LIBBPF_OPTS(bpf_test_run_opts, attr,
    .ctx_in = args,
    .ctx_size_in = sizeof(args),
    );
    struct dummy_st_ops_success *skel;
    int fd, err;
    size_t i;
    char name[8];
    skel = dummy_st_ops_success__open_and_load();
    if (!ASSERT_OK_PTR(skel, "dummy_st_ops_load"))
    return;
    fd = bpf_program__fd(skel.progs.test_2);
    err = bpf_prog_test_run_opts(fd, &attr);
    ASSERT_OK(err, "test_run");
    args[0] = 7;
    for (i = 0; i < ARRAY_SIZE(args); i++) {
    snprintf(name, sizeof(name), "arg %zu", i);
    ASSERT_EQ(skel.bss.test_2_args[i], args[i], name);
    }
    dummy_st_ops_success__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_dummy_sleepable() {
    static void test_dummy_sleepable(void)
    {
    struct bpf_dummy_ops_state st;
    __u64 args[1] = {(__u64)&st};
    LIBBPF_OPTS(bpf_test_run_opts, attr,
    .ctx_in = args,
    .ctx_size_in = sizeof(args),
    );
    struct dummy_st_ops_success *skel;
    int fd, err;
    skel = dummy_st_ops_success__open_and_load();
    if (!ASSERT_OK_PTR(skel, "dummy_st_ops_load"))
    return;
    fd = bpf_program__fd(skel.progs.test_sleepable);
    err = bpf_prog_test_run_opts(fd, &attr);
    ASSERT_OK(err, "test_run");
    dummy_st_ops_success__destroy(skel);
    }
// dummy_st_ops.test_sleepable() parameter is not marked as nullable,
// thus bpf_prog_test_run_opts() below should be rejected as it tries
// to pass NULL for this parameter.
//
#[no_mangle]
unsafe extern "C" fn test_dummy_sleepable_reject_null() {
    static void test_dummy_sleepable_reject_null(void)
    {
    __u64 args[1] = {0};
    LIBBPF_OPTS(bpf_test_run_opts, attr,
    .ctx_in = args,
    .ctx_size_in = sizeof(args),
    );
    struct dummy_st_ops_success *skel;
    int fd, err;
    skel = dummy_st_ops_success__open_and_load();
    if (!ASSERT_OK_PTR(skel, "dummy_st_ops_load"))
    return;
    fd = bpf_program__fd(skel.progs.test_sleepable);
    err = bpf_prog_test_run_opts(fd, &attr);
    ASSERT_EQ(err, -EINVAL, "test_run");
    dummy_st_ops_success__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_dummy_st_ops() {
    void test_dummy_st_ops(void)
    {
    if (test__start_subtest("dummy_st_ops_attach"))
    test_dummy_st_ops_attach();
    if (test__start_subtest("dummy_init_ret_value"))
    test_dummy_init_ret_value();
    if (test__start_subtest("dummy_init_ptr_arg"))
    test_dummy_init_ptr_arg();
    if (test__start_subtest("dummy_multiple_args"))
    test_dummy_multiple_args();
    if (test__start_subtest("dummy_sleepable"))
    test_dummy_sleepable();
    if (test__start_subtest("dummy_sleepable_reject_null"))
    test_dummy_sleepable_reject_null();
    RUN_TESTS(dummy_st_ops_fail);
    }
