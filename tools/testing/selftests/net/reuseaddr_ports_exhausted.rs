//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/reuseaddr_ports_exhausted.c
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
// Check if we can fully utilize 4-tuples for connect().
//
// Rules to bind sockets to the same port when all ephemeral ports are
// exhausted.
//
// 1. if there are TCP_LISTEN sockets on the port, fail to bind.
// 2. if there are sockets without SO_REUSEADDR, fail to bind.
// 3. if SO_REUSEADDR is disabled, fail to bind.
// 4. if SO_REUSEADDR is enabled and SO_REUSEPORT is disabled,
// succeed to bind.
// 5. if SO_REUSEADDR and SO_REUSEPORT are enabled and
// there is no socket having the both options and the same EUID,
// succeed to bind.
// 6. fail to bind.
//
// Author: Kuniyuki Iwashima <kuniyu@amazon.co.jp>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reuse_opts {
    pub reuseaddr: [c_int; 2],
    pub reuseport: [c_int; 2],
}

    struct reuse_opts unreusable_opts[12] = {
    {{0, 0}, {0, 0}},
    {{0, 0}, {0, 1}},
    {{0, 0}, {1, 0}},
    {{0, 0}, {1, 1}},
    {{0, 1}, {0, 0}},
    {{0, 1}, {0, 1}},
    {{0, 1}, {1, 0}},
    {{0, 1}, {1, 1}},
    {{1, 0}, {0, 0}},
    {{1, 0}, {0, 1}},
    {{1, 0}, {1, 0}},
    {{1, 0}, {1, 1}},
    };
    struct reuse_opts reusable_opts[4] = {
    {{1, 1}, {0, 0}},
    {{1, 1}, {0, 1}},
    {{1, 1}, {1, 0}},
    {{1, 1}, {1, 1}},
    };
#[no_mangle]
pub unsafe extern "C" fn bind_port(_metadata: *mut __test_metadata, reuseaddr: c_int, reuseport: c_int) -> c_int {
    int bind_port(struct __test_metadata *_metadata, int reuseaddr, int reuseport)
    {
    struct sockaddr_in local_addr;
    let mut len: c_int = sizeof(local_addr);
    int fd, ret;
    fd = socket(AF_INET, SOCK_STREAM, 0);
    ASSERT_NE(-1, fd) TH_LOG("failed to open socket.");
    ret = setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &reuseaddr, sizeof(int));
    ASSERT_EQ(0, ret) TH_LOG("failed to setsockopt: SO_REUSEADDR.");
    ret = setsockopt(fd, SOL_SOCKET, SO_REUSEPORT, &reuseport, sizeof(int));
    ASSERT_EQ(0, ret) TH_LOG("failed to setsockopt: SO_REUSEPORT.");
    local_addr.sin_family = AF_INET;
    local_addr.sin_addr.s_addr = inet_addr("127.0.0.1");
    local_addr.sin_port = 0;
    if (bind(fd, (struct sockaddr *)&local_addr, len) == -1) {
    close(fd);
    return -1;
    }
    return fd;
    }
    TEST(reuseaddr_ports_exhausted_unreusable)
    {
    struct reuse_opts *opts;
    int i, j, fd[2];
    for (i = 0; i < 12; i++) {
    opts = &unreusable_opts[i];
    for (j = 0; j < 2; j++)
    fd[j] = bind_port(_metadata, opts.reuseaddr[j], opts.reuseport[j]);
    ASSERT_NE(-1, fd[0]) TH_LOG("failed to bind.");
    EXPECT_EQ(-1, fd[1]) TH_LOG("should fail to bind.");
    for (j = 0; j < 2; j++)
    if (fd[j] != -1)
    close(fd[j]);
    }
    }
    TEST(reuseaddr_ports_exhausted_reusable_same_euid)
    {
    struct reuse_opts *opts;
    int i, j, fd[2];
    for (i = 0; i < 4; i++) {
    opts = &reusable_opts[i];
    for (j = 0; j < 2; j++)
    fd[j] = bind_port(_metadata, opts.reuseaddr[j], opts.reuseport[j]);
    ASSERT_NE(-1, fd[0]) TH_LOG("failed to bind.");
    if (opts.reuseport[0] && opts.reuseport[1]) {
    EXPECT_EQ(-1, fd[1]) TH_LOG("should fail to bind because both sockets successfully listened.");
    } else {
    EXPECT_NE(-1, fd[1]) TH_LOG("should succeed to bind to connect to different destinations.");
    }
    for (j = 0; j < 2; j++)
    if (fd[j] != -1)
    close(fd[j]);
    }
    }
    TEST(reuseaddr_ports_exhausted_reusable_different_euid)
    {
    struct reuse_opts *opts;
    int i, j, ret, fd[2];
    uid_t euid[2] = {10, 20};
    for (i = 0; i < 4; i++) {
    opts = &reusable_opts[i];
    for (j = 0; j < 2; j++) {
    ret = seteuid(euid[j]);
    ASSERT_EQ(0, ret) TH_LOG("failed to seteuid: %d.", euid[j]);
    fd[j] = bind_port(_metadata, opts.reuseaddr[j], opts.reuseport[j]);
    ret = seteuid(0);
    ASSERT_EQ(0, ret) TH_LOG("failed to seteuid: 0.");
    }
    ASSERT_NE(-1, fd[0]) TH_LOG("failed to bind.");
    EXPECT_NE(-1, fd[1]) TH_LOG("should succeed to bind because one socket can be bound in each euid.");
    if (fd[1] != -1) {
    ret = listen(fd[0], 5);
    ASSERT_EQ(0, ret) TH_LOG("failed to listen.");
    ret = listen(fd[1], 5);
    EXPECT_EQ(-1, ret) TH_LOG("should fail to listen because only one uid reserves the port in TCP_LISTEN.");
    }
    for (j = 0; j < 2; j++)
    if (fd[j] != -1)
    close(fd[j]);
    }
    }
    TEST_HARNESS_MAIN
