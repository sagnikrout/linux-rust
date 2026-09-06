//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/cgrp_kfunc.c
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
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
// Macro flag: #define _GNU_SOURCE

    static struct cgrp_kfunc_success *open_load_cgrp_kfunc_skel(void)
    {
    struct cgrp_kfunc_success *skel;
    int err;
    skel = cgrp_kfunc_success__open();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return core::ptr::null_mut();
    skel.bss.pid = getpid();
    err = cgrp_kfunc_success__load(skel);
    if (!ASSERT_OK(err, "skel_load"))
    goto cleanup;
    return skel;
    cleanup:
    cgrp_kfunc_success__destroy(skel);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn mkdir_rm_test_dir() -> c_int {
    static int mkdir_rm_test_dir(void)
    {
    int fd;
    const char *cgrp_path = "cgrp_kfunc";
    fd = create_and_get_cgroup(cgrp_path);
    if (!ASSERT_GT(fd, 0, "mkdir_cgrp_fd"))
    return -1;
    close(fd);
    remove_cgroup(cgrp_path);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn run_success_test(prog_name: *const c_char) {
    static void run_success_test(const char *prog_name)
    {
    struct cgrp_kfunc_success *skel;
    struct bpf_program *prog;
    struct bpf_link *link = core::ptr::null_mut();
    skel = open_load_cgrp_kfunc_skel();
    if (!ASSERT_OK_PTR(skel, "open_load_skel"))
    return;
    if (!ASSERT_OK(skel.bss.err, "pre_mkdir_err"))
    goto cleanup;
    prog = bpf_object__find_program_by_name(skel.obj, prog_name);
    if (!ASSERT_OK_PTR(prog, "bpf_object__find_program_by_name"))
    goto cleanup;
    link = bpf_program__attach(prog);
    if (!ASSERT_OK_PTR(link, "attached_link"))
    goto cleanup;
    ASSERT_EQ(skel.bss.invocations, 0, "pre_rmdir_count");
    if (!ASSERT_OK(mkdir_rm_test_dir(), "cgrp_mkdir"))
    goto cleanup;
    ASSERT_EQ(skel.bss.invocations, 1, "post_rmdir_count");
    ASSERT_OK(skel.bss.err, "post_rmdir_err");
    cleanup:
    bpf_link__destroy(link);
    cgrp_kfunc_success__destroy(skel);
    }
    static const char * const success_tests[] = {
    "test_cgrp_acquire_release_argument",
    "test_cgrp_acquire_leave_in_map",
    "test_cgrp_xchg_release",
    "test_cgrp_get_release",
    "test_cgrp_get_ancestors",
    "test_cgrp_from_id",
    };
#[no_mangle]
unsafe extern "C" fn test_cgrp_from_id_ns() {
    static void test_cgrp_from_id_ns(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts);
    struct cgrp_kfunc_success *skel;
    struct bpf_program *prog;
    int pid, pipe_fd[2];
    skel = open_load_cgrp_kfunc_skel();
    if (!ASSERT_OK_PTR(skel, "open_load_skel"))
    return;
    if (!ASSERT_OK(skel.bss.err, "pre_mkdir_err"))
    goto cleanup;
    prog = skel.progs.test_cgrp_from_id_ns;
    if (!ASSERT_OK(pipe(pipe_fd), "pipe"))
    goto cleanup;
    pid = fork();
    if (!ASSERT_GE(pid, 0, "fork result")) {
    close(pipe_fd[0]);
    close(pipe_fd[1]);
    goto cleanup;
    }
    if (pid == 0) {
    let mut ret: c_int = 0;
    close(pipe_fd[0]);
    if (!ASSERT_GE(cgroup_setup_and_join("cgrp_from_id_ns"), 0, "join cgroup"))
    exit(1);
    if (!ASSERT_OK(unshare(CLONE_NEWCGROUP), "unshare cgns"))
    exit(1);
    ret = bpf_prog_test_run_opts(bpf_program__fd(prog), &opts);
    if (!ASSERT_OK(ret, "test run ret"))
    exit(1);
    if (!ASSERT_OK(opts.retval, "test run retval"))
    exit(1);
    if (!ASSERT_EQ(write(pipe_fd[1], &ret, sizeof(ret)), sizeof(ret), "write pipe"))
    exit(1);
    exit(0);
    } else {
    int res;
    close(pipe_fd[1]);
    ASSERT_EQ(read(pipe_fd[0], &res, sizeof(res)), sizeof(res), "read res");
    ASSERT_EQ(waitpid(pid, core::ptr::null_mut(), 0), pid, "wait on child");
    remove_cgroup_pid("cgrp_from_id_ns", pid);
    ASSERT_OK(res, "result from run");
    }
    close(pipe_fd[0]);
    cleanup:
    cgrp_kfunc_success__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_cgrp_kfunc() {
    void test_cgrp_kfunc(void)
    {
    int i, err;
    err = setup_cgroup_environment();
    if (!ASSERT_OK(err, "cgrp_env_setup"))
    goto cleanup;
    for (i = 0; i < ARRAY_SIZE(success_tests); i++) {
    if (!test__start_subtest(success_tests[i]))
    continue;
    run_success_test(success_tests[i]);
    }
    if (test__start_subtest("test_cgrp_from_id_ns"))
    test_cgrp_from_id_ns();
    RUN_TESTS(cgrp_kfunc_failure);
    cleanup:
    cleanup_cgroup_environment();
    }
