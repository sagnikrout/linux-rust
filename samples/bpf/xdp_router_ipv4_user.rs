//! Automatically rewritten from C to Rust
//! Source: samples/bpf/xdp_router_ipv4_user.c
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2017 Cavium, Inc.
//

    static const char *__doc__ =
    "XDP IPv4 router implementation\n"
    "Usage: xdp_router_ipv4 <IFNAME-0> ... <IFNAME-N>\n";
    static char buf[8192];
    static int lpm_map_fd;
    static int arp_table_map_fd;
    static int exact_match_map_fd;
    static int tx_port_map_fd;
    static bool routes_thread_exit;
    let mut interval: static int = 5;
    static int mask = SAMPLE_RX_CNT | SAMPLE_REDIRECT_ERR_MAP_CNT |
    SAMPLE_DEVMAP_XMIT_CNT_MULTI | SAMPLE_EXCEPTION_CNT;
    DEFINE_SAMPLE_INIT(xdp_router_ipv4);
    static const struct option long_options[] = {
    { "help", no_argument, core::ptr::null_mut(), 'h' },
    { "skb-mode", no_argument, core::ptr::null_mut(), 'S' },
    { "force", no_argument, core::ptr::null_mut(), 'F' },
    { "interval", required_argument, core::ptr::null_mut(), 'i' },
    { "verbose", no_argument, core::ptr::null_mut(), 'v' },
    { "stats", no_argument, core::ptr::null_mut(), 's' },
    {}
    };
    static int get_route_table(int rtm_family);
