//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/lwt_redirect.c
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
// Test suite of lwt_xmit BPF programs that redirect packets
// The file tests focus not only if these programs work as expected normally,
// but also if they can handle abnormal situations gracefully.
//
// WARNING
// -------
// This test suite may crash the kernel, thus should be run in a VM.
//
// Setup:
// ---------
// All tests are performed in a single netns. Two lwt encap routes are setup for
// each subtest:
//
// ip route add 10.0.0.0/24 encap bpf xmit <obj> sec "<ingress_sec>" dev link_err
// ip route add 20.0.0.0/24 encap bpf xmit <obj> sec "<egress_sec>" dev link_err
//
// Here <obj> is statically defined to test_lwt_redirect.bpf.o, and each section
// of this object holds a program entry to test. The BPF object is built from
// progs/test_lwt_redirect.c. We didn't use generated BPF skeleton since the
// attachment for lwt programs are not supported by libbpf yet.
//
// For testing, ping commands are run in the test netns:
//
// ping 10.0.0.<ifindex> -c 1 -w 1 -s 100
// ping 20.0.0.<ifindex> -c 1 -w 1 -s 100
//
// Scenarios:
// --------------------------------
// 1. Redirect to a running tap/tun device
// 2. Redirect to a down tap/tun device
// 3. Redirect to a vlan device with lower layer down
//
// Case 1, ping packets should be received by packet socket on target device
// when redirected to ingress, and by tun/tap fd when redirected to egress.
//
// Case 2,3 are considered successful as long as they do not crash the kernel
// as a regression.
//
// Case 1,2 use tap device to test redirect to device that requires MAC
// header, and tun device to test the case with no MAC header added.
//

