//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/xattr/xattr_socket_test.c
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
// Copyright (c) 2026 Christian Brauner <brauner@kernel.org>
//
// Test extended attributes on path-based Unix domain sockets.
//
// Path-based Unix domain sockets are bound to a filesystem path and their
// inodes live on the underlying filesystem (e.g. tmpfs). These tests verify
// that user.* and trusted.* xattr operations work correctly on them using
// path-based syscalls (setxattr, getxattr, etc.).
//
// Covers SOCK_STREAM, SOCK_DGRAM, and SOCK_SEQPACKET socket types.
//
// Macro flag: #define _GNU_SOURCE

//
// Fixture for path-based Unix domain socket tests.
// Creates a SOCK_STREAM socket bound to a path in /tmp (typically tmpfs).
//
    FIXTURE(xattr_socket)
    {
    char socket_path[PATH_MAX];
    int sockfd;
    };
    FIXTURE_VARIANT(xattr_socket)
    {
    int sock_type;
    const char *name;
    };
    FIXTURE_VARIANT_ADD(xattr_socket, stream) {
    .sock_type = SOCK_STREAM,
    .name = "stream",
    };
    FIXTURE_VARIANT_ADD(xattr_socket, dgram) {
    .sock_type = SOCK_DGRAM,
    .name = "dgram",
    };
    FIXTURE_VARIANT_ADD(xattr_socket, seqpacket) {
    .sock_type = SOCK_SEQPACKET,
    .name = "seqpacket",
    };
    FIXTURE_SETUP(xattr_socket)
    {
    struct sockaddr_un addr;
    int ret;
    self.sockfd = -1;
    snprintf(self.socket_path, sizeof(self.socket_path),
    "/tmp/xattr_socket_test_%s.%d", variant.name, getpid());
    unlink(self.socket_path);
    self.sockfd = socket(AF_UNIX, variant.sock_type, 0);
    ASSERT_GE(self.sockfd, 0) {
    TH_LOG("Failed to create socket: %s", strerror(errno));
    }
    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;
    strncpy(addr.sun_path, self.socket_path, sizeof(addr.sun_path) - 1);
    ret = bind(self.sockfd, (struct sockaddr *)&addr, sizeof(addr));
    ASSERT_EQ(ret, 0) {
    TH_LOG("Failed to bind socket to %s: %s",
    self.socket_path, strerror(errno));
    }
    }
    FIXTURE_TEARDOWN(xattr_socket)
    {
    if (self.sockfd >= 0)
    close(self.sockfd);
    unlink(self.socket_path);
    }
    TEST_F(xattr_socket, set_user_xattr)
    {
    int ret;
    ret = setxattr(self.socket_path, TEST_XATTR_NAME,
    TEST_XATTR_VALUE, strlen(TEST_XATTR_VALUE), 0);
    ASSERT_EQ(ret, 0) {
    TH_LOG("setxattr failed: %s (errno=%d)", strerror(errno), errno);
    }
    }
    TEST_F(xattr_socket, get_user_xattr)
    {
    char buf[256];
    ssize_t ret;
    ret = setxattr(self.socket_path, TEST_XATTR_NAME,
    TEST_XATTR_VALUE, strlen(TEST_XATTR_VALUE), 0);
    ASSERT_EQ(ret, 0) {
    TH_LOG("setxattr failed: %s", strerror(errno));
    }
    memset(buf, 0, sizeof(buf));
    ret = getxattr(self.socket_path, TEST_XATTR_NAME, buf, sizeof(buf));
    ASSERT_EQ(ret, (ssize_t)strlen(TEST_XATTR_VALUE)) {
    TH_LOG("getxattr returned %zd, expected %zu: %s",
    ret, strlen(TEST_XATTR_VALUE), strerror(errno));
    }
    ASSERT_STREQ(buf, TEST_XATTR_VALUE);
    }
    TEST_F(xattr_socket, list_user_xattr)
    {
    char list[1024];
    ssize_t ret;
    let mut found: bool = false;
    char *ptr;
    ret = setxattr(self.socket_path, TEST_XATTR_NAME,
    TEST_XATTR_VALUE, strlen(TEST_XATTR_VALUE), 0);
    ASSERT_EQ(ret, 0) {
    TH_LOG("setxattr failed: %s", strerror(errno));
    }
    memset(list, 0, sizeof(list));
    ret = listxattr(self.socket_path, list, sizeof(list));
    ASSERT_GT(ret, 0) {
    TH_LOG("listxattr failed: %s", strerror(errno));
    }
    for (ptr = list; ptr < list + ret; ptr += strlen(ptr) + 1) {
    if (strcmp(ptr, TEST_XATTR_NAME) == 0) {
    found = true;
    break;
    }
    }
    ASSERT_TRUE(found) {
    TH_LOG("xattr %s not found in list", TEST_XATTR_NAME);
    }
    }
    TEST_F(xattr_socket, remove_user_xattr)
    {
    char buf[256];
    ssize_t ret;
    ret = setxattr(self.socket_path, TEST_XATTR_NAME,
    TEST_XATTR_VALUE, strlen(TEST_XATTR_VALUE), 0);
    ASSERT_EQ(ret, 0) {
    TH_LOG("setxattr failed: %s", strerror(errno));
    }
    ret = removexattr(self.socket_path, TEST_XATTR_NAME);
    ASSERT_EQ(ret, 0) {
    TH_LOG("removexattr failed: %s", strerror(errno));
    }
    ret = getxattr(self.socket_path, TEST_XATTR_NAME, buf, sizeof(buf));
    ASSERT_EQ(ret, -1);
    ASSERT_EQ(errno, ENODATA) {
    TH_LOG("Expected ENODATA, got %s", strerror(errno));
    }
    }
