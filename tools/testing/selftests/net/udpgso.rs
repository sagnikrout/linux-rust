//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/udpgso.c
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
// Macro flag: #define _GNU_SOURCE

pub const ETH_MAX_MTU: c_uint = 0xFFFFU;

pub const UDP_SEGMENT: c_int = 103;

pub const CONST_MTU_TEST: c_int = 1500;

    static bool		cfg_do_ipv4;
    static bool		cfg_do_ipv6;
    static bool		cfg_do_connected;
    static bool		cfg_do_connectionless;
    static bool		cfg_do_msgmore;
    let mut cfg_do_recv: static bool = true;
    static bool		cfg_do_setsockopt;
    let mut cfg_specific_test_id: static int = -1;
    let mut cfg_port: static unsigned short = 9000;
    static char buf[ETH_MAX_MTU];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct testcase {
    pub /: *mut *mut int tlen; / send() buffer size, may exceed mss,
    pub /: *mut *mut bool tfail; / send() call is expected to fail,
    pub /: *mut *mut int gso_len; / mss after applying gso,
    pub /: *mut *mut int r_num_mss; / recv(): number of calls of full mss,
    pub /: *mut *mut int r_len_last; / recv(): size of last non-mss dgram, if any,
    pub /: *mut *mut bool v6_ext_hdr; / send() dgrams with IPv6 extension headers,
}

    const struct in6_addr addr6 = {
    { { 0xfd, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 } }, /* fd00::1 */
    };
    const struct in_addr addr4 = {
    __constant_htonl(0x0a000001), /* 10.0.0.1 */
    };
    static const char ipv6_hopopts_pad1[8] = { 0 };
    struct testcase testcases_v4[] = {
    {
// no GSO: send a single byte
    .tlen = 1,
    .r_len_last = 1,
    },
    {
// no GSO: send a single MSS
    .tlen = CONST_MSS_V4,
    .r_num_mss = 1,
    },
    {
// no GSO: send a single MSS + 1B: fail
    .tlen = CONST_MSS_V4 + 1,
    .tfail = true,
    },
    {
// send a single MSS: will fall back to no GSO
    .tlen = CONST_MSS_V4,
    .gso_len = CONST_MSS_V4,
    .r_num_mss = 1,
    },
    {
// datalen <= MSS < gso_len: will fall back to no GSO
    .tlen = CONST_MSS_V4,
    .gso_len = CONST_MSS_V4 + 1,
    .r_num_mss = 0,
    .r_len_last = CONST_MSS_V4,
    },
    {
// MSS < datalen < gso_len: fail
    .tlen = CONST_MSS_V4 + 1,
    .gso_len = CONST_MSS_V4 + 2,
    .tfail = true,
    },
    {
// send a single MSS + 1B
    .tlen = CONST_MSS_V4 + 1,
    .gso_len = CONST_MSS_V4,
    .r_num_mss = 1,
    .r_len_last = 1,
    },
    {
// send exactly 2 MSS
    .tlen = CONST_MSS_V4 * 2,
    .gso_len = CONST_MSS_V4,
    .r_num_mss = 2,
    },
    {
// send 2 MSS + 1B
    .tlen = (CONST_MSS_V4 * 2) + 1,
    .gso_len = CONST_MSS_V4,
    .r_num_mss = 2,
    .r_len_last = 1,
    },
    {
// send MAX segs
    .tlen = (ETH_MAX_MTU / CONST_MSS_V4) * CONST_MSS_V4,
    .gso_len = CONST_MSS_V4,
    .r_num_mss = (ETH_MAX_MTU / CONST_MSS_V4),
    },
    {
// send MAX bytes
    .tlen = ETH_MAX_MTU - CONST_HDRLEN_V4,
    .gso_len = CONST_MSS_V4,
    .r_num_mss = CONST_MAX_SEGS_V4,
    .r_len_last = ETH_MAX_MTU - CONST_HDRLEN_V4 -
    (CONST_MAX_SEGS_V4 * CONST_MSS_V4),
    },
    {
// send MAX + 1: fail
    .tlen = ETH_MAX_MTU - CONST_HDRLEN_V4 + 1,
    .gso_len = CONST_MSS_V4,
    .tfail = true,
    },
    {
// send a single 1B MSS: will fall back to no GSO
    .tlen = 1,
    .gso_len = 1,
    .r_num_mss = 1,
    },
    {
// send 2 1B segments
    .tlen = 2,
    .gso_len = 1,
    .r_num_mss = 2,
    },
    {
// send 2B + 2B + 1B segments
    .tlen = 5,
    .gso_len = 2,
    .r_num_mss = 2,
    .r_len_last = 1,
    },
    {
// send max number of min sized segments
    .tlen = UDP_MAX_SEGMENTS,
    .gso_len = 1,
    .r_num_mss = UDP_MAX_SEGMENTS,
    },
    {
// send max number + 1 of min sized segments: fail
    .tlen = UDP_MAX_SEGMENTS + 1,
    .gso_len = 1,
    .tfail = true,
    },
    {
// EOL
    }
    };

    struct testcase testcases_v6[] = {
    {
// no GSO: send a single byte
    .tlen = 1,
    .r_len_last = 1,
    },
    {
// no GSO: send a single MSS
    .tlen = CONST_MSS_V6,
    .r_num_mss = 1,
    },
    {
// no GSO: send a single MSS + 1B: fail
    .tlen = CONST_MSS_V6 + 1,
    .tfail = true,
    },
    {
// send a single MSS: will fall back to no GSO
    .tlen = CONST_MSS_V6,
    .gso_len = CONST_MSS_V6,
    .r_num_mss = 1,
    },
    {
// datalen <= MSS < gso_len: will fall back to no GSO
    .tlen = CONST_MSS_V6,
    .gso_len = CONST_MSS_V6 + 1,
    .r_num_mss = 0,
    .r_len_last = CONST_MSS_V6,
    },
    {
// MSS < datalen < gso_len: fail
    .tlen = CONST_MSS_V6 + 1,
    .gso_len = CONST_MSS_V6 + 2,
    .tfail = true
    },
    {
// send a single MSS + 1B
    .tlen = CONST_MSS_V6 + 1,
    .gso_len = CONST_MSS_V6,
    .r_num_mss = 1,
    .r_len_last = 1,
    },
    {
// send exactly 2 MSS
    .tlen = CONST_MSS_V6 * 2,
    .gso_len = CONST_MSS_V6,
    .r_num_mss = 2,
    },
    {
// send 2 MSS + 1B
    .tlen = (CONST_MSS_V6 * 2) + 1,
    .gso_len = CONST_MSS_V6,
    .r_num_mss = 2,
    .r_len_last = 1,
    },
    {
// send MAX segs
    .tlen = (IP6_MAX_MTU / CONST_MSS_V6) * CONST_MSS_V6,
    .gso_len = CONST_MSS_V6,
    .r_num_mss = (IP6_MAX_MTU / CONST_MSS_V6),
    },
    {
// send MAX bytes
    .tlen = IP6_MAX_MTU - CONST_HDRLEN_V6,
    .gso_len = CONST_MSS_V6,
    .r_num_mss = CONST_MAX_SEGS_V6,
    .r_len_last = IP6_MAX_MTU - CONST_HDRLEN_V6 -
    (CONST_MAX_SEGS_V6 * CONST_MSS_V6),
    },
    {
// send MAX + 1: fail
    .tlen = IP6_MAX_MTU - CONST_HDRLEN_V6 + 1,
    .gso_len = CONST_MSS_V6,
    .tfail = true,
    },
    {
// send a single 1B MSS: will fall back to no GSO
    .tlen = 1,
    .gso_len = 1,
    .r_num_mss = 1,
    },
    {
// send 2 1B segments
    .tlen = 2,
    .gso_len = 1,
    .r_num_mss = 2,
    },
    {
// send 2 1B segments with extension headers
    .tlen = 2,
    .gso_len = 1,
    .r_num_mss = 2,
    .v6_ext_hdr = true,
    },
    {
// send 2B + 2B + 1B segments
    .tlen = 5,
    .gso_len = 2,
    .r_num_mss = 2,
    .r_len_last = 1,
    },
    {
// send max number of min sized segments
    .tlen = UDP_MAX_SEGMENTS,
    .gso_len = 1,
    .r_num_mss = UDP_MAX_SEGMENTS,
    },
    {
// send max number + 1 of min sized segments: fail
    .tlen = UDP_MAX_SEGMENTS + 1,
    .gso_len = 1,
    .tfail = true,
    },
    {
// EOL
    }
    };
