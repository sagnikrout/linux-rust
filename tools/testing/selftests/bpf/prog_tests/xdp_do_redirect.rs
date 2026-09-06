//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/xdp_do_redirect.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_packet {
    pub eth: ethhdr,
    pub iph: ipv6hdr,
    pub udp: udphdr,
    __u8 payload[64 - sizeof(struct udphdr)
    pub ipv6hdr)]: - sizeof(struct ethhdr) - sizeof(struct,
    pub __packed: },
    static struct udp_packet pkt_udp = {
    .eth.h_proto = __bpf_constant_htons(ETH_P_IPV6),
    .eth.h_dest = {0x00, 0x11, 0x22, 0x33, 0x44, 0x55},
    .eth.h_source = {0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb},
    .iph.version = 6,
    .iph.nexthdr = IPPROTO_UDP,
    .iph.payload_len = bpf_htons(sizeof(struct udp_packet)
    - offsetof(struct udp_packet, udp)),
    .iph.hop_limit = 2,
    .iph.saddr.s6_addr16 = {bpf_htons(0xfc00), 0, 0, 0, 0, 0, 0, bpf_htons(1)},
    .iph.daddr.s6_addr16 = {bpf_htons(0xfc00), 0, 0, 0, 0, 0, 0, bpf_htons(2)},
    .udp.source = bpf_htons(1),
    .udp.dest = bpf_htons(1),
    .udp.len = bpf_htons(sizeof(struct udp_packet)
    - offsetof(struct udp_packet, udp)),
    .payload = {0x42}, /* receiver XDP program matches on this */
}

