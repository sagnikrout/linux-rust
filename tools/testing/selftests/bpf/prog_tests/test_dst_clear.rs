//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_dst_clear.c
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
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

pub const UDP_TEST_PORT: c_int = 7777;
#[no_mangle]
pub unsafe extern "C" fn test_ns_dst_clear() {
    void test_ns_dst_clear(void)
    {
    LIBBPF_OPTS(bpf_tcx_opts, tcx_opts);
    struct test_dst_clear *skel;
    struct sockaddr_in addr;
    struct bpf_link *link;
    socklen_t addrlen;
    char buf[128] = {};
    int sockfd, err;
    skel = test_dst_clear__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel open_and_load"))
    return;
    SYS(fail, "ip addr add %s/8 dev lo", IPV4_IFACE_ADDR);
    link = bpf_program__attach_tcx(skel.progs.dst_clear,
    if_nametoindex("lo"), &tcx_opts);
    if (!ASSERT_OK_PTR(link, "attach_tcx"))
    goto fail;
    skel.links.dst_clear = link;
    addrlen = sizeof(addr);
    err = make_sockaddr(AF_INET, IPV4_IFACE_ADDR, UDP_TEST_PORT,
    (void *)&addr, &addrlen);
    if (!ASSERT_OK(err, "make_sockaddr"))
    goto fail;
    sockfd = socket(AF_INET, SOCK_DGRAM, 0);
    if (!ASSERT_NEQ(sockfd, -1, "socket"))
    goto fail;
    err = sendto(sockfd, buf, sizeof(buf), 0, (void *)&addr, addrlen);
    close(sockfd);
    if (!ASSERT_EQ(err, sizeof(buf), "send"))
    goto fail;
    ASSERT_TRUE(skel.bss.had_dst, "had_dst");
    ASSERT_TRUE(skel.bss.dst_cleared, "dst_cleared");
    fail:
    test_dst_clear__destroy(skel);
    }
