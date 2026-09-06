//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/vrf_socket_lookup.c
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
//
// Topology:
// ---------
// NS0 namespace         |   NS1 namespace
// |
// +--------------+      |   +--------------+
// |    veth01    |----------|    veth10    |
// | 172.16.1.100 |      |   | 172.16.1.200 |
// |     bpf      |      |   +--------------+
// +--------------+      |
// server(UDP/TCP)      |
// +-------------------+    |
// |        vrf1       |    |
// |  +--------------+ |    |   +--------------+
// |  |    veth02    |----------|    veth20    |
// |  | 172.16.2.100 | |    |   | 172.16.2.200 |
// |  |     bpf      | |    |   +--------------+
// |  +--------------+ |    |
// |   server(UDP/TCP) |    |
// +-------------------+    |
//
// Test flow
// -----------
// The tests verifies that socket lookup via TC is VRF aware:
// 1) Creates two veth pairs between NS0 and NS1:
// a) veth01 <-> veth10 outside the VRF
// b) veth02 <-> veth20 in the VRF
// 2) Attaches to veth01 and veth02 a program that calls:
// a) bpf_skc_lookup_tcp() with TCP and tcp_skc is true
// b) bpf_sk_lookup_tcp() with TCP and tcp_skc is false
// c) bpf_sk_lookup_udp() with UDP
// The program stores the lookup result in bss->lookup_status.
// 3) Creates a socket TCP/UDP server in/outside the VRF.
// 4) The test expects lookup_status to be:
// a) 0 from device in VRF to server outside VRF
// b) 0 from device outside VRF to server in VRF
// c) 1 from device in VRF to server in VRF
// d) 1 from device outside VRF to server outside VRF
//

