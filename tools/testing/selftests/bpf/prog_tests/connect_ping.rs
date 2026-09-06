//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/connect_ping.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2022 Google LLC.
//
// Macro flag: #define _GNU_SOURCE

// 2001:db8::1

    let mut bindaddr_v6: static struct in6_addr = BINDADDR_V6;
    static void subtest(int cgroup_fd, struct connect_ping *skel,
    int family, int do_bind)
    {
    struct sockaddr_in sa4 = {
    .sin_family = AF_INET,
    .sin_addr.s_addr = htonl(INADDR_LOOPBACK),
    };
    struct sockaddr_in6 sa6 = {
    .sin6_family = AF_INET6,
    .sin6_addr = IN6ADDR_LOOPBACK_INIT,
    };
    struct sockaddr *sa = core::ptr::null_mut();
    socklen_t sa_len;
    let mut protocol: c_int = -1;
    int sock_fd;
    switch (family) {
    case AF_INET:
    sa = (struct sockaddr *)&sa4;
    sa_len = sizeof(sa4);
    protocol = IPPROTO_ICMP;
    break;
    case AF_INET6:
    sa = (struct sockaddr *)&sa6;
    sa_len = sizeof(sa6);
    protocol = IPPROTO_ICMPV6;
    break;
    }
    memset(skel.bss, 0, sizeof(*skel.bss));
    skel.bss.do_bind = do_bind;
    sock_fd = socket(family, SOCK_DGRAM, protocol);
    if (!ASSERT_GE(sock_fd, 0, "sock-create"))
    return;
    if (!ASSERT_OK(connect(sock_fd, sa, sa_len), "connect"))
    goto close_sock;
    if (!ASSERT_EQ(skel.bss.invocations_v4, family == AF_INET ? 1 : 0,
    "invocations_v4"))
    goto close_sock;
    if (!ASSERT_EQ(skel.bss.invocations_v6, family == AF_INET6 ? 1 : 0,
    "invocations_v6"))
    goto close_sock;
    if (!ASSERT_EQ(skel.bss.has_error, 0, "has_error"))
    goto close_sock;
    if (!ASSERT_OK(getsockname(sock_fd, sa, &sa_len),
    "getsockname"))
    goto close_sock;
    switch (family) {
    case AF_INET:
    if (!ASSERT_EQ(sa4.sin_family, family, "sin_family"))
    goto close_sock;
    if (!ASSERT_EQ(sa4.sin_addr.s_addr,
    htonl(do_bind ? 0x01010101 : INADDR_LOOPBACK),
    "sin_addr"))
    goto close_sock;
    break;
    case AF_INET6:
    if (!ASSERT_EQ(sa6.sin6_family, AF_INET6, "sin6_family"))
    goto close_sock;
    if (!ASSERT_EQ(memcmp(&sa6.sin6_addr,
    do_bind ? &bindaddr_v6 : &in6addr_loopback,
    sizeof(sa6.sin6_addr)),
    0, "sin6_addr"))
    goto close_sock;
    break;
    }
    close_sock:
    close(sock_fd);
    }
#[no_mangle]
pub unsafe extern "C" fn test_connect_ping() {
    void test_connect_ping(void)
    {
    struct connect_ping *skel;
    int cgroup_fd;
    if (!ASSERT_OK(unshare(CLONE_NEWNET | CLONE_NEWNS), "unshare"))
    return;
// overmount sysfs, and making original sysfs private so overmount
// does not propagate to other mntns.
//
    if (!ASSERT_OK(mount("none", "/sys", core::ptr::null_mut(), MS_PRIVATE, core::ptr::null_mut()),
    "remount-private-sys"))
    return;
    if (!ASSERT_OK(mount("sysfs", "/sys", "sysfs", 0, core::ptr::null_mut()),
    "mount-sys"))
    return;
    if (!ASSERT_OK(mount("bpffs", "/sys/fs/bpf", "bpf", 0, core::ptr::null_mut()),
    "mount-bpf"))
    goto clean_mount;
    if (!ASSERT_OK(system("ip link set dev lo up"), "lo-up"))
    goto clean_mount;
    if (!ASSERT_OK(system("ip addr add 1.1.1.1 dev lo"), "lo-addr-v4"))
    goto clean_mount;
    if (!ASSERT_OK(system("ip -6 addr add 2001:db8::1 dev lo"), "lo-addr-v6"))
    goto clean_mount;
    if (write_sysctl("/proc/sys/net/ipv4/ping_group_range", "0 0"))
    goto clean_mount;
    cgroup_fd = test__join_cgroup("/connect_ping");
    if (!ASSERT_GE(cgroup_fd, 0, "cg-create"))
    goto clean_mount;
    skel = connect_ping__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel-load"))
    goto close_cgroup;
    skel.links.connect_v4_prog =
    bpf_program__attach_cgroup(skel.progs.connect_v4_prog, cgroup_fd);
    if (!ASSERT_OK_PTR(skel.links.connect_v4_prog, "cg-attach-v4"))
    goto skel_destroy;
    skel.links.connect_v6_prog =
    bpf_program__attach_cgroup(skel.progs.connect_v6_prog, cgroup_fd);
    if (!ASSERT_OK_PTR(skel.links.connect_v6_prog, "cg-attach-v6"))
    goto skel_destroy;
// Connect a v4 ping socket to localhost, assert that only v4 is called,
// and called exactly once, and that the socket's bound address is
// original loopback address.
//
    if (test__start_subtest("ipv4"))
    subtest(cgroup_fd, skel, AF_INET, 0);
// Connect a v4 ping socket to localhost, assert that only v4 is called,
// and called exactly once, and that the socket's bound address is
// address we explicitly bound.
//
    if (test__start_subtest("ipv4-bind"))
    subtest(cgroup_fd, skel, AF_INET, 1);
// Connect a v6 ping socket to localhost, assert that only v6 is called,
// and called exactly once, and that the socket's bound address is
// original loopback address.
//
    if (test__start_subtest("ipv6"))
    subtest(cgroup_fd, skel, AF_INET6, 0);
// Connect a v6 ping socket to localhost, assert that only v6 is called,
// and called exactly once, and that the socket's bound address is
// address we explicitly bound.
//
    if (test__start_subtest("ipv6-bind"))
    subtest(cgroup_fd, skel, AF_INET6, 1);
    skel_destroy:
    connect_ping__destroy(skel);
    close_cgroup:
    close(cgroup_fd);
    clean_mount:
    umount2("/sys", MNT_DETACH);
    }
