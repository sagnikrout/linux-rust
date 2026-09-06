//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/skc_to_unix_sock.c
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
// Copyright (c) 2021 Hengqi Chen

    static const char *sock_path = "@skc_to_unix_sock";
#[no_mangle]
pub unsafe extern "C" fn test_skc_to_unix_sock() {
    void test_skc_to_unix_sock(void)
    {
    struct test_skc_to_unix_sock *skel;
    struct sockaddr_un sockaddr;
    int err, sockfd = 0;
    skel = test_skc_to_unix_sock__open();
    if (!ASSERT_OK_PTR(skel, "could not open BPF object"))
    return;
    skel.rodata.my_pid = getpid();
    err = test_skc_to_unix_sock__load(skel);
    if (!ASSERT_OK(err, "could not load BPF object"))
    goto cleanup;
    err = test_skc_to_unix_sock__attach(skel);
    if (!ASSERT_OK(err, "could not attach BPF object"))
    goto cleanup;
// trigger unix_listen
    sockfd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (!ASSERT_GT(sockfd, 0, "socket failed"))
    goto cleanup;
    memset(&sockaddr, 0, sizeof(sockaddr));
    sockaddr.sun_family = AF_UNIX;
    strscpy(sockaddr.sun_path, sock_path);
    sockaddr.sun_path[0] = '\0';
    err = bind(sockfd, (struct sockaddr *)&sockaddr, sizeof(sockaddr));
    if (!ASSERT_OK(err, "bind failed"))
    goto cleanup;
    err = listen(sockfd, 1);
    if (!ASSERT_OK(err, "listen failed"))
    goto cleanup;
    ASSERT_EQ(strcmp(skel.bss.path, sock_path), 0, "bpf_skc_to_unix_sock failed");
    cleanup:
    if (sockfd)
    close(sockfd);
    test_skc_to_unix_sock__destroy(skel);
    }