//
// Test that xattrs persist across socket close and reopen.
// The xattr is on the filesystem inode, not the socket fd.
//
    TEST_F(xattr_socket, xattr_persistence)
    {
    char buf[256];
    ssize_t ret;
    ret = setxattr(self.socket_path, TEST_XATTR_NAME,
    TEST_XATTR_VALUE, strlen(TEST_XATTR_VALUE), 0);
    ASSERT_EQ(ret, 0) {
    TH_LOG("setxattr failed: %s", strerror(errno));
    }
    close(self.sockfd);
    self.sockfd = -1;
    memset(buf, 0, sizeof(buf));
    ret = getxattr(self.socket_path, TEST_XATTR_NAME, buf, sizeof(buf));
    ASSERT_EQ(ret, (ssize_t)strlen(TEST_XATTR_VALUE)) {
    TH_LOG("getxattr after close failed: %s", strerror(errno));
    }
    ASSERT_STREQ(buf, TEST_XATTR_VALUE);
    }
    TEST_F(xattr_socket, update_user_xattr)
    {
    char buf[256];
    ssize_t ret;
    ret = setxattr(self.socket_path, TEST_XATTR_NAME,
    TEST_XATTR_VALUE, strlen(TEST_XATTR_VALUE), 0);
    ASSERT_EQ(ret, 0);
    ret = setxattr(self.socket_path, TEST_XATTR_NAME,
    TEST_XATTR_VALUE2, strlen(TEST_XATTR_VALUE2), 0);
    ASSERT_EQ(ret, 0);
    memset(buf, 0, sizeof(buf));
    ret = getxattr(self.socket_path, TEST_XATTR_NAME, buf, sizeof(buf));
    ASSERT_EQ(ret, (ssize_t)strlen(TEST_XATTR_VALUE2));
    ASSERT_STREQ(buf, TEST_XATTR_VALUE2);
    }
    TEST_F(xattr_socket, xattr_create_flag)
    {
    int ret;
    ret = setxattr(self.socket_path, TEST_XATTR_NAME,
    TEST_XATTR_VALUE, strlen(TEST_XATTR_VALUE), 0);
    ASSERT_EQ(ret, 0);
    ret = setxattr(self.socket_path, TEST_XATTR_NAME,
    TEST_XATTR_VALUE2, strlen(TEST_XATTR_VALUE2), XATTR_CREATE);
    ASSERT_EQ(ret, -1);
    ASSERT_EQ(errno, EEXIST);
    }
    TEST_F(xattr_socket, xattr_replace_flag)
    {
    int ret;
    ret = setxattr(self.socket_path, TEST_XATTR_NAME,
    TEST_XATTR_VALUE, strlen(TEST_XATTR_VALUE), XATTR_REPLACE);
    ASSERT_EQ(ret, -1);
    ASSERT_EQ(errno, ENODATA);
    }
    TEST_F(xattr_socket, multiple_xattrs)
    {
    char buf[256];
    ssize_t ret;
    int i;
    char name[64], value[64];
    let mut num_xattrs: c_int = 5;
    for (i = 0; i < num_xattrs; i++) {
    snprintf(name, sizeof(name), "user.test%d", i);
    snprintf(value, sizeof(value), "value%d", i);
    ret = setxattr(self.socket_path, name, value, strlen(value), 0);
    ASSERT_EQ(ret, 0) {
    TH_LOG("setxattr %s failed: %s", name, strerror(errno));
    }
    }
    for (i = 0; i < num_xattrs; i++) {
    snprintf(name, sizeof(name), "user.test%d", i);
    snprintf(value, sizeof(value), "value%d", i);
    memset(buf, 0, sizeof(buf));
    ret = getxattr(self.socket_path, name, buf, sizeof(buf));
    ASSERT_EQ(ret, (ssize_t)strlen(value));
    ASSERT_STREQ(buf, value);
    }
    }
    TEST_F(xattr_socket, xattr_empty_value)
    {
    char buf[256];
    ssize_t ret;
    ret = setxattr(self.socket_path, TEST_XATTR_NAME, "", 0, 0);
    ASSERT_EQ(ret, 0);
    ret = getxattr(self.socket_path, TEST_XATTR_NAME, buf, sizeof(buf));
    ASSERT_EQ(ret, 0);
    }
    TEST_F(xattr_socket, xattr_get_size)
    {
    ssize_t ret;
    ret = setxattr(self.socket_path, TEST_XATTR_NAME,
    TEST_XATTR_VALUE, strlen(TEST_XATTR_VALUE), 0);
    ASSERT_EQ(ret, 0);
    ret = getxattr(self.socket_path, TEST_XATTR_NAME, core::ptr::null_mut(), 0);
    ASSERT_EQ(ret, (ssize_t)strlen(TEST_XATTR_VALUE));
    }
    TEST_F(xattr_socket, xattr_buffer_too_small)
    {
    char buf[2];
    ssize_t ret;
    ret = setxattr(self.socket_path, TEST_XATTR_NAME,
    TEST_XATTR_VALUE, strlen(TEST_XATTR_VALUE), 0);
    ASSERT_EQ(ret, 0);
    ret = getxattr(self.socket_path, TEST_XATTR_NAME, buf, sizeof(buf));
    ASSERT_EQ(ret, -1);
    ASSERT_EQ(errno, ERANGE);
    }
    TEST_F(xattr_socket, xattr_nonexistent)
    {
    char buf[256];
    ssize_t ret;
    ret = getxattr(self.socket_path, "user.nonexistent", buf, sizeof(buf));
    ASSERT_EQ(ret, -1);
    ASSERT_EQ(errno, ENODATA);
    }
    TEST_F(xattr_socket, remove_nonexistent_xattr)
    {
    int ret;
    ret = removexattr(self.socket_path, "user.nonexistent");
    ASSERT_EQ(ret, -1);
    ASSERT_EQ(errno, ENODATA);
    }
    TEST_F(xattr_socket, large_xattr_value)
    {
    char large_value[4096];
    char read_buf[4096];
    ssize_t ret;
    memset(large_value, 'A', sizeof(large_value));
    ret = setxattr(self.socket_path, TEST_XATTR_NAME,
    large_value, sizeof(large_value), 0);
    ASSERT_EQ(ret, 0) {
    TH_LOG("setxattr with large value failed: %s", strerror(errno));
    }
    memset(read_buf, 0, sizeof(read_buf));
    ret = getxattr(self.socket_path, TEST_XATTR_NAME,
    read_buf, sizeof(read_buf));
    ASSERT_EQ(ret, (ssize_t)sizeof(large_value));
    ASSERT_EQ(memcmp(large_value, read_buf, sizeof(large_value)), 0);
    }
