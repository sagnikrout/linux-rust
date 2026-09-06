//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/setget_sockopt.c
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
// Copyright (c) Meta Platforms, Inc. and affiliates.
// Macro flag: #define _GNU_SOURCE

    static const char addr4_str[] = "127.0.0.1";
    static const char addr6_str[] = "::1";
    static struct setget_sockopt *skel;
    static int cg_fd;
#[no_mangle]
unsafe extern "C" fn create_netns() -> c_int {
    static int create_netns(void)
    {
    if (!ASSERT_OK(unshare(CLONE_NEWNET), "create netns"))
    return -1;
    if (!ASSERT_OK(system("ip link set dev lo up"), "set lo up"))
    return -1;
    if (!ASSERT_OK(system("ip link add dev binddevtest1 type veth peer name binddevtest2"),
    "add veth"))
    return -1;
    if (!ASSERT_OK(system("ip link set dev binddevtest1 up"),
    "bring veth up"))
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_tcp(family: c_int) {
    static void test_tcp(int family)
    {
    struct setget_sockopt__bss *bss = skel.bss;
    int sfd, cfd;
    memset(bss, 0, sizeof(*bss));
    sfd = start_server(family, SOCK_STREAM,
    family == AF_INET6 ? addr6_str : addr4_str, 0, 0);
    if (!ASSERT_GE(sfd, 0, "start_server"))
    return;
    cfd = connect_to_fd(sfd, 0);
    if (!ASSERT_GE(cfd, 0, "connect_to_fd_server")) {
    close(sfd);
    return;
    }
    close(sfd);
    close(cfd);
    ASSERT_EQ(bss.nr_listen, 1, "nr_listen");
    ASSERT_EQ(bss.nr_connect, 1, "nr_connect");
    ASSERT_EQ(bss.nr_active, 1, "nr_active");
    ASSERT_EQ(bss.nr_passive, 1, "nr_passive");
    ASSERT_EQ(bss.nr_socket_post_create, 2, "nr_socket_post_create");
    ASSERT_EQ(bss.nr_binddev, 2, "nr_bind");
    }
#[no_mangle]
unsafe extern "C" fn test_udp(family: c_int) {
    static void test_udp(int family)
    {
    struct setget_sockopt__bss *bss = skel.bss;
    int sfd;
    memset(bss, 0, sizeof(*bss));
    sfd = start_server(family, SOCK_DGRAM,
    family == AF_INET6 ? addr6_str : addr4_str, 0, 0);
    if (!ASSERT_GE(sfd, 0, "start_server"))
    return;
    close(sfd);
    ASSERT_GE(bss.nr_socket_post_create, 1, "nr_socket_post_create");
    ASSERT_EQ(bss.nr_binddev, 1, "nr_bind");
    }
#[no_mangle]
unsafe extern "C" fn test_ktls(family: c_int) {
    static void test_ktls(int family)
    {
    struct tls12_crypto_info_aes_gcm_128 aes128;
    struct setget_sockopt__bss *bss = skel.bss;
    let mut cfd: c_int = -1, sfd = -1, fd = -1, ret;
    char buf;
    memset(bss, 0, sizeof(*bss));
    sfd = start_server(family, SOCK_STREAM,
    family == AF_INET6 ? addr6_str : addr4_str, 0, 0);
    if (!ASSERT_GE(sfd, 0, "start_server"))
    return;
    fd = connect_to_fd(sfd, 0);
    if (!ASSERT_GE(fd, 0, "connect_to_fd"))
    goto err_out;
    cfd = accept(sfd, core::ptr::null_mut(), 0);
    if (!ASSERT_GE(cfd, 0, "accept"))
    goto err_out;
    close(sfd);
    sfd = -1;
// Setup KTLS
    ret = setsockopt(fd, IPPROTO_TCP, TCP_ULP, "tls", sizeof("tls"));
    if (!ASSERT_OK(ret, "setsockopt"))
    goto err_out;
    ret = setsockopt(cfd, IPPROTO_TCP, TCP_ULP, "tls", sizeof("tls"));
    if (!ASSERT_OK(ret, "setsockopt"))
    goto err_out;
    memset(&aes128, 0, sizeof(aes128));
    aes128.info.version = TLS_1_2_VERSION;
    aes128.info.cipher_type = TLS_CIPHER_AES_GCM_128;
    ret = setsockopt(fd, SOL_TLS, TLS_TX, &aes128, sizeof(aes128));
    if (!ASSERT_OK(ret, "setsockopt"))
    goto err_out;
    ret = setsockopt(cfd, SOL_TLS, TLS_RX, &aes128, sizeof(aes128));
    if (!ASSERT_OK(ret, "setsockopt"))
    goto err_out;
// KTLS is enabled
    close(fd);
// At this point, the cfd socket is at the CLOSE_WAIT state
// and still run TLS protocol.  The test for
// BPF_TCP_CLOSE_WAIT should be run at this point.
//
    ret = read(cfd, &buf, sizeof(buf));
    ASSERT_EQ(ret, 0, "read");
    close(cfd);
    ASSERT_EQ(bss.nr_listen, 1, "nr_listen");
    ASSERT_EQ(bss.nr_connect, 1, "nr_connect");
    ASSERT_EQ(bss.nr_active, 1, "nr_active");
    ASSERT_EQ(bss.nr_passive, 1, "nr_passive");
    ASSERT_EQ(bss.nr_socket_post_create, 2, "nr_socket_post_create");
    ASSERT_EQ(bss.nr_binddev, 2, "nr_bind");
    ASSERT_EQ(bss.nr_fin_wait1, 1, "nr_fin_wait1");
    return;
    err_out:
    close(fd);
    close(cfd);
    close(sfd);
    }
#[no_mangle]
unsafe extern "C" fn test_nonstandard_opt(family: c_int) {
    static void test_nonstandard_opt(int family)
    {
    struct setget_sockopt__bss *bss = skel.bss;
    struct bpf_link *getsockopt_link = core::ptr::null_mut();
    let mut sfd: c_int = -1, fd = -1, cfd = -1, flags;
    let mut flagslen: socklen_t = sizeof(flags);
    memset(bss, 0, sizeof(*bss));
    sfd = start_server(family, SOCK_STREAM,
    family == AF_INET6 ? addr6_str : addr4_str, 0, 0);
    if (!ASSERT_GE(sfd, 0, "start_server"))
    return;
    fd = connect_to_fd(sfd, 0);
    if (!ASSERT_GE(fd, 0, "connect_to_fd_server"))
    goto err_out;
// cgroup/getsockopt prog will intercept getsockopt() below and
// retrieve the tcp socket bpf_sock_ops_cb_flags value for the
// accept()ed socket; this was set earlier in the passive established
// callback for the accept()ed socket via bpf_setsockopt().
//
    getsockopt_link = bpf_program__attach_cgroup(skel.progs._getsockopt, cg_fd);
    if (!ASSERT_OK_PTR(getsockopt_link, "getsockopt prog"))
    goto err_out;
    cfd = accept(sfd, core::ptr::null_mut(), 0);
    if (!ASSERT_GE(cfd, 0, "accept"))
    goto err_out;
    if (!ASSERT_OK(getsockopt(cfd, SOL_TCP, TCP_BPF_SOCK_OPS_CB_FLAGS, &flags, &flagslen),
    "getsockopt_flags"))
    goto err_out;
    ASSERT_EQ(flags & BPF_SOCK_OPS_STATE_CB_FLAG, BPF_SOCK_OPS_STATE_CB_FLAG,
    "cb_flags_set");
    err_out:
    close(sfd);
    if (fd != -1)
    close(fd);
    if (cfd != -1)
    close(cfd);
    bpf_link__destroy(getsockopt_link);
    }
#[no_mangle]
unsafe extern "C" fn connect_to_v4mapped_v6_fd(server_fd: c_int) -> c_int {
    static int connect_to_v4mapped_v6_fd(int server_fd)
    {
    struct sockaddr_storage addr;
    struct sockaddr_in *addr4 = (void *)&addr;
    let mut addrlen: socklen_t = sizeof(addr);
    let mut addr6: sockaddr_in6 = {};
    let mut fd: c_int = -1, v6only = 0, err;
    err = getsockname(server_fd, (struct sockaddr *)&addr, &addrlen);
    if (!ASSERT_OK(err, "getsockname"))
    return -1;
    fd = socket(AF_INET6, SOCK_STREAM, 0);
    if (!ASSERT_GE(fd, 0, "socket"))
    return -1;
    err = settimeo(fd, 0);
    if (!ASSERT_OK(err, "settimeo"))
    goto err_out;
    err = setsockopt(fd, IPPROTO_IPV6, IPV6_V6ONLY, &v6only, sizeof(v6only));
    if (!ASSERT_OK(err, "clear_v6only"))
    goto err_out;
    addr6.sin6_family = AF_INET6;
    addr6.sin6_port = addr4.sin_port;
    addr6.sin6_addr.s6_addr[10] = 0xff;
    addr6.sin6_addr.s6_addr[11] = 0xff;
    memcpy(&addr6.sin6_addr.s6_addr[12], &addr4.sin_addr, sizeof(addr4.sin_addr));
    err = connect(fd, (struct sockaddr *)&addr6, sizeof(addr6));
    if (!ASSERT_OK(err, "connect"))
    goto err_out;
    return fd;
    err_out:
    close(fd);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn test_v4mapped_v6_ip_tos() {
    static void test_v4mapped_v6_ip_tos(void)
    {
    struct setget_sockopt__bss *bss = skel.bss;
    let mut sfd: c_int = -1, fd = -1, got = 0, exp = 0x1c;
    socklen_t optlen;
    memset(bss, 0, sizeof(*bss));
    bss.v4mapped_v6_ip_tos_enable = 1;
    bss.v4mapped_v6_ip_tos_ret = -1;
    bss.v4mapped_v6_ip_tos_val = exp;
    sfd = start_server(AF_INET, SOCK_STREAM, addr4_str, 0, 0);
    if (!ASSERT_GE(sfd, 0, "start_server"))
    goto err_out;
    fd = connect_to_v4mapped_v6_fd(sfd);
    if (!ASSERT_GE(fd, 0, "connect_to_v4mapped_v6_fd"))
    goto err_out;
    ASSERT_GT(bss.v4mapped_v6_ip_tos_cnt, 0, "v4mapped_v6_ip_tos_cnt");
    ASSERT_EQ(bss.v4mapped_v6_ip_tos_ret, 0, "v4mapped_v6_ip_tos_ret");
    optlen = sizeof(got);
    if (!ASSERT_OK(getsockopt(fd, SOL_IP, IP_TOS, &got, &optlen), "getsockopt_ip_tos"))
    goto err_out;
    ASSERT_EQ(got, exp, "ip_tos");
    err_out:
    bss.v4mapped_v6_ip_tos_enable = 0;
    if (fd >= 0)
    close(fd);
    if (sfd >= 0)
    close(sfd);
    }
#[no_mangle]
pub unsafe extern "C" fn test_setget_sockopt() {
    void test_setget_sockopt(void)
    {
    cg_fd = test__join_cgroup(CG_NAME);
    if (!ASSERT_OK_FD(cg_fd, "join cgroup"))
    return;
    if (create_netns())
    goto done;
    skel = setget_sockopt__open();
    if (!ASSERT_OK_PTR(skel, "open skel"))
    goto done;
    strscpy(skel.rodata.veth, "binddevtest1");
    skel.rodata.veth_ifindex = if_nametoindex("binddevtest1");
    if (!ASSERT_GT(skel.rodata.veth_ifindex, 0, "if_nametoindex"))
    goto done;
    if (!ASSERT_OK(setget_sockopt__load(skel), "load skel"))
    goto done;
    skel.links.skops_sockopt =
    bpf_program__attach_cgroup(skel.progs.skops_sockopt, cg_fd);
    if (!ASSERT_OK_PTR(skel.links.skops_sockopt, "attach cgroup"))
    goto done;
    skel.links.socket_post_create =
    bpf_program__attach_cgroup(skel.progs.socket_post_create, cg_fd);
    if (!ASSERT_OK_PTR(skel.links.socket_post_create, "attach_cgroup"))
    goto done;
    test_tcp(AF_INET6);
    test_tcp(AF_INET);
    test_udp(AF_INET6);
    test_udp(AF_INET);
    test_ktls(AF_INET6);
    test_ktls(AF_INET);
    test_nonstandard_opt(AF_INET);
    test_nonstandard_opt(AF_INET6);
    test_v4mapped_v6_ip_tos();
    done:
    setget_sockopt__destroy(skel);
    close(cg_fd);
    }
