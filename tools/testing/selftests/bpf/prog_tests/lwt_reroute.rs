//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/lwt_reroute.c
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
// Test suite of lwt BPF programs that reroutes packets
// The file tests focus not only if these programs work as expected normally,
// but also if they can handle abnormal situations gracefully. This test
// suite currently only covers lwt_xmit hook. lwt_in tests have not been
// implemented.
//
// WARNING
// -------
// This test suite can crash the kernel, thus should be run in a VM.
//
// Setup:
// ---------
// all tests are performed in a single netns. A lwt encap route is setup for
// each subtest:
//
// ip route add 10.0.0.0/24 encap bpf xmit <obj> sec "<section_N>" dev link_err
//
// Here <obj> is statically defined to test_lwt_reroute.bpf.o, and it contains
// a single test program entry. This program sets packet mark by last byte of
// the IPv4 daddr. For example, a packet going to 1.2.3.4 will receive a skb
// mark 4. A packet will only be marked once, and IP x.x.x.0 will be skipped
// to avoid route loop. We didn't use generated BPF skeleton since the
// attachment for lwt programs are not supported by libbpf yet.
//
// The test program will bring up a tun device, and sets up the following
// routes:
//
// ip rule add pref 100 from all fwmark <tun_index> lookup 100
// ip route add table 100 default dev tun0
//
// For normal testing, a ping command is running in the test netns:
//
// ping 10.0.0.<tun_index> -c 1 -w 1 -s 100
//
// For abnormal testing, fq is used as the qdisc of the tun device. Then a UDP
// socket will try to overflow the fq queue and trigger qdisc drop error.
//
// Scenarios:
// --------------------------------
// 1. Reroute to a running tun device
// 2. Reroute to a device where qdisc drop
//
// For case 1, ping packets should be received by the tun device.
//
// For case 2, force UDP packets to overflow fq limit. As long as kernel
// is not crashed, it is considered successful.
//

