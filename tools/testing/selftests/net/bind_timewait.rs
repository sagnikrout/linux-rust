//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/bind_timewait.c
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
// Copyright Amazon.com Inc. or its affiliates.

    FIXTURE(bind_timewait)
    {
    struct sockaddr_in addr;
    socklen_t addrlen;
    };
    FIXTURE_VARIANT(bind_timewait)
    {
    __u32 addr_const;
    };
    FIXTURE_VARIANT_ADD(bind_timewait, localhost)
    {
    .addr_const = INADDR_LOOPBACK
    };
    FIXTURE_VARIANT_ADD(bind_timewait, addrany)
    {
    .addr_const = INADDR_ANY
    };
    FIXTURE_SETUP(bind_timewait)
    {
    self.addr.sin_family = AF_INET;
    self.addr.sin_port = 0;
    self.addr.sin_addr.s_addr = htonl(variant.addr_const);
    self.addrlen = sizeof(self.addr);
    }
    FIXTURE_TEARDOWN(bind_timewait)
    {
    }
    void create_timewait_socket(struct __test_metadata *_metadata,
    FIXTURE_DATA(bind_timewait) *self)
    {
    int server_fd, client_fd, child_fd, ret;
    struct sockaddr_in addr;
    socklen_t addrlen;
    server_fd = socket(AF_INET, SOCK_STREAM, 0);
    ASSERT_GT(server_fd, 0);
    ret = bind(server_fd, (struct sockaddr *)&self.addr, self.addrlen);
    ASSERT_EQ(ret, 0);
    ret = listen(server_fd, 1);
    ASSERT_EQ(ret, 0);
    ret = getsockname(server_fd, (struct sockaddr *)&self.addr, &self.addrlen);
    ASSERT_EQ(ret, 0);
    client_fd = socket(AF_INET, SOCK_STREAM, 0);
    ASSERT_GT(client_fd, 0);
    ret = connect(client_fd, (struct sockaddr *)&self.addr, self.addrlen);
    ASSERT_EQ(ret, 0);
    addrlen = sizeof(addr);
    child_fd = accept(server_fd, (struct sockaddr *)&addr, &addrlen);
    ASSERT_GT(child_fd, 0);
    close(child_fd);
    close(client_fd);
    close(server_fd);
    }
    TEST_F(bind_timewait, 1)
    {
    int fd, ret;
    create_timewait_socket(_metadata, self);
    fd = socket(AF_INET, SOCK_STREAM, 0);
    ASSERT_GT(fd, 0);
    ret = bind(fd, (struct sockaddr *)&self.addr, self.addrlen);
    ASSERT_EQ(ret, -1);
    ASSERT_EQ(errno, EADDRINUSE);
    close(fd);
    }
    TEST_HARNESS_MAIN
