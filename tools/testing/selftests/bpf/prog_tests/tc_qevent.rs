//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/tc_qevent.c
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

#[no_mangle]
unsafe extern "C" fn blast_udp() {
    static void blast_udp(void)
    {
    let mut dst: sockaddr_in = {};
    char buf[1400] = {};
    int fd, i;
    fd = socket(AF_INET, SOCK_DGRAM, 0);
    if (!ASSERT_GE(fd, 0, "udp socket"))
    return;
    dst.sin_family = AF_INET;
    dst.sin_port = htons(12345);
    inet_pton(AF_INET, IP_RX, &dst.sin_addr);
//
// Push far more than the RED queue can hold. Once qavg crosses qth_min
// every further packet hits the congestion_drop / early_drop qevent.
//
    for (i = 0; i < 50000; i++)
    sendto(fd, buf, sizeof(buf), MSG_DONTWAIT,
    (struct sockaddr *)&dst, sizeof(dst));
    close(fd);
    }
#[no_mangle]
unsafe extern "C" fn run_qevent_redirect(prog: *mut bpf_program, counter: *mut __u64) {
    static void run_qevent_redirect(struct bpf_program *prog, __u64 *counter)
    {
    struct nstoken *tok = core::ptr::null_mut();
    int err;
    SYS_NOFAIL("ip netns del %s", NS_TX);
    SYS_NOFAIL("ip netns del %s", NS_RX);
    unlink(PIN_PATH);
    err = bpf_program__pin(prog, PIN_PATH);
    if (!ASSERT_OK(err, "pin prog"))
    return;
    SYS(unpin,  "ip netns add %s", NS_TX);
    SYS(del_tx, "ip netns add %s", NS_RX);
    SYS(del_rx, "ip -n %s link add veth0 type veth peer name veth1 netns %s", NS_TX, NS_RX);
    SYS(del_rx, "ip -n %s addr add %s/24 dev veth0", NS_TX, IP_TX);
    SYS(del_rx, "ip -n %s link set veth0 up", NS_TX);
    SYS(del_rx, "ip -n %s addr add %s/24 dev veth1", NS_RX, IP_RX);
    SYS(del_rx, "ip -n %s link set veth1 up", NS_RX);
    tok = open_netns(NS_TX);
    if (!ASSERT_OK_PTR(tok, "open_netns"))
    goto del_rx;
    SYS(close_ns, "tc qdisc add dev veth0 root handle 1: htb default 1");
    SYS(close_ns, "tc class add dev veth0 parent 1: classid 1:1 htb rate 1mbit ceil 1mbit");
    if (system("tc qdisc add dev veth0 parent 1:1 handle 11: red "
    "limit 500000 avpkt 1000 probability 1 min 5000 max 6000 "
    "burst 6 qevent early_drop block 10 2>/dev/null")) {
    test__skip();
    goto close_ns;
    }
    if (system("tc filter add block 10 bpf da object-pinned "
    PIN_PATH " 2>/dev/null")) {
    test__skip();
    goto close_ns;
    }
    blast_udp();
    ASSERT_GT(*counter, 0, "qevent classifier ran");
    close_ns:
    close_netns(tok);
    del_rx:
    SYS_NOFAIL("ip netns del %s", NS_RX);
    del_tx:
    SYS_NOFAIL("ip netns del %s", NS_TX);
    unpin:
    bpf_program__unpin(prog, PIN_PATH);
    }
#[no_mangle]
pub unsafe extern "C" fn test_tc_qevent() {
    void test_tc_qevent(void)
    {
    struct test_tc_qevent *skel;
    skel = test_tc_qevent__open_and_load();
    if (!ASSERT_OK_PTR(skel, "open_and_load"))
    return;
    if (test__start_subtest("redirect_verdict"))
    run_qevent_redirect(skel.progs.qevent_redirect_verdict,
    &skel.bss.verdict_calls);
    if (test__start_subtest("redirect_helper"))
    run_qevent_redirect(skel.progs.qevent_redirect_helper,
    &skel.bss.helper_calls);
    test_tc_qevent__destroy(skel);
    }