#[no_mangle]
unsafe extern "C" fn attach_tc_prog(hook: *mut bpf_tc_hook, fd: c_int) -> c_int {
    static int attach_tc_prog(struct bpf_tc_hook *hook, int fd)
    {
    DECLARE_LIBBPF_OPTS(bpf_tc_opts, opts, .handle = 1, .priority = 1, .prog_fd = fd);
    int ret;
    ret = bpf_tc_hook_create(hook);
    if (!ASSERT_OK(ret, "create tc hook"))
    return ret;
    ret = bpf_tc_attach(hook, &opts);
    if (!ASSERT_OK(ret, "bpf_tc_attach")) {
    bpf_tc_hook_destroy(hook);
    return ret;
    }
    return 0;
    }
// The maximum permissible size is: PAGE_SIZE - sizeof(struct xdp_page_head) -
// SKB_DATA_ALIGN(sizeof(struct skb_shared_info)) - XDP_PACKET_HEADROOM =
// 3408 bytes for 64-byte cacheline and 3216 for 256-byte one.
//

pub const MAX_PKT_SIZE: c_int = 3216;

pub const MAX_PKT_SIZE: c_int = 3408;

pub const PAGE_SIZE_4K: c_int = 4096;
pub const PAGE_SIZE_64K: c_int = 65536;
#[no_mangle]
unsafe extern "C" fn test_max_pkt_size(fd: c_int) {
    static void test_max_pkt_size(int fd)
    {
    char data[PAGE_SIZE_64K + 1] = {};
    int err;
    DECLARE_LIBBPF_OPTS(bpf_test_run_opts, opts,
    .data_in = &data,
    .flags = BPF_F_TEST_XDP_LIVE_FRAMES,
    .repeat = 1,
    );
    if (getpagesize() == PAGE_SIZE_64K)
    opts.data_size_in = MAX_PKT_SIZE + PAGE_SIZE_64K - PAGE_SIZE_4K;
    else
    opts.data_size_in = MAX_PKT_SIZE;
    err = bpf_prog_test_run_opts(fd, &opts);
    ASSERT_OK(err, "prog_run_max_size");
    opts.data_size_in += 1;
    err = bpf_prog_test_run_opts(fd, &opts);
    ASSERT_EQ(err, -EINVAL, "prog_run_too_big");
    }
pub const NUM_PKTS: c_int = 10000;
#[no_mangle]
pub unsafe extern "C" fn test_xdp_do_redirect() {
    void test_xdp_do_redirect(void)
    {
    int err, xdp_prog_fd, tc_prog_fd, ifindex_src, ifindex_dst;
    char data[sizeof(pkt_udp) + sizeof(__u64)];
    struct test_xdp_do_redirect *skel = core::ptr::null_mut();
    struct nstoken *nstoken = core::ptr::null_mut();
    struct bpf_link *link;
    LIBBPF_OPTS(bpf_xdp_query_opts, query_opts);
    struct xdp_md ctx_in = { .data = sizeof(__u64),
    .data_end = sizeof(data) };
    DECLARE_LIBBPF_OPTS(bpf_test_run_opts, opts,
    .data_in = &data,
    .data_size_in = sizeof(data),
    .ctx_in = &ctx_in,
    .ctx_size_in = sizeof(ctx_in),
    .flags = BPF_F_TEST_XDP_LIVE_FRAMES,
    .repeat = NUM_PKTS,
    .batch_size = 64,
    );
    DECLARE_LIBBPF_OPTS(bpf_tc_hook, tc_hook,
    .attach_point = BPF_TC_INGRESS);
    memcpy(&data[sizeof(__u64)], &pkt_udp, sizeof(pkt_udp));
    ((__u32 *)data)[0] = 0x42; /* metadata test value */
    ((__u32 *)data)[1] = 0;
    skel = test_xdp_do_redirect__open();
    if (!ASSERT_OK_PTR(skel, "skel"))
    return;
// The XDP program we run with bpf_prog_run() will cycle through all
// three xmit (PASS/TX/REDIRECT) return codes starting from above, and
// ending up with PASS, so we should end up with two packets on the dst
// iface and NUM_PKTS-2 in the TC hook. We match the packets on the UDP
// payload.
//
    SYS(out, "ip netns add testns");
    nstoken = open_netns("testns");
    if (!ASSERT_OK_PTR(nstoken, "setns"))
    goto out;
    SYS(out, "ip link add veth_src type veth peer name veth_dst");
    SYS(out, "ip link set dev veth_src address 00:11:22:33:44:55");
    SYS(out, "ip link set dev veth_dst address 66:77:88:99:aa:bb");
    SYS(out, "ip link set dev veth_src up");
    SYS(out, "ip link set dev veth_dst up");
    SYS(out, "ip addr add dev veth_src fc00::1/64");
    SYS(out, "ip addr add dev veth_dst fc00::2/64");
    SYS(out, "ip neigh add fc00::2 dev veth_src lladdr 66:77:88:99:aa:bb");
// We enable forwarding in the test namespace because that will cause
// the packets that go through the kernel stack (with XDP_PASS) to be
// forwarded back out the same interface (because of the packet dst
// combined with the interface addresses). When this happens, the
// regular forwarding path will end up going through the same
// veth_xdp_xmit() call as the XDP_REDIRECT code, which can cause a
// deadlock if it happens on the same CPU. There's a local_bh_disable()
// in the test_run code to prevent this, but an earlier version of the
// code didn't have this, so we keep the test behaviour to make sure the
// bug doesn't resurface.
//
    SYS(out, "sysctl -qw net.ipv6.conf.all.forwarding=1");
    ifindex_src = if_nametoindex("veth_src");
    ifindex_dst = if_nametoindex("veth_dst");
    if (!ASSERT_NEQ(ifindex_src, 0, "ifindex_src") ||
    !ASSERT_NEQ(ifindex_dst, 0, "ifindex_dst"))
    goto out;
// Check xdp features supported by veth driver
    err = bpf_xdp_query(ifindex_src, XDP_FLAGS_DRV_MODE, &query_opts);
    if (!ASSERT_OK(err, "veth_src bpf_xdp_query"))
    goto out;
    if (!ASSERT_EQ(query_opts.feature_flags,
    NETDEV_XDP_ACT_BASIC | NETDEV_XDP_ACT_REDIRECT |
    NETDEV_XDP_ACT_RX_SG,
    "veth_src query_opts.feature_flags"))
    goto out;
    err = bpf_xdp_query(ifindex_dst, XDP_FLAGS_DRV_MODE, &query_opts);
    if (!ASSERT_OK(err, "veth_dst bpf_xdp_query"))
    goto out;
    if (!ASSERT_EQ(query_opts.feature_flags,
    NETDEV_XDP_ACT_BASIC | NETDEV_XDP_ACT_REDIRECT |
    NETDEV_XDP_ACT_RX_SG,
    "veth_dst query_opts.feature_flags"))
    goto out;
// Enable GRO
    SYS(out, "ethtool -K veth_src gro on");
    SYS(out, "ethtool -K veth_dst gro on");
    err = bpf_xdp_query(ifindex_src, XDP_FLAGS_DRV_MODE, &query_opts);
    if (!ASSERT_OK(err, "veth_src bpf_xdp_query gro on"))
    goto out;
    if (!ASSERT_EQ(query_opts.feature_flags,
    NETDEV_XDP_ACT_BASIC | NETDEV_XDP_ACT_REDIRECT |
    NETDEV_XDP_ACT_NDO_XMIT | NETDEV_XDP_ACT_RX_SG |
    NETDEV_XDP_ACT_NDO_XMIT_SG,
    "veth_src query_opts.feature_flags gro on"))
    goto out;
    err = bpf_xdp_query(ifindex_dst, XDP_FLAGS_DRV_MODE, &query_opts);
    if (!ASSERT_OK(err, "veth_dst bpf_xdp_query gro on"))
    goto out;
    if (!ASSERT_EQ(query_opts.feature_flags,
    NETDEV_XDP_ACT_BASIC | NETDEV_XDP_ACT_REDIRECT |
    NETDEV_XDP_ACT_NDO_XMIT | NETDEV_XDP_ACT_RX_SG |
    NETDEV_XDP_ACT_NDO_XMIT_SG,
    "veth_dst query_opts.feature_flags gro on"))
    goto out;
    memcpy(skel.rodata.expect_dst, &pkt_udp.eth.h_dest, ETH_ALEN);
    skel.rodata.ifindex_out = ifindex_src; /* redirect back to the same iface */
    skel.rodata.ifindex_in = ifindex_src;
    ctx_in.ingress_ifindex = ifindex_src;
    tc_hook.ifindex = ifindex_src;
    if (!ASSERT_OK(test_xdp_do_redirect__load(skel), "load"))
    goto out;
    link = bpf_program__attach_xdp(skel.progs.xdp_count_pkts, ifindex_dst);
    if (!ASSERT_OK_PTR(link, "prog_attach"))
    goto out;
    skel.links.xdp_count_pkts = link;
    tc_prog_fd = bpf_program__fd(skel.progs.tc_count_pkts);
    if (attach_tc_prog(&tc_hook, tc_prog_fd))
    goto out;
    xdp_prog_fd = bpf_program__fd(skel.progs.xdp_redirect);
    err = bpf_prog_test_run_opts(xdp_prog_fd, &opts);
    if (!ASSERT_OK(err, "prog_run"))
    goto out_tc;
// wait for the packets to be flushed
    kern_sync_rcu();
// There will be one packet sent through XDP_REDIRECT and one through
// XDP_TX; these will show up on the XDP counting program, while the
// rest will be counted at the TC ingress hook (and the counting program
// resets the packet payload so they don't get counted twice even though
// they are re-xmited out the veth device
//
    ASSERT_EQ(skel.bss.pkts_seen_xdp, 2, "pkt_count_xdp");
    ASSERT_EQ(skel.bss.pkts_seen_zero, 2, "pkt_count_zero");
    ASSERT_EQ(skel.bss.pkts_seen_tc, NUM_PKTS - 2, "pkt_count_tc");
    test_max_pkt_size(bpf_program__fd(skel.progs.xdp_count_pkts));
    out_tc:
    bpf_tc_hook_destroy(&tc_hook);
    out:
    if (nstoken)
    close_netns(nstoken);
    SYS_NOFAIL("ip netns del testns");
    test_xdp_do_redirect__destroy(skel);
    }
pub const NS_NB: c_int = 3;

pub const VETH1_INDEX: c_int = 111;
pub const VETH2_INDEX: c_int = 222;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_data {
    pub ns: [*mut netns_obj; NS_NB],
    pub xdp_flags: u32,
}

#[no_mangle]
unsafe extern "C" fn cleanup(data: *mut test_data) {
    static void cleanup(struct test_data *data)
    {
    int i;
    for (i = 0; i < NS_NB; i++)
    netns_free(data.ns[i]);
    }
//
// ping_setup -
// Create two veth peers and forward packets in-between using XDP
//
// ------------           ------------
// |    NS1   |           |    NS2   |
// |   veth0  |           |   veth0  |
// | 10.1.1.1 |           | 10.1.1.2 |
// -----|------           ------|-----
// |                       |
// -----|-----------------------|-------
// |  veth1                   veth2    |
// | (id:111)                (id:222)  |
// |    |                        |     |
// |    ----- xdp forwarding -----     |
// |                                   |
// |               NS0                 |
// -------------------------------------
//
#[no_mangle]
unsafe extern "C" fn ping_setup(data: *mut test_data) -> c_int {
    static int ping_setup(struct test_data *data)
    {
    int i;
    data.ns[0] = netns_new(NS0, false);
    if (!ASSERT_OK_PTR(data.ns[0], "create ns"))
    return -1;
    for (i = 1; i < NS_NB; i++) {
    char ns_name[4] = {};
    snprintf(ns_name, 4, "NS%d", i);
    data.ns[i] = netns_new(ns_name, false);
    if (!ASSERT_OK_PTR(data.ns[i], "create ns"))
    goto fail;
    SYS(fail,
    "ip -n %s link add veth%d index %d%d%d type veth peer name veth0 netns %s",
    NS0, i, i, i, i, ns_name);
    SYS(fail, "ip -n %s link set veth%d up", NS0, i);
    SYS(fail, "ip -n %s addr add %s.%d/24 dev veth0", ns_name, IPV4_NETWORK, i);
    SYS(fail, "ip -n %s link set veth0 up", ns_name);
    }
    return 0;
    fail:
    cleanup(data);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn ping_test(data: *mut test_data) {
    static void ping_test(struct test_data *data)
    {
    struct test_xdp_do_redirect *skel = core::ptr::null_mut();
    struct xdp_dummy *skel_dummy = core::ptr::null_mut();
    struct nstoken *nstoken = core::ptr::null_mut();
    int i, ret;
    skel_dummy = xdp_dummy__open_and_load();
    if (!ASSERT_OK_PTR(skel_dummy, "open and load xdp_dummy skeleton"))
    goto close;
    for (i = 1; i < NS_NB; i++) {
    char ns_name[4] = {};
    snprintf(ns_name, 4, "NS%d", i);
    nstoken = open_netns(ns_name);
    if (!ASSERT_OK_PTR(nstoken, "open ns"))
    goto close;
    ret = bpf_xdp_attach(if_nametoindex("veth0"),
    bpf_program__fd(skel_dummy.progs.xdp_dummy_prog),
    data.xdp_flags, core::ptr::null_mut());
    if (!ASSERT_GE(ret, 0, "bpf_xdp_attach dummy_prog"))
    goto close;
    close_netns(nstoken);
    nstoken = core::ptr::null_mut();
    }
    skel = test_xdp_do_redirect__open_and_load();
    if (!ASSERT_OK_PTR(skel, "open and load skeleton"))
    goto close;
    nstoken = open_netns(NS0);
    if (!ASSERT_OK_PTR(nstoken, "open NS0"))
    goto close;
    ret = bpf_xdp_attach(VETH2_INDEX,
    bpf_program__fd(skel.progs.xdp_redirect_to_111),
    data.xdp_flags, core::ptr::null_mut());
    if (!ASSERT_GE(ret, 0, "bpf_xdp_attach"))
    goto close;
    ret = bpf_xdp_attach(VETH1_INDEX,
    bpf_program__fd(skel.progs.xdp_redirect_to_222),
    data.xdp_flags, core::ptr::null_mut());
    if (!ASSERT_GE(ret, 0, "bpf_xdp_attach"))
    goto close;
    close_netns(nstoken);
    nstoken = core::ptr::null_mut();
    nstoken = open_netns(NS1);
    if (!ASSERT_OK_PTR(nstoken, "open NS1"))
    goto close;
    SYS(close, "ping -c 1 %s.2 > /dev/null", IPV4_NETWORK);
    close:
    close_netns(nstoken);
    xdp_dummy__destroy(skel_dummy);
    test_xdp_do_redirect__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn xdp_redirect_ping(xdp_flags: u32) {
    static void xdp_redirect_ping(u32 xdp_flags)
    {
    let mut data: test_data = {};
    if (ping_setup(&data) < 0)
    return;
    data.xdp_flags = xdp_flags;
    ping_test(&data);
    cleanup(&data);
    }
#[no_mangle]
pub unsafe extern "C" fn test_xdp_index_redirect() {
    void test_xdp_index_redirect(void)
    {
    if (test__start_subtest("noflag"))
    xdp_redirect_ping(0);
    if (test__start_subtest("drvflag"))
    xdp_redirect_ping(XDP_FLAGS_DRV_MODE);
    if (test__start_subtest("skbflag"))
    xdp_redirect_ping(XDP_FLAGS_SKB_MODE);
    }
