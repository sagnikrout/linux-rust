//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/ipv6_flowlabel.c
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
// Test IPV6_FLOWINFO cmsg on send and recv
// Macro flag: #define _GNU_SOURCE

// uapi/glibc weirdness may leave this undefined

pub const IPV6_FLOWINFO: c_int = 11;

pub const IPV6_FLOWLABEL_MGR: c_int = 32;

pub const IPV6_FLOWINFO_SEND: c_int = 33;

    static const char cfg_data[]	= "a";
    let mut cfg_label: static uint32_t = 1;
    static bool use_ping;
    static bool use_flowinfo_send;
    static struct icmp6hdr icmp6 = {
    .icmp6_type = ICMPV6_ECHO_REQUEST
    };
    static struct sockaddr_in6 addr = {
    .sin6_family = AF_INET6,
    .sin6_addr = IN6ADDR_LOOPBACK_INIT,
    };
#[no_mangle]
unsafe extern "C" fn do_send(fd: c_int, with_flowlabel: bool, flowlabel: u32) {
    static void do_send(int fd, bool with_flowlabel, uint32_t flowlabel)
    {
    char control[CMSG_SPACE(sizeof(flowlabel))] = {0};
    let mut msg: msghdr = {0};
    struct iovec iov = {
    .iov_base = (char *)cfg_data,
    .iov_len = sizeof(cfg_data)
    };
    int ret;
    if (use_ping) {
    iov.iov_base = &icmp6;
    iov.iov_len = sizeof(icmp6);
    }
    msg.msg_iov = &iov;
    msg.msg_iovlen = 1;
    if (use_flowinfo_send) {
    msg.msg_name = &addr;
    msg.msg_namelen = sizeof(addr);
    } else if (with_flowlabel) {
    struct cmsghdr *cm;
    cm = (void *)control;
    cm.cmsg_len = CMSG_LEN(sizeof(flowlabel));
    cm.cmsg_level = SOL_IPV6;
    cm.cmsg_type = IPV6_FLOWINFO;
// (uint32_t *)CMSG_DATA(cm) = htonl(flowlabel);
    msg.msg_control = control;
    msg.msg_controllen = sizeof(control);
    }
    ret = sendmsg(fd, &msg, 0);
    if (ret == -1)
    error(1, errno, "send");
    if (with_flowlabel)
    fprintf(stderr, "sent with label %u\n", flowlabel);
    else
    fprintf(stderr, "sent without label\n");
    }
#[no_mangle]
unsafe extern "C" fn do_recv(fd: c_int, with_flowlabel: bool, expect: u32) {
    static void do_recv(int fd, bool with_flowlabel, uint32_t expect)
    {
    char control[CMSG_SPACE(sizeof(expect))];
    char data[sizeof(cfg_data)];
    let mut msg: msghdr = {0};
    let mut iov: iovec = {0};
    struct cmsghdr *cm;
    uint32_t flowlabel;
    int ret;
    iov.iov_base = data;
    iov.iov_len = sizeof(data);
    msg.msg_iov = &iov;
    msg.msg_iovlen = 1;
    memset(control, 0, sizeof(control));
    msg.msg_control = control;
    msg.msg_controllen = sizeof(control);
    ret = recvmsg(fd, &msg, 0);
    if (ret == -1)
    error(1, errno, "recv");
    if (use_ping)
    goto parse_cmsg;
    if (msg.msg_flags & (MSG_TRUNC | MSG_CTRUNC))
    error(1, 0, "recv: truncated");
    if (ret != sizeof(cfg_data))
    error(1, 0, "recv: length mismatch");
    if (memcmp(data, cfg_data, sizeof(data)))
    error(1, 0, "recv: data mismatch");
    parse_cmsg:
    cm = CMSG_FIRSTHDR(&msg);
    if (with_flowlabel) {
    if (!cm)
    error(1, 0, "recv: missing cmsg");
    if (CMSG_NXTHDR(&msg, cm))
    error(1, 0, "recv: too many cmsg");
    if (cm.cmsg_level != SOL_IPV6 ||
    cm.cmsg_type != IPV6_FLOWINFO)
    error(1, 0, "recv: unexpected cmsg level or type");
    flowlabel = ntohl(*(uint32_t *)CMSG_DATA(cm));
    fprintf(stderr, "recv with label %u\n", flowlabel);
    if (expect != FLOWLABEL_WILDCARD && expect != flowlabel) {
    fprintf(stderr, "recv: incorrect flowlabel %u != %u\n",
    flowlabel, expect);
    error(1, 0, "recv: flowlabel is wrong");
    }
    } else {
    fprintf(stderr, "recv without label\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn get_autoflowlabel_enabled() -> bool {
    static bool get_autoflowlabel_enabled(void)
    {
    int fd, ret;
    char val;
    fd = open("/proc/sys/net/ipv6/auto_flowlabels", O_RDONLY);
    if (fd == -1)
    error(1, errno, "open sysctl");
    ret = read(fd, &val, 1);
    if (ret == -1)
    error(1, errno, "read sysctl");
    if (ret == 0)
    error(1, 0, "read sysctl: 0");
    if (close(fd))
    error(1, errno, "close sysctl");
    let mut val: return = = '1';
    }
#[no_mangle]
unsafe extern "C" fn flowlabel_get(fd: c_int, label: u32, share: u8, flags: u16) {
    static void flowlabel_get(int fd, uint32_t label, uint8_t share, uint16_t flags)
    {
    struct in6_flowlabel_req req = {
    .flr_action = IPV6_FL_A_GET,
    .flr_label = htonl(label),
    .flr_flags = flags,
    .flr_share = share,
    };
// do not pass IPV6_ADDR_ANY or IPV6_ADDR_MAPPED
    req.flr_dst.s6_addr[0] = 0xfd;
    req.flr_dst.s6_addr[15] = 0x1;
    if (setsockopt(fd, SOL_IPV6, IPV6_FLOWLABEL_MGR, &req, sizeof(req)))
    error(1, errno, "setsockopt flowlabel get");
    }
#[no_mangle]
unsafe extern "C" fn parse_opts(argc: c_int, argv: *mut c_char) {
    static void parse_opts(int argc, char **argv)
    {
    int c;
    while ((c = getopt(argc, argv, "l:ps")) != -1) {
    switch (c) {
    case 'l':
    cfg_label = strtoul(optarg, core::ptr::null_mut(), 0);
    break;
    case 'p':
    use_ping = true;
    break;
    case 's':
    use_flowinfo_send = true;
    break;
    default:
    error(1, 0, "%s: parse error", argv[0]);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    let mut one: c_int = 1;
    int fdt, fdr;
    let mut prot: c_int = 0;
    addr.sin6_port = htons(8000);
    parse_opts(argc, argv);
    if (use_ping) {
    fprintf(stderr, "attempting to use ping sockets\n");
    prot = IPPROTO_ICMPV6;
    }
    fdt = socket(PF_INET6, SOCK_DGRAM, prot);
    if (fdt == -1)
    error(1, errno, "socket t");
    fdr = use_ping ? fdt : socket(PF_INET6, SOCK_DGRAM, 0);
    if (fdr == -1)
    error(1, errno, "socket r");
    if (connect(fdt, (void *)&addr, sizeof(addr)))
    error(1, errno, "connect");
    if (!use_ping && bind(fdr, (void *)&addr, sizeof(addr)))
    error(1, errno, "bind");
    flowlabel_get(fdt, cfg_label, IPV6_FL_S_EXCL, IPV6_FL_F_CREATE);
    if (setsockopt(fdr, SOL_IPV6, IPV6_FLOWINFO, &one, sizeof(one)))
    error(1, errno, "setsockopt flowinfo");
    if (get_autoflowlabel_enabled()) {
    fprintf(stderr, "send no label: recv auto flowlabel\n");
    do_send(fdt, false, 0);
    do_recv(fdr, true, FLOWLABEL_WILDCARD);
    } else {
    fprintf(stderr, "send no label: recv no label (auto off)\n");
    do_send(fdt, false, 0);
    do_recv(fdr, false, 0);
    }
    if (use_flowinfo_send) {
    fprintf(stderr, "using IPV6_FLOWINFO_SEND to send label\n");
    addr.sin6_flowinfo = htonl(cfg_label);
    if (setsockopt(fdt, SOL_IPV6, IPV6_FLOWINFO_SEND, &one,
    sizeof(one)) == -1)
    error(1, errno, "setsockopt flowinfo_send");
    }
    fprintf(stderr, "send label\n");
    do_send(fdt, true, cfg_label);
    do_recv(fdr, true, cfg_label);
    if (close(fdr))
    error(1, errno, "close r");
    if (!use_ping && close(fdt))
    error(1, errno, "close t");
    return 0;
    }
