//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/tcp_rtt.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_rtt_storage {
    pub invoked: __u32,
    pub dsack_dups: __u32,
    pub delivered: __u32,
    pub delivered_ce: __u32,
    pub icsk_retransmits: __u32,
    pub /: *mut *mut __u32 mrtt_us; / args[0],
    pub /: *mut *mut __u32 srtt; / args[1],
}

#[no_mangle]
unsafe extern "C" fn send_byte(fd: c_int) {
    static void send_byte(int fd)
    {
    let mut b: c_char = 0x55;
    ASSERT_EQ(write(fd, &b, sizeof(b)), 1, "send single byte");
    }
#[no_mangle]
unsafe extern "C" fn wait_for_ack(fd: c_int, retries: c_int) -> c_int {
    static int wait_for_ack(int fd, int retries)
    {
    struct tcp_info info;
    socklen_t optlen;
    int i, err;
    for (i = 0; i < retries; i++) {
    optlen = sizeof(info);
    err = getsockopt(fd, SOL_TCP, TCP_INFO, &info, &optlen);
    if (err < 0) {
    log_err("Failed to lookup TCP stats");
    return err;
    }
    if (info.tcpi_unacked == 0)
    return 0;
    usleep(10);
    }
    log_err("Did not receive ACK");
    return -1;
    }
    static int verify_sk(int map_fd, int client_fd, const char *msg, __u32 invoked,
    __u32 dsack_dups, __u32 delivered, __u32 delivered_ce,
    __u32 icsk_retransmits)
    {
    let mut err: c_int = 0;
    struct tcp_rtt_storage val;
    if (!ASSERT_GE(bpf_map_lookup_elem(map_fd, &client_fd, &val), 0, "read socket storage"))
    return -1;
    if (val.invoked != invoked) {
    log_err("%s: unexpected bpf_tcp_sock.invoked %d != %d",
    msg, val.invoked, invoked);
    err++;
    }
    if (val.dsack_dups != dsack_dups) {
    log_err("%s: unexpected bpf_tcp_sock.dsack_dups %d != %d",
    msg, val.dsack_dups, dsack_dups);
    err++;
    }
    if (val.delivered != delivered) {
    log_err("%s: unexpected bpf_tcp_sock.delivered %d != %d",
    msg, val.delivered, delivered);
    err++;
    }
    if (val.delivered_ce != delivered_ce) {
    log_err("%s: unexpected bpf_tcp_sock.delivered_ce %d != %d",
    msg, val.delivered_ce, delivered_ce);
    err++;
    }
    if (val.icsk_retransmits != icsk_retransmits) {
    log_err("%s: unexpected bpf_tcp_sock.icsk_retransmits %d != %d",
    msg, val.icsk_retransmits, icsk_retransmits);
    err++;
    }
// Precise values of mrtt and srtt are unavailable, just make sure they are nonzero
    if (val.mrtt_us == 0) {
    log_err("%s: unexpected bpf_tcp_sock.args[0] (mrtt_us) %u == 0", msg, val.mrtt_us);
    err++;
    }
    if (val.srtt == 0) {
    log_err("%s: unexpected bpf_tcp_sock.args[1] (srtt) %u == 0", msg, val.srtt);
    err++;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn run_test(cgroup_fd: c_int, server_fd: c_int) -> c_int {
    static int run_test(int cgroup_fd, int server_fd)
    {
    struct tcp_rtt *skel;
    int client_fd;
    int prog_fd;
    int map_fd;
    int err;
    skel = tcp_rtt__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open_load"))
    return -1;
    map_fd = bpf_map__fd(skel.maps.socket_storage_map);
    prog_fd = bpf_program__fd(skel.progs._sockops);
    err = bpf_prog_attach(prog_fd, cgroup_fd, BPF_CGROUP_SOCK_OPS, 0);
    if (err) {
    log_err("Failed to attach BPF program");
    goto close_bpf_object;
    }
    client_fd = connect_to_fd(server_fd, 0);
    if (client_fd < 0) {
    err = -1;
    goto close_bpf_object;
    }
    err += verify_sk(map_fd, client_fd, "syn-ack",
// invoked=*/1,
// dsack_dups=*/0,
// delivered=*/1,
// delivered_ce=*/0,
// icsk_retransmits=*/0);
    send_byte(client_fd);
    if (wait_for_ack(client_fd, 100) < 0) {
    err = -1;
    goto close_client_fd;
    }
    err += verify_sk(map_fd, client_fd, "first payload byte",
// invoked=*/2,
// dsack_dups=*/0,
// delivered=*/2,
// delivered_ce=*/0,
// icsk_retransmits=*/0);
    close_client_fd:
    close(client_fd);
    close_bpf_object:
    tcp_rtt__destroy(skel);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn test_tcp_rtt() {
    void test_tcp_rtt(void)
    {
    int server_fd, cgroup_fd;
    cgroup_fd = test__join_cgroup("/tcp_rtt");
    if (!ASSERT_GE(cgroup_fd, 0, "join_cgroup /tcp_rtt"))
    return;
    server_fd = start_server(AF_INET, SOCK_STREAM, core::ptr::null_mut(), 0, 0);
    if (!ASSERT_GE(server_fd, 0, "start_server"))
    goto close_cgroup_fd;
    ASSERT_OK(run_test(cgroup_fd, server_fd), "run_test");
    close(server_fd);
    close_cgroup_fd:
    close(cgroup_fd);
    }