//
// Test lsetxattr/lgetxattr (don't follow symlinks).
// Socket files aren't symlinks, so this should work the same.
//
    TEST_F(xattr_socket, lsetxattr_lgetxattr)
    {
    char buf[256];
    ssize_t ret;
    ret = lsetxattr(self.socket_path, TEST_XATTR_NAME,
    TEST_XATTR_VALUE, strlen(TEST_XATTR_VALUE), 0);
    ASSERT_EQ(ret, 0) {
    TH_LOG("lsetxattr failed: %s", strerror(errno));
    }
    memset(buf, 0, sizeof(buf));
    ret = lgetxattr(self.socket_path, TEST_XATTR_NAME, buf, sizeof(buf));
    ASSERT_EQ(ret, (ssize_t)strlen(TEST_XATTR_VALUE));
    ASSERT_STREQ(buf, TEST_XATTR_VALUE);
    }
//
// Fixture for trusted.* xattr tests.
// These require CAP_SYS_ADMIN.
//
    FIXTURE(xattr_socket_trusted)
    {
    char socket_path[PATH_MAX];
    int sockfd;
    };
    FIXTURE_VARIANT(xattr_socket_trusted)
    {
    int sock_type;
    const char *name;
    };
    FIXTURE_VARIANT_ADD(xattr_socket_trusted, stream) {
    .sock_type = SOCK_STREAM,
    .name = "stream",
    };
    FIXTURE_VARIANT_ADD(xattr_socket_trusted, dgram) {
    .sock_type = SOCK_DGRAM,
    .name = "dgram",
    };
    FIXTURE_VARIANT_ADD(xattr_socket_trusted, seqpacket) {
    .sock_type = SOCK_SEQPACKET,
    .name = "seqpacket",
    };
    FIXTURE_SETUP(xattr_socket_trusted)
    {
    struct sockaddr_un addr;
    int ret;
    self.sockfd = -1;
    snprintf(self.socket_path, sizeof(self.socket_path),
    "/tmp/xattr_socket_trusted_%s.%d", variant.name, getpid());
    unlink(self.socket_path);
    self.sockfd = socket(AF_UNIX, variant.sock_type, 0);
    ASSERT_GE(self.sockfd, 0);
    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;
    strncpy(addr.sun_path, self.socket_path, sizeof(addr.sun_path) - 1);
    ret = bind(self.sockfd, (struct sockaddr *)&addr, sizeof(addr));
    ASSERT_EQ(ret, 0);
    }
    FIXTURE_TEARDOWN(xattr_socket_trusted)
    {
    if (self.sockfd >= 0)
    close(self.sockfd);
    unlink(self.socket_path);
    }
    TEST_F(xattr_socket_trusted, set_trusted_xattr)
    {
    char buf[256];
    ssize_t len;
    int ret;
    ret = setxattr(self.socket_path, "trusted.testattr",
    TEST_XATTR_VALUE, strlen(TEST_XATTR_VALUE), 0);
    if (ret == -1 && errno == EPERM)
    SKIP(return, "Need CAP_SYS_ADMIN for trusted.* xattrs");
    ASSERT_EQ(ret, 0) {
    TH_LOG("setxattr trusted.testattr failed: %s", strerror(errno));
    }
    memset(buf, 0, sizeof(buf));
    len = getxattr(self.socket_path, "trusted.testattr",
    buf, sizeof(buf));
    ASSERT_EQ(len, (ssize_t)strlen(TEST_XATTR_VALUE));
    ASSERT_STREQ(buf, TEST_XATTR_VALUE);
    }
    TEST_F(xattr_socket_trusted, get_trusted_xattr_unprivileged)
    {
    char buf[256];
    ssize_t ret;
    ret = getxattr(self.socket_path, "trusted.testattr", buf, sizeof(buf));
    ASSERT_EQ(ret, -1);
    ASSERT_TRUE(errno == ENODATA || errno == EPERM) {
    TH_LOG("Expected ENODATA or EPERM, got %s", strerror(errno));
    }
    }
    TEST_HARNESS_MAIN