// ping to redirect toward given dev, with last byte of dest IP being the target
// device index.
//
// Note: ping command inside BPF-CI is busybox version, so it does not have certain
// function, such like -m option to set packet mark.
//
#[no_mangle]
unsafe extern "C" fn ping_dev(dev: *const c_char, is_ingress: bool) {
    static void ping_dev(const char *dev, bool is_ingress)
    {
    let mut link_index: c_int = if_nametoindex(dev);
    char ip[256];
    if (!ASSERT_GE(link_index, 0, "if_nametoindex"))
    return;
    if (is_ingress)
    snprintf(ip, sizeof(ip), "10.0.0.%d", link_index);
    else
    snprintf(ip, sizeof(ip), "20.0.0.%d", link_index);
// We won't get a reply. Don't fail here
    SYS_NOFAIL("ping %s -c1 -W1 -s %d",
    ip, ICMP_PAYLOAD_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn new_packet_sock(ifname: *const c_char) -> c_int {
    static int new_packet_sock(const char *ifname)
    {
    let mut err: c_int = 0;
    let mut ignore_outgoing: c_int = 1;
    let mut ifindex: c_int = -1;
    let mut s: c_int = -1;
    s = socket(AF_PACKET, SOCK_RAW, 0);
    if (!ASSERT_GE(s, 0, "socket(AF_PACKET)"))
    return -1;
    ifindex = if_nametoindex(ifname);
    if (!ASSERT_GE(ifindex, 0, "if_nametoindex")) {
    close(s);
    return -1;
    }
    struct sockaddr_ll addr = {
    .sll_family = AF_PACKET,
    .sll_protocol = htons(ETH_P_IP),
    .sll_ifindex = ifindex,
    };
    err = bind(s, (struct sockaddr *)&addr, sizeof(addr));
    if (!ASSERT_OK(err, "bind(AF_PACKET)")) {
    close(s);
    return -1;
    }
// Use packet socket to capture only the ingress, so we can distinguish
// the case where a regression that actually redirects the packet to
// the egress.
//
    err = setsockopt(s, SOL_PACKET, PACKET_IGNORE_OUTGOING,
    &ignore_outgoing, sizeof(ignore_outgoing));
    if (!ASSERT_OK(err, "setsockopt(PACKET_IGNORE_OUTGOING)")) {
    close(s);
    return -1;
    }
    err = fcntl(s, F_SETFL, O_NONBLOCK);
    if (!ASSERT_OK(err, "fcntl(O_NONBLOCK)")) {
    close(s);
    return -1;
    }
    return s;
    }
#[no_mangle]
unsafe extern "C" fn expect_icmp(buf: *mut c_char, len: isize) -> c_int {
    static int expect_icmp(char *buf, ssize_t len)
    {
    struct ethhdr *eth = (struct ethhdr *)buf;
    if (len < (ssize_t)sizeof(*eth))
    return -1;
    if (eth.h_proto == htons(ETH_P_IP))
    return __expect_icmp_ipv4((char *)(eth + 1), len - sizeof(*eth));
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn expect_icmp_nomac(buf: *mut c_char, len: isize) -> c_int {
    static int expect_icmp_nomac(char *buf, ssize_t len)
    {
    return __expect_icmp_ipv4(buf, len);
    }
    static void send_and_capture_test_packets(const char *test_name, int tap_fd,
    const char *target_dev, bool need_mac)
    {
    let mut psock: c_int = -1;
    struct timeval timeo = {
    .tv_sec = 0,
    .tv_usec = 250000,
    };
    let mut ret: c_int = -1;
    let mut filter: filter_t = need_mac ? expect_icmp : expect_icmp_nomac;
    ping_dev(target_dev, false);
    ret = wait_for_packet(tap_fd, filter, &timeo);
    if (!ASSERT_EQ(ret, 1, "wait_for_epacket")) {
    log_err("%s egress test fails", test_name);
    goto out;
    }
    psock = new_packet_sock(target_dev);
    ping_dev(target_dev, true);
    ret = wait_for_packet(psock, filter, &timeo);
    if (!ASSERT_EQ(ret, 1, "wait_for_ipacket")) {
    log_err("%s ingress test fails", test_name);
    goto out;
    }
    out:
    if (psock >= 0)
    close(psock);
    }
#[no_mangle]
unsafe extern "C" fn setup_redirect_target(target_dev: *const c_char, need_mac: bool) -> c_int {
    static int setup_redirect_target(const char *target_dev, bool need_mac)
    {
    let mut target_index: c_int = -1;
    let mut tap_fd: c_int = -1;
    tap_fd = open_tuntap(target_dev, need_mac);
    if (!ASSERT_GE(tap_fd, 0, "open_tuntap"))
    goto fail;
    target_index = if_nametoindex(target_dev);
    if (!ASSERT_GE(target_index, 0, "if_nametoindex"))
    goto fail;
    SYS(fail, "sysctl -w net.ipv6.conf.all.disable_ipv6=1");
    SYS(fail, "ip link add link_err type dummy");
    SYS(fail, "ip link set lo up");
    SYS(fail, "ip addr add dev lo " LOCAL_SRC "/32");
    SYS(fail, "ip link set link_err up");
    SYS(fail, "ip link set %s up", target_dev);
    SYS(fail, "ip route add %s dev link_err encap bpf xmit obj %s sec %s",
    CIDR_TO_INGRESS, BPF_OBJECT, INGRESS_SEC(need_mac));
    SYS(fail, "ip route add %s dev link_err encap bpf xmit obj %s sec %s",
    CIDR_TO_EGRESS, BPF_OBJECT, EGRESS_SEC(need_mac));
    return tap_fd;
    fail:
    if (tap_fd >= 0)
    close(tap_fd);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn test_lwt_redirect_normal() {
    static void test_lwt_redirect_normal(void)
    {
    const char *target_dev = "tap0";
    let mut tap_fd: c_int = -1;
    let mut need_mac: bool = true;
    tap_fd = setup_redirect_target(target_dev, need_mac);
    if (!ASSERT_GE(tap_fd, 0, "setup_redirect_target"))
    return;
    send_and_capture_test_packets(__func__, tap_fd, target_dev, need_mac);
    close(tap_fd);
    }
#[no_mangle]
unsafe extern "C" fn test_lwt_redirect_normal_nomac() {
    static void test_lwt_redirect_normal_nomac(void)
    {
    const char *target_dev = "tun0";
    let mut tap_fd: c_int = -1;
    let mut need_mac: bool = false;
    tap_fd = setup_redirect_target(target_dev, need_mac);
    if (!ASSERT_GE(tap_fd, 0, "setup_redirect_target"))
    return;
    send_and_capture_test_packets(__func__, tap_fd, target_dev, need_mac);
    close(tap_fd);
    }
// This test aims to prevent regression of future. As long as the kernel does
// not panic, it is considered as success.
//
#[no_mangle]
unsafe extern "C" fn __test_lwt_redirect_dev_down(need_mac: bool) {
    static void __test_lwt_redirect_dev_down(bool need_mac)
    {
    const char *target_dev = "tap0";
    let mut tap_fd: c_int = -1;
    tap_fd = setup_redirect_target(target_dev, need_mac);
    if (!ASSERT_GE(tap_fd, 0, "setup_redirect_target"))
    return;
    SYS(out, "ip link set %s down", target_dev);
    ping_dev(target_dev, true);
    ping_dev(target_dev, false);
    out:
    close(tap_fd);
    }
#[no_mangle]
unsafe extern "C" fn test_lwt_redirect_dev_down() {
    static void test_lwt_redirect_dev_down(void)
    {
    __test_lwt_redirect_dev_down(true);
    }
#[no_mangle]
unsafe extern "C" fn test_lwt_redirect_dev_down_nomac() {
    static void test_lwt_redirect_dev_down_nomac(void)
    {
    __test_lwt_redirect_dev_down(false);
    }
// This test aims to prevent regression of future. As long as the kernel does
// not panic, it is considered as success.
//
#[no_mangle]
unsafe extern "C" fn test_lwt_redirect_dev_carrier_down() {
    static void test_lwt_redirect_dev_carrier_down(void)
    {
    const char *lower_dev = "tap0";
    const char *vlan_dev = "vlan100";
    let mut tap_fd: c_int = -1;
    tap_fd = setup_redirect_target(lower_dev, true);
    if (!ASSERT_GE(tap_fd, 0, "setup_redirect_target"))
    return;
    SYS(out, "ip link add vlan100 link %s type vlan id 100", lower_dev);
    SYS(out, "ip link set %s up", vlan_dev);
    SYS(out, "ip link set %s down", lower_dev);
    ping_dev(vlan_dev, true);
    ping_dev(vlan_dev, false);
    out:
    close(tap_fd);
    }
    static void *test_lwt_redirect_run(void *arg)
    {
    netns_delete();
    RUN_TEST(lwt_redirect_normal);
    RUN_TEST(lwt_redirect_normal_nomac);
    RUN_TEST(lwt_redirect_dev_down);
    RUN_TEST(lwt_redirect_dev_down_nomac);
    RUN_TEST(lwt_redirect_dev_carrier_down);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn test_lwt_redirect() {
    void test_lwt_redirect(void)
    {
    pthread_t test_thread;
    int err;
// Run the tests in their own thread to isolate the namespace changes
// so they do not affect the environment of other tests.
// (specifically needed because of unshare(CLONE_NEWNS) in open_netns())
//
    err = pthread_create(&test_thread, core::ptr::null_mut(), &test_lwt_redirect_run, core::ptr::null_mut());
    if (ASSERT_OK(err, "pthread_create"))
    ASSERT_OK(pthread_join(test_thread, core::ptr::null_mut()), "pthread_join");
    }
