//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/sk_storage_omem_uncharge.c
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
// Copyright (c) 2023 Facebook

#[no_mangle]
pub unsafe extern "C" fn test_sk_storage_omem_uncharge() {
    void test_sk_storage_omem_uncharge(void)
    {
    struct sk_storage_omem_uncharge *skel;
    let mut sk_fd: c_int = -1, map_fd, err, value;
    socklen_t optlen;
    skel = sk_storage_omem_uncharge__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel open_and_load"))
    return;
    map_fd = bpf_map__fd(skel.maps.sk_storage);
// A standalone socket not binding to addr:port,
// so nentns is not needed.
//
    sk_fd = socket(AF_INET6, SOCK_STREAM, 0);
    if (!ASSERT_GE(sk_fd, 0, "socket"))
    goto done;
    optlen = sizeof(skel.bss.cookie);
    err = getsockopt(sk_fd, SOL_SOCKET, SO_COOKIE, &skel.bss.cookie, &optlen);
    if (!ASSERT_OK(err, "getsockopt(SO_COOKIE)"))
    goto done;
    value = 0;
    err = bpf_map_update_elem(map_fd, &sk_fd, &value, 0);
    if (!ASSERT_OK(err, "bpf_map_update_elem(value=0)"))
    goto done;
    value = 0xdeadbeef;
    err = bpf_map_update_elem(map_fd, &sk_fd, &value, 0);
    if (!ASSERT_OK(err, "bpf_map_update_elem(value=0xdeadbeef)"))
    goto done;
    err = sk_storage_omem_uncharge__attach(skel);
    if (!ASSERT_OK(err, "attach"))
    goto done;
    close(sk_fd);
    sk_fd = -1;
    ASSERT_EQ(skel.bss.cookie_found, 2, "cookie_found");
    ASSERT_EQ(skel.bss.omem, 0, "omem");
    done:
    sk_storage_omem_uncharge__destroy(skel);
    if (sk_fd != -1)
    close(sk_fd);
    }
