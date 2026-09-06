//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/af_unix/unix_connect.c
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

    FIXTURE(unix_connect)
    {
    int server, client;
    int family;
    };
    FIXTURE_VARIANT(unix_connect)
    {
    int type;
    char sun_path[8];
    int len;
    int flags;
    int err;
    };
    FIXTURE_VARIANT_ADD(unix_connect, stream_pathname)
    {
    .type = SOCK_STREAM,
    .sun_path = "test",
    .len = 4 + 1,
    .flags = 0,
    .err = 0,
    };
    FIXTURE_VARIANT_ADD(unix_connect, stream_abstract)
    {
    .type = SOCK_STREAM,
    .sun_path = "\0test",
    .len = 5,
    .flags = 0,
    .err = 0,
    };
    FIXTURE_VARIANT_ADD(unix_connect, stream_pathname_netns)
    {
    .type = SOCK_STREAM,
    .sun_path = "test",
    .len = 4 + 1,
    .flags = CLONE_NEWNET,
    .err = 0,
    };
    FIXTURE_VARIANT_ADD(unix_connect, stream_abstract_netns)
    {
    .type = SOCK_STREAM,
    .sun_path = "\0test",
    .len = 5,
    .flags = CLONE_NEWNET,
    .err = ECONNREFUSED,
    };
    FIXTURE_VARIANT_ADD(unix_connect, dgram_pathname)
    {
    .type = SOCK_DGRAM,
    .sun_path = "test",
    .len = 4 + 1,
    .flags = 0,
    .err = 0,
    };
    FIXTURE_VARIANT_ADD(unix_connect, dgram_abstract)
    {
    .type = SOCK_DGRAM,
    .sun_path = "\0test",
    .len = 5,
    .flags = 0,
    .err = 0,
    };
    FIXTURE_VARIANT_ADD(unix_connect, dgram_pathname_netns)
    {
    .type = SOCK_DGRAM,
    .sun_path = "test",
    .len = 4 + 1,
    .flags = CLONE_NEWNET,
    .err = 0,
    };
    FIXTURE_VARIANT_ADD(unix_connect, dgram_abstract_netns)
    {
    .type = SOCK_DGRAM,
    .sun_path = "\0test",
    .len = 5,
    .flags = CLONE_NEWNET,
    .err = ECONNREFUSED,
    };
    FIXTURE_SETUP(unix_connect)
    {
    self.family = AF_UNIX;
    }
    FIXTURE_TEARDOWN(unix_connect)
    {
    close(self.server);
    close(self.client);
    if (variant.sun_path[0])
    remove("test");
    }
    TEST_F(unix_connect, test)
    {
    socklen_t addrlen;
    struct sockaddr_un addr = {
    .sun_family = self.family,
    };
    int err;
    self.server = socket(self.family, variant.type, 0);
    ASSERT_NE(-1, self.server);
    addrlen = offsetof(struct sockaddr_un, sun_path) + variant.len;
    memcpy(&addr.sun_path, variant.sun_path, variant.len);
    err = bind(self.server, (struct sockaddr *)&addr, addrlen);
    ASSERT_EQ(0, err);
    if (variant.type == SOCK_STREAM) {
    err = listen(self.server, 32);
    ASSERT_EQ(0, err);
    }
    err = unshare(variant.flags);
    ASSERT_EQ(0, err);
    self.client = socket(self.family, variant.type, 0);
    ASSERT_LT(0, self.client);
    err = connect(self.client, (struct sockaddr *)&addr, addrlen);
    ASSERT_EQ(variant.err, err == -1 ? errno : 0);
    }
    TEST_HARNESS_MAIN
