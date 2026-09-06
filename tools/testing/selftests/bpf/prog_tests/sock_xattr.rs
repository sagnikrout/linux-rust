//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/sock_xattr.c
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
// Copyright (c) 2026 Christian Brauner

    static const char xattr_value[] = "bpf_sock_value";
    static const char xattr_name[] = "user.bpf_test";
#[no_mangle]
unsafe extern "C" fn test_read_sock_xattr() {
    static void test_read_sock_xattr(void)
    {
    let mut addr: sockaddr_in = {};
    struct sock_read_xattr *skel = core::ptr::null_mut();
    struct bpf_link *link = core::ptr::null_mut();
    let mut sock_fd: c_int = -1, err;
    sock_fd = socket(AF_INET, SOCK_STREAM, 0);
    if (!ASSERT_OK_FD(sock_fd, "socket"))
    return;
    err = fsetxattr(sock_fd, xattr_name, xattr_value, sizeof(xattr_value), 0);
    if (!ASSERT_OK(err, "fsetxattr"))
    goto out;
    skel = sock_read_xattr__open_and_load();
    if (!ASSERT_OK_PTR(skel, "sock_read_xattr__open_and_load"))
    goto out;
    skel.bss.monitored_pid = sys_gettid();
// Only attach the functional program; the verifier-only programs
// above are not pid-gated and would clobber the shared globals.
//
    link = bpf_program__attach(skel.progs.read_sock_xattr);
    if (!ASSERT_OK_PTR(link, "attach read_sock_xattr"))
    goto out;
    addr.sin_family = AF_INET;
    addr.sin_port = htons(1234);
    addr.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
// Only the lsm/socket_connect hook matters; the connect may fail.
    connect(sock_fd, (struct sockaddr *)&addr, sizeof(addr));
    ASSERT_EQ(skel.data.read_ret, sizeof(xattr_value), "read_ret");
    ASSERT_STREQ(skel.bss.value, xattr_value, "value");
    out:
    bpf_link__destroy(link);
    if (sock_fd >= 0)
    close(sock_fd);
    sock_read_xattr__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_sock_xattr() {
    void test_sock_xattr(void)
    {
    RUN_TESTS(sock_read_xattr);
    if (test__start_subtest("read_sock_xattr"))
    test_read_sock_xattr();
    }
