//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/bind_perm.c
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

#[no_mangle]
unsafe extern "C" fn create_netns() -> c_int {
    static int create_netns(void)
    {
    if (!ASSERT_OK(unshare(CLONE_NEWNET), "create netns"))
    return -1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn try_bind(family: c_int, port: c_int, expected_errno: c_int) {
    void try_bind(int family, int port, int expected_errno)
    {
    let mut addr: sockaddr_storage = {};
    struct sockaddr_in6 *sin6;
    struct sockaddr_in *sin;
    let mut fd: c_int = -1;
    fd = socket(family, SOCK_STREAM, 0);
    if (!ASSERT_GE(fd, 0, "socket"))
    goto close_socket;
    if (family == AF_INET) {
    sin = (struct sockaddr_in *)&addr;
    sin.sin_family = family;
    sin.sin_port = htons(port);
    } else {
    sin6 = (struct sockaddr_in6 *)&addr;
    sin6.sin6_family = family;
    sin6.sin6_port = htons(port);
    }
    errno = 0;
    bind(fd, (struct sockaddr *)&addr, sizeof(addr));
    ASSERT_EQ(errno, expected_errno, "bind");
    close_socket:
    if (fd >= 0)
    close(fd);
    }
#[no_mangle]
pub unsafe extern "C" fn test_bind_perm() {
    void test_bind_perm(void)
    {
    let mut net_bind_svc_cap: __u64 = 1ULL << CAP_NET_BIND_SERVICE;
    struct bind_perm *skel;
    let mut old_caps: __u64 = 0;
    int cgroup_fd;
    if (create_netns())
    return;
    cgroup_fd = test__join_cgroup("/bind_perm");
    if (!ASSERT_GE(cgroup_fd, 0, "test__join_cgroup"))
    return;
    skel = bind_perm__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel"))
    goto close_cgroup_fd;
    skel.links.bind_v4_prog = bpf_program__attach_cgroup(skel.progs.bind_v4_prog, cgroup_fd);
    if (!ASSERT_OK_PTR(skel, "bind_v4_prog"))
    goto close_skeleton;
    skel.links.bind_v6_prog = bpf_program__attach_cgroup(skel.progs.bind_v6_prog, cgroup_fd);
    if (!ASSERT_OK_PTR(skel, "bind_v6_prog"))
    goto close_skeleton;
    ASSERT_OK(cap_disable_effective(net_bind_svc_cap, &old_caps),
    "cap_disable_effective");
    try_bind(AF_INET, 110, EACCES);
    try_bind(AF_INET6, 110, EACCES);
    try_bind(AF_INET, 111, 0);
    try_bind(AF_INET6, 111, 0);
    if (old_caps & net_bind_svc_cap)
    ASSERT_OK(cap_enable_effective(net_bind_svc_cap, core::ptr::null_mut()),
    "cap_enable_effective");
    close_skeleton:
    bind_perm__destroy(skel);
    close_cgroup_fd:
    close(cgroup_fd);
    }
