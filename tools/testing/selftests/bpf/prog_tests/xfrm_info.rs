//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/xfrm_info.c
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
// NS0 namespace         |   NS1 namespace        | NS2 namespace
// |                        |
// +---------------+     |   +---------------+    |
// |    ipsec0     |---------|    ipsec0     |    |
// | 192.168.1.100 |     |   | 192.168.1.200 |    |
// | if_id: bpf    |     |   +---------------+    |
// +---------------+     |                        |
// |             |                        |   +---------------+
// |             |                        |   |    ipsec0     |
// \------------------------------------------| 192.168.1.200 |
// |                        |   +---------------+
// |                        |
// |                        | (overlay network)
// ------------------------------------------------------
// |                        | (underlay network)
// +--------------+      |   +--------------+     |
// |    veth01    |----------|    veth10    |     |
// | 172.16.1.100 |      |   | 172.16.1.200 |     |
// ---------------+      |   +--------------+     |
// |                        |
// +--------------+      |                        |   +--------------+
// |    veth02    |-----------------------------------|    veth20    |
// | 172.16.2.100 |      |                        |   | 172.16.2.200 |
// +--------------+      |                        |   +--------------+
//
// Test Packet flow
// -----------
// The tests perform 'ping 192.168.1.200' from the NS0 namespace:
// 1) request is routed to NS0 ipsec0
// 2) NS0 ipsec0 tc egress BPF program is triggered and sets the if_id based
// on the requested value. This makes the ipsec0 device in external mode
// select the destination tunnel
// 3) ping reaches the other namespace (NS1 or NS2 based on which if_id was
// used) and response is sent
// 4) response is received on NS0 ipsec0, tc ingress program is triggered and
// records the response if_id
// 5) requested if_id is compared with received if_id
//

pub const IF_ID_0_TO_1: c_int = 1;
pub const IF_ID_0_TO_2: c_int = 2;
pub const IF_ID_1: c_int = 3;
pub const IF_ID_2: c_int = 4;

    "proto esp aead 'rfc4106(gcm(aes))' " \
    "0xe4d8f4b4da1df18a3510b3781496daa82488b713 128 mode tunnel "
