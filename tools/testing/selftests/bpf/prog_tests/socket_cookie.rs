//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/socket_cookie.c
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
// Copyright (c) 2020 Google LLC.
// Copyright (c) 2018 Facebook

    static int duration;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct socket_cookie {
    pub cookie_key: __u64,
    pub cookie_value: __u32,
}

#[no_mangle]
pub unsafe extern "C" fn test_socket_cookie() {
    void test_socket_cookie(void)
    {
    let mut server_fd: c_int = 0, client_fd = 0, cgroup_fd = 0, err = 0;
    let mut addr_len: socklen_t = sizeof(struct sockaddr_in6);
    struct socket_cookie_prog *skel;
    __u32 cookie_expected_value;
    struct sockaddr_in6 addr;
    struct socket_cookie val;
    skel = socket_cookie_prog__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    cgroup_fd = test__join_cgroup("/socket_cookie");
    if (CHECK(cgroup_fd < 0, "join_cgroup", "cgroup creation failed\n"))
    goto out;
    skel.links.set_cookie = bpf_program__attach_cgroup(
    skel.progs.set_cookie, cgroup_fd);
    if (!ASSERT_OK_PTR(skel.links.set_cookie, "prog_attach"))
    goto close_cgroup_fd;
    skel.links.update_cookie_sockops = bpf_program__attach_cgroup(
    skel.progs.update_cookie_sockops, cgroup_fd);
    if (!ASSERT_OK_PTR(skel.links.update_cookie_sockops, "prog_attach"))
    goto close_cgroup_fd;
    skel.links.update_cookie_tracing = bpf_program__attach(
    skel.progs.update_cookie_tracing);
    if (!ASSERT_OK_PTR(skel.links.update_cookie_tracing, "prog_attach"))
    goto close_cgroup_fd;
    server_fd = start_server(AF_INET6, SOCK_STREAM, "::1", 0, 0);
    if (CHECK(server_fd < 0, "start_server", "errno %d\n", errno))
    goto close_cgroup_fd;
    client_fd = connect_to_fd(server_fd, 0);
    if (CHECK(client_fd < 0, "connect_to_fd", "errno %d\n", errno))
    goto close_server_fd;
    err = bpf_map_lookup_elem(bpf_map__fd(skel.maps.socket_cookies),
    &client_fd, &val);
    if (!ASSERT_OK(err, "map_lookup(socket_cookies)"))
    goto close_client_fd;
    err = getsockname(client_fd, (struct sockaddr *)&addr, &addr_len);
    if (!ASSERT_OK(err, "getsockname"))
    goto close_client_fd;
    cookie_expected_value = (ntohs(addr.sin6_port) << 8) | 0xFF;
    ASSERT_EQ(val.cookie_value, cookie_expected_value, "cookie_value");
    close_client_fd:
    close(client_fd);
    close_server_fd:
    close(server_fd);
    close_cgroup_fd:
    close(cgroup_fd);
    out:
    socket_cookie_prog__destroy(skel);
    }
