//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/icmp_send_kfunc.c
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

pub const TIMEOUT_MS: c_int = 1000;
pub const ICMP_DEST_UNREACH: c_int = 3;
pub const ICMPV6_DEST_UNREACH: c_int = 1;
pub const ICMP_HOST_UNREACH: c_int = 1;
pub const ICMP_FRAG_NEEDED: c_int = 4;
pub const NR_ICMP_UNREACH: c_int = 15;
pub const ICMPV6_REJECT_ROUTE: c_int = 6;

#[no_mangle]
unsafe extern "C" fn connect_to_fd_nonblock(server_fd: c_int) -> c_int {
    static int connect_to_fd_nonblock(int server_fd)
    {
    struct sockaddr_storage addr;
    let mut len: socklen_t = sizeof(addr);
    int fd, err, on = 1;
    if (getsockname(server_fd, (struct sockaddr *)&addr, &len))
    return -1;
    fd = socket(addr.ss_family, SOCK_STREAM | SOCK_NONBLOCK, 0);
    if (fd < 0)
    return -1;
    if (addr.ss_family == AF_INET6 &&
    setsockopt(fd, IPPROTO_IPV6, IPV6_RECVERR, &on, sizeof(on)) < 0) {
    close(fd);
    return -1;
    }
    err = connect(fd, (struct sockaddr *)&addr, len);
    if (err < 0 && errno != EINPROGRESS) {
    close(fd);
    return -1;
    }
    return fd;
    }
#[no_mangle]
unsafe extern "C" fn read_icmp_errqueue(sockfd: c_int, expected_code: c_int, af: c_int) {
    static void read_icmp_errqueue(int sockfd, int expected_code, int af)
    {
    int expected_ee_type = (af == AF_INET) ? ICMP_DEST_UNREACH :
    ICMPV6_DEST_UNREACH;
    int expected_origin = (af == AF_INET) ? SO_EE_ORIGIN_ICMP :
    SO_EE_ORIGIN_ICMP6;
    let mut expected_level: c_int = (af == AF_INET) ? IPPROTO_IP : IPPROTO_IPV6;
    let mut expected_type: c_int = (af == AF_INET) ? IP_RECVERR : IPV6_RECVERR;
    struct sock_extended_err *sock_err;
    char ctrl_buf[512];
    struct msghdr msg = {
    .msg_control = ctrl_buf,
    .msg_controllen = sizeof(ctrl_buf),
    };
    struct pollfd pfd = {
    .fd = sockfd,
    .events = POLLERR,
    };
    struct cmsghdr *cm;
    ssize_t n;
    if (!ASSERT_GE(poll(&pfd, 1, TIMEOUT_MS), 1, "poll_errqueue"))
    return;
    n = recvmsg(sockfd, &msg, MSG_ERRQUEUE);
    if (!ASSERT_GE(n, 0, "recvmsg_errqueue"))
    return;
    cm = CMSG_FIRSTHDR(&msg);
    if (!ASSERT_NEQ(cm, core::ptr::null_mut(), "cm_firsthdr_null"))
    return;
    for (; cm; cm = CMSG_NXTHDR(&msg, cm)) {
    if (cm.cmsg_level != expected_level ||
    cm.cmsg_type != expected_type)
    continue;
    sock_err = (struct sock_extended_err *)CMSG_DATA(cm);
    if (!ASSERT_EQ(sock_err.ee_origin, expected_origin,
    "sock_err_origin"))
    return;
    if (!ASSERT_EQ(sock_err.ee_type, expected_ee_type,
    "sock_err_type_dest_unreach"))
    return;
    ASSERT_EQ(sock_err.ee_code, expected_code, "sock_err_code");
    return;
    }
    ASSERT_FAIL("no IP_RECVERR/IPV6_RECVERR control message found");
    }
#[no_mangle]
unsafe extern "C" fn valid_unreach_code(code: c_int, af: c_int) -> bool {
    static bool valid_unreach_code(int code, int af)
    {
    if (code < 0)
    return false;
    if (af == AF_INET)
    return code <= NR_ICMP_UNREACH && code != ICMP_FRAG_NEEDED;
    return code <= ICMPV6_REJECT_ROUTE;
    }
    static void trigger_prog_read_icmp_errqueue(struct icmp_send *skel, int code,
    int af, const char *ip)
    {
    let mut srv_fd: c_int = -1, client_fd = -1;
    int port;
    srv_fd = start_server(af, SOCK_STREAM, ip, 0, TIMEOUT_MS);
    if (!ASSERT_OK_FD(srv_fd, "start_server"))
    return;
    port = get_socket_local_port(srv_fd);
    if (!ASSERT_GE(port, 0, "get_socket_local_port")) {
    close(srv_fd);
    return;
    }
    skel.bss.server_port = ntohs(port);
    skel.bss.unreach_type = (af == AF_INET) ? ICMP_DEST_UNREACH :
    ICMPV6_DEST_UNREACH;
    skel.bss.unreach_code = code;
    skel.data.kfunc_ret = KFUNC_RET_UNSET;
    client_fd = connect_to_fd_nonblock(srv_fd);
    if (!ASSERT_OK_FD(client_fd, "client_connect_nonblock")) {
    close(srv_fd);
    return;
    }
    if (valid_unreach_code(code, af))
    read_icmp_errqueue(client_fd, code, af);
    close(client_fd);
    close(srv_fd);
    }
    static void run_icmp_test(struct icmp_send *skel, int af, const char *ip,
    int max_code)
    {
    for (int code = 0; code <= max_code; code++) {
    if (af == AF_INET && code == ICMP_FRAG_NEEDED)
    continue;
    trigger_prog_read_icmp_errqueue(skel, code, af, ip);
    ASSERT_EQ(skel.data.kfunc_ret, 0, "kfunc_ret");
    }
// Test invalid codes
    trigger_prog_read_icmp_errqueue(skel, -1, af, ip);
    ASSERT_EQ(skel.data.kfunc_ret, -EINVAL, "kfunc_ret");
    trigger_prog_read_icmp_errqueue(skel, max_code + 1, af, ip);
    ASSERT_EQ(skel.data.kfunc_ret, -EINVAL, "kfunc_ret");
    if (af == AF_INET) {
    trigger_prog_read_icmp_errqueue(skel, ICMP_FRAG_NEEDED, af, ip);
    ASSERT_EQ(skel.data.kfunc_ret, -EINVAL, "kfunc_ret");
    }
    }
#[no_mangle]
unsafe extern "C" fn run_icmp_no_route_test(skel: *mut icmp_send, af: c_int) {
    static void run_icmp_no_route_test(struct icmp_send *skel, int af)
    {
    union {
    struct ipv4_packet v4;
    struct ipv6_packet v6;
    } pkt;
    DECLARE_LIBBPF_OPTS(bpf_test_run_opts, opts,
    .data_in = &pkt,
    );
    int err;
    switch (af) {
    case AF_INET:
    pkt.v4 = pkt_v4;
    pkt.v4.iph.version = 4;
    pkt.v4.iph.daddr = htonl(INADDR_LOOPBACK);
    pkt.v4.tcp.dest = htons(80);
    opts.data_size_in = sizeof(pkt.v4);
    skel.bss.unreach_type = ICMP_DEST_UNREACH;
    break;
    case AF_INET6:
    pkt.v6 = pkt_v6;
    pkt.v6.iph.version = 6;
    pkt.v6.iph.daddr = in6addr_loopback;
    pkt.v6.tcp.dest = htons(80);
    opts.data_size_in = sizeof(pkt.v6);
    skel.bss.unreach_type = ICMPV6_DEST_UNREACH;
    break;
    default:
    ASSERT_FAIL("af_not_supported");
    return;
    }
    skel.bss.server_port = 80;
    skel.data.kfunc_ret = KFUNC_RET_UNSET;
    err = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.egress), &opts);
    if (!ASSERT_OK(err, "test_run"))
    return;
    ASSERT_EQ(skel.data.kfunc_ret, -ENETUNREACH, "kfunc_ret_no_route");
    }
#[no_mangle]
pub unsafe extern "C" fn test_icmp_send_unreach_cgroup() {
    void test_icmp_send_unreach_cgroup(void)
    {
    struct icmp_send *skel;
    let mut cgroup_fd: c_int = -1;
    skel = icmp_send__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    goto cleanup;
    cgroup_fd = test__join_cgroup("/icmp_send_unreach_cgroup");
    if (!ASSERT_OK_FD(cgroup_fd, "join_cgroup"))
    goto cleanup;
    skel.links.egress =
    bpf_program__attach_cgroup(skel.progs.egress, cgroup_fd);
    if (!ASSERT_OK_PTR(skel.links.egress, "prog_attach_cgroup"))
    goto cleanup;
    if (test__start_subtest("ipv4"))
    run_icmp_test(skel, AF_INET, "127.0.0.1", NR_ICMP_UNREACH);
    if (test__start_subtest("ipv6"))
    run_icmp_test(skel, AF_INET6, "::1", ICMPV6_REJECT_ROUTE);
    if (test__start_subtest("no_route_ipv4"))
    run_icmp_no_route_test(skel, AF_INET);
    if (test__start_subtest("no_route_ipv6"))
    run_icmp_no_route_test(skel, AF_INET6);
    cleanup:
    icmp_send__destroy(skel);
    if (cgroup_fd >= 0)
    close(cgroup_fd);
    }
#[no_mangle]
pub unsafe extern "C" fn test_icmp_send_unreach_recursion() {
    void test_icmp_send_unreach_recursion(void)
    {
    struct icmp_send *skel;
    let mut cgroup_fd: c_int = -1;
    int err;
    err = setup_cgroup_environment();
    if (!ASSERT_OK(err, "setup_cgroup_environment"))
    return;
    skel = icmp_send__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    goto cleanup;
    cgroup_fd = get_root_cgroup();
    if (!ASSERT_OK_FD(cgroup_fd, "get_root_cgroup"))
    goto cleanup;
    skel.data.target_pid = getpid();
    skel.links.recursion =
    bpf_program__attach_cgroup(skel.progs.recursion, cgroup_fd);
    if (!ASSERT_OK_PTR(skel.links.recursion, "prog_attach_cgroup"))
    goto cleanup;
    trigger_prog_read_icmp_errqueue(skel, ICMP_HOST_UNREACH, AF_INET,
    "127.0.0.1");
//
// Because there's recursion involved, the first call will return at
// index 1 since it will return the second, and the second call will
// return at index 0 since it will return the first.
//
    ASSERT_EQ(skel.bss.rec_count, 2, "rec_count");
    ASSERT_EQ(skel.data.rec_kfunc_rets[0], -EBUSY, "kfunc_rets[0]");
    ASSERT_EQ(skel.data.rec_kfunc_rets[1], 0, "kfunc_rets[1]");
    cleanup:
    icmp_send__destroy(skel);
    if (cgroup_fd >= 0)
    close(cgroup_fd);
    cleanup_cgroup_environment();
    }