#[no_mangle]
unsafe extern "C" fn attach_tc_prog(hook: *mut bpf_tc_hook, igr_fd: c_int, egr_fd: c_int) -> c_int {
    static int attach_tc_prog(struct bpf_tc_hook *hook, int igr_fd, int egr_fd)
    {
    LIBBPF_OPTS(bpf_tc_opts, opts1, .handle = 1, .priority = 1,
    .prog_fd = igr_fd);
    LIBBPF_OPTS(bpf_tc_opts, opts2, .handle = 1, .priority = 1,
    .prog_fd = egr_fd);
    int ret;
    ret = bpf_tc_hook_create(hook);
    if (!ASSERT_OK(ret, "create tc hook"))
    return ret;
    if (igr_fd >= 0) {
    hook.attach_point = BPF_TC_INGRESS;
    ret = bpf_tc_attach(hook, &opts1);
    if (!ASSERT_OK(ret, "bpf_tc_attach")) {
    bpf_tc_hook_destroy(hook);
    return ret;
    }
    }
    if (egr_fd >= 0) {
    hook.attach_point = BPF_TC_EGRESS;
    ret = bpf_tc_attach(hook, &opts2);
    if (!ASSERT_OK(ret, "bpf_tc_attach")) {
    bpf_tc_hook_destroy(hook);
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cleanup() {
    static void cleanup(void)
    {
    SYS_NOFAIL("test -f /var/run/netns/" NS0 " && ip netns delete " NS0);
    SYS_NOFAIL("test -f /var/run/netns/" NS1 " && ip netns delete " NS1);
    SYS_NOFAIL("test -f /var/run/netns/" NS2 " && ip netns delete " NS2);
    }
#[no_mangle]
unsafe extern "C" fn config_underlay() -> c_int {
    static int config_underlay(void)
    {
    SYS(fail, "ip netns add " NS0);
    SYS(fail, "ip netns add " NS1);
    SYS(fail, "ip netns add " NS2);
// NS0 <-> NS1 [veth01 <-> veth10]
    SYS(fail, "ip link add veth01 netns " NS0 " type veth peer name veth10 netns " NS1);
    SYS(fail, "ip -net " NS0 " addr add " IP4_ADDR_VETH01 "/24 dev veth01");
    SYS(fail, "ip -net " NS0 " link set dev veth01 up");
    SYS(fail, "ip -net " NS1 " addr add " IP4_ADDR_VETH10 "/24 dev veth10");
    SYS(fail, "ip -net " NS1 " link set dev veth10 up");
// NS0 <-> NS2 [veth02 <-> veth20]
    SYS(fail, "ip link add veth02 netns " NS0 " type veth peer name veth20 netns " NS2);
    SYS(fail, "ip -net " NS0 " addr add " IP4_ADDR_VETH02 "/24 dev veth02");
    SYS(fail, "ip -net " NS0 " link set dev veth02 up");
    SYS(fail, "ip -net " NS2 " addr add " IP4_ADDR_VETH20 "/24 dev veth20");
    SYS(fail, "ip -net " NS2 " link set dev veth20 up");
    return 0;
    fail:
    return -1;
    }
    static int setup_xfrm_tunnel_ns(const char *ns, const char *ipv4_local,
    const char *ipv4_remote, int if_id)
    {
// State: local -> remote
    SYS(fail, "ip -net %s xfrm state add src %s dst %s spi 1 "
    ESP_DUMMY_PARAMS "if_id %d", ns, ipv4_local, ipv4_remote, if_id);
// State: local <- remote
    SYS(fail, "ip -net %s xfrm state add src %s dst %s spi 1 "
    ESP_DUMMY_PARAMS "if_id %d", ns, ipv4_remote, ipv4_local, if_id);
// Policy: local -> remote
    SYS(fail, "ip -net %s xfrm policy add dir out src 0.0.0.0/0 dst 0.0.0.0/0 "
    "if_id %d tmpl src %s dst %s proto esp mode tunnel if_id %d", ns,
    if_id, ipv4_local, ipv4_remote, if_id);
// Policy: local <- remote
    SYS(fail, "ip -net %s xfrm policy add dir in src 0.0.0.0/0 dst 0.0.0.0/0 "
    "if_id %d tmpl src %s dst %s proto esp mode tunnel if_id %d", ns,
    if_id, ipv4_remote, ipv4_local, if_id);
    return 0;
    fail:
    return -1;
    }
    static int setup_xfrm_tunnel(const char *ns_a, const char *ns_b,
    const char *ipv4_a, const char *ipv4_b,
    int if_id_a, int if_id_b)
    {
    return setup_xfrm_tunnel_ns(ns_a, ipv4_a, ipv4_b, if_id_a) ||
    setup_xfrm_tunnel_ns(ns_b, ipv4_b, ipv4_a, if_id_b);
    }
    static struct rtattr *rtattr_add(struct nlmsghdr *nh, unsigned short type,
    unsigned short len)
    {
    struct rtattr *rta =
    (struct rtattr *)((uint8_t *)nh + RTA_ALIGN(nh.nlmsg_len));
    rta.rta_type = type;
    rta.rta_len = RTA_LENGTH(len);
    nh.nlmsg_len = RTA_ALIGN(nh.nlmsg_len) + RTA_ALIGN(rta.rta_len);
    return rta;
    }
    static struct rtattr *rtattr_add_str(struct nlmsghdr *nh, unsigned short type,
    const char *s)
    {
    struct rtattr *rta = rtattr_add(nh, type, strlen(s));
    memcpy(RTA_DATA(rta), s, strlen(s));
    return rta;
    }
    static struct rtattr *rtattr_begin(struct nlmsghdr *nh, unsigned short type)
    {
    return rtattr_add(nh, type, 0);
    }
#[no_mangle]
unsafe extern "C" fn rtattr_end(nh: *mut nlmsghdr, attr: *mut rtattr) {
    static void rtattr_end(struct nlmsghdr *nh, struct rtattr *attr)
    {
    uint8_t *end = (uint8_t *)nh + nh.nlmsg_len;
    attr.rta_len = end - (uint8_t *)attr;
    }
#[no_mangle]
unsafe extern "C" fn setup_xfrmi_external_dev(ns: *const c_char) -> c_int {
    static int setup_xfrmi_external_dev(const char *ns)
    {
    struct {
    struct nlmsghdr nh;
    struct ifinfomsg info;
    unsigned char data[128];
    } req;
    struct rtattr *link_info, *info_data;
    struct nstoken *nstoken;
    let mut ret: c_int = -1, sock = -1;
    struct nlmsghdr *nh;
    memset(&req, 0, sizeof(req));
    nh = &req.nh;
    nh.nlmsg_len = NLMSG_LENGTH(sizeof(req.info));
    nh.nlmsg_type = RTM_NEWLINK;
    nh.nlmsg_flags |= NLM_F_CREATE | NLM_F_REQUEST;
    rtattr_add_str(nh, IFLA_IFNAME, "ipsec0");
    link_info = rtattr_begin(nh, IFLA_LINKINFO);
    rtattr_add_str(nh, IFLA_INFO_KIND, "xfrm");
    info_data = rtattr_begin(nh, IFLA_INFO_DATA);
    rtattr_add(nh, IFLA_XFRM_COLLECT_METADATA, 0);
    rtattr_end(nh, info_data);
    rtattr_end(nh, link_info);
    nstoken = open_netns(ns);
    if (!ASSERT_OK_PTR(nstoken, "setns"))
    goto done;
    sock = socket(AF_NETLINK, SOCK_RAW | SOCK_CLOEXEC, NETLINK_ROUTE);
    if (!ASSERT_GE(sock, 0, "netlink socket"))
    goto done;
    ret = send(sock, nh, nh.nlmsg_len, 0);
    if (!ASSERT_EQ(ret, nh.nlmsg_len, "netlink send length"))
    goto done;
    ret = 0;
    done:
    if (sock != -1)
    close(sock);
    if (nstoken)
    close_netns(nstoken);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn config_overlay() -> c_int {
    static int config_overlay(void)
    {
    if (setup_xfrm_tunnel(NS0, NS1, IP4_ADDR_VETH01, IP4_ADDR_VETH10,
    IF_ID_0_TO_1, IF_ID_1))
    goto fail;
    if (setup_xfrm_tunnel(NS0, NS2, IP4_ADDR_VETH02, IP4_ADDR_VETH20,
    IF_ID_0_TO_2, IF_ID_2))
    goto fail;
// Older iproute2 doesn't support this option
    if (!ASSERT_OK(setup_xfrmi_external_dev(NS0), "xfrmi"))
    goto fail;
    SYS(fail, "ip -net " NS0 " addr add 192.168.1.100/24 dev ipsec0");
    SYS(fail, "ip -net " NS0 " link set dev ipsec0 up");
    SYS(fail, "ip -net " NS1 " link add ipsec0 type xfrm if_id %d", IF_ID_1);
    SYS(fail, "ip -net " NS1 " addr add 192.168.1.200/24 dev ipsec0");
    SYS(fail, "ip -net " NS1 " link set dev ipsec0 up");
    SYS(fail, "ip -net " NS2 " link add ipsec0 type xfrm if_id %d", IF_ID_2);
    SYS(fail, "ip -net " NS2 " addr add 192.168.1.200/24 dev ipsec0");
    SYS(fail, "ip -net " NS2 " link set dev ipsec0 up");
    return 0;
    fail:
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn test_xfrm_ping(skel: *mut xfrm_info, if_id: u32) -> c_int {
    static int test_xfrm_ping(struct xfrm_info *skel, u32 if_id)
    {
    skel.bss.req_if_id = if_id;
    SYS(fail, "ping -i 0.01 -c 3 -w 10 -q 192.168.1.200 > /dev/null");
    if (!ASSERT_EQ(skel.bss.resp_if_id, if_id, "if_id"))
    goto fail;
    return 0;
    fail:
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn _test_xfrm_info() {
    static void _test_xfrm_info(void)
    {
    LIBBPF_OPTS(bpf_tc_hook, tc_hook, .attach_point = BPF_TC_INGRESS);
    int get_xfrm_info_prog_fd, set_xfrm_info_prog_fd;
    struct nstoken *nstoken = core::ptr::null_mut();
    struct xfrm_info *skel;
    int ifindex;
// load and attach bpf progs to ipsec dev tc hook point
    skel = xfrm_info__open_and_load();
    if (!ASSERT_OK_PTR(skel, "xfrm_info__open_and_load"))
    goto done;
    nstoken = open_netns(NS0);
    if (!ASSERT_OK_PTR(nstoken, "setns " NS0))
    goto done;
    ifindex = if_nametoindex("ipsec0");
    if (!ASSERT_NEQ(ifindex, 0, "ipsec0 ifindex"))
    goto done;
    tc_hook.ifindex = ifindex;
    set_xfrm_info_prog_fd = bpf_program__fd(skel.progs.set_xfrm_info);
    get_xfrm_info_prog_fd = bpf_program__fd(skel.progs.get_xfrm_info);
    if (!ASSERT_GE(set_xfrm_info_prog_fd, 0, "bpf_program__fd"))
    goto done;
    if (!ASSERT_GE(get_xfrm_info_prog_fd, 0, "bpf_program__fd"))
    goto done;
    if (attach_tc_prog(&tc_hook, get_xfrm_info_prog_fd,
    set_xfrm_info_prog_fd))
    goto done;
// perform test
    if (!ASSERT_EQ(test_xfrm_ping(skel, IF_ID_0_TO_1), 0, "ping " NS1))
    goto done;
    if (!ASSERT_EQ(test_xfrm_ping(skel, IF_ID_0_TO_2), 0, "ping " NS2))
    goto done;
    done:
    if (nstoken)
    close_netns(nstoken);
    xfrm_info__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_xfrm_info() {
    void test_xfrm_info(void)
    {
    cleanup();
    if (!ASSERT_OK(config_underlay(), "config_underlay"))
    goto done;
    if (!ASSERT_OK(config_overlay(), "config_overlay"))
    goto done;
    if (test__start_subtest("xfrm_info"))
    _test_xfrm_info();
    done:
    cleanup();
    }
