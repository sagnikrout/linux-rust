//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/drivers/net/so_txtime.c
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
// Test the SO_TXTIME API
//
// Takes a stream of { payload, delivery time }[], to be sent across two
// processes. Start this program on two separate network namespaces or
// connected hosts, one instance in transmit mode and the other in receive
// mode using the '-r' option. Receiver will compare arrival timestamps to
// the expected stream. Sender will read transmit timestamps from the error
// queue. The streams can differ due to out-of-order delivery and drops.
//
// Macro flag: #define _GNU_SOURCE

    let mut cfg_clockid: static int = CLOCK_TAI;
    let mut cfg_port: static uint16_t = 8000;
    let mut cfg_variance_us: static int = 8000;
    static bool	cfg_machine_slow;
    static uint64_t	cfg_start_time_ns;
    static int	cfg_mark;
    static bool	cfg_rx;
    static uint64_t glob_tstart;
    static uint64_t tdeliver_max;
    static int errors;
// encode one timed transmission (of a 1B payload)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timed_send {
    pub data: c_char,
    pub delay_us: i64,
}

pub const MAX_NUM_PKT: c_int = 8;
    static struct timed_send cfg_buf[MAX_NUM_PKT];
    static int cfg_num_pkt;
    static int cfg_errq_level;
    static int cfg_errq_type;
    static struct sockaddr_storage cfg_dst_addr;
    static struct sockaddr_storage cfg_src_addr;
    static socklen_t cfg_alen;
