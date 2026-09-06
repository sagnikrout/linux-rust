//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/kfunc_call.c
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

    static size_t log_buf_sz = 1048576; /* 1 MB */
    static char obj_log_buf[1048576];
    enum kfunc_test_type {
    tc_test = 0,
    syscall_test,
    syscall_null_ctx_test,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfunc_test_params {
    pub prog_name: *const c_char,
    pub lskel_prog_desc_offset: c_ulong,
    pub retval: c_int,
    pub test_type: enum kfunc_test_type,
    pub expected_err_msg: *const c_char,
}

    { \
    .prog_name = #name, \
    .lskel_prog_desc_offset = offsetof(struct kfunc_call_test_lskel, progs.name), \
    .retval = __retval, \
    .test_type = type, \
    .expected_err_msg = core::ptr::null_mut(), \
    }

    { \
    .prog_name = #name, \
    .lskel_prog_desc_offset = 0 /* unused when test is failing */, \
    .retval = __retval, \
    .test_type = type, \
    .expected_err_msg = error_msg, \
    }

    __BPF_TEST_FAIL(name, retval, syscall_null_ctx_test, error_msg)
    static struct kfunc_test_params kfunc_tests[] = {
// failure cases:
// if retval is 0 -> the program will fail to load and the error message is an error
// if retval is not 0 -> the program can be loaded but running it will gives the
// provided return value. The error message is thus the one
// from a successful load
//
    SYSCALL_NULL_CTX_FAIL(kfunc_syscall_test_fail, -EINVAL, "processed 4 insns"),
    SYSCALL_NULL_CTX_FAIL(kfunc_syscall_test_null_fail, -EINVAL, "processed 4 insns"),
    TC_FAIL(kfunc_call_test_get_mem_fail_rdonly, 0, "R0 cannot write into rdonly_mem"),
    TC_FAIL(kfunc_call_test_get_mem_fail_use_after_free, 0, "invalid mem access 'scalar'"),
    TC_FAIL(kfunc_call_test_get_mem_fail_oob, 0, "min value is outside of the allowed memory range"),
    TC_FAIL(kfunc_call_test_get_mem_fail_zero_size, 0, "min value is outside of the allowed memory range"),
    TC_FAIL(kfunc_call_test_get_mem_fail_oversized, 0, "allocation size exceeds u32 max"),
    TC_FAIL(kfunc_call_test_get_mem_fail_not_const, 0, "is not a const"),
    TC_FAIL(kfunc_call_test_mem_acquire_fail, 0, "acquire kernel function does not return PTR_TO_BTF_ID"),
    TC_FAIL(kfunc_call_test_pointer_arg_type_mismatch, 0, "R1 expected pointer to ctx, but got scalar"),
    TC_FAIL(kfunc_call_test_spin_lock_unsafe, 0, "function calls are not allowed while holding a lock"),
// success cases
    TC_TEST(kfunc_call_test_spin_lock_safe, 0),
    TC_TEST(kfunc_call_test1, 12),
    TC_TEST(kfunc_call_test2, 3),
    TC_TEST(kfunc_call_test4, -1234),
    TC_TEST(kfunc_call_test5, 0),
    TC_TEST(kfunc_call_test5_asm, 0),
    TC_TEST(kfunc_call_test_ref_btf_id, 0),
    TC_TEST(kfunc_call_test_get_mem, 42),
    SYSCALL_TEST(kfunc_syscall_test, 0),
    SYSCALL_NULL_CTX_TEST(kfunc_syscall_test_null, 0),
    TC_TEST(kfunc_call_test_static_unused_arg, 0),
    TC_TEST(kfunc_call_ctx, 0),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_test_args {
    pub data: [__u8; 16],
    pub size: usize,
}

#[no_mangle]
unsafe extern "C" fn verify_success(param: *mut kfunc_test_params) {
    static void verify_success(struct kfunc_test_params *param)
    {
    struct kfunc_call_test_lskel *lskel = core::ptr::null_mut();
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    struct bpf_prog_desc *lskel_prog;
    struct kfunc_call_test *skel;
    struct bpf_program *prog;
    int prog_fd, err;
    struct syscall_test_args args = {
    .size = 10,
    };
    switch (param.test_type) {
    case syscall_test:
    topts.ctx_in = &args;
    topts.ctx_size_in = sizeof(args);
// fallthrough
    case syscall_null_ctx_test:
    break;
    case tc_test:
    topts.data_in = &pkt_v4;
    topts.data_size_in = sizeof(pkt_v4);
    topts.repeat = 1;
    break;
    }
// first test with normal libbpf
    skel = kfunc_call_test__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel"))
    return;
    prog = bpf_object__find_program_by_name(skel.obj, param.prog_name);
    if (!ASSERT_OK_PTR(prog, "bpf_object__find_program_by_name"))
    goto cleanup;
    prog_fd = bpf_program__fd(prog);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    if (!ASSERT_OK(err, param.prog_name))
    goto cleanup;
    if (!ASSERT_EQ(topts.retval, param.retval, "retval"))
    goto cleanup;
// second test with light skeletons
    lskel = kfunc_call_test_lskel__open_and_load();
    if (!ASSERT_OK_PTR(lskel, "lskel"))
    goto cleanup;
    lskel_prog = (struct bpf_prog_desc *)((char *)lskel + param.lskel_prog_desc_offset);
    prog_fd = lskel_prog.prog_fd;
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    if (!ASSERT_OK(err, param.prog_name))
    goto cleanup;
    ASSERT_EQ(topts.retval, param.retval, "retval");
    cleanup:
    kfunc_call_test__destroy(skel);
    if (lskel)
    kfunc_call_test_lskel__destroy(lskel);
    }
#[no_mangle]
unsafe extern "C" fn verify_fail(param: *mut kfunc_test_params) {
    static void verify_fail(struct kfunc_test_params *param)
    {
    LIBBPF_OPTS(bpf_object_open_opts, opts);
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    struct bpf_program *prog;
    struct kfunc_call_fail *skel;
    int prog_fd, err;
    struct syscall_test_args args = {
    .size = 10,
    };
    opts.kernel_log_buf = obj_log_buf;
    opts.kernel_log_size = log_buf_sz;
    opts.kernel_log_level = 1;
    switch (param.test_type) {
    case syscall_test:
    topts.ctx_in = &args;
    topts.ctx_size_in = sizeof(args);
// fallthrough
    case syscall_null_ctx_test:
    break;
    case tc_test:
    topts.data_in = &pkt_v4;
    topts.data_size_in = sizeof(pkt_v4);
    topts.repeat = 1;
    break;
    }
    skel = kfunc_call_fail__open_opts(&opts);
    if (!ASSERT_OK_PTR(skel, "kfunc_call_fail__open_opts"))
    goto cleanup;
    prog = bpf_object__find_program_by_name(skel.obj, param.prog_name);
    if (!ASSERT_OK_PTR(prog, "bpf_object__find_program_by_name"))
    goto cleanup;
    bpf_program__set_autoload(prog, true);
    err = kfunc_call_fail__load(skel);
    if (!param.retval) {
// the verifier is supposed to complain and refuses to load
    if (!ASSERT_ERR(err, "unexpected load success"))
    goto out_err;
    } else {
// the program is loaded but must dynamically fail
    if (!ASSERT_OK(err, "unexpected load error"))
    goto out_err;
    prog_fd = bpf_program__fd(prog);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    if (!ASSERT_EQ(err, param.retval, param.prog_name))
    goto out_err;
    }
    out_err:
    if (!ASSERT_OK_PTR(strstr(obj_log_buf, param.expected_err_msg), "expected_err_msg")) {
    fprintf(stderr, "Expected err_msg: %s\n", param.expected_err_msg);
    fprintf(stderr, "Verifier output: %s\n", obj_log_buf);
    }
    cleanup:
    kfunc_call_fail__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_main() {
    static void test_main(void)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(kfunc_tests); i++) {
    if (!test__start_subtest(kfunc_tests[i].prog_name))
    continue;
    if (!kfunc_tests[i].expected_err_msg)
    verify_success(&kfunc_tests[i]);
    else
    verify_fail(&kfunc_tests[i]);
    }
    }
#[no_mangle]
unsafe extern "C" fn test_subprog() {
    static void test_subprog(void)
    {
    struct kfunc_call_test_subprog *skel;
    int prog_fd, err;
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    skel = kfunc_call_test_subprog__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel"))
    return;
    prog_fd = bpf_program__fd(skel.progs.kfunc_call_test1);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "bpf_prog_test_run(test1)");
    ASSERT_EQ(topts.retval, 10, "test1-retval");
    ASSERT_NEQ(skel.data.active_res, -1, "active_res");
    ASSERT_EQ(skel.data.sk_state_res, BPF_TCP_CLOSE, "sk_state_res");
    kfunc_call_test_subprog__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_subprog_lskel() {
    static void test_subprog_lskel(void)
    {
    struct kfunc_call_test_subprog_lskel *skel;
    int prog_fd, err;
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    skel = kfunc_call_test_subprog_lskel__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel"))
    return;
    prog_fd = skel.progs.kfunc_call_test1.prog_fd;
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "bpf_prog_test_run(test1)");
    ASSERT_EQ(topts.retval, 10, "test1-retval");
    ASSERT_NEQ(skel.data.active_res, -1, "active_res");
    ASSERT_EQ(skel.data.sk_state_res, BPF_TCP_CLOSE, "sk_state_res");
    kfunc_call_test_subprog_lskel__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_destructive_open_and_load() -> c_int {
    static int test_destructive_open_and_load(void)
    {
    struct kfunc_call_destructive *skel;
    int err;
    skel = kfunc_call_destructive__open();
    if (!ASSERT_OK_PTR(skel, "prog_open"))
    return -1;
    err = kfunc_call_destructive__load(skel);
    kfunc_call_destructive__destroy(skel);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn test_destructive() {
    static void test_destructive(void)
    {
    let mut save_caps: __u64 = 0;
    ASSERT_OK(test_destructive_open_and_load(), "successful_load");
    if (!ASSERT_OK(cap_disable_effective(1ULL << CAP_SYS_BOOT, &save_caps), "drop_caps"))
    return;
    ASSERT_EQ(test_destructive_open_and_load(), -13, "no_caps_failure");
    cap_enable_effective(save_caps, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn test_kfunc_call() {
    void test_kfunc_call(void)
    {
    test_main();
    if (test__start_subtest("subprog"))
    test_subprog();
    if (test__start_subtest("subprog_lskel"))
    test_subprog_lskel();
    if (test__start_subtest("destructive"))
    test_destructive();
    }
