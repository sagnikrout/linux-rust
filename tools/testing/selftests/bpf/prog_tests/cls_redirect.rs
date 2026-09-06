//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/cls_redirect.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright (c) 2020 Cloudflare
// Macro flag: #define _GNU_SOURCE

    let mut duration: static int = 0;
    static bool set_up_conn(const struct sockaddr_storage *addr, socklen_t len, int type,
    int *server, int *conn,
    struct sockaddr_storage *src,
    struct sockaddr_storage *dst)
    {
    struct sockaddr_storage ss;
    let mut slen: socklen_t = sizeof(ss);
// server = start_server_addr(type, addr, len, NULL);
    if (*server < 0)
    return false;
    if (CHECK_FAIL(getsockname(*server, (struct sockaddr *)&ss, &slen)))
    goto close_server;
// conn = connect_to_addr(type, &ss, slen, NULL);
    if (*conn < 0)
    goto close_server;
// We want to simulate packets arriving at conn, so we have to
// swap src and dst.
//
    slen = sizeof(*dst);
    if (CHECK_FAIL(getsockname(*conn, (struct sockaddr *)dst, &slen)))
    goto close_conn;
    slen = sizeof(*src);
    if (CHECK_FAIL(getpeername(*conn, (struct sockaddr *)src, &slen)))
    goto close_conn;
    return true;
    close_conn:
    close(*conn);
// conn = -1;
    close_server:
    close(*server);
// server = -1;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn prepare_addr(addr: *mut sockaddr_storage, family: c_int) -> socklen_t {
    static socklen_t prepare_addr(struct sockaddr_storage *addr, int family)
    {
    struct sockaddr_in *addr4;
    struct sockaddr_in6 *addr6;
    memset(addr, 0, sizeof(*addr));
    switch (family) {
    case AF_INET:
    addr4 = (struct sockaddr_in *)addr;
    addr4.sin_family = family;
    addr4.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    return sizeof(*addr4);
    case AF_INET6:
    addr6 = (struct sockaddr_in6 *)addr;
    addr6.sin6_family = family;
    addr6.sin6_addr = in6addr_loopback;
    return sizeof(*addr6);
    default:
    fprintf(stderr, "Invalid family %d", family);
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn was_decapsulated(tattr: *mut bpf_test_run_opts) -> bool {
    static bool was_decapsulated(struct bpf_test_run_opts *tattr)
    {
    return tattr.data_size_out < tattr.data_size_in;
    }
    enum type {
    UDP,
    TCP,
    __NR_KIND,
    };
    enum hops {
    NO_HOPS,
    ONE_HOP,
    };
    enum flags {
    NONE,
    SYN,
    ACK,
    };
    enum conn {
    KNOWN_CONN,
    UNKNOWN_CONN,
    };
    enum result {
    ACCEPT,
    FORWARD,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_cfg {
    pub type: enum type,
    pub result: enum result,
    pub conn: enum conn,
    pub hops: enum hops,
    pub flags: enum flags,
}

    static int test_str(void *buf, size_t len, const struct test_cfg *test,
    int family)
    {
    const char *family_str, *type, *conn, *hops, *result, *flags;
    family_str = "IPv4";
    if (family == AF_INET6)
    family_str = "IPv6";
    type = "TCP";
    if (test.type == UDP)
    type = "UDP";
    conn = "known";
    if (test.conn == UNKNOWN_CONN)
    conn = "unknown";
    hops = "no hops";
    if (test.hops == ONE_HOP)
    hops = "one hop";
    result = "accept";
    if (test.result == FORWARD)
    result = "forward";
    flags = "none";
    if (test.flags == SYN)
    flags = "SYN";
#[no_mangle]
pub unsafe extern "C" fn if(ACK: test->flags ==) -> else {
    else if (test.flags == ACK)
    flags = "ACK";
    return snprintf(buf, len, "%s %s %s %s (%s, flags: %s)", family_str,
    type, result, conn, hops, flags);
    }
    static struct test_cfg tests[] = {
    { TCP, ACCEPT, UNKNOWN_CONN, NO_HOPS, SYN },
    { TCP, ACCEPT, UNKNOWN_CONN, NO_HOPS, ACK },
    { TCP, FORWARD, UNKNOWN_CONN, ONE_HOP, ACK },
    { TCP, ACCEPT, KNOWN_CONN, ONE_HOP, ACK },
    { UDP, ACCEPT, UNKNOWN_CONN, NO_HOPS, NONE },
    { UDP, FORWARD, UNKNOWN_CONN, ONE_HOP, NONE },
    { UDP, ACCEPT, KNOWN_CONN, ONE_HOP, NONE },
    };
#[no_mangle]
unsafe extern "C" fn encap_init(encap: *mut encap_headers_t, hop_count: u8, proto: u8) {
    static void encap_init(encap_headers_t *encap, uint8_t hop_count, uint8_t proto)
    {
    const uint8_t hlen =
    (sizeof(struct guehdr) / sizeof(uint32_t)) + hop_count;
// encap = (encap_headers_t){
    .eth = { .h_proto = htons(ETH_P_IP) },
    .ip = {
    .ihl = 5,
    .version = 4,
    .ttl = IPDEFTTL,
    .protocol = IPPROTO_UDP,
    .daddr = htonl(ENCAP_IP)
    },
    .udp = {
    .dest = htons(ENCAP_PORT),
    },
    .gue = {
    .hlen = hlen,
    .proto_ctype = proto
    },
    .unigue = {
    .hop_count = hop_count
    },
    };
    }
    static size_t build_input(const struct test_cfg *test, void *const buf,
    const struct sockaddr_storage *src,
    const struct sockaddr_storage *dst)
    {
    struct sockaddr_in6 *src_in6 = (struct sockaddr_in6 *)src;
    struct sockaddr_in6 *dst_in6 = (struct sockaddr_in6 *)dst;
    struct sockaddr_in *src_in = (struct sockaddr_in *)src;
    struct sockaddr_in *dst_in = (struct sockaddr_in *)dst;
    let mut family: sa_family_t = src.ss_family;
    in_port_t sport, dport;
    encap_headers_t encap;
    struct iphdr ip;
    struct ipv6hdr ipv6;
    struct tcphdr tcp;
    struct udphdr udp;
    struct in_addr next_hop;
    uint8_t *p = buf;
    int proto;
    sport = (family == AF_INET) ? src_in.sin_port : src_in6.sin6_port;
    dport = (family == AF_INET) ? dst_in.sin_port : dst_in6.sin6_port;
    proto = IPPROTO_IPIP;
    if (family == AF_INET6)
    proto = IPPROTO_IPV6;
    encap_init(&encap, test.hops == ONE_HOP ? 1 : 0, proto);
    p = mempcpy(p, &encap, sizeof(encap));
    if (test.hops == ONE_HOP) {
    next_hop = (struct in_addr){ .s_addr = htonl(0x7f000002) };
    p = mempcpy(p, &next_hop, sizeof(next_hop));
    }
    proto = IPPROTO_TCP;
    if (test.type == UDP)
    proto = IPPROTO_UDP;
    switch (family) {
    case AF_INET:
    ip = (struct iphdr){
    .ihl = 5,
    .version = 4,
    .ttl = IPDEFTTL,
    .protocol = proto,
    .saddr = src_in.sin_addr.s_addr,
    .daddr = dst_in.sin_addr.s_addr,
    };
    p = mempcpy(p, &ip, sizeof(ip));
    break;
    case AF_INET6:
    ipv6 = (struct ipv6hdr){
    .version = 6,
    .hop_limit = IPDEFTTL,
    .nexthdr = proto,
    .saddr = src_in6.sin6_addr,
    .daddr = dst_in6.sin6_addr,
    };
    p = mempcpy(p, &ipv6, sizeof(ipv6));
    break;
    default:
    return 0;
    }
    if (test.conn == UNKNOWN_CONN)
    sport--;
    switch (test.type) {
    case TCP:
    tcp = (struct tcphdr){
    .source = sport,
    .dest = dport,
    .syn = (test.flags == SYN),
    .ack = (test.flags == ACK),
    };
    p = mempcpy(p, &tcp, sizeof(tcp));
    break;
    case UDP:
    udp = (struct udphdr){
    .source = sport,
    .dest = dport,
    };
    p = mempcpy(p, &udp, sizeof(udp));
    break;
    default:
    return 0;
    }
    return (void *)p - buf;
    }
#[no_mangle]
unsafe extern "C" fn close_fds(fds: *mut c_int, n: c_int) {
    static void close_fds(int *fds, int n)
    {
    int i;
    for (i = 0; i < n; i++)
    if (fds[i] > 0)
    close(fds[i]);
    }
#[no_mangle]
unsafe extern "C" fn test_cls_redirect_common(prog: *mut bpf_program) {
    static void test_cls_redirect_common(struct bpf_program *prog)
    {
    LIBBPF_OPTS(bpf_test_run_opts, tattr);
    int families[] = { AF_INET, AF_INET6 };
    struct sockaddr_storage ss;
    socklen_t slen;
    int i, j, err, prog_fd;
    int servers[__NR_KIND][ARRAY_SIZE(families)] = {};
    int conns[__NR_KIND][ARRAY_SIZE(families)] = {};
    struct sockaddr_storage srcs[__NR_KIND][ARRAY_SIZE(families)];
    struct sockaddr_storage dsts[__NR_KIND][ARRAY_SIZE(families)];
    for (i = 0; i < ARRAY_SIZE(families); i++) {
    slen = prepare_addr(&ss, families[i]);
    if (CHECK_FAIL(!slen))
    goto cleanup;
    if (CHECK_FAIL(!set_up_conn(&ss, slen, SOCK_DGRAM,
    &servers[UDP][i], &conns[UDP][i],
    &srcs[UDP][i], &dsts[UDP][i])))
    goto cleanup;
    if (CHECK_FAIL(!set_up_conn(&ss, slen, SOCK_STREAM,
    &servers[TCP][i], &conns[TCP][i],
    &srcs[TCP][i], &dsts[TCP][i])))
    goto cleanup;
    }
    prog_fd = bpf_program__fd(prog);
    for (i = 0; i < ARRAY_SIZE(tests); i++) {
    struct test_cfg *test = &tests[i];
    for (j = 0; j < ARRAY_SIZE(families); j++) {
    struct sockaddr_storage *src = &srcs[test.type][j];
    struct sockaddr_storage *dst = &dsts[test.type][j];
    char input[256];
    char tmp[256];
    test_str(tmp, sizeof(tmp), test, families[j]);
    if (!test__start_subtest(tmp))
    continue;
    tattr.data_out = tmp;
    tattr.data_size_out = sizeof(tmp);
    tattr.data_in = input;
    tattr.data_size_in = build_input(test, input, src, dst);
    if (CHECK_FAIL(!tattr.data_size_in))
    continue;
    err = bpf_prog_test_run_opts(prog_fd, &tattr);
    if (CHECK_FAIL(err))
    continue;
    if (tattr.retval != TC_ACT_REDIRECT) {
    PRINT_FAIL("expected TC_ACT_REDIRECT, got %d\n",
    tattr.retval);
    continue;
    }
    switch (test.result) {
    case ACCEPT:
    if (CHECK_FAIL(!was_decapsulated(&tattr)))
    continue;
    break;
    case FORWARD:
    if (CHECK_FAIL(was_decapsulated(&tattr)))
    continue;
    break;
    default:
    PRINT_FAIL("unknown result %d\n", test.result);
    continue;
    }
    }
    }
    cleanup:
    close_fds((int *)servers, sizeof(servers) / sizeof(servers[0][0]));
    close_fds((int *)conns, sizeof(conns) / sizeof(conns[0][0]));
    }
#[no_mangle]
unsafe extern "C" fn test_cls_redirect_dynptr() {
    static void test_cls_redirect_dynptr(void)
    {
    struct test_cls_redirect_dynptr *skel;
    int err;
    skel = test_cls_redirect_dynptr__open();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    skel.rodata.ENCAPSULATION_IP = htonl(ENCAP_IP);
    skel.rodata.ENCAPSULATION_PORT = htons(ENCAP_PORT);
    err = test_cls_redirect_dynptr__load(skel);
    if (!ASSERT_OK(err, "skel_load"))
    goto cleanup;
    test_cls_redirect_common(skel.progs.cls_redirect);
    cleanup:
    test_cls_redirect_dynptr__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_cls_redirect_inlined() {
    static void test_cls_redirect_inlined(void)
    {
    struct test_cls_redirect *skel;
    int err;
    skel = test_cls_redirect__open();
    if (CHECK(!skel, "skel_open", "failed\n"))
    return;
    skel.rodata.ENCAPSULATION_IP = htonl(ENCAP_IP);
    skel.rodata.ENCAPSULATION_PORT = htons(ENCAP_PORT);
    err = test_cls_redirect__load(skel);
    if (CHECK(err, "skel_load", "failed: %d\n", err))
    goto cleanup;
    test_cls_redirect_common(skel.progs.cls_redirect);
    cleanup:
    test_cls_redirect__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_cls_redirect_subprogs() {
    static void test_cls_redirect_subprogs(void)
    {
    struct test_cls_redirect_subprogs *skel;
    int err;
    skel = test_cls_redirect_subprogs__open();
    if (CHECK(!skel, "skel_open", "failed\n"))
    return;
    skel.rodata.ENCAPSULATION_IP = htonl(ENCAP_IP);
    skel.rodata.ENCAPSULATION_PORT = htons(ENCAP_PORT);
    err = test_cls_redirect_subprogs__load(skel);
    if (CHECK(err, "skel_load", "failed: %d\n", err))
    goto cleanup;
    test_cls_redirect_common(skel.progs.cls_redirect);
    cleanup:
    test_cls_redirect_subprogs__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_cls_redirect() {
    void test_cls_redirect(void)
    {
    if (test__start_subtest("cls_redirect_inlined"))
    test_cls_redirect_inlined();
    if (test__start_subtest("cls_redirect_subprogs"))
    test_cls_redirect_subprogs();
    if (test__start_subtest("cls_redirect_dynptr"))
    test_cls_redirect_dynptr();
    }
