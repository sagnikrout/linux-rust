//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/mptcp/mptcp_diag.c
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
// Copyright (c) 2025, Kylin Software

pub const IPPROTO_MPTCP: c_int = 262;

    (parse_rtattr_flags((tb), (max), RTA_DATA(rta), RTA_PAYLOAD(rta), \
    NLA_F_NESTED))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct params {
    pub target_token: __u32,
    pub subflow_addrs: [c_char; 1024],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_info {
    pub mptcpi_subflows: __u8,
    pub mptcpi_add_addr_signal: __u8,
    pub mptcpi_add_addr_accepted: __u8,
    pub mptcpi_subflows_max: __u8,
    pub mptcpi_add_addr_signal_max: __u8,
    pub mptcpi_add_addr_accepted_max: __u8,
    pub mptcpi_flags: __u32,
    pub mptcpi_token: __u32,
    pub mptcpi_write_seq: __u64,
    pub mptcpi_snd_una: __u64,
    pub mptcpi_rcv_nxt: __u64,
    pub mptcpi_local_addr_used: __u8,
    pub mptcpi_local_addr_max: __u8,
    pub mptcpi_csum_enabled: __u8,
    pub mptcpi_retransmits: __u32,
    pub mptcpi_bytes_retrans: __u64,
    pub mptcpi_bytes_sent: __u64,
    pub mptcpi_bytes_received: __u64,
    pub mptcpi_bytes_acked: __u64,
    pub mptcpi_subflows_total: __u8,
    pub reserved: [__u8; 3],
    pub mptcpi_last_data_sent: __u32,
    pub mptcpi_last_data_recv: __u32,
    pub mptcpi_last_ack_recv: __u32,
}

    enum {
    MPTCP_SUBFLOW_ATTR_UNSPEC,
    MPTCP_SUBFLOW_ATTR_TOKEN_REM,
    MPTCP_SUBFLOW_ATTR_TOKEN_LOC,
    MPTCP_SUBFLOW_ATTR_RELWRITE_SEQ,
    MPTCP_SUBFLOW_ATTR_MAP_SEQ,
    MPTCP_SUBFLOW_ATTR_MAP_SFSEQ,
    MPTCP_SUBFLOW_ATTR_SSN_OFFSET,
    MPTCP_SUBFLOW_ATTR_MAP_DATALEN,
    MPTCP_SUBFLOW_ATTR_FLAGS,
    MPTCP_SUBFLOW_ATTR_ID_REM,
    MPTCP_SUBFLOW_ATTR_ID_LOC,
    MPTCP_SUBFLOW_ATTR_PAD,
    __MPTCP_SUBFLOW_ATTR_MAX
    };

#[no_mangle]
unsafe extern "C" fn die_perror(msg: *const c_char) -> void __noreturn {
    static void __noreturn die_perror(const char *msg)
    {
    perror(msg);
    exit(1);
    }
#[no_mangle]
unsafe extern "C" fn die_usage(r: c_int) {
    static void die_usage(int r)
    {
    fprintf(stderr, "Usage:\n"
    "mptcp_diag -t <token>\n"
    "mptcp_diag -s \"<saddr>:<sport> <daddr>:<dport>\"\n");
    exit(r);
    }
#[no_mangle]
unsafe extern "C" fn send_query(fd: c_int, r: *mut inet_diag_req_v2, proto: __u32) {
    static void send_query(int fd, struct inet_diag_req_v2 *r, __u32 proto)
    {
    struct sockaddr_nl nladdr = {
    .nl_family = AF_NETLINK
    };
    struct {
    struct nlmsghdr nlh;
    struct inet_diag_req_v2 r;
    } req = {
    .nlh = {
    .nlmsg_len = sizeof(req),
    .nlmsg_type = SOCK_DIAG_BY_FAMILY,
    .nlmsg_flags = NLM_F_REQUEST
    },
    .r = *r
    };
    struct rtattr rta_proto;
    struct iovec iov[6];
    let mut iovlen: c_int = 0;
    iov[iovlen++] = (struct iovec) {
    .iov_base = &req,
    .iov_len = sizeof(req)
    };
    if (proto == IPPROTO_MPTCP) {
    rta_proto.rta_type = INET_DIAG_REQ_PROTOCOL;
    rta_proto.rta_len = RTA_LENGTH(sizeof(proto));
    iov[iovlen++] = (struct iovec){ &rta_proto, sizeof(rta_proto)};
    iov[iovlen++] = (struct iovec){ &proto, sizeof(proto)};
    req.nlh.nlmsg_len += RTA_LENGTH(sizeof(proto));
    }
    struct msghdr msg = {
    .msg_name = &nladdr,
    .msg_namelen = sizeof(nladdr),
    .msg_iov = iov,
    .msg_iovlen = iovlen
    };
    for (;;) {
    if (sendmsg(fd, &msg, 0) < 0) {
    if (errno == EINTR)
    continue;
    die_perror("sendmsg");
    }
    break;
    }
    }
    static void parse_rtattr_flags(struct rtattr *tb[], int max, struct rtattr *rta,
    int len, unsigned short flags)
    {
    unsigned short type;
    memset(tb, 0, sizeof(struct rtattr *) * (max + 1));
    while (RTA_OK(rta, len)) {
    type = rta.rta_type & ~flags;
    if (type <= max && !tb[type])
    tb[type] = rta;
    rta = RTA_NEXT(rta, len);
    }
    }
#[no_mangle]
unsafe extern "C" fn print_info_msg(info: *mut mptcp_info) {
    static void print_info_msg(struct mptcp_info *info)
    {
    printf("Token & Flags\n");
    printf("token:        %x\n", info.mptcpi_token);
    printf("flags:        %x\n", info.mptcpi_flags);
    printf("csum_enabled: %u\n", info.mptcpi_csum_enabled);
    printf("\nBasic Info\n");
    printf("subflows:              %u\n", info.mptcpi_subflows);
    printf("subflows_max:          %u\n", info.mptcpi_subflows_max);
    printf("subflows_total:        %u\n", info.mptcpi_subflows_total);
    printf("local_addr_used:       %u\n", info.mptcpi_local_addr_used);
    printf("local_addr_max:        %u\n", info.mptcpi_local_addr_max);
    printf("add_addr_signal:       %u\n", info.mptcpi_add_addr_signal);
    printf("add_addr_accepted:     %u\n", info.mptcpi_add_addr_accepted);
    printf("add_addr_signal_max:   %u\n", info.mptcpi_add_addr_signal_max);
    printf("add_addr_accepted_max: %u\n", info.mptcpi_add_addr_accepted_max);
    printf("\nTransmission Info\n");
    printf("write_seq:        %llu\n", info.mptcpi_write_seq);
    printf("snd_una:          %llu\n", info.mptcpi_snd_una);
    printf("rcv_nxt:          %llu\n", info.mptcpi_rcv_nxt);
    printf("last_data_sent:   %u\n", info.mptcpi_last_data_sent);
    printf("last_data_recv:   %u\n", info.mptcpi_last_data_recv);
    printf("last_ack_recv:    %u\n", info.mptcpi_last_ack_recv);
    printf("retransmits:      %u\n", info.mptcpi_retransmits);
    printf("retransmit bytes: %llu\n", info.mptcpi_bytes_retrans);
    printf("bytes_sent:       %llu\n", info.mptcpi_bytes_sent);
    printf("bytes_received:   %llu\n", info.mptcpi_bytes_received);
    printf("bytes_acked:      %llu\n", info.mptcpi_bytes_acked);
    }
//
// 'print_subflow_info' is from 'mptcp_subflow_info'
// which is a function in 'misc/ss.c' of iproute2.
//
#[no_mangle]
unsafe extern "C" fn print_subflow_info(tb[]: *mut rtattr) {
    static void print_subflow_info(struct rtattr *tb[])
    {
    let mut flags: u_int32_t = 0;
    printf("It's a mptcp subflow, the subflow info:\n");
    if (tb[MPTCP_SUBFLOW_ATTR_FLAGS]) {
    char caps[32 + 1] = { 0 }, *cap = &caps[0];
    flags = rta_getattr(__u32, tb[MPTCP_SUBFLOW_ATTR_FLAGS]);
    if (flags & MPTCP_SUBFLOW_FLAG_MCAP_REM)
// cap++ = 'M';
    if (flags & MPTCP_SUBFLOW_FLAG_MCAP_LOC)
// cap++ = 'm';
    if (flags & MPTCP_SUBFLOW_FLAG_JOIN_REM)
// cap++ = 'J';
    if (flags & MPTCP_SUBFLOW_FLAG_JOIN_LOC)
// cap++ = 'j';
    if (flags & MPTCP_SUBFLOW_FLAG_BKUP_REM)
// cap++ = 'B';
    if (flags & MPTCP_SUBFLOW_FLAG_BKUP_LOC)
// cap++ = 'b';
    if (flags & MPTCP_SUBFLOW_FLAG_FULLY_ESTABLISHED)
// cap++ = 'e';
    if (flags & MPTCP_SUBFLOW_FLAG_CONNECTED)
// cap++ = 'c';
    if (flags & MPTCP_SUBFLOW_FLAG_MAPVALID)
// cap++ = 'v';
    if (flags)
    printf(" flags:%s", caps);
    }
    if (tb[MPTCP_SUBFLOW_ATTR_TOKEN_REM] &&
    tb[MPTCP_SUBFLOW_ATTR_TOKEN_LOC] &&
    tb[MPTCP_SUBFLOW_ATTR_ID_REM] &&
    tb[MPTCP_SUBFLOW_ATTR_ID_LOC])
    printf(" token:%04x(id:%u)/%04x(id:%u)",
    rta_getattr(__u32, tb[MPTCP_SUBFLOW_ATTR_TOKEN_REM]),
    rta_getattr(__u8, tb[MPTCP_SUBFLOW_ATTR_ID_REM]),
    rta_getattr(__u32, tb[MPTCP_SUBFLOW_ATTR_TOKEN_LOC]),
    rta_getattr(__u8, tb[MPTCP_SUBFLOW_ATTR_ID_LOC]));
    if (tb[MPTCP_SUBFLOW_ATTR_MAP_SEQ])
    printf(" seq:%llu",
    rta_getattr(__u64, tb[MPTCP_SUBFLOW_ATTR_MAP_SEQ]));
    if (tb[MPTCP_SUBFLOW_ATTR_MAP_SFSEQ])
    printf(" sfseq:%u",
    rta_getattr(__u32, tb[MPTCP_SUBFLOW_ATTR_MAP_SFSEQ]));
    if (tb[MPTCP_SUBFLOW_ATTR_SSN_OFFSET])
    printf(" ssnoff:%u",
    rta_getattr(__u32, tb[MPTCP_SUBFLOW_ATTR_SSN_OFFSET]));
    if (tb[MPTCP_SUBFLOW_ATTR_MAP_DATALEN])
    printf(" maplen:%u",
    rta_getattr(__u32, tb[MPTCP_SUBFLOW_ATTR_MAP_DATALEN]));
    printf("\n");
    }
#[no_mangle]
unsafe extern "C" fn parse_nlmsg(nlh: *mut nlmsghdr, proto: __u32) {
    static void parse_nlmsg(struct nlmsghdr *nlh, __u32 proto)
    {
    struct inet_diag_msg *r = NLMSG_DATA(nlh);
    struct rtattr *tb[INET_DIAG_MAX + 1];
    parse_rtattr_flags(tb, INET_DIAG_MAX, (struct rtattr *)(r + 1),
    nlh.nlmsg_len - NLMSG_LENGTH(sizeof(*r)),
    NLA_F_NESTED);
    if (proto == IPPROTO_MPTCP && tb[INET_DIAG_INFO]) {
    let mut len: c_int = RTA_PAYLOAD(tb[INET_DIAG_INFO]);
    struct mptcp_info *info;
// workaround fort older kernels with less fields
    if (len < sizeof(*info)) {
    info = alloca(sizeof(*info));
    memcpy(info, RTA_DATA(tb[INET_DIAG_INFO]), len);
    memset((char *)info + len, 0, sizeof(*info) - len);
    } else {
    info = RTA_DATA(tb[INET_DIAG_INFO]);
    }
    print_info_msg(info);
    }
    if (proto == IPPROTO_TCP && tb[INET_DIAG_ULP_INFO]) {
    struct rtattr *ulpinfo[INET_ULP_INFO_MAX + 1] = { 0 };
    parse_rtattr_nested(ulpinfo, INET_ULP_INFO_MAX,
    tb[INET_DIAG_ULP_INFO]);
    if (ulpinfo[INET_ULP_INFO_MPTCP]) {
    struct rtattr *sfinfo[MPTCP_SUBFLOW_ATTR_MAX + 1] = { 0 };
    parse_rtattr_nested(sfinfo, MPTCP_SUBFLOW_ATTR_MAX,
    ulpinfo[INET_ULP_INFO_MPTCP]);
    print_subflow_info(sfinfo);
    } else {
    printf("It's a normal TCP!\n");
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn recv_nlmsg(fd: c_int, proto: __u32) {
    static void recv_nlmsg(int fd, __u32 proto)
    {
    char rcv_buff[8192];
    struct nlmsghdr *nlh = (struct nlmsghdr *)rcv_buff;
    struct sockaddr_nl rcv_nladdr = {
    .nl_family = AF_NETLINK
    };
    struct iovec rcv_iov = {
    .iov_base = rcv_buff,
    .iov_len = sizeof(rcv_buff)
    };
    struct msghdr rcv_msg = {
    .msg_name = &rcv_nladdr,
    .msg_namelen = sizeof(rcv_nladdr),
    .msg_iov = &rcv_iov,
    .msg_iovlen = 1
    };
    int len;
    len = recvmsg(fd, &rcv_msg, 0);
    while (NLMSG_OK(nlh, len)) {
    if (nlh.nlmsg_type == NLMSG_DONE) {
    printf("NLMSG_DONE\n");
    break;
    } else if (nlh.nlmsg_type == NLMSG_ERROR) {
    struct nlmsgerr *err;
    err = (struct nlmsgerr *)NLMSG_DATA(nlh);
    printf("Error %d:%s\n",
    -(err.error), strerror(-(err.error)));
    break;
    }
    parse_nlmsg(nlh, proto);
    nlh = NLMSG_NEXT(nlh, len);
    }
    }
#[no_mangle]
unsafe extern "C" fn get_mptcpinfo(token: __u32) {
    static void get_mptcpinfo(__u32 token)
    {
    struct inet_diag_req_v2 r = {
    .sdiag_family           = AF_INET,
// Real proto is set via INET_DIAG_REQ_PROTOCOL
    .sdiag_protocol         = IPPROTO_TCP,
    .idiag_ext              = 1 << (INET_DIAG_INFO - 1),
    .id.idiag_cookie[0]     = token,
    };
    let mut proto: __u32 = IPPROTO_MPTCP;
    int fd;
    fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_SOCK_DIAG);
    if (fd < 0)
    die_perror("Netlink socket");
    send_query(fd, &r, proto);
    recv_nlmsg(fd, proto);
    close(fd);
    }
#[no_mangle]
unsafe extern "C" fn get_subflow_info(subflow_addrs: *mut c_char) {
    static void get_subflow_info(char *subflow_addrs)
    {
    struct inet_diag_req_v2 r = {
    .sdiag_family           = AF_INET,
    .sdiag_protocol         = IPPROTO_TCP,
    .idiag_ext              = 1 << (INET_DIAG_INFO - 1),
    .id.idiag_cookie[0]     = INET_DIAG_NOCOOKIE,
    .id.idiag_cookie[1]     = INET_DIAG_NOCOOKIE,
    };
    char saddr[64], daddr[64];
    int sport, dport;
    int ret;
    int fd;
    ret = sscanf(subflow_addrs, "%63[^:]:%d %63[^:]:%d",
    saddr, &sport, daddr, &dport);
    if (ret != 4)
    die_perror("IP PORT Pairs has style problems!");
    printf("%s:%d . %s:%d\n", saddr, sport, daddr, dport);
    fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_SOCK_DIAG);
    if (fd < 0)
    die_perror("Netlink socket");
    r.id.idiag_sport = htons(sport);
    r.id.idiag_dport = htons(dport);
    inet_pton(AF_INET, saddr, &r.id.idiag_src);
    inet_pton(AF_INET, daddr, &r.id.idiag_dst);
    send_query(fd, &r, IPPROTO_TCP);
    recv_nlmsg(fd, IPPROTO_TCP);
    }
#[no_mangle]
unsafe extern "C" fn parse_opts(argc: c_int, argv: *mut c_char, p: *mut params) {
    static void parse_opts(int argc, char **argv, struct params *p)
    {
    int c;
    if (argc < 2)
    die_usage(1);
    while ((c = getopt(argc, argv, "ht:s:")) != -1) {
    switch (c) {
    case 'h':
    die_usage(0);
    break;
    case 't':
    sscanf(optarg, "%x", &p.target_token);
    break;
    case 's':
    strncpy(p.subflow_addrs, optarg,
    sizeof(p.subflow_addrs) - 1);
    break;
    default:
    die_usage(1);
    break;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    let mut p: params = { 0 };
    parse_opts(argc, argv, &p);
    if (p.target_token)
    get_mptcpinfo(p.target_token);
    if (p.subflow_addrs[0] != '\0')
    get_subflow_info(p.subflow_addrs);
    return 0;
    }