// send a ping to be rerouted to the target device
#[no_mangle]
unsafe extern "C" fn ping_once(ip: *const c_char) {
    static void ping_once(const char *ip)
    {
// We won't get a reply. Don't fail here
    SYS_NOFAIL("ping %s -c1 -W1 -s %d",
    ip, ICMP_PAYLOAD_SIZE);
    }
// Send snd_target UDP packets to overflow the fq queue and trigger qdisc drop
// error. This is done via TX tstamp to force buffering delayed packets.
//
#[no_mangle]
unsafe extern "C" fn overflow_fq(snd_target: c_int, target_ip: *const c_char) -> c_int {
    static int overflow_fq(int snd_target, const char *target_ip)
    {
    struct sockaddr_in addr = {
    .sin_family = AF_INET,
    .sin_port = htons(1234),
    };
    char data_buf[8]; /* only #pkts matter, so use a random small buffer */
    char control_buf[CMSG_SPACE(sizeof(uint64_t))];
    struct iovec iov = {
    .iov_base = data_buf,
    .iov_len = sizeof(data_buf),
    };
    let mut err: c_int = -1;
    let mut s: c_int = -1;
    struct sock_txtime txtime_on = {
    .clockid = CLOCK_MONOTONIC,
    .flags = 0,
    };
    struct msghdr msg = {
    .msg_name = &addr,
    .msg_namelen = sizeof(addr),
    .msg_control = control_buf,
    .msg_controllen = sizeof(control_buf),
    .msg_iovlen = 1,
    .msg_iov = &iov,
    };
    struct cmsghdr *cmsg = CMSG_FIRSTHDR(&msg);
    memset(data_buf, 0, sizeof(data_buf));
    s = socket(AF_INET, SOCK_DGRAM, 0);
    if (!ASSERT_GE(s, 0, "socket"))
    goto out;
    err = setsockopt(s, SOL_SOCKET, SO_TXTIME, &txtime_on, sizeof(txtime_on));
    if (!ASSERT_OK(err, "setsockopt(SO_TXTIME)"))
    goto out;
    err = inet_pton(AF_INET, target_ip, &addr.sin_addr);
    if (!ASSERT_EQ(err, 1, "inet_pton"))
    goto out;
    while (snd_target > 0) {
    struct timespec now;
    memset(control_buf, 0, sizeof(control_buf));
    cmsg.cmsg_type = SCM_TXTIME;
    cmsg.cmsg_level = SOL_SOCKET;
    cmsg.cmsg_len = CMSG_LEN(sizeof(uint64_t));
    err = clock_gettime(CLOCK_MONOTONIC, &now);
    if (!ASSERT_OK(err, "clock_gettime(CLOCK_MONOTONIC)")) {
    err = -1;
    goto out;
    }
// (uint64_t *)CMSG_DATA(cmsg) = (now.tv_nsec + 1) * NSEC_PER_SEC +
    now.tv_nsec;
// we will intentionally send more than fq limit, so ignore
// the error here.
//
    sendmsg(s, &msg, MSG_NOSIGNAL);
    snd_target--;
    }
// no kernel crash so far is considered success
    err = 0;
    out:
    if (s >= 0)
    close(s);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn setup(tun_dev: *const c_char) -> c_int {
    static int setup(const char *tun_dev)
    {
    let mut target_index: c_int = -1;
    let mut tap_fd: c_int = -1;
    tap_fd = open_tuntap(tun_dev, false);
    if (!ASSERT_GE(tap_fd, 0, "open_tun"))
    return -1;
    target_index = if_nametoindex(tun_dev);
    if (!ASSERT_GE(target_index, 0, "if_nametoindex"))
    return -1;
    SYS(fail, "ip link add link_err type dummy");
    SYS(fail, "ip link set lo up");
    SYS(fail, "ip addr add dev lo " LOCAL_SRC "/32");
    SYS(fail, "ip link set link_err up");
    SYS(fail, "ip link set %s up", tun_dev);
    SYS(fail, "ip route add %s dev link_err encap bpf xmit obj %s sec lwt_xmit",
    TEST_CIDR, BPF_OBJECT);
    SYS(fail, "ip rule add pref 100 from all fwmark %d lookup 100",
    target_index);
    SYS(fail, "ip route add t 100 default dev %s", tun_dev);
    return tap_fd;
    fail:
    if (tap_fd >= 0)
    close(tap_fd);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn test_lwt_reroute_normal_xmit() {
    static void test_lwt_reroute_normal_xmit(void)
    {
    const char *tun_dev = "tun0";
    let mut tun_fd: c_int = -1;
    let mut ifindex: c_int = -1;
    char ip[256];
    struct timeval timeo = {
    .tv_sec = 0,
    .tv_usec = 250000,
    };
    tun_fd = setup(tun_dev);
    if (!ASSERT_GE(tun_fd, 0, "setup_reroute"))
    return;
    ifindex = if_nametoindex(tun_dev);
    if (!ASSERT_GE(ifindex, 0, "if_nametoindex"))
    return;
    snprintf(ip, 256, "10.0.0.%d", ifindex);
// ping packets should be received by the tun device
    ping_once(ip);
    if (!ASSERT_EQ(wait_for_packet(tun_fd, __expect_icmp_ipv4, &timeo), 1,
    "wait_for_packet"))
    log_err("%s xmit", __func__);
    }
//
// Test the failure case when the skb is dropped at the qdisc. This is a
// regression prevention at the xmit hook only.
//
#[no_mangle]
unsafe extern "C" fn test_lwt_reroute_qdisc_dropped() {
    static void test_lwt_reroute_qdisc_dropped(void)
    {
    const char *tun_dev = "tun0";
    let mut tun_fd: c_int = -1;
    let mut ifindex: c_int = -1;
    char ip[256];
    tun_fd = setup(tun_dev);
    if (!ASSERT_GE(tun_fd, 0, "setup_reroute"))
    goto fail;
    SYS(fail, "tc qdisc replace dev %s root fq limit 5 flow_limit 5", tun_dev);
    ifindex = if_nametoindex(tun_dev);
    if (!ASSERT_GE(ifindex, 0, "if_nametoindex"))
    return;
    snprintf(ip, 256, "10.0.0.%d", ifindex);
    ASSERT_EQ(overflow_fq(10, ip), 0, "overflow_fq");
    fail:
    if (tun_fd >= 0)
    close(tun_fd);
    }
    static void *test_lwt_reroute_run(void *arg)
    {
    netns_delete();
    RUN_TEST(lwt_reroute_normal_xmit);
    RUN_TEST(lwt_reroute_qdisc_dropped);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn test_lwt_reroute() {
    void test_lwt_reroute(void)
    {
    pthread_t test_thread;
    int err;
// Run the tests in their own thread to isolate the namespace changes
// so they do not affect the environment of other tests.
// (specifically needed because of unshare(CLONE_NEWNS) in open_netns())
//
    err = pthread_create(&test_thread, core::ptr::null_mut(), &test_lwt_reroute_run, core::ptr::null_mut());
    if (ASSERT_OK(err, "pthread_create"))
    ASSERT_OK(pthread_join(test_thread, core::ptr::null_mut()), "pthread_join");
    }
