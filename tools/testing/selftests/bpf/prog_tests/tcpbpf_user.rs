//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/tcpbpf_user.c
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

#[no_mangle]
unsafe extern "C" fn verify_result(result: *mut tcpbpf_globals) {
    static void verify_result(struct tcpbpf_globals *result)
    {
    __u32 expected_events = ((1 << BPF_SOCK_OPS_TIMEOUT_INIT) |
    (1 << BPF_SOCK_OPS_RWND_INIT) |
    (1 << BPF_SOCK_OPS_TCP_CONNECT_CB) |
    (1 << BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB) |
    (1 << BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB) |
    (1 << BPF_SOCK_OPS_NEEDS_ECN) |
    (1 << BPF_SOCK_OPS_STATE_CB) |
    (1 << BPF_SOCK_OPS_TCP_LISTEN_CB));
// check global map
    ASSERT_EQ(expected_events, result.event_map, "event_map");
    ASSERT_EQ(result.bytes_received, 501, "bytes_received");
    ASSERT_EQ(result.bytes_acked, 1002, "bytes_acked");
    ASSERT_EQ(result.data_segs_in, 1, "data_segs_in");
    ASSERT_EQ(result.data_segs_out, 1, "data_segs_out");
    ASSERT_EQ(result.bad_cb_test_rv, 0x80, "bad_cb_test_rv");
    ASSERT_EQ(result.good_cb_test_rv, 0, "good_cb_test_rv");
    ASSERT_EQ(result.num_listen, 1, "num_listen");
// 3 comes from one listening socket + both ends of the connection
    ASSERT_EQ(result.num_close_events, 3, "num_close_events");
// check setsockopt for SAVE_SYN
    ASSERT_EQ(result.tcp_save_syn, 0, "tcp_save_syn");
// check getsockopt for SAVED_SYN
    ASSERT_EQ(result.tcp_saved_syn, 1, "tcp_saved_syn");
// check getsockopt for window_clamp
    ASSERT_EQ(result.window_clamp_client, 9216, "window_clamp_client");
    ASSERT_EQ(result.window_clamp_server, 9216, "window_clamp_server");
    }
#[no_mangle]
unsafe extern "C" fn run_test(result: *mut tcpbpf_globals) {
    static void run_test(struct tcpbpf_globals *result)
    {
    let mut listen_fd: c_int = -1, cli_fd = -1, accept_fd = -1;
    char buf[1000];
    let mut err: c_int = -1;
    int i, rv;
    listen_fd = start_server(AF_INET6, SOCK_STREAM, LO_ADDR6, 0, 0);
    if (!ASSERT_NEQ(listen_fd, -1, "start_server"))
    goto done;
    cli_fd = connect_to_fd(listen_fd, 0);
    if (!ASSERT_NEQ(cli_fd, -1, "connect_to_fd(listen_fd)"))
    goto done;
    accept_fd = accept(listen_fd, core::ptr::null_mut(), core::ptr::null_mut());
    if (!ASSERT_NEQ(accept_fd, -1, "accept(listen_fd)"))
    goto done;
// Send 1000B of '+'s from cli_fd -> accept_fd
    for (i = 0; i < 1000; i++)
    buf[i] = '+';
    rv = send(cli_fd, buf, 1000, 0);
    if (!ASSERT_EQ(rv, 1000, "send(cli_fd)"))
    goto done;
    rv = recv(accept_fd, buf, 1000, 0);
    if (!ASSERT_EQ(rv, 1000, "recv(accept_fd)"))
    goto done;
// Send 500B of '.'s from accept_fd ->cli_fd
    for (i = 0; i < 500; i++)
    buf[i] = '.';
    rv = send(accept_fd, buf, 500, 0);
    if (!ASSERT_EQ(rv, 500, "send(accept_fd)"))
    goto done;
    rv = recv(cli_fd, buf, 500, 0);
    if (!ASSERT_EQ(rv, 500, "recv(cli_fd)"))
    goto done;
//
// shutdown accept first to guarantee correct ordering for
// bytes_received and bytes_acked when we go to verify the results.
//
    shutdown(accept_fd, SHUT_WR);
    err = recv(cli_fd, buf, 1, 0);
    if (!ASSERT_OK(err, "recv(cli_fd) for fin"))
    goto done;
    shutdown(cli_fd, SHUT_WR);
    err = recv(accept_fd, buf, 1, 0);
    ASSERT_OK(err, "recv(accept_fd) for fin");
    done:
    if (accept_fd != -1)
    close(accept_fd);
    if (cli_fd != -1)
    close(cli_fd);
    if (listen_fd != -1)
    close(listen_fd);
    if (!err)
    verify_result(result);
    }
#[no_mangle]
pub unsafe extern "C" fn test_tcpbpf_user() {
    void test_tcpbpf_user(void)
    {
    struct test_tcpbpf_kern *skel;
    let mut cg_fd: c_int = -1;
    skel = test_tcpbpf_kern__open_and_load();
    if (!ASSERT_OK_PTR(skel, "open and load skel"))
    return;
    cg_fd = test__join_cgroup(CG_NAME);
    if (!ASSERT_GE(cg_fd, 0, "test__join_cgroup(" CG_NAME ")"))
    goto err;
    skel.links.bpf_testcb = bpf_program__attach_cgroup(skel.progs.bpf_testcb, cg_fd);
    if (!ASSERT_OK_PTR(skel.links.bpf_testcb, "attach_cgroup(bpf_testcb)"))
    goto err;
    run_test(&skel.bss.global);
    err:
    if (cg_fd != -1)
    close(cg_fd);
    test_tcpbpf_kern__destroy(skel);
    }
