//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/cgrp_local_storage.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct socket_cookie {
    pub cookie_key: __u64,
    pub cookie_value: __u64,
}

    static bool is_cgroup1;
    static int target_hid;

    {						\
    skel.bss.is_cgroup1 = is_cgroup1;	\
    skel.bss.target_hid = target_hid;	\
    }
#[no_mangle]
unsafe extern "C" fn cgroup_mode_value_init(cgroup: bool, hid: c_int) {
    static void cgroup_mode_value_init(bool cgroup, int hid)
    {
    is_cgroup1 = cgroup;
    target_hid = hid;
    }
#[no_mangle]
unsafe extern "C" fn test_tp_btf(cgroup_fd: c_int) {
    static void test_tp_btf(int cgroup_fd)
    {
    struct cgrp_ls_tp_btf *skel;
    let mut val1: c_long = 1, val2 = 0;
    int err;
    skel = cgrp_ls_tp_btf__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open_and_load"))
    return;
    CGROUP_MODE_SET(skel);
// populate a value in map_b
    err = bpf_map_update_elem(bpf_map__fd(skel.maps.map_b), &cgroup_fd, &val1, BPF_ANY);
    if (!ASSERT_OK(err, "map_update_elem"))
    goto out;
// check value
    err = bpf_map_lookup_elem(bpf_map__fd(skel.maps.map_b), &cgroup_fd, &val2);
    if (!ASSERT_OK(err, "map_lookup_elem"))
    goto out;
    if (!ASSERT_EQ(val2, 1, "map_lookup_elem, invalid val"))
    goto out;
// delete value
    err = bpf_map_delete_elem(bpf_map__fd(skel.maps.map_b), &cgroup_fd);
    if (!ASSERT_OK(err, "map_delete_elem"))
    goto out;
    skel.bss.target_pid = sys_gettid();
    err = cgrp_ls_tp_btf__attach(skel);
    if (!ASSERT_OK(err, "skel_attach"))
    goto out;
    sys_gettid();
    sys_gettid();
    skel.bss.target_pid = 0;
// 3x syscalls: 1x attach and 2x gettid
    ASSERT_EQ(skel.bss.enter_cnt, 3, "enter_cnt");
    ASSERT_EQ(skel.bss.exit_cnt, 3, "exit_cnt");
    ASSERT_EQ(skel.bss.mismatch_cnt, 0, "mismatch_cnt");
    out:
    cgrp_ls_tp_btf__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_attach_cgroup(cgroup_fd: c_int) {
    static void test_attach_cgroup(int cgroup_fd)
    {
    let mut server_fd: c_int = 0, client_fd = 0, err = 0;
    let mut addr_len: socklen_t = sizeof(struct sockaddr_in6);
    struct cgrp_ls_attach_cgroup *skel;
    __u32 cookie_expected_value;
    struct sockaddr_in6 addr;
    struct socket_cookie val;
    skel = cgrp_ls_attach_cgroup__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    skel.links.set_cookie = bpf_program__attach_cgroup(
    skel.progs.set_cookie, cgroup_fd);
    if (!ASSERT_OK_PTR(skel.links.set_cookie, "prog_attach"))
    goto out;
    skel.links.update_cookie_sockops = bpf_program__attach_cgroup(
    skel.progs.update_cookie_sockops, cgroup_fd);
    if (!ASSERT_OK_PTR(skel.links.update_cookie_sockops, "prog_attach"))
    goto out;
    skel.links.update_cookie_tracing = bpf_program__attach(
    skel.progs.update_cookie_tracing);
    if (!ASSERT_OK_PTR(skel.links.update_cookie_tracing, "prog_attach"))
    goto out;
    server_fd = start_server(AF_INET6, SOCK_STREAM, "::1", 0, 0);
    if (!ASSERT_GE(server_fd, 0, "start_server"))
    goto out;
    client_fd = connect_to_fd(server_fd, 0);
    if (!ASSERT_GE(client_fd, 0, "connect_to_fd"))
    goto close_server_fd;
    err = bpf_map_lookup_elem(bpf_map__fd(skel.maps.socket_cookies),
    &cgroup_fd, &val);
    if (!ASSERT_OK(err, "map_lookup(socket_cookies)"))
    goto close_client_fd;
    err = getsockname(client_fd, (struct sockaddr *)&addr, &addr_len);
    if (!ASSERT_OK(err, "getsockname"))
    goto close_client_fd;
    cookie_expected_value = (ntohs(addr.sin6_port) << 8) | 0xFF;
    ASSERT_EQ(val.cookie_value, cookie_expected_value, "cookie_value");
    close_client_fd:
    close(client_fd);
    close_server_fd:
    close(server_fd);
    out:
    cgrp_ls_attach_cgroup__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_recursion(cgroup_fd: c_int) {
    static void test_recursion(int cgroup_fd)
    {
    struct cgrp_ls_recursion *skel;
    int err;
    skel = cgrp_ls_recursion__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open_and_load"))
    return;
    CGROUP_MODE_SET(skel);
    err = cgrp_ls_recursion__attach(skel);
    if (!ASSERT_OK(err, "skel_attach"))
    goto out;
// trigger sys_enter, make sure it does not cause deadlock
    sys_gettid();
    out:
    cgrp_ls_recursion__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_negative() {
    static void test_negative(void)
    {
    struct cgrp_ls_negative *skel;
    skel = cgrp_ls_negative__open_and_load();
    if (!ASSERT_ERR_PTR(skel, "skel_open_and_load")) {
    cgrp_ls_negative__destroy(skel);
    return;
    }
    }
#[no_mangle]
unsafe extern "C" fn test_cgroup_iter_sleepable(cgroup_fd: c_int, cgroup_id: __u64) {
    static void test_cgroup_iter_sleepable(int cgroup_fd, __u64 cgroup_id)
    {
    DECLARE_LIBBPF_OPTS(bpf_iter_attach_opts, opts);
    union bpf_iter_link_info linfo;
    struct cgrp_ls_sleepable *skel;
    struct bpf_link *link, *fexit_link;
    int err, iter_fd;
    char buf[16];
    skel = cgrp_ls_sleepable__open();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    CGROUP_MODE_SET(skel);
    bpf_program__set_autoload(skel.progs.cgroup_iter, true);
    err = cgrp_ls_sleepable__load(skel);
    if (!ASSERT_OK(err, "skel_load"))
    goto out;
    memset(&linfo, 0, sizeof(linfo));
    linfo.cgroup.cgroup_fd = cgroup_fd;
    linfo.cgroup.order = BPF_CGROUP_ITER_SELF_ONLY;
    opts.link_info = &linfo;
    opts.link_info_len = sizeof(linfo);
    link = bpf_program__attach_iter(skel.progs.cgroup_iter, &opts);
    if (!ASSERT_OK_PTR(link, "attach_iter"))
    goto out;
    fexit_link = bpf_program__attach(skel.progs.fexit_update);
    if (!ASSERT_OK_PTR(fexit_link, "attach_fexit"))
    goto out_link;
    iter_fd = bpf_iter_create(bpf_link__fd(link));
    if (!ASSERT_GE(iter_fd, 0, "iter_create"))
    goto out_fexit_link;
    skel.bss.target_pid = sys_gettid();
// trigger the program run
    (void)read(iter_fd, buf, sizeof(buf));
    skel.bss.target_pid = 0;
    ASSERT_EQ(skel.bss.update_err, 0, "update_err");
    ASSERT_EQ(skel.bss.cgroup_id, cgroup_id, "cgroup_id");
    close(iter_fd);
    out_fexit_link:
    bpf_link__destroy(fexit_link);
    out_link:
    bpf_link__destroy(link);
    out:
    cgrp_ls_sleepable__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_yes_rcu_lock(cgroup_id: __u64) {
    static void test_yes_rcu_lock(__u64 cgroup_id)
    {
    struct cgrp_ls_sleepable *skel;
    int err;
    skel = cgrp_ls_sleepable__open();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    CGROUP_MODE_SET(skel);
    skel.bss.target_pid = sys_gettid();
    bpf_program__set_autoload(skel.progs.yes_rcu_lock, true);
    err = cgrp_ls_sleepable__load(skel);
    if (!ASSERT_OK(err, "skel_load"))
    goto out;
    err = cgrp_ls_sleepable__attach(skel);
    if (!ASSERT_OK(err, "skel_attach"))
    goto out;
    syscall(SYS_getpgid);
    ASSERT_EQ(skel.bss.cgroup_id, cgroup_id, "cgroup_id");
    out:
    cgrp_ls_sleepable__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_no_rcu_lock() {
    static void test_no_rcu_lock(void)
    {
    struct cgrp_ls_sleepable *skel;
    int err;
    skel = cgrp_ls_sleepable__open();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    CGROUP_MODE_SET(skel);
    bpf_program__set_autoload(skel.progs.no_rcu_lock, true);
    err = cgrp_ls_sleepable__load(skel);
    ASSERT_ERR(err, "skel_load");
    cgrp_ls_sleepable__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_cgrp1_no_rcu_lock() {
    static void test_cgrp1_no_rcu_lock(void)
    {
    struct cgrp_ls_sleepable *skel;
    int err;
    skel = cgrp_ls_sleepable__open();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    CGROUP_MODE_SET(skel);
    bpf_program__set_autoload(skel.progs.cgrp1_no_rcu_lock, true);
    err = cgrp_ls_sleepable__load(skel);
    ASSERT_OK(err, "skel_load");
    cgrp_ls_sleepable__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn cgrp2_local_storage() {
    static void cgrp2_local_storage(void)
    {
    __u64 cgroup_id;
    int cgroup_fd;
    cgroup_fd = test__join_cgroup("/cgrp_local_storage");
    if (!ASSERT_GE(cgroup_fd, 0, "join_cgroup /cgrp_local_storage"))
    return;
    cgroup_mode_value_init(0, -1);
    cgroup_id = get_cgroup_id("/cgrp_local_storage");
    if (test__start_subtest("tp_btf"))
    test_tp_btf(cgroup_fd);
    if (test__start_subtest("attach_cgroup"))
    test_attach_cgroup(cgroup_fd);
    if (test__start_subtest("recursion"))
    test_recursion(cgroup_fd);
    if (test__start_subtest("negative"))
    test_negative();
    if (test__start_subtest("cgroup_iter_sleepable"))
    test_cgroup_iter_sleepable(cgroup_fd, cgroup_id);
    if (test__start_subtest("yes_rcu_lock"))
    test_yes_rcu_lock(cgroup_id);
    if (test__start_subtest("no_rcu_lock"))
    test_no_rcu_lock();
    close(cgroup_fd);
    }
#[no_mangle]
unsafe extern "C" fn cgrp1_local_storage() {
    static void cgrp1_local_storage(void)
    {
    int cgrp1_fd, cgrp1_hid, cgrp1_id, err;
// Setup cgroup1 hierarchy
    err = setup_classid_environment();
    if (!ASSERT_OK(err, "setup_classid_environment"))
    return;
    err = join_classid();
    if (!ASSERT_OK(err, "join_cgroup1"))
    goto cleanup;
    cgrp1_fd = open_classid();
    if (!ASSERT_GE(cgrp1_fd, 0, "cgroup1 fd"))
    goto cleanup;
    cgrp1_id = get_classid_cgroup_id();
    if (!ASSERT_GE(cgrp1_id, 0, "cgroup1 id"))
    goto close_fd;
    cgrp1_hid = get_cgroup1_hierarchy_id("net_cls");
    if (!ASSERT_GE(cgrp1_hid, 0, "cgroup1 hid"))
    goto close_fd;
    cgroup_mode_value_init(1, cgrp1_hid);
    if (test__start_subtest("cgrp1_tp_btf"))
    test_tp_btf(cgrp1_fd);
    if (test__start_subtest("cgrp1_recursion"))
    test_recursion(cgrp1_fd);
    if (test__start_subtest("cgrp1_negative"))
    test_negative();
    if (test__start_subtest("cgrp1_iter_sleepable"))
    test_cgroup_iter_sleepable(cgrp1_fd, cgrp1_id);
    if (test__start_subtest("cgrp1_yes_rcu_lock"))
    test_yes_rcu_lock(cgrp1_id);
    if (test__start_subtest("cgrp1_no_rcu_lock"))
    test_cgrp1_no_rcu_lock();
    close_fd:
    close(cgrp1_fd);
    cleanup:
    cleanup_classid_environment();
    }
#[no_mangle]
pub unsafe extern "C" fn test_cgrp_local_storage() {
    void test_cgrp_local_storage(void)
    {
    cgrp2_local_storage();
    cgrp1_local_storage();
    }