pub const NON_VRF_PORT: c_int = 5000;
pub const IN_VRF_PORT: c_int = 5001;
pub const TIMEOUT_MS: c_int = 3000;
    static int make_socket(int sotype, const char *ip, int port,
    struct sockaddr_storage *addr)
    {
    int err, fd;
    err = make_sockaddr(AF_INET, ip, port, addr, core::ptr::null_mut());
    if (!ASSERT_OK(err, "make_address"))
    return -1;
    fd = socket(AF_INET, sotype, 0);
    if (!ASSERT_GE(fd, 0, "socket"))
    return -1;
    if (!ASSERT_OK(settimeo(fd, TIMEOUT_MS), "settimeo"))
    goto fail;
    return fd;
    fail:
    close(fd);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn make_server(sotype: c_int, ip: *const c_char, port: c_int, ifname: *const c_char) -> c_int {
    static int make_server(int sotype, const char *ip, int port, const char *ifname)
    {
    int err, fd = -1;
    fd = start_server(AF_INET, sotype, ip, port, TIMEOUT_MS);
    if (!ASSERT_GE(fd, 0, "start_server"))
    return -1;
    if (ifname) {
    err = setsockopt(fd, SOL_SOCKET, SO_BINDTODEVICE,
    ifname, strlen(ifname) + 1);
    if (!ASSERT_OK(err, "setsockopt(SO_BINDTODEVICE)"))
    goto fail;
    }
    return fd;
    fail:
    close(fd);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn attach_progs(ifname: *mut c_char, tc_prog_fd: c_int, xdp_prog_fd: c_int) -> c_int {
    static int attach_progs(char *ifname, int tc_prog_fd, int xdp_prog_fd)
    {
    LIBBPF_OPTS(bpf_tc_hook, hook, .attach_point = BPF_TC_INGRESS);
    LIBBPF_OPTS(bpf_tc_opts, opts, .handle = 1, .priority = 1,
    .prog_fd = tc_prog_fd);
    int ret, ifindex;
    ifindex = if_nametoindex(ifname);
    if (!ASSERT_NEQ(ifindex, 0, "if_nametoindex"))
    return -1;
    hook.ifindex = ifindex;
    ret = bpf_tc_hook_create(&hook);
    if (!ASSERT_OK(ret, "bpf_tc_hook_create"))
    return ret;
    ret = bpf_tc_attach(&hook, &opts);
    if (!ASSERT_OK(ret, "bpf_tc_attach")) {
    bpf_tc_hook_destroy(&hook);
    return ret;
    }
    ret = bpf_xdp_attach(ifindex, xdp_prog_fd, 0, core::ptr::null_mut());
    if (!ASSERT_OK(ret, "bpf_xdp_attach")) {
    bpf_tc_hook_destroy(&hook);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cleanup() {
    static void cleanup(void)
    {
    SYS_NOFAIL("test -f /var/run/netns/" NS0 " && ip netns delete "
    NS0);
    SYS_NOFAIL("test -f /var/run/netns/" NS1 " && ip netns delete "
    NS1);
    }
#[no_mangle]
unsafe extern "C" fn setup(skel: *mut vrf_socket_lookup) -> c_int {
    static int setup(struct vrf_socket_lookup *skel)
    {
    int tc_prog_fd, xdp_prog_fd, ret = 0;
    struct nstoken *nstoken = core::ptr::null_mut();
    SYS(fail, "ip netns add " NS0);
    SYS(fail, "ip netns add " NS1);
// NS0 <-> NS1 [veth01 <-> veth10]
    SYS(fail, "ip link add veth01 netns " NS0 " type veth peer name veth10"
    " netns " NS1);
    SYS(fail, "ip -net " NS0 " addr add " IP4_ADDR_VETH01 "/24 dev veth01");
    SYS(fail, "ip -net " NS0 " link set dev veth01 up");
    SYS(fail, "ip -net " NS1 " addr add " IP4_ADDR_VETH10 "/24 dev veth10");
    SYS(fail, "ip -net " NS1 " link set dev veth10 up");
// NS0 <-> NS1 [veth02 <-> veth20]
    SYS(fail, "ip link add veth02 netns " NS0 " type veth peer name veth20"
    " netns " NS1);
    SYS(fail, "ip -net " NS0 " addr add " IP4_ADDR_VETH02 "/24 dev veth02");
    SYS(fail, "ip -net " NS0 " link set dev veth02 up");
    SYS(fail, "ip -net " NS1 " addr add " IP4_ADDR_VETH20 "/24 dev veth20");
    SYS(fail, "ip -net " NS1 " link set dev veth20 up");
// veth02 -> vrf1
    SYS(fail, "ip -net " NS0 " link add vrf1 type vrf table 11");
    SYS(fail, "ip -net " NS0 " route add vrf vrf1 unreachable default"
    " metric 4278198272");
    SYS(fail, "ip -net " NS0 " link set vrf1 alias vrf");
    SYS(fail, "ip -net " NS0 " link set vrf1 up");
    SYS(fail, "ip -net " NS0 " link set veth02 master vrf1");
// Attach TC and XDP progs to veth devices in NS0
    nstoken = open_netns(NS0);
    if (!ASSERT_OK_PTR(nstoken, "setns " NS0))
    goto fail;
    tc_prog_fd = bpf_program__fd(skel.progs.tc_socket_lookup);
    if (!ASSERT_GE(tc_prog_fd, 0, "bpf_program__tc_fd"))
    goto fail;
    xdp_prog_fd = bpf_program__fd(skel.progs.xdp_socket_lookup);
    if (!ASSERT_GE(xdp_prog_fd, 0, "bpf_program__xdp_fd"))
    goto fail;
    if (attach_progs("veth01", tc_prog_fd, xdp_prog_fd))
    goto fail;
    if (attach_progs("veth02", tc_prog_fd, xdp_prog_fd))
    goto fail;
    goto close;
    fail:
    ret = -1;
    close:
    if (nstoken)
    close_netns(nstoken);
    return ret;
    }
    static int test_lookup(struct vrf_socket_lookup *skel, int sotype,
    const char *ip, int port, bool test_xdp, bool tcp_skc,
    int lookup_status_exp)
    {
    static const char msg[] = "Hello Server";
    let mut addr: sockaddr_storage = {};
    int fd, ret = 0;
    fd = make_socket(sotype, ip, port, &addr);
    if (fd < 0)
    return -1;
    skel.bss.test_xdp = test_xdp;
    skel.bss.tcp_skc = tcp_skc;
    skel.bss.lookup_status = -1;
    if (sotype == SOCK_STREAM)
    connect(fd, (void *)&addr, sizeof(struct sockaddr_in));
    else
    sendto(fd, msg, sizeof(msg), 0, (void *)&addr,
    sizeof(struct sockaddr_in));
    if (!ASSERT_EQ(skel.bss.lookup_status, lookup_status_exp,
    "lookup_status"))
    goto fail;
    goto close;
    fail:
    ret = -1;
    close:
    close(fd);
    return ret;
    }
    static void _test_vrf_socket_lookup(struct vrf_socket_lookup *skel, int sotype,
    bool test_xdp, bool tcp_skc)
    {
    let mut in_vrf_server: c_int = -1, non_vrf_server = -1;
    struct nstoken *nstoken = core::ptr::null_mut();
    nstoken = open_netns(NS0);
    if (!ASSERT_OK_PTR(nstoken, "setns " NS0))
    goto done;
// Open sockets in and outside VRF
    non_vrf_server = make_server(sotype, "0.0.0.0", NON_VRF_PORT, core::ptr::null_mut());
    if (!ASSERT_GE(non_vrf_server, 0, "make_server__outside_vrf_fd"))
    goto done;
    in_vrf_server = make_server(sotype, "0.0.0.0", IN_VRF_PORT, "veth02");
    if (!ASSERT_GE(in_vrf_server, 0, "make_server__in_vrf_fd"))
    goto done;
// Perform test from NS1
    close_netns(nstoken);
    nstoken = open_netns(NS1);
    if (!ASSERT_OK_PTR(nstoken, "setns " NS1))
    goto done;
    if (!ASSERT_OK(test_lookup(skel, sotype, IP4_ADDR_VETH02, NON_VRF_PORT,
    test_xdp, tcp_skc, 0), "in_to_out"))
    goto done;
    if (!ASSERT_OK(test_lookup(skel, sotype, IP4_ADDR_VETH02, IN_VRF_PORT,
    test_xdp, tcp_skc, 1), "in_to_in"))
    goto done;
    if (!ASSERT_OK(test_lookup(skel, sotype, IP4_ADDR_VETH01, NON_VRF_PORT,
    test_xdp, tcp_skc, 1), "out_to_out"))
    goto done;
    if (!ASSERT_OK(test_lookup(skel, sotype, IP4_ADDR_VETH01, IN_VRF_PORT,
    test_xdp, tcp_skc, 0), "out_to_in"))
    goto done;
    done:
    if (non_vrf_server >= 0)
    close(non_vrf_server);
    if (in_vrf_server >= 0)
    close(in_vrf_server);
    if (nstoken)
    close_netns(nstoken);
    }
#[no_mangle]
pub unsafe extern "C" fn test_vrf_socket_lookup() {
    void test_vrf_socket_lookup(void)
    {
    struct vrf_socket_lookup *skel;
    cleanup();
    skel = vrf_socket_lookup__open_and_load();
    if (!ASSERT_OK_PTR(skel, "vrf_socket_lookup__open_and_load"))
    return;
    if (!ASSERT_OK(setup(skel), "setup"))
    goto done;
    if (test__start_subtest("tc_socket_lookup_tcp"))
    _test_vrf_socket_lookup(skel, SOCK_STREAM, false, false);
    if (test__start_subtest("tc_socket_lookup_tcp_skc"))
    _test_vrf_socket_lookup(skel, SOCK_STREAM, false, false);
    if (test__start_subtest("tc_socket_lookup_udp"))
    _test_vrf_socket_lookup(skel, SOCK_STREAM, false, false);
    if (test__start_subtest("xdp_socket_lookup_tcp"))
    _test_vrf_socket_lookup(skel, SOCK_STREAM, true, false);
    if (test__start_subtest("xdp_socket_lookup_tcp_skc"))
    _test_vrf_socket_lookup(skel, SOCK_STREAM, true, false);
    if (test__start_subtest("xdp_socket_lookup_udp"))
    _test_vrf_socket_lookup(skel, SOCK_STREAM, true, false);
    done:
    vrf_socket_lookup__destroy(skel);
    cleanup();
    }
