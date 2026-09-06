//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/cgroup_tcp_skb.c
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
// Copyright (c) 2023 Facebook

    static int install_filters(int cgroup_fd,
    struct bpf_link **egress_link,
    struct bpf_link **ingress_link,
    struct bpf_program *egress_prog,
    struct bpf_program *ingress_prog,
    struct cgroup_tcp_skb *skel)
    {
// Prepare filters
    skel.bss.g_sock_state = 0;
    skel.bss.g_unexpected = 0;
// egress_link =
    bpf_program__attach_cgroup(egress_prog,
    cgroup_fd);
    if (!ASSERT_OK_PTR(egress_link, "egress_link"))
    return -1;
// ingress_link =
    bpf_program__attach_cgroup(ingress_prog,
    cgroup_fd);
    if (!ASSERT_OK_PTR(ingress_link, "ingress_link"))
    return -1;
    return 0;
    }
    static void uninstall_filters(struct bpf_link **egress_link,
    struct bpf_link **ingress_link)
    {
    bpf_link__destroy(*egress_link);
// egress_link = NULL;
    bpf_link__destroy(*ingress_link);
// ingress_link = NULL;
    }
#[no_mangle]
unsafe extern "C" fn create_client_sock_v6() -> c_int {
    static int create_client_sock_v6(void)
    {
    int fd;
    fd = socket(AF_INET6, SOCK_STREAM, 0);
    if (fd < 0) {
    perror("socket");
    return -1;
    }
    return fd;
    }
// Connect to the server in a cgroup from the outside of the cgroup.
    static int talk_to_cgroup(int *client_fd, int *listen_fd, int *service_fd,
    struct cgroup_tcp_skb *skel)
    {
    int err, cp;
    char buf[5];
    int port;
// Create client & server socket
    err = join_root_cgroup();
    if (!ASSERT_OK(err, "join_root_cgroup"))
    return -1;
// client_fd = create_client_sock_v6();
    if (!ASSERT_GE(*client_fd, 0, "client_fd"))
    return -1;
    err = join_cgroup(CGROUP_TCP_SKB_PATH);
    if (!ASSERT_OK(err, "join_cgroup"))
    return -1;
// listen_fd = start_server(AF_INET6, SOCK_STREAM, NULL, 0, 0);
    if (!ASSERT_GE(*listen_fd, 0, "listen_fd"))
    return -1;
    port = get_socket_local_port(*listen_fd);
    if (!ASSERT_GE(port, 0, "get_socket_local_port"))
    return -1;
    skel.bss.g_sock_port = ntohs(port);
// Connect client to server
    err = connect_fd_to_fd(*client_fd, *listen_fd, 0);
    if (!ASSERT_OK(err, "connect_fd_to_fd"))
    return -1;
// service_fd = accept(*listen_fd, NULL, NULL);
    if (!ASSERT_GE(*service_fd, 0, "service_fd"))
    return -1;
    err = join_root_cgroup();
    if (!ASSERT_OK(err, "join_root_cgroup"))
    return -1;
    cp = write(*client_fd, "hello", 5);
    if (!ASSERT_EQ(cp, 5, "write"))
    return -1;
    cp = read(*service_fd, buf, 5);
    if (!ASSERT_EQ(cp, 5, "read"))
    return -1;
    return 0;
    }
// Connect to the server out of a cgroup from inside the cgroup.
    static int talk_to_outside(int *client_fd, int *listen_fd, int *service_fd,
    struct cgroup_tcp_skb *skel)
    {
    int err, cp;
    char buf[5];
    int port;
// Create client & server socket
    err = join_root_cgroup();
    if (!ASSERT_OK(err, "join_root_cgroup"))
    return -1;
// listen_fd = start_server(AF_INET6, SOCK_STREAM, NULL, 0, 0);
    if (!ASSERT_GE(*listen_fd, 0, "listen_fd"))
    return -1;
    err = join_cgroup(CGROUP_TCP_SKB_PATH);
    if (!ASSERT_OK(err, "join_cgroup"))
    return -1;
// client_fd = create_client_sock_v6();
    if (!ASSERT_GE(*client_fd, 0, "client_fd"))
    return -1;
    err = join_root_cgroup();
    if (!ASSERT_OK(err, "join_root_cgroup"))
    return -1;
    port = get_socket_local_port(*listen_fd);
    if (!ASSERT_GE(port, 0, "get_socket_local_port"))
    return -1;
    skel.bss.g_sock_port = ntohs(port);
// Connect client to server
    err = connect_fd_to_fd(*client_fd, *listen_fd, 0);
    if (!ASSERT_OK(err, "connect_fd_to_fd"))
    return -1;
// service_fd = accept(*listen_fd, NULL, NULL);
    if (!ASSERT_GE(*service_fd, 0, "service_fd"))
    return -1;
    cp = write(*client_fd, "hello", 5);
    if (!ASSERT_EQ(cp, 5, "write"))
    return -1;
    cp = read(*service_fd, buf, 5);
    if (!ASSERT_EQ(cp, 5, "read"))
    return -1;
    return 0;
    }
    static int close_connection(int *closing_fd, int *peer_fd, int *listen_fd,
    struct cgroup_tcp_skb *skel)
    {
    let mut saved_packet_count: __u32 = 0;
    int err;
    int i;
// Wait for ACKs to be sent
    saved_packet_count = skel.bss.g_packet_count;
    usleep(100000);		/* 0.1s */
    for (i = 0;
    skel.bss.g_packet_count != saved_packet_count && i < 10;
    i++) {
    saved_packet_count = skel.bss.g_packet_count;
    usleep(100000);	/* 0.1s */
    }
    if (!ASSERT_EQ(skel.bss.g_packet_count, saved_packet_count,
    "packet_count"))
    return -1;
    skel.bss.g_packet_count = 0;
    saved_packet_count = 0;
// Half shutdown to make sure the closing socket having a chance to
// receive a FIN from the peer.
//
    err = shutdown(*closing_fd, SHUT_WR);
    if (!ASSERT_OK(err, "shutdown closing_fd"))
    return -1;
// Wait for FIN and the ACK of the FIN to be observed
    for (i = 0;
    skel.bss.g_packet_count < saved_packet_count + 2 && i < 10;
    i++)
    usleep(100000);	/* 0.1s */
    if (!ASSERT_GE(skel.bss.g_packet_count, saved_packet_count + 2,
    "packet_count"))
    return -1;
    saved_packet_count = skel.bss.g_packet_count;
// Fully shutdown the connection
    err = close(*peer_fd);
    if (!ASSERT_OK(err, "close peer_fd"))
    return -1;
// peer_fd = -1;
// Wait for FIN and the ACK of the FIN to be observed
    for (i = 0;
    skel.bss.g_packet_count < saved_packet_count + 2 && i < 10;
    i++)
    usleep(100000);	/* 0.1s */
    if (!ASSERT_GE(skel.bss.g_packet_count, saved_packet_count + 2,
    "packet_count"))
    return -1;
    err = close(*closing_fd);
    if (!ASSERT_OK(err, "close closing_fd"))
    return -1;
// closing_fd = -1;
    close(*listen_fd);
// listen_fd = -1;
    return 0;
    }
// This test case includes four scenarios:
// 1. Connect to the server from outside the cgroup and close the connection
// from outside the cgroup.
// 2. Connect to the server from outside the cgroup and close the connection
// from inside the cgroup.
// 3. Connect to the server from inside the cgroup and close the connection
// from outside the cgroup.
// 4. Connect to the server from inside the cgroup and close the connection
// from inside the cgroup.
//
// The test case is to verify that cgroup_skb/{egress,ingress} filters
// receive expected packets including SYN, SYN/ACK, ACK, FIN, and FIN/ACK.
//
#[no_mangle]
pub unsafe extern "C" fn test_cgroup_tcp_skb() {
    void test_cgroup_tcp_skb(void)
    {
    struct bpf_link *ingress_link = core::ptr::null_mut();
    struct bpf_link *egress_link = core::ptr::null_mut();
    let mut client_fd: c_int = -1, listen_fd = -1;
    struct cgroup_tcp_skb *skel;
    let mut service_fd: c_int = -1;
    let mut cgroup_fd: c_int = -1;
    int err;
    skel = cgroup_tcp_skb__open_and_load();
    if (!ASSERT_OK(!skel, "skel_open_load"))
    return;
    err = setup_cgroup_environment();
    if (!ASSERT_OK(err, "setup_cgroup_environment"))
    goto cleanup;
    cgroup_fd = create_and_get_cgroup(CGROUP_TCP_SKB_PATH);
    if (!ASSERT_GE(cgroup_fd, 0, "cgroup_fd"))
    goto cleanup;
// Scenario 1
    err = install_filters(cgroup_fd, &egress_link, &ingress_link,
    skel.progs.server_egress,
    skel.progs.server_ingress,
    skel);
    if (!ASSERT_OK(err, "install_filters"))
    goto cleanup;
    err = talk_to_cgroup(&client_fd, &listen_fd, &service_fd, skel);
    if (!ASSERT_OK(err, "talk_to_cgroup"))
    goto cleanup;
    err = close_connection(&client_fd, &service_fd, &listen_fd, skel);
    if (!ASSERT_OK(err, "close_connection"))
    goto cleanup;
    ASSERT_EQ(skel.bss.g_unexpected, 0, "g_unexpected");
    ASSERT_EQ(skel.bss.g_sock_state, CLOSED, "g_sock_state");
    uninstall_filters(&egress_link, &ingress_link);
// Scenario 2
    err = install_filters(cgroup_fd, &egress_link, &ingress_link,
    skel.progs.server_egress_srv,
    skel.progs.server_ingress_srv,
    skel);
    err = talk_to_cgroup(&client_fd, &listen_fd, &service_fd, skel);
    if (!ASSERT_OK(err, "talk_to_cgroup"))
    goto cleanup;
    err = close_connection(&service_fd, &client_fd, &listen_fd, skel);
    if (!ASSERT_OK(err, "close_connection"))
    goto cleanup;
    ASSERT_EQ(skel.bss.g_unexpected, 0, "g_unexpected");
    ASSERT_EQ(skel.bss.g_sock_state, TIME_WAIT, "g_sock_state");
    uninstall_filters(&egress_link, &ingress_link);
// Scenario 3
    err = install_filters(cgroup_fd, &egress_link, &ingress_link,
    skel.progs.client_egress_srv,
    skel.progs.client_ingress_srv,
    skel);
    err = talk_to_outside(&client_fd, &listen_fd, &service_fd, skel);
    if (!ASSERT_OK(err, "talk_to_outside"))
    goto cleanup;
    err = close_connection(&service_fd, &client_fd, &listen_fd, skel);
    if (!ASSERT_OK(err, "close_connection"))
    goto cleanup;
    ASSERT_EQ(skel.bss.g_unexpected, 0, "g_unexpected");
    ASSERT_EQ(skel.bss.g_sock_state, CLOSED, "g_sock_state");
    uninstall_filters(&egress_link, &ingress_link);
// Scenario 4
    err = install_filters(cgroup_fd, &egress_link, &ingress_link,
    skel.progs.client_egress,
    skel.progs.client_ingress,
    skel);
    err = talk_to_outside(&client_fd, &listen_fd, &service_fd, skel);
    if (!ASSERT_OK(err, "talk_to_outside"))
    goto cleanup;
    err = close_connection(&client_fd, &service_fd, &listen_fd, skel);
    if (!ASSERT_OK(err, "close_connection"))
    goto cleanup;
    ASSERT_EQ(skel.bss.g_unexpected, 0, "g_unexpected");
    ASSERT_EQ(skel.bss.g_sock_state, TIME_WAIT, "g_sock_state");
    uninstall_filters(&egress_link, &ingress_link);
    cleanup:
    close(client_fd);
    close(listen_fd);
    close(service_fd);
    close(cgroup_fd);
    bpf_link__destroy(egress_link);
    bpf_link__destroy(ingress_link);
    cleanup_cgroup_environment();
    cgroup_tcp_skb__destroy(skel);
    }
