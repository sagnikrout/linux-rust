//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/btf_skc_cls_ingress.c
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
// Macro flag: #define _GNU_SOURCE

// RFC791, 576 for minimal IPv4 datagram, minus 40 bytes of TCP header
pub const MIN_IPV4_MSS: c_int = 536;
    static struct netns_obj *prepare_netns(struct test_btf_skc_cls_ingress *skel)
    {
    LIBBPF_OPTS(bpf_tc_hook, qdisc_lo, .attach_point = BPF_TC_INGRESS);
    LIBBPF_OPTS(bpf_tc_opts, tc_attach,
    .prog_fd = bpf_program__fd(skel.progs.cls_ingress));
    struct netns_obj *ns = core::ptr::null_mut();
    ns = netns_new(TEST_NS, true);
    if (!ASSERT_OK_PTR(ns, "create and join netns"))
    return ns;
    qdisc_lo.ifindex = if_nametoindex("lo");
    if (!ASSERT_OK(bpf_tc_hook_create(&qdisc_lo), "qdisc add dev lo clsact"))
    goto free_ns;
    if (!ASSERT_OK(bpf_tc_attach(&qdisc_lo, &tc_attach),
    "filter add dev lo ingress"))
    goto free_ns;
// Ensure 20 bytes options (i.e. in total 40 bytes tcp header) for the
// bpf_tcp_gen_syncookie() helper.
//
    if (write_sysctl("/proc/sys/net/ipv4/tcp_window_scaling", "1") ||
    write_sysctl("/proc/sys/net/ipv4/tcp_timestamps", "1") ||
    write_sysctl("/proc/sys/net/ipv4/tcp_sack", "1"))
    goto free_ns;
    return ns;
    free_ns:
    netns_free(ns);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn reset_test(skel: *mut test_btf_skc_cls_ingress) {
    static void reset_test(struct test_btf_skc_cls_ingress *skel)
    {
    memset(&skel.bss.srv_sa4, 0, sizeof(skel.bss.srv_sa4));
    memset(&skel.bss.srv_sa6, 0, sizeof(skel.bss.srv_sa6));
    skel.bss.listen_tp_sport = 0;
    skel.bss.req_sk_sport = 0;
    skel.bss.recv_cookie = 0;
    skel.bss.gen_cookie = 0;
    skel.bss.linum = 0;
    skel.bss.mss = 0;
    }
#[no_mangle]
unsafe extern "C" fn print_err_line(skel: *mut test_btf_skc_cls_ingress) {
    static void print_err_line(struct test_btf_skc_cls_ingress *skel)
    {
    if (skel.bss.linum)
    printf("bpf prog error at line %u\n", skel.bss.linum);
    }
#[no_mangle]
unsafe extern "C" fn v6only_true(fd: c_int, opts: *mut c_void) -> c_int {
    static int v6only_true(int fd, void *opts)
    {
    let mut mode: c_int = true;
    return setsockopt(fd, IPPROTO_IPV6, IPV6_V6ONLY, &mode, sizeof(mode));
    }
#[no_mangle]
unsafe extern "C" fn v6only_false(fd: c_int, opts: *mut c_void) -> c_int {
    static int v6only_false(int fd, void *opts)
    {
    let mut mode: c_int = false;
    return setsockopt(fd, IPPROTO_IPV6, IPV6_V6ONLY, &mode, sizeof(mode));
    }
    static void run_test(struct test_btf_skc_cls_ingress *skel, bool gen_cookies,
    int ip_mode)
    {
    const char *tcp_syncookies = gen_cookies ? "2" : "1";
    let mut listen_fd: c_int = -1, cli_fd = -1, srv_fd = -1, err;
    let mut opts: network_helper_opts = { 0 };
    struct sockaddr_storage *addr;
    struct sockaddr_in6 srv_sa6;
    struct sockaddr_in srv_sa4;
    socklen_t addr_len;
    int sock_family;
    char *srv_addr;
    int srv_port;
    switch (ip_mode) {
    case TEST_MODE_IPV4:
    sock_family = AF_INET;
    srv_addr = SERVER_ADDR_IPV4;
    addr = (struct sockaddr_storage *)&srv_sa4;
    addr_len = sizeof(srv_sa4);
    break;
    case TEST_MODE_IPV6:
    opts.post_socket_cb = v6only_true;
    sock_family = AF_INET6;
    srv_addr = SERVER_ADDR_IPV6;
    addr = (struct sockaddr_storage *)&srv_sa6;
    addr_len = sizeof(srv_sa6);
    break;
    case TEST_MODE_DUAL:
    opts.post_socket_cb = v6only_false;
    sock_family = AF_INET6;
    srv_addr = SERVER_ADDR_DUAL;
    addr = (struct sockaddr_storage *)&srv_sa6;
    addr_len = sizeof(srv_sa6);
    break;
    default:
    PRINT_FAIL("Unknown IP mode %d", ip_mode);
    return;
    }
    if (write_sysctl("/proc/sys/net/ipv4/tcp_syncookies", tcp_syncookies))
    return;
    listen_fd = start_server_str(sock_family, SOCK_STREAM, srv_addr,  0,
    &opts);
    if (!ASSERT_OK_FD(listen_fd, "start server"))
    return;
    err = getsockname(listen_fd, (struct sockaddr *)addr, &addr_len);
    if (!ASSERT_OK(err, "getsockname(listen_fd)"))
    goto done;
    switch (ip_mode) {
    case TEST_MODE_IPV4:
    memcpy(&skel.bss.srv_sa4, &srv_sa4, sizeof(srv_sa4));
    srv_port = ntohs(srv_sa4.sin_port);
    break;
    case TEST_MODE_IPV6:
    case TEST_MODE_DUAL:
    memcpy(&skel.bss.srv_sa6, &srv_sa6, sizeof(srv_sa6));
    srv_port = ntohs(srv_sa6.sin6_port);
    break;
    default:
    goto done;
    }
    cli_fd = connect_to_fd(listen_fd, 0);
    if (!ASSERT_OK_FD(cli_fd, "connect client"))
    goto done;
    srv_fd = accept(listen_fd, core::ptr::null_mut(), core::ptr::null_mut());
    if (!ASSERT_OK_FD(srv_fd, "accept connection"))
    goto done;
    ASSERT_EQ(skel.bss.listen_tp_sport, srv_port, "listen tp src port");
    if (!gen_cookies) {
    ASSERT_EQ(skel.bss.req_sk_sport, srv_port,
    "request socket source port with syncookies disabled");
    ASSERT_EQ(skel.bss.gen_cookie, 0,
    "generated syncookie with syncookies disabled");
    ASSERT_EQ(skel.bss.recv_cookie, 0,
    "received syncookie with syncookies disabled");
    } else {
    ASSERT_EQ(skel.bss.req_sk_sport, 0,
    "request socket source port with syncookies enabled");
    ASSERT_NEQ(skel.bss.gen_cookie, 0,
    "syncookie properly generated");
    ASSERT_EQ(skel.bss.gen_cookie, skel.bss.recv_cookie,
    "matching syncookies on client and server");
    ASSERT_GT(skel.bss.mss, MIN_IPV4_MSS,
    "MSS in cookie min value");
    ASSERT_LT(skel.bss.mss, USHRT_MAX,
    "MSS in cookie max value");
    }
    done:
    if (listen_fd != -1)
    close(listen_fd);
    if (cli_fd != -1)
    close(cli_fd);
    if (srv_fd != -1)
    close(srv_fd);
    }
#[no_mangle]
unsafe extern "C" fn test_conn_ipv4(skel: *mut test_btf_skc_cls_ingress) {
    static void test_conn_ipv4(struct test_btf_skc_cls_ingress *skel)
    {
    run_test(skel, false, TEST_MODE_IPV4);
    }
#[no_mangle]
unsafe extern "C" fn test_conn_ipv6(skel: *mut test_btf_skc_cls_ingress) {
    static void test_conn_ipv6(struct test_btf_skc_cls_ingress *skel)
    {
    run_test(skel, false, TEST_MODE_IPV6);
    }
#[no_mangle]
unsafe extern "C" fn test_conn_dual(skel: *mut test_btf_skc_cls_ingress) {
    static void test_conn_dual(struct test_btf_skc_cls_ingress *skel)
    {
    run_test(skel, false, TEST_MODE_DUAL);
    }
#[no_mangle]
unsafe extern "C" fn test_syncookie_ipv4(skel: *mut test_btf_skc_cls_ingress) {
    static void test_syncookie_ipv4(struct test_btf_skc_cls_ingress *skel)
    {
    run_test(skel, true, TEST_MODE_IPV4);
    }
#[no_mangle]
unsafe extern "C" fn test_syncookie_ipv6(skel: *mut test_btf_skc_cls_ingress) {
    static void test_syncookie_ipv6(struct test_btf_skc_cls_ingress *skel)
    {
    run_test(skel, true, TEST_MODE_IPV6);
    }
#[no_mangle]
unsafe extern "C" fn test_syncookie_dual(skel: *mut test_btf_skc_cls_ingress) {
    static void test_syncookie_dual(struct test_btf_skc_cls_ingress *skel)
    {
    run_test(skel, true, TEST_MODE_DUAL);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test {
    pub desc: *const c_char,
    pub skel): *mut *mut void (run)(struct test_btf_skc_cls_ingress,
}

    static struct test tests[] = {
    DEF_TEST(conn_ipv4),
    DEF_TEST(conn_ipv6),
    DEF_TEST(conn_dual),
    DEF_TEST(syncookie_ipv4),
    DEF_TEST(syncookie_ipv6),
    DEF_TEST(syncookie_dual),
    };
#[no_mangle]
pub unsafe extern "C" fn test_btf_skc_cls_ingress() {
    void test_btf_skc_cls_ingress(void)
    {
    struct test_btf_skc_cls_ingress *skel;
    struct netns_obj *ns;
    int i;
    skel = test_btf_skc_cls_ingress__open_and_load();
    if (!ASSERT_OK_PTR(skel, "test_btf_skc_cls_ingress__open_and_load"))
    return;
    for (i = 0; i < ARRAY_SIZE(tests); i++) {
    if (!test__start_subtest(tests[i].desc))
    continue;
    ns = prepare_netns(skel);
    if (!ns)
    break;
    tests[i].run(skel);
    print_err_line(skel);
    reset_test(skel);
    netns_free(ns);
    }
    test_btf_skc_cls_ingress__destroy(skel);
    }
