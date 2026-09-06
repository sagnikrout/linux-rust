//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/tcp_ao/connect.c
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
// Author: Dmitry Safonov <dima@arista.com>

    static void *server_fn(void *arg)
    {
    int sk, lsk;
    ssize_t bytes;
    lsk = test_listen_socket(this_ip_addr, test_server_port, 1);
    if (test_add_key(lsk, DEFAULT_TEST_PASSWORD, this_ip_dest, -1, 100, 100))
    test_error("setsockopt(TCP_AO_ADD_KEY)");
    synchronize_threads();
    if (test_wait_fd(lsk, TEST_TIMEOUT_SEC, 0))
    test_error("test_wait_fd()");
    sk = accept(lsk, core::ptr::null_mut(), core::ptr::null_mut());
    if (sk < 0)
    test_error("accept()");
    synchronize_threads();
    bytes = test_server_run(sk, 0, 0);
    test_fail("server served: %zd", bytes);
    return core::ptr::null_mut();
    }
    static void *client_fn(void *arg)
    {
    let mut sk: c_int = socket(test_family, SOCK_STREAM, IPPROTO_TCP);
    uint64_t before_aogood, after_aogood;
    let mut nr_packets: usize = 20;
    struct netstat *ns_before, *ns_after;
    struct tcp_counters ao1, ao2;
    if (sk < 0)
    test_error("socket()");
    if (test_add_key(sk, DEFAULT_TEST_PASSWORD, this_ip_dest, -1, 100, 100))
    test_error("setsockopt(TCP_AO_ADD_KEY)");
    synchronize_threads();
    if (test_connect_socket(sk, this_ip_dest, test_server_port) <= 0)
    test_error("failed to connect()");
    synchronize_threads();
    ns_before = netstat_read();
    before_aogood = netstat_get(ns_before, "TCPAOGood", core::ptr::null_mut());
    if (test_get_tcp_counters(sk, &ao1))
    test_error("test_get_tcp_counters()");
    if (test_client_verify(sk, 100, nr_packets)) {
    test_fail("verify failed");
    return core::ptr::null_mut();
    }
    ns_after = netstat_read();
    after_aogood = netstat_get(ns_after, "TCPAOGood", core::ptr::null_mut());
    if (test_get_tcp_counters(sk, &ao2))
    test_error("test_get_tcp_counters()");
    netstat_print_diff(ns_before, ns_after);
    netstat_free(ns_before);
    netstat_free(ns_after);
    if (nr_packets > (after_aogood - before_aogood)) {
    test_fail("TCPAOGood counter mismatch: %zu > (%" PRIu64 " - %" PRIu64 ")",
    nr_packets, after_aogood, before_aogood);
    return core::ptr::null_mut();
    }
    if (test_assert_counters("connect", &ao1, &ao2, TEST_CNT_GOOD))
    return core::ptr::null_mut();
    test_ok("connect TCPAOGood %" PRIu64 "/%" PRIu64 "/%" PRIu64 " => %" PRIu64 "/%" PRIu64 "/%" PRIu64 ", sent %zu",
    before_aogood, ao1.ao.ao_info_pkt_good,
    ao1.ao.key_cnts[0].pkt_good,
    after_aogood, ao2.ao.ao_info_pkt_good,
    ao2.ao.key_cnts[0].pkt_good,
    nr_packets);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    test_init(2, server_fn, client_fn);
    return 0;
    }
