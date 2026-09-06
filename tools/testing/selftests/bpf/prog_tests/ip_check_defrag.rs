//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/ip_check_defrag.c
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
// This selftest spins up a client and an echo server, each in their own
// network namespace. The client will send a fragmented message to the server.
// The prog attached to the server will shoot down any fragments. Thus, if
// the server is able to correctly echo back the message to the client, we will
// have verified that netfilter is reassembling packets for us.
//
// Topology:
// =========
// NS0         |         NS1
// |
// client        |       server
// ----------      |     ----------
// |  veth0  | --------- |  veth1  |
// ----------    peer    ----------
// |
// |       with bpf
//

// The following constants must stay in sync with `generate_udp_fragments.py`

pub const CLIENT_PORT: c_int = 48878;
pub const SERVER_PORT: c_int = 48879;

#[no_mangle]
unsafe extern "C" fn setup_topology(ipv6: bool) -> c_int {
    static int setup_topology(bool ipv6)
    {
    bool up;
    int i;
    SYS(fail, "ip netns add " NS0);
    SYS(fail, "ip netns add " NS1);
    SYS(fail, "ip link add " VETH0 " netns " NS0 " type veth peer name " VETH1 " netns " NS1);
    if (ipv6) {
    SYS(fail, "ip -6 -net " NS0 " addr add " VETH0_ADDR6 "/64 dev " VETH0 " nodad");
    SYS(fail, "ip -6 -net " NS1 " addr add " VETH1_ADDR6 "/64 dev " VETH1 " nodad");
    } else {
    SYS(fail, "ip -net " NS0 " addr add " VETH0_ADDR "/24 dev " VETH0);
    SYS(fail, "ip -net " NS1 " addr add " VETH1_ADDR "/24 dev " VETH1);
    }
    SYS(fail, "ip -net " NS0 " link set dev " VETH0 " up");
    SYS(fail, "ip -net " NS1 " link set dev " VETH1 " up");
// Wait for up to 5s for links to come up
    for (i = 0; i < 5; ++i) {
    if (ipv6)
    up = !SYS_NOFAIL("ip netns exec " NS0 " ping -6 -c 1 -W 1 " VETH1_ADDR6);
    else
    up = !SYS_NOFAIL("ip netns exec " NS0 " ping -c 1 -W 1 " VETH1_ADDR);
    if (up)
    break;
    }
    return 0;
    fail:
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_topology() {
    static void cleanup_topology(void)
    {
    SYS_NOFAIL("test -f /var/run/netns/" NS0 " && ip netns delete " NS0);
    SYS_NOFAIL("test -f /var/run/netns/" NS1 " && ip netns delete " NS1);
    }
#[no_mangle]
unsafe extern "C" fn attach(skel: *mut ip_check_defrag, ipv6: bool) -> c_int {
    static int attach(struct ip_check_defrag *skel, bool ipv6)
    {
    LIBBPF_OPTS(bpf_netfilter_opts, opts,
    .pf = ipv6 ? NFPROTO_IPV6 : NFPROTO_IPV4,
    .priority = 42,
    .flags = BPF_F_NETFILTER_IP_DEFRAG);
    struct nstoken *nstoken;
    let mut err: c_int = -1;
    nstoken = open_netns(NS1);
    if (!ASSERT_OK_PTR(nstoken, "setns"))
    goto out;
    skel.links.defrag = bpf_program__attach_netfilter(skel.progs.defrag, &opts);
    if (!ASSERT_OK_PTR(skel.links.defrag, "program attach"))
    goto out;
    err = 0;
    out:
    close_netns(nstoken);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn send_frags(client: c_int) -> c_int {
    static int send_frags(int client)
    {
    struct sockaddr_storage saddr;
    struct sockaddr *saddr_p;
    socklen_t saddr_len;
    int err;
    saddr_p = (struct sockaddr *)&saddr;
    err = make_sockaddr(AF_INET, VETH1_ADDR, SERVER_PORT, &saddr, &saddr_len);
    if (!ASSERT_OK(err, "make_sockaddr"))
    return -1;
    err = sendto(client, frag_0, sizeof(frag_0), 0, saddr_p, saddr_len);
    if (!ASSERT_GE(err, 0, "sendto frag_0"))
    return -1;
    err = sendto(client, frag_1, sizeof(frag_1), 0, saddr_p, saddr_len);
    if (!ASSERT_GE(err, 0, "sendto frag_1"))
    return -1;
    err = sendto(client, frag_2, sizeof(frag_2), 0, saddr_p, saddr_len);
    if (!ASSERT_GE(err, 0, "sendto frag_2"))
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn send_frags6(client: c_int) -> c_int {
    static int send_frags6(int client)
    {
    struct sockaddr_storage saddr;
    struct sockaddr *saddr_p;
    socklen_t saddr_len;
    int err;
    saddr_p = (struct sockaddr *)&saddr;
// Port needs to be set to 0 for raw ipv6 socket for some reason
    err = make_sockaddr(AF_INET6, VETH1_ADDR6, 0, &saddr, &saddr_len);
    if (!ASSERT_OK(err, "make_sockaddr"))
    return -1;
    err = sendto(client, frag6_0, sizeof(frag6_0), 0, saddr_p, saddr_len);
    if (!ASSERT_GE(err, 0, "sendto frag6_0"))
    return -1;
    err = sendto(client, frag6_1, sizeof(frag6_1), 0, saddr_p, saddr_len);
    if (!ASSERT_GE(err, 0, "sendto frag6_1"))
    return -1;
    err = sendto(client, frag6_2, sizeof(frag6_2), 0, saddr_p, saddr_len);
    if (!ASSERT_GE(err, 0, "sendto frag6_2"))
    return -1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn test_bpf_ip_check_defrag_ok(ipv6: bool) {
    void test_bpf_ip_check_defrag_ok(bool ipv6)
    {
    let mut family: c_int = ipv6 ? AF_INET6 : AF_INET;
    struct network_helper_opts rx_opts = {
    .timeout_ms = 1000,
    };
    struct network_helper_opts tx_ops = {
    .timeout_ms = 1000,
    .proto = IPPROTO_RAW,
    };
    struct sockaddr_storage caddr;
    struct ip_check_defrag *skel;
    struct nstoken *nstoken;
    let mut client_tx_fd: c_int = -1;
    let mut client_rx_fd: c_int = -1;
    socklen_t caddr_len;
    let mut srv_fd: c_int = -1;
    char buf[1024];
    int len, err;
    skel = ip_check_defrag__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    if (!ASSERT_OK(setup_topology(ipv6), "setup_topology"))
    goto out;
    if (!ASSERT_OK(attach(skel, ipv6), "attach"))
    goto out;
// Start server in ns1
    nstoken = open_netns(NS1);
    if (!ASSERT_OK_PTR(nstoken, "setns ns1"))
    goto out;
    srv_fd = start_server(family, SOCK_DGRAM, core::ptr::null_mut(), SERVER_PORT, 0);
    close_netns(nstoken);
    if (!ASSERT_GE(srv_fd, 0, "start_server"))
    goto out;
// Open tx raw socket in ns0
    nstoken = open_netns(NS0);
    if (!ASSERT_OK_PTR(nstoken, "setns ns0"))
    goto out;
    client_tx_fd = client_socket(family, SOCK_RAW, &tx_ops);
    close_netns(nstoken);
    if (!ASSERT_GE(client_tx_fd, 0, "client_socket"))
    goto out;
// Open rx socket in ns0
    nstoken = open_netns(NS0);
    if (!ASSERT_OK_PTR(nstoken, "setns ns0"))
    goto out;
    client_rx_fd = client_socket(family, SOCK_DGRAM, &rx_opts);
    close_netns(nstoken);
    if (!ASSERT_GE(client_rx_fd, 0, "client_socket"))
    goto out;
// Bind rx socket to a premeditated port
    memset(&caddr, 0, sizeof(caddr));
    nstoken = open_netns(NS0);
    if (!ASSERT_OK_PTR(nstoken, "setns ns0"))
    goto out;
    if (ipv6) {
    struct sockaddr_in6 *c = (struct sockaddr_in6 *)&caddr;
    c.sin6_family = AF_INET6;
    inet_pton(AF_INET6, VETH0_ADDR6, &c.sin6_addr);
    c.sin6_port = htons(CLIENT_PORT);
    err = bind(client_rx_fd, (struct sockaddr *)c, sizeof(*c));
    } else {
    struct sockaddr_in *c = (struct sockaddr_in *)&caddr;
    c.sin_family = AF_INET;
    inet_pton(AF_INET, VETH0_ADDR, &c.sin_addr);
    c.sin_port = htons(CLIENT_PORT);
    err = bind(client_rx_fd, (struct sockaddr *)c, sizeof(*c));
    }
    close_netns(nstoken);
    if (!ASSERT_OK(err, "bind"))
    goto out;
// Send message in fragments
    if (ipv6) {
    if (!ASSERT_OK(send_frags6(client_tx_fd), "send_frags6"))
    goto out;
    } else {
    if (!ASSERT_OK(send_frags(client_tx_fd), "send_frags"))
    goto out;
    }
    if (!ASSERT_EQ(skel.bss.shootdowns, 0, "shootdowns"))
    goto out;
// Receive reassembled msg on server and echo back to client
    caddr_len = sizeof(caddr);
    len = recvfrom(srv_fd, buf, sizeof(buf), 0, (struct sockaddr *)&caddr, &caddr_len);
    if (!ASSERT_GE(len, 0, "server recvfrom"))
    goto out;
    len = sendto(srv_fd, buf, len, 0, (struct sockaddr *)&caddr, caddr_len);
    if (!ASSERT_GE(len, 0, "server sendto"))
    goto out;
// Expect reassembed message to be echoed back
    len = recvfrom(client_rx_fd, buf, sizeof(buf), 0, core::ptr::null_mut(), core::ptr::null_mut());
    if (!ASSERT_EQ(len, sizeof(MAGIC_MESSAGE) - 1, "client short read"))
    goto out;
    out:
    if (client_rx_fd != -1)
    close(client_rx_fd);
    if (client_tx_fd != -1)
    close(client_tx_fd);
    if (srv_fd != -1)
    close(srv_fd);
    cleanup_topology();
    ip_check_defrag__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_bpf_ip_check_defrag() {
    void test_bpf_ip_check_defrag(void)
    {
    if (test__start_subtest("v4"))
    test_bpf_ip_check_defrag_ok(false);
    if (test__start_subtest("v6"))
    test_bpf_ip_check_defrag_ok(true);
    }
