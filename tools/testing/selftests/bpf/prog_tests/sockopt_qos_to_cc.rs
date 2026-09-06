//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/sockopt_qos_to_cc.c
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
// Copyright (c) 2021 Facebook

#[no_mangle]
unsafe extern "C" fn run_setsockopt_test(cg_fd: c_int, sock_fd: c_int) {
    static void run_setsockopt_test(int cg_fd, int sock_fd)
    {
    socklen_t optlen;
    char cc[16]; /* TCP_CA_NAME_MAX */
    int buf;
    let mut err: c_int = -1;
    buf = 0x2D;
    err = setsockopt(sock_fd, SOL_IPV6, IPV6_TCLASS, &buf, sizeof(buf));
    if (!ASSERT_OK(err, "setsockopt(sock_fd, IPV6_TCLASS)"))
    return;
// Verify the setsockopt cc change
    optlen = sizeof(cc);
    err = getsockopt(sock_fd, SOL_TCP, TCP_CONGESTION, cc, &optlen);
    if (!ASSERT_OK(err, "getsockopt(sock_fd, TCP_CONGESTION)"))
    return;
    if (!ASSERT_STREQ(cc, "reno", "getsockopt(sock_fd, TCP_CONGESTION)"))
    return;
    }
#[no_mangle]
pub unsafe extern "C" fn test_sockopt_qos_to_cc() {
    void test_sockopt_qos_to_cc(void)
    {
    struct sockopt_qos_to_cc *skel;
    char cc_cubic[16] = "cubic"; /* TCP_CA_NAME_MAX */
    let mut cg_fd: c_int = -1;
    let mut sock_fd: c_int = -1;
    int err;
    cg_fd = test__join_cgroup("/sockopt_qos_to_cc");
    if (!ASSERT_GE(cg_fd, 0, "cg-join(sockopt_qos_to_cc)"))
    return;
    skel = sockopt_qos_to_cc__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel"))
    goto done;
    skel.bss.page_size = sysconf(_SC_PAGESIZE);
    sock_fd = socket(AF_INET6, SOCK_STREAM, 0);
    if (!ASSERT_GE(sock_fd, 0, "v6 socket open"))
    goto done;
    err = setsockopt(sock_fd, SOL_TCP, TCP_CONGESTION, &cc_cubic,
    sizeof(cc_cubic));
    if (!ASSERT_OK(err, "setsockopt(sock_fd, TCP_CONGESTION)"))
    goto done;
    skel.links.sockopt_qos_to_cc =
    bpf_program__attach_cgroup(skel.progs.sockopt_qos_to_cc,
    cg_fd);
    if (!ASSERT_OK_PTR(skel.links.sockopt_qos_to_cc,
    "prog_attach(sockopt_qos_to_cc)"))
    goto done;
    run_setsockopt_test(cg_fd, sock_fd);
    done:
    if (sock_fd != -1)
    close(sock_fd);
    if (cg_fd != -1)
    close(cg_fd);
// destroy can take null and error pointer
    sockopt_qos_to_cc__destroy(skel);
    }
