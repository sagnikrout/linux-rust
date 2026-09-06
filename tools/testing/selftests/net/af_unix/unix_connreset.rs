//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/af_unix/unix_connreset.c
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
//
// Selftest for AF_UNIX socket close and ECONNRESET behaviour.
//
// This test verifies:
// 1. SOCK_STREAM returns EOF when the peer closes normally.
// 2. SOCK_STREAM returns ECONNRESET if peer closes with unread data.
// 3. SOCK_SEQPACKET returns EOF when the peer closes normally.
// 4. SOCK_SEQPACKET returns ECONNRESET if the peer closes with unread data.
// 5. SOCK_DGRAM does not return ECONNRESET when the peer closes.
//
// These tests document the intended Linux behaviour.
//
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
unsafe extern "C" fn remove_socket_file() {
    static void remove_socket_file(void)
    {
    unlink(SOCK_PATH);
    }
    FIXTURE(unix_sock)
    {
    int server;
    int client;
    int child;
    };
    FIXTURE_VARIANT(unix_sock)
    {
    int socket_type;
    const char *name;
    };
    FIXTURE_VARIANT_ADD(unix_sock, stream) {
    .socket_type = SOCK_STREAM,
    .name = "SOCK_STREAM",
    };
    FIXTURE_VARIANT_ADD(unix_sock, dgram) {
    .socket_type = SOCK_DGRAM,
    .name = "SOCK_DGRAM",
    };
    FIXTURE_VARIANT_ADD(unix_sock, seqpacket) {
    .socket_type = SOCK_SEQPACKET,
    .name = "SOCK_SEQPACKET",
    };
    FIXTURE_SETUP(unix_sock)
    {
    let mut addr: sockaddr_un = {};
    int err;
    addr.sun_family = AF_UNIX;
    strcpy(addr.sun_path, SOCK_PATH);
    remove_socket_file();
    self.server = socket(AF_UNIX, variant.socket_type, 0);
    ASSERT_LT(-1, self.server);
    err = bind(self.server, (struct sockaddr *)&addr, sizeof(addr));
    ASSERT_EQ(0, err);
    if (variant.socket_type == SOCK_STREAM ||
    variant.socket_type == SOCK_SEQPACKET) {
    err = listen(self.server, 1);
    ASSERT_EQ(0, err);
    }
    self.client = socket(AF_UNIX, variant.socket_type | SOCK_NONBLOCK, 0);
    ASSERT_LT(-1, self.client);
    err = connect(self.client, (struct sockaddr *)&addr, sizeof(addr));
    ASSERT_EQ(0, err);
    }
    FIXTURE_TEARDOWN(unix_sock)
    {
    if (variant.socket_type == SOCK_STREAM ||
    variant.socket_type == SOCK_SEQPACKET)
    close(self.child);
    close(self.client);
    close(self.server);
    remove_socket_file();
    }
// Test 1: peer closes normally
    TEST_F(unix_sock, eof)
    {
    char buf[16] = {};
    ssize_t n;
    if (variant.socket_type == SOCK_STREAM ||
    variant.socket_type == SOCK_SEQPACKET) {
    self.child = accept(self.server, core::ptr::null_mut(), core::ptr::null_mut());
    ASSERT_LT(-1, self.child);
    close(self.child);
    } else {
    close(self.server);
    }
    n = recv(self.client, buf, sizeof(buf), 0);
    if (variant.socket_type == SOCK_STREAM ||
    variant.socket_type == SOCK_SEQPACKET) {
    ASSERT_EQ(0, n);
    } else {
    ASSERT_EQ(-1, n);
    ASSERT_EQ(EAGAIN, errno);
    }
    }
// Test 2: peer closes with unread data
    TEST_F(unix_sock, reset_unread_behavior)
    {
    char buf[16] = {};
    ssize_t n;
// Send data that will remain unread
    send(self.client, "hello", 5, 0);
    if (variant.socket_type == SOCK_DGRAM) {
// No real connection, just close the server
    close(self.server);
    } else {
    self.child = accept(self.server, core::ptr::null_mut(), core::ptr::null_mut());
    ASSERT_LT(-1, self.child);
// Peer closes before client reads
    close(self.child);
    }
    n = recv(self.client, buf, sizeof(buf), 0);
    ASSERT_EQ(-1, n);
    if (variant.socket_type == SOCK_STREAM ||
    variant.socket_type == SOCK_SEQPACKET) {
    ASSERT_EQ(ECONNRESET, errno);
    } else {
    ASSERT_EQ(EAGAIN, errno);
    }
    }
// Test 3: closing unaccepted (embryo) server socket should reset client.
    TEST_F(unix_sock, reset_closed_embryo)
    {
    char buf[16] = {};
    ssize_t n;
    if (variant.socket_type == SOCK_DGRAM) {
    snprintf(_metadata.results.reason,
    sizeof(_metadata.results.reason),
    "Test only applies to SOCK_STREAM and SOCK_SEQPACKET");
    exit(KSFT_XFAIL);
    }
// Close server without accept()ing
    close(self.server);
    n = recv(self.client, buf, sizeof(buf), 0);
    ASSERT_EQ(-1, n);
    ASSERT_EQ(ECONNRESET, errno);
    }
    TEST_HARNESS_MAIN