#[no_mangle]
unsafe extern "C" fn recv_msg(sock_addr: sockaddr_nl, sock: c_int) -> c_int {
    static int recv_msg(struct sockaddr_nl sock_addr, int sock)
    {
    struct nlmsghdr *nh;
    int len, nll = 0;
    char *buf_ptr;
    buf_ptr = buf;
    while (1) {
    len = recv(sock, buf_ptr, sizeof(buf) - nll, 0);
    if (len < 0)
    return len;
    nh = (struct nlmsghdr *)buf_ptr;
    if (nh.nlmsg_type == NLMSG_DONE)
    break;
    buf_ptr += len;
    nll += len;
    if ((sock_addr.nl_groups & RTMGRP_NEIGH) == RTMGRP_NEIGH)
    break;
    if ((sock_addr.nl_groups & RTMGRP_IPV4_ROUTE) == RTMGRP_IPV4_ROUTE)
    break;
    }
    return nll;
    }
// Function to parse the route entry returned by netlink
// Updates the route entry related map entries
//
#[no_mangle]
unsafe extern "C" fn read_route(nh: *mut nlmsghdr, nll: c_int) {
    static void read_route(struct nlmsghdr *nh, int nll)
    {
    char dsts[24], gws[24], ifs[16], dsts_len[24], metrics[24];
    struct bpf_lpm_trie_key_u8 *prefix_key;
    struct rtattr *rt_attr;
    struct rtmsg *rt_msg;
    int rtm_family;
    int rtl;
    int i;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct route_table {
    pub metric: int dst_len, iface,,
    pub gw: __be32 dst,,
    pub mac: __be64,
    pub route: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arp_table {
    pub mac: __be64,
    pub dst: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct direct_map {
    pub arp: arp_table,
    pub ifindex: c_int,
    pub mac: __be64,
    pub direct_entry: },
    pub sizeof(route)): memset(&route, 0,,
    pub {: for (; NLMSG_OK(nh, nll); nh = NLMSG_NEXT(nh, nll)),
    pub )NLMSG_DATA(nh): *mut rt_msg = (struct rtmsg,
    pub rt_msg->rtm_family: rtm_family =,
    if (rtm_family == AF_INET)
    if (rt_msg.rtm_table != RT_TABLE_MAIN)
    pub )RTM_RTA(rt_msg): *mut rt_attr = (struct rtattr,
    pub RTM_PAYLOAD(nh): rtl =,
    pub {: for (; RTA_OK(rt_attr, rtl); rt_attr = RTA_NEXT(rt_attr, rtl)),
    switch (rt_attr.rta_type) {
    case NDA_DST:
    sprintf(dsts, "%u",
    pub )RTA_DATA(rt_attr)))): *mut *mut (((__be32,
    case RTA_GATEWAY:
    sprintf(gws, "%u",
// ((__be32 *)RTA_DATA(rt_attr)));
    case RTA_OIF:
    sprintf(ifs, "%u",
// ((int *)RTA_DATA(rt_attr)));
    case RTA_METRICS:
    sprintf(metrics, "%u",
// ((int *)RTA_DATA(rt_attr)));
    default:
    }
    }
    pub rt_msg->rtm_dst_len): sprintf(dsts_len, "%d",,
    pub atoi(dsts): route.dst =,
    pub atoi(dsts_len): route.dst_len =,
    pub atoi(gws): route.gw =,
    pub atoi(ifs): route.iface =,
    pub atoi(metrics): route.metric =,
    pub 0): assert(get_mac_addr(route.iface, &route.mac) ==,
    assert(bpf_map_update_elem(tx_port_map_fd,
    pub 0): &route.iface, &route.iface, 0) ==,
    if (rtm_family == AF_INET) {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trie_value {
    pub prefix: [__u8; 4],
    pub value: __be64,
    pub ifindex: c_int,
    pub metric: c_int,
    pub gw: __be32,
    pub prefix_value: *mut },
    pub 4): *mut *mut prefix_key = alloca(sizeof(prefix_key) +,
    pub alloca(sizeof(*prefix_value)): *mut prefix_value =,
    pub 32: prefix_key->prefixlen =,
    pub route.dst_len: prefix_key->prefixlen =,
    pub 0xffffffffffff: direct_entry.mac = route.mac &,
    pub route.iface: direct_entry.ifindex =,
    pub 0: direct_entry.arp.mac =,
    pub 0: direct_entry.arp.dst =,
    if (route.dst_len == 32) {
    if (nh.nlmsg_type == RTM_DELROUTE) {
    assert(bpf_map_delete_elem(exact_match_map_fd,
    pub 0): &route.dst) ==,
    } else {
    if (bpf_map_lookup_elem(arp_table_map_fd,
    &route.dst,
    &direct_entry.arp.mac) == 0)
    pub route.dst: direct_entry.arp.dst =,
    assert(bpf_map_update_elem(exact_match_map_fd,
    &route.dst,
    pub 0): &direct_entry, 0) ==,
    }
    }
    pub i++): for (i = 0; i < 4;,
    pub 0xff: *mut *mut prefix_key->data[i] = (route.dst >> i  8) &,
    if (bpf_map_lookup_elem(lpm_map_fd, prefix_key,
    prefix_value) < 0) {
    pub i++): for (i = 0; i < 4;,
    pub prefix_key->data[i]: prefix_value->prefix[i] =,
    pub 0xffffffffffff: prefix_value->value = route.mac &,
    pub route.iface: prefix_value->ifindex =,
    pub route.gw: prefix_value->gw =,
    pub route.metric: prefix_value->metric =,
    assert(bpf_map_update_elem(lpm_map_fd,
    prefix_key,
    prefix_value, 0
    pub 0): ) ==,
    } else {
    if (nh.nlmsg_type == RTM_DELROUTE) {
    assert(bpf_map_delete_elem(lpm_map_fd,
    prefix_key
    pub 0): ) ==,
// Rereading the route table to check if
// there is an entry with the same
// prefix but a different metric as the
// deleted entry.
//
    } else if (prefix_key.data[0] ==
    prefix_value.prefix[0] &&
    prefix_key.data[1] ==
    prefix_value.prefix[1] &&
    prefix_key.data[2] ==
    prefix_value.prefix[2] &&
    prefix_key.data[3] ==
    prefix_value.prefix[3] &&
    route.metric >= prefix_value.metric) {
    } else {
    pub i++): for (i = 0; i < 4;,
    prefix_value.prefix[i] =
    prefix_value.value =
    pub 0xffffffffffff: route.mac &,
    pub route.iface: prefix_value->ifindex =,
    pub route.gw: prefix_value->gw =,
    pub route.metric: prefix_value->metric =,
    assert(bpf_map_update_elem(lpm_map_fd,
    prefix_key,
    prefix_value,
    pub 0): 0) ==,
    }
    }
    }
    pub sizeof(route)): memset(&route, 0,,
    pub sizeof(dsts)): memset(dsts, 0,,
    pub sizeof(dsts_len)): memset(dsts_len, 0,,
    pub sizeof(gws)): memset(gws, 0,,
    pub sizeof(ifs)): memset(ifs, 0,,
    pub sizeof(route)): memset(&route, 0,,
    }
    }
// Function to read the existing route table  when the process is launched
#[no_mangle]
unsafe extern "C" fn get_route_table(rtm_family: c_int) -> c_int {
    static int get_route_table(int rtm_family)
    {
    pub sa: sockaddr_nl,
    pub nh: *mut nlmsghdr,
    pub 0: int sock, seq =,
    pub msg: msghdr,
    pub iov: iovec,
    pub 0: int ret =,
    pub nll: c_int,
    struct {
    pub nl: nlmsghdr,
    pub rt: rtmsg,
    pub buf: [c_char; 8192],
    pub req: },
    pub NETLINK_ROUTE): sock = socket(AF_NETLINK, SOCK_RAW,,
    if (sock < 0) {
    pub strerror(errno)): fprintf(stderr, "open netlink socket: %s\n",,
    pub -errno: return,
    }
    pub sizeof(sa)): memset(&sa, 0,,
    pub AF_NETLINK: sa.nl_family =,
    if (bind(sock, (struct sockaddr *)&sa, sizeof(sa)) < 0) {
    pub strerror(errno)): fprintf(stderr, "bind netlink socket: %s\n",,
    pub -errno: ret =,
    pub cleanup: goto,
    }
    pub sizeof(req)): memset(&req, 0,,
    pub rtmsg)): req.nl.nlmsg_len = NLMSG_LENGTH(sizeof(struct,
    pub NLM_F_DUMP: req.nl.nlmsg_flags = NLM_F_REQUEST |,
    pub RTM_GETROUTE: req.nl.nlmsg_type =,
    pub rtm_family: req.rt.rtm_family =,
    pub RT_TABLE_MAIN: req.rt.rtm_table =,
    pub 0: req.nl.nlmsg_pid =,
    pub ++seq: req.nl.nlmsg_seq =,
    pub sizeof(msg)): memset(&msg, 0,,
    pub )&req.nl: *mut iov.iov_base = (void,
    pub req.nl.nlmsg_len: iov.iov_len =,
    pub &iov: msg.msg_iov =,
    pub 1: msg.msg_iovlen =,
    pub 0): ret = sendmsg(sock, &msg,,
    if (ret < 0) {
    pub strerror(errno)): fprintf(stderr, "send to netlink: %s\n",,
    pub -errno: ret =,
    pub cleanup: goto,
    }
    pub sizeof(buf)): memset(buf, 0,,
    pub sock): nll = recv_msg(sa,,
    if (nll < 0) {
    pub strerror(nll)): fprintf(stderr, "recv from netlink: %s\n",,
    pub nll: ret =,
    pub cleanup: goto,
    }
    pub )buf: *mut nh = (struct nlmsghdr,
    pub nll): read_route(nh,,
    cleanup:
    pub ret: return,
    }
// Function to parse the arp entry returned by netlink
// Updates the arp entry related map entries
//
#[no_mangle]
unsafe extern "C" fn read_arp(nh: *mut nlmsghdr, nll: c_int) {
    static void read_arp(struct nlmsghdr *nh, int nll)
    {
    pub rt_attr: *mut rtattr,
    pub mac: [char dsts[24],; 24],
    pub rt_msg: *mut ndmsg,
    pub ndm_family: int rtl,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arp_table {
    pub mac: __be64,
    pub dst: __be32,
    pub arp_entry: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct direct_map {
    pub arp: arp_table,
    pub ifindex: c_int,
    pub mac: __be64,
    pub direct_entry: },
    pub {: for (; NLMSG_OK(nh, nll); nh = NLMSG_NEXT(nh, nll)),
    pub )NLMSG_DATA(nh): *mut rt_msg = (struct ndmsg,
    pub )RTM_RTA(rt_msg): *mut rt_attr = (struct rtattr,
    pub rt_msg->ndm_family: ndm_family =,
    pub RTM_PAYLOAD(nh): rtl =,
    pub {: for (; RTA_OK(rt_attr, rtl); rt_attr = RTA_NEXT(rt_attr, rtl)),
    switch (rt_attr.rta_type) {
    case NDA_DST:
    sprintf(dsts, "%u",
// ((__be32 *)RTA_DATA(rt_attr)));
    case NDA_LLADDR:
    sprintf(mac, "%lld",
// ((__be64 *)RTA_DATA(rt_attr)));
    default:
    }
    }
    pub atoi(dsts): arp_entry.dst =,
    pub atol(mac): arp_entry.mac =,
    if (ndm_family == AF_INET) {
    if (bpf_map_lookup_elem(exact_match_map_fd,
    &arp_entry.dst,
    &direct_entry) == 0) {
    if (nh.nlmsg_type == RTM_DELNEIGH) {
    pub 0: direct_entry.arp.dst =,
    pub 0: direct_entry.arp.mac =,
    } else if (nh.nlmsg_type == RTM_NEWNEIGH) {
    pub arp_entry.dst: direct_entry.arp.dst =,
    pub arp_entry.mac: direct_entry.arp.mac =,
    }
    assert(bpf_map_update_elem(exact_match_map_fd,
    &arp_entry.dst,
    &direct_entry, 0
    pub 0): ) ==,
    pub sizeof(direct_entry)): memset(&direct_entry, 0,,
    }
    if (nh.nlmsg_type == RTM_DELNEIGH) {
    assert(bpf_map_delete_elem(arp_table_map_fd,
    pub 0): &arp_entry.dst) ==,
    } else if (nh.nlmsg_type == RTM_NEWNEIGH) {
    assert(bpf_map_update_elem(arp_table_map_fd,
    &arp_entry.dst,
    &arp_entry.mac, 0
    pub 0): ) ==,
    }
    }
    pub sizeof(arp_entry)): memset(&arp_entry, 0,,
    pub sizeof(dsts)): memset(dsts, 0,,
    }
    }
// Function to read the existing arp table  when the process is launched
#[no_mangle]
unsafe extern "C" fn get_arp_table(rtm_family: c_int) -> c_int {
    static int get_arp_table(int rtm_family)
    {
    pub sa: sockaddr_nl,
    pub nh: *mut nlmsghdr,
    pub 0: int sock, seq =,
    pub msg: msghdr,
    pub iov: iovec,
    pub 0: int ret =,
    pub nll: c_int,
    struct {
    pub nl: nlmsghdr,
    pub rt: ndmsg,
    pub buf: [c_char; 8192],
    pub req: },
    pub NETLINK_ROUTE): sock = socket(AF_NETLINK, SOCK_RAW,,
    if (sock < 0) {
    pub strerror(errno)): fprintf(stderr, "open netlink socket: %s\n",,
    pub -errno: return,
    }
    pub sizeof(sa)): memset(&sa, 0,,
    pub AF_NETLINK: sa.nl_family =,
    if (bind(sock, (struct sockaddr *)&sa, sizeof(sa)) < 0) {
    pub strerror(errno)): fprintf(stderr, "bind netlink socket: %s\n",,
    pub -errno: ret =,
    pub cleanup: goto,
    }
    pub sizeof(req)): memset(&req, 0,,
    pub rtmsg)): req.nl.nlmsg_len = NLMSG_LENGTH(sizeof(struct,
    pub NLM_F_DUMP: req.nl.nlmsg_flags = NLM_F_REQUEST |,
    pub RTM_GETNEIGH: req.nl.nlmsg_type =,
    pub NUD_REACHABLE: req.rt.ndm_state =,
    pub rtm_family: req.rt.ndm_family =,
    pub 0: req.nl.nlmsg_pid =,
    pub ++seq: req.nl.nlmsg_seq =,
    pub sizeof(msg)): memset(&msg, 0,,
    pub )&req.nl: *mut iov.iov_base = (void,
    pub req.nl.nlmsg_len: iov.iov_len =,
    pub &iov: msg.msg_iov =,
    pub 1: msg.msg_iovlen =,
    pub 0): ret = sendmsg(sock, &msg,,
    if (ret < 0) {
    pub strerror(errno)): fprintf(stderr, "send to netlink: %s\n",,
    pub -errno: ret =,
    pub cleanup: goto,
    }
    pub sizeof(buf)): memset(buf, 0,,
    pub sock): nll = recv_msg(sa,,
    if (nll < 0) {
    pub strerror(nll)): fprintf(stderr, "recv from netlink: %s\n",,
    pub nll: ret =,
    pub cleanup: goto,
    }
    pub )buf: *mut nh = (struct nlmsghdr,
    pub nll): read_arp(nh,,
    cleanup:
    pub ret: return,
    }
// Function to keep track and update changes in route and arp table
// Give regular statistics of packets forwarded
//
    static void *monitor_routes_thread(void *arg)
    {
    pub fds_arp: pollfd fds_route,,
    pub lr: sockaddr_nl la,,
    pub nll: int sock, sock_arp,,
    pub nh: *mut nlmsghdr,
    pub NETLINK_ROUTE): sock = socket(AF_NETLINK, SOCK_RAW,,
    if (sock < 0) {
    pub strerror(errno)): fprintf(stderr, "open netlink socket: %s\n",,
    pub NULL: return,
    }
    pub O_NONBLOCK): fcntl(sock, F_SETFL,,
    pub sizeof(lr)): memset(&lr, 0,,
    pub AF_NETLINK: lr.nl_family =,
    pub RTMGRP_NOTIFY: lr.nl_groups = RTMGRP_IPV6_ROUTE | RTMGRP_IPV4_ROUTE |,
    if (bind(sock, (struct sockaddr *)&lr, sizeof(lr)) < 0) {
    pub strerror(errno)): fprintf(stderr, "bind netlink socket: %s\n",,
    pub NULL: return,
    }
    pub sock: fds_route.fd =,
    pub POLL_IN: fds_route.events =,
    pub NETLINK_ROUTE): sock_arp = socket(AF_NETLINK, SOCK_RAW,,
    if (sock_arp < 0) {
    pub strerror(errno)): fprintf(stderr, "open netlink socket: %s\n",,
    pub NULL: return,
    }
    pub O_NONBLOCK): fcntl(sock_arp, F_SETFL,,
    pub sizeof(la)): memset(&la, 0,,
    pub AF_NETLINK: la.nl_family =,
    pub RTMGRP_NOTIFY: la.nl_groups = RTMGRP_NEIGH |,
    if (bind(sock_arp, (struct sockaddr *)&la, sizeof(la)) < 0) {
    pub strerror(errno)): fprintf(stderr, "bind netlink socket: %s\n",,
    pub cleanup: goto,
    }
    pub sock_arp: fds_arp.fd =,
    pub POLL_IN: fds_arp.events =,
// dump route and arp tables
    if (get_arp_table(AF_INET) < 0) {
    pub table\n"): fprintf(stderr, "Failed reading arp,
    pub cleanup: goto,
    }
    if (get_route_table(AF_INET) < 0) {
    pub table\n"): fprintf(stderr, "Failed reading route,
    pub cleanup: goto,
    }
    while (!routes_thread_exit) {
    pub sizeof(buf)): memset(buf, 0,,
    if (poll(&fds_route, 1, 3) == POLL_IN) {
    pub sock): nll = recv_msg(lr,,
    if (nll < 0) {
    fprintf(stderr, "recv from netlink: %s\n",
    pub cleanup: goto,
    }
    pub )buf: *mut nh = (struct nlmsghdr,
    pub nll): read_route(nh,,
    }
    pub sizeof(buf)): memset(buf, 0,,
    if (poll(&fds_arp, 1, 3) == POLL_IN) {
    pub sock_arp): nll = recv_msg(la,,
    if (nll < 0) {
    fprintf(stderr, "recv from netlink: %s\n",
    pub cleanup: goto,
    }
    pub )buf: *mut nh = (struct nlmsghdr,
    pub nll): read_arp(nh,,
    }
    }
    cleanup:
    pub NULL: return,
    }
    static void usage(char *argv[], const struct option *long_options,
    const char *doc, int mask, bool error,
    struct bpf_object *obj)
    {
    pub error): sample_usage(argv, long_options, doc, mask,,
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    pub false: bool error = true, generic = false, force =,
    pub EXIT_FAIL_BPF: int opt, ret =,
    pub skel: *mut xdp_router_ipv4,
    pub 1: int i, total_ifindex = argc -,
    pub 1: *mut *mut *mut char ifname_list = argv +,
    pub routes_thread: pthread_t,
    pub 0: int longindex =,
    if (libbpf_set_strict_mode(LIBBPF_STRICT_ALL) < 0) {
    fprintf(stderr, "Failed to set libbpf strict mode: %s\n",
    pub end: goto,
    }
    pub xdp_router_ipv4__open(): skel =,
    if (!skel) {
    fprintf(stderr, "Failed to xdp_router_ipv4__open: %s\n",
    pub end: goto,
    }
    pub sample_init_pre_load(skel): ret =,
    if (ret < 0) {
    fprintf(stderr, "Failed to sample_init_pre_load: %s\n",
    pub EXIT_FAIL_BPF: ret =,
    pub end_destroy: goto,
    }
    pub xdp_router_ipv4__load(skel): ret =,
    if (ret < 0) {
    fprintf(stderr, "Failed to xdp_router_ipv4__load: %s\n",
    pub end_destroy: goto,
    }
    pub mask): ret = sample_init(skel,,
    if (ret < 0) {
    pub strerror(-ret)): fprintf(stderr, "Failed to initialize sample: %s\n",,
    pub EXIT_FAIL: ret =,
    pub end_destroy: goto,
    }
    while ((opt = getopt_long(argc, argv, "si:SFvh",
    long_options, &longindex)) != -1) {
    switch (opt) {
    case 's':
    pub SAMPLE_REDIRECT_MAP_CNT: mask |=,
    case 'i':
    pub 0): interval = strtoul(optarg, NULL,,
    pub 2: total_ifindex -=,
    pub 2: ifname_list +=,
    case 'S':
    pub true: generic =,
    case 'F':
    pub true: force =,
    case 'v':
    case 'h':
    pub false: error =,
    default:
    pub skel->obj): usage(argv, long_options, __doc__, mask, error,,
    pub end_destroy: goto,
    }
    }
    pub EXIT_FAIL_OPTION: ret =,
    if (optind == argc) {
    pub skel->obj): usage(argv, long_options, __doc__, mask, true,,
    pub end_destroy: goto,
    }
    pub bpf_map__fd(skel->maps.lpm_map): lpm_map_fd =,
    if (lpm_map_fd < 0) {
    fprintf(stderr, "Failed loading lpm_map %s\n",
    pub end_destroy: goto,
    }
    pub bpf_map__fd(skel->maps.arp_table): arp_table_map_fd =,
    if (arp_table_map_fd < 0) {
    fprintf(stderr, "Failed loading arp_table_map_fd %s\n",
    pub end_destroy: goto,
    }
    pub bpf_map__fd(skel->maps.exact_match): exact_match_map_fd =,
    if (exact_match_map_fd < 0) {
    fprintf(stderr, "Failed loading exact_match_map_fd %s\n",
    pub end_destroy: goto,
    }
    pub bpf_map__fd(skel->maps.tx_port): tx_port_map_fd =,
    if (tx_port_map_fd < 0) {
    fprintf(stderr, "Failed loading tx_port_map_fd %s\n",
    pub end_destroy: goto,
    }
    pub EXIT_FAIL_XDP: ret =,
    pub {: for (i = 0; i < total_ifindex; i++),
    pub if_nametoindex(ifname_list[i]): int index =,
    if (!index) {
    fprintf(stderr, "Interface %s not found %s\n",
    pub strerror(-tx_port_map_fd)): ifname_list[i],,
    pub end_destroy: goto,
    }
    if (sample_install_xdp(skel.progs.xdp_router_ipv4_prog,
    index, generic, force) < 0)
    pub end_destroy: goto,
    }
    pub NULL): ret = pthread_create(&routes_thread, NULL, monitor_routes_thread,,
    if (ret) {
    pub strerror(-ret)): fprintf(stderr, "Failed creating routes_thread: %s\n",,
    pub EXIT_FAIL: ret =,
    pub end_destroy: goto,
    }
    pub NULL): ret = sample_run(interval, NULL,,
    pub true: routes_thread_exit =,
    if (ret < 0) {
    pub strerror(-ret)): fprintf(stderr, "Failed during sample run: %s\n",,
    pub EXIT_FAIL: ret =,
    pub end_thread_wait: goto,
    }
    pub EXIT_OK: ret =,
    end_thread_wait:
    pub NULL): pthread_join(routes_thread,,
    end_destroy:
    end:
    }