#[no_mangle]
unsafe extern "C" fn gettime_ns(clock: clockid_t) -> u64 {
    static uint64_t gettime_ns(clockid_t clock)
    {
    struct timespec ts;
    if (clock_gettime(clock, &ts))
    error(1, errno, "gettime");
    return ts.tv_sec * (1000ULL * 1000 * 1000) + ts.tv_nsec;
    }
#[no_mangle]
unsafe extern "C" fn do_send_one(fdt: c_int, ts: *mut timed_send) {
    static void do_send_one(int fdt, struct timed_send *ts)
    {
    char control[CMSG_SPACE(sizeof(uint64_t))];
    let mut msg: msghdr = {0};
    let mut iov: iovec = {0};
    struct cmsghdr *cm;
    uint64_t tdeliver;
    int ret;
    iov.iov_base = &ts.data;
    iov.iov_len = 1;
    msg.msg_iov = &iov;
    msg.msg_iovlen = 1;
    msg.msg_name = (struct sockaddr *)&cfg_dst_addr;
    msg.msg_namelen = cfg_alen;
    if (ts.delay_us >= 0) {
    memset(control, 0, sizeof(control));
    msg.msg_control = &control;
    msg.msg_controllen = sizeof(control);
    tdeliver = glob_tstart + ts.delay_us * 1000;
    tdeliver_max = tdeliver_max > tdeliver ?
    tdeliver_max : tdeliver;
    cm = CMSG_FIRSTHDR(&msg);
    cm.cmsg_level = SOL_SOCKET;
    cm.cmsg_type = SCM_TXTIME;
    cm.cmsg_len = CMSG_LEN(sizeof(tdeliver));
    memcpy(CMSG_DATA(cm), &tdeliver, sizeof(tdeliver));
    }
    ret = sendmsg(fdt, &msg, 0);
    if (ret == -1)
    error(1, errno, "write");
    if (ret == 0)
    error(1, 0, "write: 0B");
    }
#[no_mangle]
unsafe extern "C" fn do_recv_one(fdr: c_int, ts: *mut timed_send) {
    static void do_recv_one(int fdr, struct timed_send *ts)
    {
    int64_t tstop, texpect;
    char rbuf[2];
    int ret;
    ret = recv(fdr, rbuf, sizeof(rbuf), 0);
    if (ret == -1 && errno == EAGAIN)
    error(1, EAGAIN, "recv: timeout");
    if (ret == -1)
    error(1, errno, "read");
    if (ret != 1)
    error(1, 0, "read: %dB", ret);
    tstop = (gettime_ns(cfg_clockid) - glob_tstart) / 1000;
    texpect = ts.delay_us >= 0 ? ts.delay_us : 0;
    fprintf(stderr, "payload:%c delay:%lld expected:%lld (us)\n",
    rbuf[0], (long long)tstop, (long long)texpect);
    if (rbuf[0] != ts.data) {
    fprintf(stderr, "payload mismatch. expected %c\n", ts.data);
    errors++;
    }
    if (llabs(tstop - texpect) > cfg_variance_us) {
    fprintf(stderr, "exceeds variance (%d us)\n", cfg_variance_us);
    if (!cfg_machine_slow)
    errors++;
    }
    }
#[no_mangle]
unsafe extern "C" fn do_recv_verify_empty(fdr: c_int) {
    static void do_recv_verify_empty(int fdr)
    {
    char rbuf[1];
    int ret;
    ret = recv(fdr, rbuf, sizeof(rbuf), 0);
    if (ret != -1 || errno != EAGAIN)
    error(1, 0, "recv: not empty as expected (%d, %d)", ret, errno);
    }
#[no_mangle]
unsafe extern "C" fn do_recv_errqueue_timeout(fdt: c_int) -> c_int {
    static int do_recv_errqueue_timeout(int fdt)
    {
    char control[CMSG_SPACE(sizeof(struct sock_extended_err)) +
    CMSG_SPACE(sizeof(struct sockaddr_in6))] = {0};
    char data[sizeof(struct ethhdr) + sizeof(struct ipv6hdr) +
    sizeof(struct udphdr) + 1];
    struct sock_extended_err *err;
    int ret, num_tstamp = 0;
    let mut msg: msghdr = {0};
    let mut iov: iovec = {0};
    struct cmsghdr *cm;
    let mut tstamp: i64 = 0;
    iov.iov_base = data;
    iov.iov_len = sizeof(data);
    msg.msg_iov = &iov;
    msg.msg_iovlen = 1;
    msg.msg_control = control;
    msg.msg_controllen = sizeof(control);
    while (1) {
    const char *reason = core::ptr::null_mut();
    ret = recvmsg(fdt, &msg, MSG_ERRQUEUE);
    if (ret == -1 && errno == EAGAIN)
    break;
    if (ret == -1)
    error(1, errno, "errqueue");
    if (msg.msg_flags != MSG_ERRQUEUE)
    error(1, 0, "errqueue: flags 0x%x\n", msg.msg_flags);
    cm = CMSG_FIRSTHDR(&msg);
    if (cm.cmsg_level != cfg_errq_level ||
    cm.cmsg_type != cfg_errq_type)
    error(1, 0, "errqueue: type 0x%x.0x%x\n",
    cm.cmsg_level, cm.cmsg_type);
    err = (struct sock_extended_err *)CMSG_DATA(cm);
    if (err.ee_origin != SO_EE_ORIGIN_TXTIME)
    error(1, 0, "errqueue: origin 0x%x\n", err.ee_origin);
    switch (err.ee_errno) {
    case ECANCELED:
    if (err.ee_code != SO_EE_CODE_TXTIME_MISSED)
    error(1, 0, "errqueue: unknown ECANCELED %u\n",
    err.ee_code);
    reason = "missed txtime";
    break;
    case EINVAL:
    if (err.ee_code != SO_EE_CODE_TXTIME_INVALID_PARAM)
    error(1, 0, "errqueue: unknown EINVAL %u\n",
    err.ee_code);
    reason = "invalid txtime";
    break;
    default:
    error(1, 0, "errqueue: errno %u code %u\n",
    err.ee_errno, err.ee_code);
    }
    tstamp = ((int64_t) err.ee_data) << 32 | err.ee_info;
    tstamp -= (int64_t) glob_tstart;
    tstamp /= 1000 * 1000;
    fprintf(stderr, "send: pkt %c at %" PRId64 "ms dropped: %s\n",
    data[ret - 1], tstamp, reason);
    msg.msg_flags = 0;
    msg.msg_controllen = sizeof(control);
    num_tstamp++;
    }
    return num_tstamp;
    }
#[no_mangle]
unsafe extern "C" fn recv_errqueue_msgs(fdt: c_int) {
    static void recv_errqueue_msgs(int fdt)
    {
    let mut pfd: pollfd = { .fd = fdt, .events = POLLERR };
    let mut timeout_ms: c_int = 10;
    int ret, num_tstamp = 0;
    do {
    ret = poll(&pfd, 1, timeout_ms);
    if (ret == -1)
    error(1, errno, "poll");
    if (ret && (pfd.revents & POLLERR))
    num_tstamp += do_recv_errqueue_timeout(fdt);
    if (num_tstamp == cfg_num_pkt)
    break;
    } while (gettime_ns(cfg_clockid) < tdeliver_max);
    }
#[no_mangle]
unsafe extern "C" fn start_time_wait() {
    static void start_time_wait(void)
    {
    uint64_t now;
    int err;
    if (!cfg_start_time_ns)
    return;
    now = gettime_ns(CLOCK_REALTIME);
    if (cfg_start_time_ns < now) {
    fprintf(stderr, "FAIL: start time already passed\n");
    if (!cfg_machine_slow)
    errors++;
    return;
    }
    err = usleep((cfg_start_time_ns - now) / 1000);
    if (err)
    error(1, errno, "usleep");
    }
#[no_mangle]
unsafe extern "C" fn setsockopt_txtime(fd: c_int) {
    static void setsockopt_txtime(int fd)
    {
    let mut so_txtime_val: sock_txtime = { .clockid = cfg_clockid };
    let mut so_txtime_val_read: sock_txtime = { 0 };
    let mut vallen: socklen_t = sizeof(so_txtime_val);
    so_txtime_val.flags = SOF_TXTIME_REPORT_ERRORS;
    if (setsockopt(fd, SOL_SOCKET, SO_TXTIME,
    &so_txtime_val, sizeof(so_txtime_val)))
    error(1, errno, "setsockopt txtime");
    if (getsockopt(fd, SOL_SOCKET, SO_TXTIME,
    &so_txtime_val_read, &vallen))
    error(1, errno, "getsockopt txtime");
    if (vallen != sizeof(so_txtime_val) ||
    memcmp(&so_txtime_val, &so_txtime_val_read, vallen))
    error(1, 0, "getsockopt txtime: mismatch");
    }
#[no_mangle]
unsafe extern "C" fn setup_tx(addr: *mut sockaddr, alen: socklen_t) -> c_int {
    static int setup_tx(struct sockaddr *addr, socklen_t alen)
    {
    int fd;
    fd = socket(addr.sa_family, SOCK_DGRAM, 0);
    if (fd == -1)
    error(1, errno, "socket t");
    if (connect(fd, addr, alen))
    error(1, errno, "connect");
    setsockopt_txtime(fd);
    if (cfg_mark &&
    setsockopt(fd, SOL_SOCKET, SO_MARK, &cfg_mark, sizeof(cfg_mark)))
    error(1, errno, "setsockopt mark");
    return fd;
    }
#[no_mangle]
unsafe extern "C" fn setup_rx(addr: *mut sockaddr, alen: socklen_t) -> c_int {
    static int setup_rx(struct sockaddr *addr, socklen_t alen)
    {
    let mut tv: timeval = { .tv_usec = 100 * 1000 };
    int fd;
    fd = socket(addr.sa_family, SOCK_DGRAM, 0);
    if (fd == -1)
    error(1, errno, "socket r");
    if (bind(fd, addr, alen))
    error(1, errno, "bind");
    if (cfg_machine_slow)
    tv.tv_sec = 2;
    if (setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof(tv)))
    error(1, errno, "setsockopt rcv timeout");
    return fd;
    }
#[no_mangle]
unsafe extern "C" fn do_test_tx(addr: *mut sockaddr, alen: socklen_t) {
    static void do_test_tx(struct sockaddr *addr, socklen_t alen)
    {
    int fdt, i;
    fprintf(stderr, "\nSO_TXTIME ipv%c clock %s\n",
    addr.sa_family == PF_INET ? '4' : '6',
    cfg_clockid == CLOCK_TAI ? "tai" : "monotonic");
    fdt = setup_tx(addr, alen);
    start_time_wait();
    glob_tstart = gettime_ns(cfg_clockid);
    for (i = 0; i < cfg_num_pkt; i++)
    do_send_one(fdt, &cfg_buf[i]);
    recv_errqueue_msgs(fdt);
    if (close(fdt))
    error(1, errno, "close t");
    }
#[no_mangle]
unsafe extern "C" fn do_test_rx(addr: *mut sockaddr, alen: socklen_t) {
    static void do_test_rx(struct sockaddr *addr, socklen_t alen)
    {
    int fdr, i;
    fdr = setup_rx(addr, alen);
    start_time_wait();
    glob_tstart = gettime_ns(cfg_clockid);
    for (i = 0; i < cfg_num_pkt; i++)
    do_recv_one(fdr, &cfg_buf[i]);
    do_recv_verify_empty(fdr);
    if (close(fdr))
    error(1, errno, "close r");
    }
    static void setup_sockaddr(int domain, const char *str_addr,
    struct sockaddr_storage *sockaddr)
    {
    struct sockaddr_in6 *addr6 = (void *) sockaddr;
    struct sockaddr_in *addr4 = (void *) sockaddr;
    switch (domain) {
    case PF_INET:
    memset(addr4, 0, sizeof(*addr4));
    addr4.sin_family = AF_INET;
    addr4.sin_port = htons(cfg_port);
    if (str_addr &&
    inet_pton(AF_INET, str_addr, &(addr4.sin_addr)) != 1)
    error(1, 0, "ipv4 parse error: %s", str_addr);
    break;
    case PF_INET6:
    memset(addr6, 0, sizeof(*addr6));
    addr6.sin6_family = AF_INET6;
    addr6.sin6_port = htons(cfg_port);
    if (str_addr &&
    inet_pton(AF_INET6, str_addr, &(addr6.sin6_addr)) != 1)
    error(1, 0, "ipv6 parse error: %s", str_addr);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn parse_io(optarg: *const c_char, array: *mut timed_send) -> c_int {
    static int parse_io(const char *optarg, struct timed_send *array)
    {
    char *arg, *tok;
    let mut aoff: c_int = 0;
    arg = strdup(optarg);
    if (!arg)
    error(1, errno, "strdup");
    while ((tok = strtok(arg, ","))) {
    arg = core::ptr::null_mut();	/* only pass non-zero on first call */
    if (aoff / 2 == MAX_NUM_PKT)
    error(1, 0, "exceeds max pkt count (%d)", MAX_NUM_PKT);
    if (aoff & 1) {	/* parse delay */
    array.delay_us = strtol(tok, core::ptr::null_mut(), 0) * 1000;
    array++;
    } else {	/* parse character */
    array.data = tok[0];
    }
    aoff++;
    }
    free(arg);
    return aoff / 2;
    }
#[no_mangle]
unsafe extern "C" fn usage(progname: *const c_char) {
    static void usage(const char *progname)
    {
    fprintf(stderr, "\nUsage: %s [options] <payload>\n"
    "Options:\n"
    "  -4            only IPv4\n"
    "  -6            only IPv6\n"
    "  -c <clock>    monotonic or tai (default)\n"
    "  -D <addr>     destination IP address (server)\n"
    "  -S <addr>     source IP address (client)\n"
    "  -r            run rx mode\n"
    "  -t <nsec>     start time (UTC nanoseconds)\n"
    "  -m <mark>     socket mark\n"
    "\n",
    progname);
    exit(1);
    }
#[no_mangle]
unsafe extern "C" fn parse_opts(argc: c_int, argv: *mut c_char) {
    static void parse_opts(int argc, char **argv)
    {
    char *daddr = core::ptr::null_mut(), *saddr = core::ptr::null_mut();
    let mut domain: c_int = PF_UNSPEC;
    int c;
    while ((c = getopt(argc, argv, "46c:S:D:rt:m:")) != -1) {
    switch (c) {
    case '4':
    if (domain != PF_UNSPEC)
    error(1, 0, "Pass one of -4 or -6");
    domain = PF_INET;
    cfg_alen = sizeof(struct sockaddr_in);
    cfg_errq_level = SOL_IP;
    cfg_errq_type = IP_RECVERR;
    break;
    case '6':
    if (domain != PF_UNSPEC)
    error(1, 0, "Pass one of -4 or -6");
    domain = PF_INET6;
    cfg_alen = sizeof(struct sockaddr_in6);
    cfg_errq_level = SOL_IPV6;
    cfg_errq_type = IPV6_RECVERR;
    break;
    case 'c':
    if (!strcmp(optarg, "tai"))
    cfg_clockid = CLOCK_TAI;
    else if (!strcmp(optarg, "monotonic") ||
    !strcmp(optarg, "mono"))
    cfg_clockid = CLOCK_MONOTONIC;
    else
    error(1, 0, "unknown clock id %s", optarg);
    break;
    case 'S':
    saddr = optarg;
    break;
    case 'D':
    daddr = optarg;
    break;
    case 'r':
    cfg_rx = true;
    break;
    case 't':
    cfg_start_time_ns = strtoll(optarg, core::ptr::null_mut(), 0);
    break;
    case 'm':
    cfg_mark = strtol(optarg, core::ptr::null_mut(), 0);
    break;
    default:
    usage(argv[0]);
    }
    }
    if (argc - optind != 1)
    usage(argv[0]);
    if (domain == PF_UNSPEC)
    error(1, 0, "Pass one of -4 or -6");
    if (!daddr)
    error(1, 0, "-D <server addr> required\n");
    if (!cfg_rx && !saddr)
    error(1, 0, "-S <client addr> required\n");
    setup_sockaddr(domain, daddr, &cfg_dst_addr);
    setup_sockaddr(domain, saddr, &cfg_src_addr);
    cfg_num_pkt = parse_io(argv[optind], cfg_buf);
    cfg_machine_slow = getenv("KSFT_MACHINE_SLOW");
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    parse_opts(argc, argv);
    if (cfg_rx)
    do_test_rx((void *)&cfg_dst_addr, cfg_alen);
    else
    do_test_tx((void *)&cfg_src_addr, cfg_alen);
    if (errors) {
    fprintf(stderr, "FAIL: %d errors\n", errors);
    return KSFT_FAIL;
    }
    return KSFT_PASS;
    }
