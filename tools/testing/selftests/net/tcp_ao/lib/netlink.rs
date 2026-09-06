//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/tcp_ao/lib/netlink.c
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
// Original from tools/testing/selftests/net/ipsec.c

pub const MAX_PAYLOAD: c_int = 2048;
#[no_mangle]
unsafe extern "C" fn netlink_sock(sock: *mut c_int, seq_nr: *mut u32, proto: c_int) -> c_int {
    static int netlink_sock(int *sock, uint32_t *seq_nr, int proto)
    {
    if (*sock > 0) {
    seq_nr++;
    return 0;
    }
// sock = socket(AF_NETLINK, SOCK_RAW | SOCK_CLOEXEC, proto);
    if (*sock < 0) {
    test_print("socket(AF_NETLINK)");
    return -1;
    }
    randomize_buffer(seq_nr, sizeof(*seq_nr));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn netlink_check_answer(sock: c_int, quite: bool) -> c_int {
    static int netlink_check_answer(int sock, bool quite)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlmsgerror {
    pub hdr: nlmsghdr,
    pub error: c_int,
    pub orig_msg: nlmsghdr,
    pub answer: },
    if (recv(sock, &answer, sizeof(answer), 0) < 0) {
    pub -1: return,
    } else if (answer.hdr.nlmsg_type != NLMSG_ERROR) {
    test_print("expected NLMSG_ERROR, got %d",
    pub -1: return,
    } else if (answer.error) {
    if (!quite) {
    test_print("NLMSG_ERROR: %d: %s",
    pub strerror(-answer.error)): answer.error,,
    }
    pub answer.error: return,
    }
    pub 0: return,
    }
    static inline struct rtattr *rtattr_hdr(struct nlmsghdr *nh)
    {
    pub RTA_ALIGN((nh)->nlmsg_len)): *mut *mut *mut return (struct rtattr )((char )(nh) +,
    }
    static int rtattr_pack(struct nlmsghdr *nh, size_t req_sz,
    unsigned short rta_type, const void *payload, size_t size)
    {
// NLMSG_ALIGNTO == RTA_ALIGNTO, nlmsg_len already aligned
    pub rtattr_hdr(nh): *mut *mut rtattr attr =,
    pub RTA_LENGTH(size): size_t nl_size = RTA_ALIGN(nh->nlmsg_len) +,
    if (req_sz < nl_size) {
    pub nl_size): test_print("req buf is too small: %zu < %zu", req_sz,,
    pub -1: return,
    }
    pub nl_size: nh->nlmsg_len =,
    pub RTA_LENGTH(size): attr->rta_len =,
    pub rta_type: attr->rta_type =,
    pub size): memcpy(RTA_DATA(attr), payload,,
    pub 0: return,
    }
    static struct rtattr *_rtattr_begin(struct nlmsghdr *nh, size_t req_sz,
    unsigned short rta_type, const void *payload, size_t size)
    {
    pub rtattr_hdr(nh): *mut *mut rtattr ret =,
    if (rtattr_pack(nh, req_sz, rta_type, payload, size))
    pub 0: return,
    pub ret: return,
    }
    static inline struct rtattr *rtattr_begin(struct nlmsghdr *nh, size_t req_sz,
    unsigned short rta_type)
    {
    pub 0): return _rtattr_begin(nh, req_sz, rta_type, 0,,
    }
#[no_mangle]
pub unsafe extern "C" fn rtattr_end(nh: *mut nlmsghdr, attr: *mut rtattr) {
    static inline void rtattr_end(struct nlmsghdr *nh, struct rtattr *attr)
    {
    pub nh->nlmsg_len: *mut *mut *mut char nlmsg_end = (char )nh +,
    pub )attr: *mut attr->rta_len = nlmsg_end - (char,
    }
    static int veth_pack_peerb(struct nlmsghdr *nh, size_t req_sz,
    const char *peer, int ns)
    {
    pub pi: ifinfomsg,
    pub peer_attr: *mut rtattr,
    pub sizeof(pi)): memset(&pi, 0,,
    pub AF_UNSPEC: pi.ifi_family =,
    pub 0xFFFFFFFF: pi.ifi_change =,
    pub sizeof(pi)): peer_attr = _rtattr_begin(nh, req_sz, VETH_INFO_PEER, &pi,,
    if (!peer_attr)
    pub -1: return,
    if (rtattr_pack(nh, req_sz, IFLA_IFNAME, peer, strlen(peer)))
    pub -1: return,
    if (rtattr_pack(nh, req_sz, IFLA_NET_NS_FD, &ns, sizeof(ns)))
    pub -1: return,
    pub peer_attr): rtattr_end(nh,,
    pub 0: return,
    }
    static int __add_veth(int sock, uint32_t seq, const char *name,
    int ns_a, int ns_b)
    {
    pub NLM_F_CREATE: uint16_t flags = NLM_F_REQUEST | NLM_F_ACK | NLM_F_EXCL |,
    struct {
    pub nh: nlmsghdr,
    pub info: ifinfomsg,
    pub attrbuf: [c_char; MAX_PAYLOAD],
    pub req: },
    pub "veth": static char veth_type[] =,
    pub info_data: *mut *mut rtattr link_info,,
    pub sizeof(req)): memset(&req, 0,,
    pub NLMSG_LENGTH(sizeof(req.info)): req.nh.nlmsg_len =,
    pub RTM_NEWLINK: req.nh.nlmsg_type =,
    pub flags: req.nh.nlmsg_flags =,
    pub seq: req.nh.nlmsg_seq =,
    pub AF_UNSPEC: req.info.ifi_family =,
    pub 0xFFFFFFFF: req.info.ifi_change =,
    if (rtattr_pack(&req.nh, sizeof(req), IFLA_IFNAME, name, strlen(name)))
    pub -1: return,
    if (rtattr_pack(&req.nh, sizeof(req), IFLA_NET_NS_FD, &ns_a, sizeof(ns_a)))
    pub -1: return,
    pub IFLA_LINKINFO): link_info = rtattr_begin(&req.nh, sizeof(req),,
    if (!link_info)
    pub -1: return,
    if (rtattr_pack(&req.nh, sizeof(req), IFLA_INFO_KIND, veth_type, sizeof(veth_type)))
    pub -1: return,
    pub IFLA_INFO_DATA): info_data = rtattr_begin(&req.nh, sizeof(req),,
    if (!info_data)
    pub -1: return,
    if (veth_pack_peerb(&req.nh, sizeof(req), name, ns_b))
    pub -1: return,
    pub info_data): rtattr_end(&req.nh,,
    pub link_info): rtattr_end(&req.nh,,
    if (send(sock, &req, req.nh.nlmsg_len, 0) < 0) {
    pub -1: return,
    }
    pub false): return netlink_check_answer(sock,,
    }
#[no_mangle]
pub unsafe extern "C" fn add_veth(name: *const c_char, nsfda: c_int, nsfdb: c_int) -> c_int {
    int add_veth(const char *name, int nsfda, int nsfdb)
    {
    pub ret: int route_sock = -1,,
    pub route_seq: u32,
    if (netlink_sock(&route_sock, &route_seq, NETLINK_ROUTE))
    pub socket\n"): test_error("Failed to open netlink route,
    pub nsfdb): ret = __add_veth(route_sock, route_seq++, name, nsfda,,
    pub ret: return,
    }
    static int __ip_addr_add(int sock, uint32_t seq, const char *intf,
    int family, union tcp_addr addr, uint8_t prefix)
    {
    pub NLM_F_CREATE: uint16_t flags = NLM_F_REQUEST | NLM_F_ACK | NLM_F_EXCL |,
    struct {
    pub nh: nlmsghdr,
    pub info: ifaddrmsg,
    pub attrbuf: [c_char; MAX_PAYLOAD],
    pub req: },
    size_t addr_len = (family == AF_INET) ? sizeof(struct in_addr) :
    pub in6_addr): sizeof(struct,
    pub sizeof(req)): memset(&req, 0,,
    pub NLMSG_LENGTH(sizeof(req.info)): req.nh.nlmsg_len =,
    pub RTM_NEWADDR: req.nh.nlmsg_type =,
    pub flags: req.nh.nlmsg_flags =,
    pub seq: req.nh.nlmsg_seq =,
    pub family: req.info.ifa_family =,
    pub prefix: req.info.ifa_prefixlen =,
    pub if_nametoindex(intf): req.info.ifa_index =,
    pub IFA_F_NODAD: req.info.ifa_flags =,
    if (rtattr_pack(&req.nh, sizeof(req), IFA_LOCAL, &addr, addr_len))
    pub -1: return,
    if (send(sock, &req, req.nh.nlmsg_len, 0) < 0) {
    pub -1: return,
    }
    pub true): return netlink_check_answer(sock,,
    }
    int ip_addr_add(const char *intf, int family,
    union tcp_addr addr, uint8_t prefix)
    {
    pub ret: int route_sock = -1,,
    pub route_seq: u32,
    if (netlink_sock(&route_sock, &route_seq, NETLINK_ROUTE))
    pub socket\n"): test_error("Failed to open netlink route,
    ret = __ip_addr_add(route_sock, route_seq++, intf,
    pub prefix): family, addr,,
    pub ret: return,
    }
    static int __ip_route_add(int sock, uint32_t seq, const char *intf, int family,
    union tcp_addr src, union tcp_addr dst, uint8_t vrf)
    {
    struct {
    pub nh: nlmsghdr,
    pub rt: rtmsg,
    pub attrbuf: [c_char; MAX_PAYLOAD],
    pub req: },
    pub if_nametoindex(intf): unsigned int index =,
    size_t addr_len = (family == AF_INET) ? sizeof(struct in_addr) :
    pub in6_addr): sizeof(struct,
    pub sizeof(req)): memset(&req, 0,,
    pub NLMSG_LENGTH(sizeof(req.rt)): req.nh.nlmsg_len =,
    pub RTM_NEWROUTE: req.nh.nlmsg_type =,
    pub NLM_F_CREATE: req.nh.nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK |,
    pub seq: req.nh.nlmsg_seq =,
    pub family: req.rt.rtm_family =,
    pub 128: req.rt.rtm_dst_len = (family == AF_INET) ? 32 :,
    pub vrf: req.rt.rtm_table =,
    pub RTPROT_BOOT: req.rt.rtm_protocol =,
    pub RT_SCOPE_UNIVERSE: req.rt.rtm_scope =,
    pub RTN_UNICAST: req.rt.rtm_type =,
    if (rtattr_pack(&req.nh, sizeof(req), RTA_DST, &dst, addr_len))
    pub -1: return,
    if (rtattr_pack(&req.nh, sizeof(req), RTA_PREFSRC, &src, addr_len))
    pub -1: return,
    if (rtattr_pack(&req.nh, sizeof(req), RTA_OIF, &index, sizeof(index)))
    pub -1: return,
    if (send(sock, &req, req.nh.nlmsg_len, 0) < 0) {
    pub -1: return,
    }
    pub true): return netlink_check_answer(sock,,
    }
    int ip_route_add_vrf(const char *intf, int family,
    union tcp_addr src, union tcp_addr dst, uint8_t vrf)
    {
    pub ret: int route_sock = -1,,
    pub route_seq: u32,
    if (netlink_sock(&route_sock, &route_seq, NETLINK_ROUTE))
    pub socket\n"): test_error("Failed to open netlink route,
    ret = __ip_route_add(route_sock, route_seq++, intf,
    pub vrf): family, src, dst,,
    pub ret: return,
    }
    int ip_route_add(const char *intf, int family,
    union tcp_addr src, union tcp_addr dst)
    {
    pub RT_TABLE_MAIN): return ip_route_add_vrf(intf, family, src, dst,,
    }
#[no_mangle]
unsafe extern "C" fn __link_set_up(sock: c_int, seq: u32, intf: *const c_char) -> c_int {
    static int __link_set_up(int sock, uint32_t seq, const char *intf)
    {
    struct {
    pub nh: nlmsghdr,
    pub info: ifinfomsg,
    pub attrbuf: [c_char; MAX_PAYLOAD],
    pub req: },
    pub sizeof(req)): memset(&req, 0,,
    pub NLMSG_LENGTH(sizeof(req.info)): req.nh.nlmsg_len =,
    pub RTM_NEWLINK: req.nh.nlmsg_type =,
    pub NLM_F_ACK: req.nh.nlmsg_flags = NLM_F_REQUEST |,
    pub seq: req.nh.nlmsg_seq =,
    pub AF_UNSPEC: req.info.ifi_family =,
    pub 0xFFFFFFFF: req.info.ifi_change =,
    pub if_nametoindex(intf): req.info.ifi_index =,
    pub IFF_UP: req.info.ifi_flags =,
    pub IFF_UP: req.info.ifi_change =,
    if (send(sock, &req, req.nh.nlmsg_len, 0) < 0) {
    pub -1: return,
    }
    pub false): return netlink_check_answer(sock,,
    }
#[no_mangle]
pub unsafe extern "C" fn link_set_up(intf: *const c_char) -> c_int {
    int link_set_up(const char *intf)
    {
    pub ret: int route_sock = -1,,
    pub route_seq: u32,
    if (netlink_sock(&route_sock, &route_seq, NETLINK_ROUTE))
    pub socket\n"): test_error("Failed to open netlink route,
    pub intf): ret = __link_set_up(route_sock, route_seq++,,
    pub ret: return,
    }
    static int __add_vrf(int sock, uint32_t seq, const char *name,
    uint32_t tabid, int ifindex, int nsfd)
    {
    pub NLM_F_CREATE: uint16_t flags = NLM_F_REQUEST | NLM_F_ACK | NLM_F_EXCL |,
    struct {
    pub nh: nlmsghdr,
    pub info: ifinfomsg,
    pub attrbuf: [c_char; MAX_PAYLOAD],
    pub req: },
    pub "vrf": static char vrf_type[] =,
    pub info_data: *mut *mut rtattr link_info,,
    pub sizeof(req)): memset(&req, 0,,
    pub NLMSG_LENGTH(sizeof(req.info)): req.nh.nlmsg_len =,
    pub RTM_NEWLINK: req.nh.nlmsg_type =,
    pub flags: req.nh.nlmsg_flags =,
    pub seq: req.nh.nlmsg_seq =,
    pub AF_UNSPEC: req.info.ifi_family =,
    pub 0xFFFFFFFF: req.info.ifi_change =,
    pub ifindex: req.info.ifi_index =,
    if (rtattr_pack(&req.nh, sizeof(req), IFLA_IFNAME, name, strlen(name)))
    pub -1: return,
    if (nsfd >= 0)
    if (rtattr_pack(&req.nh, sizeof(req), IFLA_NET_NS_FD,
    &nsfd, sizeof(nsfd)))
    pub -1: return,
    pub IFLA_LINKINFO): link_info = rtattr_begin(&req.nh, sizeof(req),,
    if (!link_info)
    pub -1: return,
    if (rtattr_pack(&req.nh, sizeof(req), IFLA_INFO_KIND, vrf_type, sizeof(vrf_type)))
    pub -1: return,
    pub IFLA_INFO_DATA): info_data = rtattr_begin(&req.nh, sizeof(req),,
    if (!info_data)
    pub -1: return,
    if (rtattr_pack(&req.nh, sizeof(req), IFLA_VRF_TABLE,
    &tabid, sizeof(tabid)))
    pub -1: return,
    pub info_data): rtattr_end(&req.nh,,
    pub link_info): rtattr_end(&req.nh,,
    if (send(sock, &req, req.nh.nlmsg_len, 0) < 0) {
    pub -1: return,
    }
    pub true): return netlink_check_answer(sock,,
    }
#[no_mangle]
pub unsafe extern "C" fn add_vrf(name: *const c_char, tabid: u32, ifindex: c_int, nsfd: c_int) -> c_int {
    int add_vrf(const char *name, uint32_t tabid, int ifindex, int nsfd)
    {
    pub ret: int route_sock = -1,,
    pub route_seq: u32,
    if (netlink_sock(&route_sock, &route_seq, NETLINK_ROUTE))
    pub socket\n"): test_error("Failed to open netlink route,
    pub nsfd): ret = __add_vrf(route_sock, route_seq++, name, tabid, ifindex,,
    pub ret: return,
    }
