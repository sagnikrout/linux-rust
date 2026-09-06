//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_lsm.c
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
//
// Copyright (C) 2020 Google LLC.
//

    char *CMD_ARGS[] = {"true", core::ptr::null_mut()};
#[no_mangle]
pub unsafe extern "C" fn exec_cmd(monitored_pid: *mut c_int) -> c_int {
    int exec_cmd(int *monitored_pid)
    {
    int child_pid, child_status;
    child_pid = fork();
    if (child_pid == 0) {
// monitored_pid = getpid();
    execvp(CMD_ARGS[0], CMD_ARGS);
    return -EINVAL;
    } else if (child_pid > 0) {
    waitpid(child_pid, &child_status, 0);
    return child_status;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn test_lsm(skel: *mut lsm) -> c_int {
    static int test_lsm(struct lsm *skel)
    {
    struct bpf_link *link;
    let mut buf: c_int = 1234;
    int err;
    err = lsm__attach(skel);
    if (!ASSERT_OK(err, "attach"))
    return err;
// Check that already linked program can't be attached again.
    link = bpf_program__attach(skel.progs.test_int_hook);
    if (!ASSERT_ERR_PTR(link, "attach_link"))
    return -1;
    err = exec_cmd(&skel.bss.monitored_pid);
    if (!ASSERT_OK(err, "exec_cmd"))
    return err;
    ASSERT_EQ(skel.bss.bprm_count, 1, "bprm_count");
    skel.bss.monitored_pid = getpid();
    err = stack_mprotect();
    if (!ASSERT_EQ(err, -1, "stack_mprotect") ||
    !ASSERT_EQ(errno, EPERM, "stack_mprotect"))
    return err;
    ASSERT_EQ(skel.bss.mprotect_count, 1, "mprotect_count");
    syscall(__NR_setdomainname, &buf, -2L);
    syscall(__NR_setdomainname, 0, -3L);
    syscall(__NR_setdomainname, ~0L, -4L);
    ASSERT_EQ(skel.bss.copy_test, 3, "copy_test");
    lsm__detach(skel);
    skel.bss.copy_test = 0;
    skel.bss.bprm_count = 0;
    skel.bss.mprotect_count = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_lsm_basic() {
    static void test_lsm_basic(void)
    {
    struct lsm *skel = core::ptr::null_mut();
    int err;
    skel = lsm__open_and_load();
    if (!ASSERT_OK_PTR(skel, "lsm_skel_load"))
    goto close_prog;
    err = test_lsm(skel);
    if (!ASSERT_OK(err, "test_lsm_first_attach"))
    goto close_prog;
    err = test_lsm(skel);
    ASSERT_OK(err, "test_lsm_second_attach");
    close_prog:
    lsm__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_lsm_tailcall() {
    static void test_lsm_tailcall(void)
    {
    struct lsm_tailcall *skel = core::ptr::null_mut();
    int map_fd, prog_fd;
    int err, key;
    skel = lsm_tailcall__open_and_load();
    if (!ASSERT_OK_PTR(skel, "lsm_tailcall__skel_load"))
    goto close_prog;
    map_fd = bpf_map__fd(skel.maps.jmp_table);
    if (CHECK_FAIL(map_fd < 0))
    goto close_prog;
    prog_fd = bpf_program__fd(skel.progs.lsm_file_permission_prog);
    if (CHECK_FAIL(prog_fd < 0))
    goto close_prog;
    key = 0;
    err = bpf_map_update_elem(map_fd, &key, &prog_fd, BPF_ANY);
    if (CHECK_FAIL(!err))
    goto close_prog;
    prog_fd = bpf_program__fd(skel.progs.lsm_kernfs_init_security_prog);
    if (CHECK_FAIL(prog_fd < 0))
    goto close_prog;
    err = bpf_map_update_elem(map_fd, &key, &prog_fd, BPF_ANY);
    if (CHECK_FAIL(err))
    goto close_prog;
    close_prog:
    lsm_tailcall__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_test_lsm() {
    void test_test_lsm(void)
    {
    if (test__start_subtest("lsm_basic"))
    test_lsm_basic();
    if (test__start_subtest("lsm_tailcall"))
    test_lsm_tailcall();
    }
