//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/ns_current_pid_tgid.c
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
// Copyright (c) 2020 Carlos Neira cneirabustos@gmail.com
// Macro flag: #define _GNU_SOURCE

    static char child_stack[STACK_SIZE];
    static int get_pid_tgid(pid_t *pid, pid_t *tgid,
    struct test_ns_current_pid_tgid__bss *bss)
    {
    struct stat st;
    int err;
// pid = sys_gettid();
// tgid = getpid();
    err = stat("/proc/self/ns/pid", &st);
    if (!ASSERT_OK(err, "stat /proc/self/ns/pid"))
    return err;
    bss.dev = st.st_dev;
    bss.ino = st.st_ino;
    bss.user_pid = 0;
    bss.user_tgid = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_current_pid_tgid_tp(args: *mut c_void) -> c_int {
    static int test_current_pid_tgid_tp(void *args)
    {
    struct test_ns_current_pid_tgid__bss  *bss;
    struct test_ns_current_pid_tgid *skel;
    let mut ret: c_int = -1, err;
    pid_t tgid, pid;
    skel = test_ns_current_pid_tgid__open();
    if (!ASSERT_OK_PTR(skel, "test_ns_current_pid_tgid__open"))
    return ret;
    bpf_program__set_autoload(skel.progs.tp_handler, true);
    err = test_ns_current_pid_tgid__load(skel);
    if (!ASSERT_OK(err, "test_ns_current_pid_tgid__load"))
    goto cleanup;
    bss = skel.bss;
    if (get_pid_tgid(&pid, &tgid, bss))
    goto cleanup;
    err = test_ns_current_pid_tgid__attach(skel);
    if (!ASSERT_OK(err, "test_ns_current_pid_tgid__attach"))
    goto cleanup;
// trigger tracepoint
    usleep(1);
    if (!ASSERT_EQ(bss.user_pid, pid, "pid"))
    goto cleanup;
    if (!ASSERT_EQ(bss.user_tgid, tgid, "tgid"))
    goto cleanup;
    ret = 0;
    cleanup:
    test_ns_current_pid_tgid__destroy(skel);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn test_current_pid_tgid_cgrp(args: *mut c_void) -> c_int {
    static int test_current_pid_tgid_cgrp(void *args)
    {
    struct test_ns_current_pid_tgid__bss *bss;
    struct test_ns_current_pid_tgid *skel;
    let mut server_fd: c_int = -1, ret = -1, err;
    let mut cgroup_fd: c_int = *(int *)args;
    pid_t tgid, pid;
    skel = test_ns_current_pid_tgid__open();
    if (!ASSERT_OK_PTR(skel, "test_ns_current_pid_tgid__open"))
    return ret;
    bpf_program__set_autoload(skel.progs.cgroup_bind4, true);
    err = test_ns_current_pid_tgid__load(skel);
    if (!ASSERT_OK(err, "test_ns_current_pid_tgid__load"))
    goto cleanup;
    bss = skel.bss;
    if (get_pid_tgid(&pid, &tgid, bss))
    goto cleanup;
    skel.links.cgroup_bind4 = bpf_program__attach_cgroup(
    skel.progs.cgroup_bind4, cgroup_fd);
    if (!ASSERT_OK_PTR(skel.links.cgroup_bind4, "bpf_program__attach_cgroup"))
    goto cleanup;
    server_fd = start_server(AF_INET, SOCK_STREAM, core::ptr::null_mut(), 0, 0);
    if (!ASSERT_GE(server_fd, 0, "start_server"))
    goto cleanup;
    if (!ASSERT_EQ(bss.user_pid, pid, "pid"))
    goto cleanup;
    if (!ASSERT_EQ(bss.user_tgid, tgid, "tgid"))
    goto cleanup;
    ret = 0;
    cleanup:
    if (server_fd >= 0)
    close(server_fd);
    test_ns_current_pid_tgid__destroy(skel);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn test_current_pid_tgid_sk_msg(args: *mut c_void) -> c_int {
    static int test_current_pid_tgid_sk_msg(void *args)
    {
    int verdict, map, server_fd = -1, client_fd = -1;
    struct test_ns_current_pid_tgid__bss *bss;
    static const char send_msg[] = "message";
    struct test_ns_current_pid_tgid *skel;
    let mut ret: c_int = -1, err, key = 0;
    pid_t tgid, pid;
    skel = test_ns_current_pid_tgid__open();
    if (!ASSERT_OK_PTR(skel, "test_ns_current_pid_tgid__open"))
    return ret;
    bpf_program__set_autoload(skel.progs.sk_msg, true);
    err = test_ns_current_pid_tgid__load(skel);
    if (!ASSERT_OK(err, "test_ns_current_pid_tgid__load"))
    goto cleanup;
    bss = skel.bss;
    if (get_pid_tgid(&pid, &tgid, skel.bss))
    goto cleanup;
    verdict = bpf_program__fd(skel.progs.sk_msg);
    map = bpf_map__fd(skel.maps.sock_map);
    err = bpf_prog_attach(verdict, map, BPF_SK_MSG_VERDICT, 0);
    if (!ASSERT_OK(err, "prog_attach"))
    goto cleanup;
    server_fd = start_server(AF_INET6, SOCK_STREAM, "::1", 0, 0);
    if (!ASSERT_GE(server_fd, 0, "start_server"))
    goto cleanup;
    client_fd = connect_to_fd(server_fd, 0);
    if (!ASSERT_GE(client_fd, 0, "connect_to_fd"))
    goto cleanup;
    err = bpf_map_update_elem(map, &key, &client_fd, BPF_ANY);
    if (!ASSERT_OK(err, "bpf_map_update_elem"))
    goto cleanup;
    err = send(client_fd, send_msg, sizeof(send_msg), 0);
    if (!ASSERT_EQ(err, sizeof(send_msg), "send(msg)"))
    goto cleanup;
    if (!ASSERT_EQ(bss.user_pid, pid, "pid"))
    goto cleanup;
    if (!ASSERT_EQ(bss.user_tgid, tgid, "tgid"))
    goto cleanup;
    ret = 0;
    cleanup:
    if (server_fd >= 0)
    close(server_fd);
    if (client_fd >= 0)
    close(client_fd);
    test_ns_current_pid_tgid__destroy(skel);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn test_ns_current_pid_tgid_new_ns(): *mut *mut int (fn)(void, arg: *mut c_void) {
    static void test_ns_current_pid_tgid_new_ns(int (*fn)(void *), void *arg)
    {
    int wstatus;
    pid_t cpid;
// Create a process in a new namespace, this process
// will be the init process of this new namespace hence will be pid 1.
//
    cpid = clone(fn, child_stack + STACK_SIZE,
    CLONE_NEWPID | SIGCHLD, arg);
    if (!ASSERT_NEQ(cpid, -1, "clone"))
    return;
    if (!ASSERT_NEQ(waitpid(cpid, &wstatus, 0), -1, "waitpid"))
    return;
    if (!ASSERT_OK(WEXITSTATUS(wstatus), "newns_pidtgid"))
    return;
    }
// TODO: use a different tracepoint
#[no_mangle]
pub unsafe extern "C" fn serial_test_current_pid_tgid() {
    void serial_test_current_pid_tgid(void)
    {
    if (test__start_subtest("root_ns_tp"))
    test_current_pid_tgid_tp(core::ptr::null_mut());
    if (test__start_subtest("new_ns_tp"))
    test_ns_current_pid_tgid_new_ns(test_current_pid_tgid_tp, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn test_ns_current_pid_tgid_cgrp() {
    void test_ns_current_pid_tgid_cgrp(void)
    {
    let mut cgroup_fd: c_int = test__join_cgroup("/sock_addr");
    if (ASSERT_OK_FD(cgroup_fd, "join_cgroup")) {
    test_ns_current_pid_tgid_new_ns(test_current_pid_tgid_cgrp, &cgroup_fd);
    close(cgroup_fd);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn test_ns_current_pid_tgid_sk_msg() {
    void test_ns_current_pid_tgid_sk_msg(void)
    {
    test_ns_current_pid_tgid_new_ns(test_current_pid_tgid_sk_msg, core::ptr::null_mut());
    }
