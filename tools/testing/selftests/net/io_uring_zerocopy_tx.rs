//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/io_uring_zerocopy_tx.c
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


// SPDX-License-Identifier: MIT
// based on linux-kernel/tools/testing/selftests/net/msg_zerocopy.c

pub const NOTIF_TAG: c_uint = 0xfffffffULL;
pub const NONZC_TAG: c_int = 0;
pub const ZC_TAG: c_int = 1;
    enum {
    MODE_NONZC	= 0,
    MODE_ZC		= 1,
    MODE_ZC_FIXED	= 2,
    MODE_MIXED	= 3,
    };
    let mut cfg_cork: static bool = false;
    let mut cfg_mode: static int = MODE_ZC_FIXED;
    let mut cfg_nr_reqs: static int = 8;
    let mut cfg_family: static int = PF_UNSPEC;
    static int  cfg_payload_len;
    let mut cfg_port: static int = 8000;
    let mut cfg_runtime_ms: static int = 4200;
    static socklen_t cfg_alen;
    static struct sockaddr_storage cfg_dst_addr;
    static char payload[IP_MAXPACKET] __attribute__((aligned(4096)));
#[no_mangle]
unsafe extern "C" fn gettimeofday_ms() -> c_ulong {
    static unsigned long gettimeofday_ms(void)
    {
    struct timeval tv;
    gettimeofday(&tv, core::ptr::null_mut());
    return (tv.tv_sec * 1000) + (tv.tv_usec / 1000);
    }
#[no_mangle]
unsafe extern "C" fn do_setsockopt(fd: c_int, level: c_int, optname: c_int, val: c_int) {
    static void do_setsockopt(int fd, int level, int optname, int val)
    {
    if (setsockopt(fd, level, optname, &val, sizeof(val)))
    error(1, errno, "setsockopt %d.%d: %d", level, optname, val);
    }
#[no_mangle]
unsafe extern "C" fn do_setup_tx(domain: c_int, type: c_int, protocol: c_int) -> c_int {
    static int do_setup_tx(int domain, int type, int protocol)
    {
    int fd;
    fd = socket(domain, type, protocol);
    if (fd == -1)
    error(1, errno, "socket t");
    do_setsockopt(fd, SOL_SOCKET, SO_SNDBUF, 1 << 21);
    if (connect(fd, (void *) &cfg_dst_addr, cfg_alen))
    error(1, errno, "connect");
    return fd;
    }
#[no_mangle]
unsafe extern "C" fn do_tx(domain: c_int, type: c_int, protocol: c_int) {
    static void do_tx(int domain, int type, int protocol)
    {
    struct io_uring_sqe *sqe;
    struct io_uring_cqe *cqe;
    let mut packets: c_ulong = 0, bytes = 0;
    struct io_uring ring;
    struct iovec iov;
    uint64_t tstop;
    int i, fd, ret;
    let mut compl_cqes: c_int = 0;
    fd = do_setup_tx(domain, type, protocol);
    ret = io_uring_queue_init(512, &ring, 0);
    if (ret)
    error(1, -ret, "io_uring: queue init");
    iov.iov_base = payload;
    iov.iov_len = cfg_payload_len;
    ret = io_uring_register_buffers(&ring, &iov, 1);
    if (ret)
    error(1, -ret, "io_uring: buffer registration");
    tstop = gettimeofday_ms() + cfg_runtime_ms;
    do {
    if (cfg_cork)
    do_setsockopt(fd, IPPROTO_UDP, UDP_CORK, 1);
    for (i = 0; i < cfg_nr_reqs; i++) {
    let mut zc_flags: unsigned = 0;
    let mut buf_idx: unsigned = 0;
    let mut mode: unsigned = cfg_mode;
    let mut msg_flags: unsigned = MSG_WAITALL;
    if (cfg_mode == MODE_MIXED)
    mode = rand() % 3;
    sqe = io_uring_get_sqe(&ring);
    if (mode == MODE_NONZC) {
    io_uring_prep_send(sqe, fd, payload,
    cfg_payload_len, msg_flags);
    sqe.user_data = NONZC_TAG;
    } else {
    io_uring_prep_sendzc(sqe, fd, payload,
    cfg_payload_len,
    msg_flags, zc_flags);
    if (mode == MODE_ZC_FIXED) {
    sqe.ioprio |= IORING_RECVSEND_FIXED_BUF;
    sqe.buf_index = buf_idx;
    }
    sqe.user_data = ZC_TAG;
    }
    }
    ret = io_uring_submit(&ring);
    if (ret != cfg_nr_reqs)
    error(1, -ret, "submit");
    if (cfg_cork)
    do_setsockopt(fd, IPPROTO_UDP, UDP_CORK, 0);
    for (i = 0; i < cfg_nr_reqs; i++) {
    ret = io_uring_wait_cqe(&ring, &cqe);
    if (ret)
    error(1, -ret, "wait cqe");
    if (cqe.user_data != NONZC_TAG &&
    cqe.user_data != ZC_TAG)
    error(1, EINVAL, "invalid cqe.user_data");
    if (cqe.flags & IORING_CQE_F_NOTIF) {
    if (cqe.flags & IORING_CQE_F_MORE)
    error(1, EINVAL, "invalid notif flags");
    if (compl_cqes <= 0)
    error(1, EINVAL, "notification mismatch");
    compl_cqes--;
    i--;
    io_uring_cqe_seen(&ring);
    continue;
    }
    if (cqe.flags & IORING_CQE_F_MORE) {
    if (cqe.user_data != ZC_TAG)
    error(1, -cqe.res, "unexpected F_MORE");
    compl_cqes++;
    }
    if (cqe.res >= 0) {
    packets++;
    bytes += cqe.res;
    } else if (cqe.res != -EAGAIN) {
    error(1, -cqe.res, "send failed");
    }
    io_uring_cqe_seen(&ring);
    }
    } while (gettimeofday_ms() < tstop);
    while (compl_cqes) {
    ret = io_uring_wait_cqe(&ring, &cqe);
    if (ret)
    error(1, -ret, "wait cqe");
    if (cqe.flags & IORING_CQE_F_MORE)
    error(1, EINVAL, "invalid notif flags");
    if (!(cqe.flags & IORING_CQE_F_NOTIF))
    error(1, EINVAL, "missing notif flag");
    io_uring_cqe_seen(&ring);
    compl_cqes--;
    }
    fprintf(stderr, "tx=%lu (MB=%lu), tx/s=%lu (MB/s=%lu)\n",
    packets, bytes >> 20,
    packets / (cfg_runtime_ms / 1000),
    (bytes >> 20) / (cfg_runtime_ms / 1000));
    if (close(fd))
    error(1, errno, "close");
    }
#[no_mangle]
unsafe extern "C" fn do_test(domain: c_int, type: c_int, protocol: c_int) {
    static void do_test(int domain, int type, int protocol)
    {
    int i;
    for (i = 0; i < IP_MAXPACKET; i++)
    payload[i] = 'a' + (i % 26);
    do_tx(domain, type, protocol);
    }
#[no_mangle]
unsafe extern "C" fn usage(filepath: *const c_char) {
    static void usage(const char *filepath)
    {
    error(1, 0, "Usage: %s (-4|-6) (udp|tcp) -D<dst_ip> [-s<payload size>] "
    "[-t<time s>] [-n<batch>] [-p<port>] [-m<mode>]", filepath);
    }
#[no_mangle]
unsafe extern "C" fn parse_opts(argc: c_int, argv: *mut c_char) {
    static void parse_opts(int argc, char **argv)
    {
    const int max_payload_len = sizeof(payload) -
    sizeof(struct ipv6hdr) -
    sizeof(struct tcphdr) -
    40 /* max tcp options */;
    struct sockaddr_in6 *addr6 = (void *) &cfg_dst_addr;
    struct sockaddr_in *addr4 = (void *) &cfg_dst_addr;
    char *daddr = core::ptr::null_mut();
    int c;
    if (argc <= 1)
    usage(argv[0]);
    cfg_payload_len = max_payload_len;
    while ((c = getopt(argc, argv, "46D:p:s:t:n:c:m:")) != -1) {
    switch (c) {
    case '4':
    if (cfg_family != PF_UNSPEC)
    error(1, 0, "Pass one of -4 or -6");
    cfg_family = PF_INET;
    cfg_alen = sizeof(struct sockaddr_in);
    break;
    case '6':
    if (cfg_family != PF_UNSPEC)
    error(1, 0, "Pass one of -4 or -6");
    cfg_family = PF_INET6;
    cfg_alen = sizeof(struct sockaddr_in6);
    break;
    case 'D':
    daddr = optarg;
    break;
    case 'p':
    cfg_port = strtoul(optarg, core::ptr::null_mut(), 0);
    break;
    case 's':
    cfg_payload_len = strtoul(optarg, core::ptr::null_mut(), 0);
    break;
    case 't':
    cfg_runtime_ms = 200 + strtoul(optarg, core::ptr::null_mut(), 10) * 1000;
    break;
    case 'n':
    cfg_nr_reqs = strtoul(optarg, core::ptr::null_mut(), 0);
    break;
    case 'c':
    cfg_cork = strtol(optarg, core::ptr::null_mut(), 0);
    break;
    case 'm':
    cfg_mode = strtol(optarg, core::ptr::null_mut(), 0);
    break;
    }
    }
    switch (cfg_family) {
    case PF_INET:
    memset(addr4, 0, sizeof(*addr4));
    addr4.sin_family = AF_INET;
    addr4.sin_port = htons(cfg_port);
    if (daddr &&
    inet_pton(AF_INET, daddr, &(addr4.sin_addr)) != 1)
    error(1, 0, "ipv4 parse error: %s", daddr);
    break;
    case PF_INET6:
    memset(addr6, 0, sizeof(*addr6));
    addr6.sin6_family = AF_INET6;
    addr6.sin6_port = htons(cfg_port);
    if (daddr &&
    inet_pton(AF_INET6, daddr, &(addr6.sin6_addr)) != 1)
    error(1, 0, "ipv6 parse error: %s", daddr);
    break;
    default:
    error(1, 0, "illegal domain");
    }
    if (cfg_payload_len > max_payload_len)
    error(1, 0, "-s: payload exceeds max (%d)", max_payload_len);
    if (optind != argc - 1)
    usage(argv[0]);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    const char *cfg_test = argv[argc - 1];
    parse_opts(argc, argv);
    if (!strcmp(cfg_test, "tcp"))
    do_test(cfg_family, SOCK_STREAM, 0);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(cfg_test, _arg: "udp")) -> else {
    else if (!strcmp(cfg_test, "udp"))
    do_test(cfg_family, SOCK_DGRAM, 0);
    else
    error(1, 0, "unknown cfg_test %s", cfg_test);
    return 0;
    }
