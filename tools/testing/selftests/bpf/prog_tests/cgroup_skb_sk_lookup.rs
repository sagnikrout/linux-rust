//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/cgroup_skb_sk_lookup.c
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

#[no_mangle]
unsafe extern "C" fn run_lookup_test(g_serv_port: *mut __u16, out_sk: c_int) {
    static void run_lookup_test(__u16 *g_serv_port, int out_sk)
    {
    let mut serv_sk: c_int = -1, in_sk = -1, serv_in_sk = -1, err;
    let mut addr: sockaddr_in6 = {};
    let mut addr_len: socklen_t = sizeof(addr);
    let mut duration: __u32 = 0;
    serv_sk = start_server(AF_INET6, SOCK_STREAM, core::ptr::null_mut(), 0, 0);
    if (CHECK(serv_sk < 0, "start_server", "failed to start server\n"))
    return;
    err = getsockname(serv_sk, (struct sockaddr *)&addr, &addr_len);
    if (CHECK(err, "getsockname", "errno %d\n", errno))
    goto cleanup;
// g_serv_port = addr.sin6_port;
// Client outside of test cgroup should fail to connect by timeout.
    err = connect_fd_to_fd(out_sk, serv_sk, 1000);
    if (CHECK(!err || errno != EINPROGRESS, "connect_fd_to_fd",
    "unexpected result err %d errno %d\n", err, errno))
    goto cleanup;
// Client inside test cgroup should connect just fine.
    in_sk = connect_to_fd(serv_sk, 0);
    if (CHECK(in_sk < 0, "connect_to_fd", "errno %d\n", errno))
    goto cleanup;
    serv_in_sk = accept(serv_sk, core::ptr::null_mut(), core::ptr::null_mut());
    if (CHECK(serv_in_sk < 0, "accept", "errno %d\n", errno))
    goto cleanup;
    cleanup:
    close(serv_in_sk);
    close(in_sk);
    close(serv_sk);
    }
#[no_mangle]
unsafe extern "C" fn run_cgroup_bpf_test(cg_path: *const c_char, out_sk: c_int) {
    static void run_cgroup_bpf_test(const char *cg_path, int out_sk)
    {
    struct cgroup_skb_sk_lookup_kern *skel;
    struct bpf_link *link;
    let mut duration: __u32 = 0;
    let mut cgfd: c_int = -1;
    skel = cgroup_skb_sk_lookup_kern__open_and_load();
    if (CHECK(!skel, "skel_open_load", "open_load failed\n"))
    return;
    cgfd = test__join_cgroup(cg_path);
    if (CHECK(cgfd < 0, "cgroup_join", "cgroup setup failed\n"))
    goto cleanup;
    link = bpf_program__attach_cgroup(skel.progs.ingress_lookup, cgfd);
    if (!ASSERT_OK_PTR(link, "cgroup_attach"))
    goto cleanup;
    run_lookup_test(&skel.bss.g_serv_port, out_sk);
    bpf_link__destroy(link);
    cleanup:
    close(cgfd);
    cgroup_skb_sk_lookup_kern__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_cgroup_skb_sk_lookup() {
    void test_cgroup_skb_sk_lookup(void)
    {
    const char *cg_path = "/foo";
    int out_sk;
// Create a socket before joining testing cgroup so that its cgroup id
// differs from that of testing cgroup. Moving selftests process to
// testing cgroup won't change cgroup id of an already created socket.
//
    out_sk = socket(AF_INET6, SOCK_STREAM, 0);
    if (CHECK_FAIL(out_sk < 0))
    return;
    run_cgroup_bpf_test(cg_path, out_sk);
    close(out_sk);
    }