#[no_mangle]
unsafe extern "C" fn set_pmtu_discover(fd: c_int, is_ipv4: bool) {
    static void set_pmtu_discover(int fd, bool is_ipv4)
    {
    int level, name, val;
    if (is_ipv4) {
    level	= SOL_IP;
    name	= IP_MTU_DISCOVER;
    val	= IP_PMTUDISC_DO;
    } else {
    level	= SOL_IPV6;
    name	= IPV6_MTU_DISCOVER;
    val	= IPV6_PMTUDISC_DO;
    }
    if (setsockopt(fd, level, name, &val, sizeof(val)))
    error(1, errno, "setsockopt path mtu");
    }
#[no_mangle]
unsafe extern "C" fn get_path_mtu(fd: c_int, is_ipv4: bool) -> c_uint {
    static unsigned int get_path_mtu(int fd, bool is_ipv4)
    {
    socklen_t vallen;
    unsigned int mtu;
    int ret;
    vallen = sizeof(mtu);
    if (is_ipv4)
    ret = getsockopt(fd, SOL_IP, IP_MTU, &mtu, &vallen);
    else
    ret = getsockopt(fd, SOL_IPV6, IPV6_MTU, &mtu, &vallen);
    if (ret)
    error(1, errno, "getsockopt mtu");
    fprintf(stderr, "path mtu (read):  %u\n", mtu);
    return mtu;
    }
#[no_mangle]
unsafe extern "C" fn __send_one(fd: c_int, msg: *mut msghdr, flags: c_int) -> bool {
    static bool __send_one(int fd, struct msghdr *msg, int flags)
    {
    int ret;
    ret = sendmsg(fd, msg, flags);
    if (ret == -1 &&
    (errno == EMSGSIZE || errno == ENOMEM || errno == EINVAL))
    return false;
    if (ret == -1)
    error(1, errno, "sendmsg");
    if (ret != msg.msg_iov.iov_len)
    error(1, 0, "sendto: %d != %llu", ret,
    (unsigned long long)msg.msg_iov.iov_len);
    if (msg.msg_flags)
    error(1, 0, "sendmsg: return flags 0x%x\n", msg.msg_flags);
    return true;
    }
    static bool send_one(int fd, int len, int gso_len,
    struct sockaddr *addr, socklen_t alen)
    {
    char control[CMSG_SPACE(sizeof(uint16_t))] = {0};
    let mut msg: msghdr = {0};
    let mut iov: iovec = {0};
    struct cmsghdr *cm;
    iov.iov_base = buf;
    iov.iov_len = len;
    msg.msg_iov = &iov;
    msg.msg_iovlen = 1;
    msg.msg_name = addr;
    msg.msg_namelen = alen;
    if (gso_len && !cfg_do_setsockopt) {
    msg.msg_control = control;
    msg.msg_controllen = sizeof(control);
    cm = CMSG_FIRSTHDR(&msg);
    cm.cmsg_level = SOL_UDP;
    cm.cmsg_type = UDP_SEGMENT;
    cm.cmsg_len = CMSG_LEN(sizeof(uint16_t));
// ((uint16_t *) CMSG_DATA(cm)) = gso_len;
    }
// If MSG_MORE, send 1 byte followed by remainder
    if (cfg_do_msgmore && len > 1) {
    iov.iov_len = 1;
    if (!__send_one(fd, &msg, MSG_MORE))
    error(1, 0, "send 1B failed");
    iov.iov_base++;
    iov.iov_len = len - 1;
    }
    return __send_one(fd, &msg, 0);
    }
#[no_mangle]
unsafe extern "C" fn recv_one(fd: c_int, flags: c_int) -> c_int {
    static int recv_one(int fd, int flags)
    {
    int ret;
    ret = recv(fd, buf, sizeof(buf), flags);
    if (ret == -1 && errno == EAGAIN && (flags & MSG_DONTWAIT))
    return 0;
    if (ret == -1)
    error(1, errno, "recv");
    return ret;
    }
    static void run_one(struct testcase *test, int fdt, int fdr,
    struct sockaddr *addr, socklen_t alen)
    {
    int i, ret, val, mss;
    bool sent;
    fprintf(stderr, "ipv%d tx:%d gso:%d %s%s\n",
    addr.sa_family == AF_INET ? 4 : 6,
    test.tlen, test.gso_len,
    test.v6_ext_hdr ? "ext-hdr " : "",
    test.tfail ? "(fail)" : "");
    if (test.v6_ext_hdr) {
    if (setsockopt(fdt, IPPROTO_IPV6, IPV6_HOPOPTS,
    ipv6_hopopts_pad1, sizeof(ipv6_hopopts_pad1)))
    error(1, errno, "setsockopt ipv6 hopopts");
    }
    val = test.gso_len;
    if (cfg_do_setsockopt) {
    if (setsockopt(fdt, SOL_UDP, UDP_SEGMENT, &val, sizeof(val)))
    error(1, errno, "setsockopt udp segment");
    }
    sent = send_one(fdt, test.tlen, test.gso_len, addr, alen);
    if (sent && test.tfail)
    error(1, 0, "send succeeded while expecting failure");
    if (!sent && !test.tfail)
    error(1, 0, "send failed while expecting success");
    if (test.v6_ext_hdr) {
    if (setsockopt(fdt, IPPROTO_IPV6, IPV6_HOPOPTS, core::ptr::null_mut(), 0))
    error(1, errno, "setsockopt ipv6 hopopts clear");
    }
    if (!sent)
    return;
    if (!cfg_do_recv)
    return;
    if (test.gso_len)
    mss = test.gso_len;
    else
    mss = addr.sa_family == AF_INET ? CONST_MSS_V4 : CONST_MSS_V6;
// Recv all full MSS datagrams
    for (i = 0; i < test.r_num_mss; i++) {
    ret = recv_one(fdr, 0);
    if (ret != mss)
    error(1, 0, "recv.%d: %d != %d", i, ret, mss);
    }
// Recv the non-full last datagram, if tlen was not a multiple of mss
    if (test.r_len_last) {
    ret = recv_one(fdr, 0);
    if (ret != test.r_len_last)
    error(1, 0, "recv.%d: %d != %d (last)",
    i, ret, test.r_len_last);
    }
// Verify received all data
    ret = recv_one(fdr, MSG_DONTWAIT);
    if (ret)
    error(1, 0, "recv: unexpected datagram");
    }
#[no_mangle]
unsafe extern "C" fn run_all(fdt: c_int, fdr: c_int, addr: *mut sockaddr, alen: socklen_t) {
    static void run_all(int fdt, int fdr, struct sockaddr *addr, socklen_t alen)
    {
    struct testcase *tests, *test;
    tests = addr.sa_family == AF_INET ? testcases_v4 : testcases_v6;
    for (test = tests; test.tlen; test++) {
// if a specific test is given, then skip all others
    if (cfg_specific_test_id == -1 ||
    cfg_specific_test_id == test - tests)
    run_one(test, fdt, fdr, addr, alen);
    }
    }
#[no_mangle]
unsafe extern "C" fn run_test(addr: *mut sockaddr, alen: socklen_t) {
    static void run_test(struct sockaddr *addr, socklen_t alen)
    {
    let mut tv: timeval = { .tv_usec = 100 * 1000 };
    int fdr, fdt, val;
    fdr = socket(addr.sa_family, SOCK_DGRAM, 0);
    if (fdr == -1)
    error(1, errno, "socket r");
    if (cfg_do_recv) {
    if (bind(fdr, addr, alen))
    error(1, errno, "bind");
    }
// Have tests fail quickly instead of hang
    if (setsockopt(fdr, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof(tv)))
    error(1, errno, "setsockopt rcv timeout");
    fdt = socket(addr.sa_family, SOCK_DGRAM, 0);
    if (fdt == -1)
    error(1, errno, "socket t");
// Do not fragment these datagrams: only succeed if GSO works
    set_pmtu_discover(fdt, addr.sa_family == AF_INET);
    if (cfg_do_connectionless)
    run_all(fdt, fdr, addr, alen);
    if (cfg_do_connected) {
    if (connect(fdt, addr, alen))
    error(1, errno, "connect");
    val = get_path_mtu(fdt, addr.sa_family == AF_INET);
    if (val != CONST_MTU_TEST)
    error(1, 0, "bad path mtu %u\n", val);
    run_all(fdt, fdr, addr, 0 /* use connected addr */);
    }
    if (close(fdt))
    error(1, errno, "close t");
    if (close(fdr))
    error(1, errno, "close r");
    }
#[no_mangle]
unsafe extern "C" fn run_test_v4() {
    static void run_test_v4(void)
    {
    let mut addr: sockaddr_in = {0};
    addr.sin_family = AF_INET;
    addr.sin_port = htons(cfg_port);
    addr.sin_addr = addr4;
    run_test((void *)&addr, sizeof(addr));
    }
#[no_mangle]
unsafe extern "C" fn run_test_v6() {
    static void run_test_v6(void)
    {
    let mut addr: sockaddr_in6 = {0};
    addr.sin6_family = AF_INET6;
    addr.sin6_port = htons(cfg_port);
    addr.sin6_addr = addr6;
    run_test((void *)&addr, sizeof(addr));
    }
#[no_mangle]
unsafe extern "C" fn parse_opts(argc: c_int, argv: *mut c_char) {
    static void parse_opts(int argc, char **argv)
    {
    int c;
    while ((c = getopt(argc, argv, "46cCmRst:")) != -1) {
    switch (c) {
    case '4':
    cfg_do_ipv4 = true;
    break;
    case '6':
    cfg_do_ipv6 = true;
    break;
    case 'c':
    cfg_do_connected = true;
    break;
    case 'C':
    cfg_do_connectionless = true;
    break;
    case 'm':
    cfg_do_msgmore = true;
    break;
    case 'R':
    cfg_do_recv = false;
    break;
    case 's':
    cfg_do_setsockopt = true;
    break;
    case 't':
    cfg_specific_test_id = strtoul(optarg, core::ptr::null_mut(), 0);
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
    parse_opts(argc, argv);
    if (cfg_do_ipv4)
    run_test_v4();
    if (cfg_do_ipv6)
    run_test_v6();
    fprintf(stderr, "OK\n");
    return 0;
    }
