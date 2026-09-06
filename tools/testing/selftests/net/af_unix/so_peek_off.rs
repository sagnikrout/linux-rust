//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/af_unix/so_peek_off.c
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
// Copyright 2025 Google LLC

    FIXTURE(so_peek_off)
    {
    int fd[2];	/* 0: sender, 1: receiver */
    };
    FIXTURE_VARIANT(so_peek_off)
    {
    int type;
    };
    FIXTURE_VARIANT_ADD(so_peek_off, stream)
    {
    .type = SOCK_STREAM,
    };
    FIXTURE_VARIANT_ADD(so_peek_off, dgram)
    {
    .type = SOCK_DGRAM,
    };
    FIXTURE_VARIANT_ADD(so_peek_off, seqpacket)
    {
    .type = SOCK_SEQPACKET,
    };
    FIXTURE_SETUP(so_peek_off)
    {
    struct timeval timeout = {
    .tv_sec = 5,
    .tv_usec = 0,
    };
    int ret;
    ret = socketpair(AF_UNIX, variant.type, 0, self.fd);
    ASSERT_EQ(0, ret);
    ret = setsockopt(self.fd[1], SOL_SOCKET, SO_RCVTIMEO_NEW,
    &timeout, sizeof(timeout));
    ASSERT_EQ(0, ret);
    ret = setsockopt(self.fd[1], SOL_SOCKET, SO_PEEK_OFF,
    &(int){0}, sizeof(int));
    ASSERT_EQ(0, ret);
    }
    FIXTURE_TEARDOWN(so_peek_off)
    {
    close_range(self.fd[0], self.fd[1], 0);
    }

    do {							\
    int bytes, len = strlen(str);			\
    \
    bytes = send(fd, str, len, flags);		\
    ASSERT_EQ(len, bytes);				\
    } while (0)

    do {							\
    char buf[(buflen) + 1] = {};			\
    int bytes;					\
    \
    bytes = recv(fd, buf, buflen, flags);		\
    ASSERT_NE(-1, bytes);				\
    ASSERT_STREQ(str, buf);				\
    } while (0)

    do {							\
    socklen_t optlen = sizeof(int);			\
    int off = -1;					\
    int ret;					\
    \
    ret = getsockopt(fd, SOL_SOCKET, SO_PEEK_OFF,	\
    &off, &optlen);		\
    ASSERT_EQ(0, ret);				\
    ASSERT_EQ((socklen_t)sizeof(off), optlen);	\
    ASSERT_EQ(expected, off);			\
    } while (0)

    for (pid_t pid = (pid = fork(),				\
    pid < 0 ?				\
    __TH_LOG("Failed to start async {}"),	\
    _metadata.exit_code = KSFT_FAIL,	\
    __bail(1, _metadata),			\
    0xdead :				\
    pid);					\
    !pid; exit(0))
    TEST_F(so_peek_off, single_chunk)
    {
    sendeq(self.fd[0], "aaaabbbb", 0);
    recveq(self.fd[1], "aaaa", 4, MSG_PEEK);
    peekoffeq(self.fd[1], 4);
    recveq(self.fd[1], "bbbb", 100, MSG_PEEK);
    peekoffeq(self.fd[1], 8);
    recveq(self.fd[1], "aaaabbbb", 8, 0);
    peekoffeq(self.fd[1], 0);
    }
    TEST_F(so_peek_off, two_chunks)
    {
    sendeq(self.fd[0], "aaaa", 0);
    sendeq(self.fd[0], "bbbb", 0);
    recveq(self.fd[1], "aaaa", 4, MSG_PEEK);
    peekoffeq(self.fd[1], 4);
    recveq(self.fd[1], "bbbb", 100, MSG_PEEK);
    peekoffeq(self.fd[1], 8);
    recveq(self.fd[1], "aaaa", 4, 0);
    recveq(self.fd[1], "bbbb", 4, 0);
    peekoffeq(self.fd[1], 0);
    }
    TEST_F(so_peek_off, two_chunks_blocking)
    {
    async {
    usleep(1000);
    sendeq(self.fd[0], "aaaa", 0);
    }
    recveq(self.fd[1], "aaaa", 4, MSG_PEEK);
    peekoffeq(self.fd[1], 4);
    async {
    usleep(1000);
    sendeq(self.fd[0], "bbbb", 0);
    }
// goto again; -> goto redo; in unix_stream_read_generic().
    recveq(self.fd[1], "bbbb", 100, MSG_PEEK);
    peekoffeq(self.fd[1], 8);
    recveq(self.fd[1], "aaaa", 4, 0);
    recveq(self.fd[1], "bbbb", 4, 0);
    peekoffeq(self.fd[1], 0);
    }
    TEST_F(so_peek_off, two_chunks_overlap)
    {
    sendeq(self.fd[0], "aaaa", 0);
    recveq(self.fd[1], "aa", 2, MSG_PEEK);
    peekoffeq(self.fd[1], 2);
    sendeq(self.fd[0], "bbbb", 0);
    if (variant.type == SOCK_STREAM) {
// SOCK_STREAM tries to fill the buffer.
    recveq(self.fd[1], "aabb", 4, MSG_PEEK);
    peekoffeq(self.fd[1], 6);
    recveq(self.fd[1], "bb", 100, MSG_PEEK);
    peekoffeq(self.fd[1], 8);
    } else {
// SOCK_DGRAM and SOCK_SEQPACKET returns at the skb boundary.
    recveq(self.fd[1], "aa", 100, MSG_PEEK);
    peekoffeq(self.fd[1], 4);
    recveq(self.fd[1], "bbbb", 100, MSG_PEEK);
    peekoffeq(self.fd[1], 8);
    }
    recveq(self.fd[1], "aaaa", 4, 0);
    recveq(self.fd[1], "bbbb", 4, 0);
    peekoffeq(self.fd[1], 0);
    }
    TEST_F(so_peek_off, two_chunks_overlap_blocking)
    {
    async {
    usleep(1000);
    sendeq(self.fd[0], "aaaa", 0);
    }
    recveq(self.fd[1], "aa", 2, MSG_PEEK);
    peekoffeq(self.fd[1], 2);
    async {
    usleep(1000);
    sendeq(self.fd[0], "bbbb", 0);
    }
// Even SOCK_STREAM does not wait if at least one byte is read.
    recveq(self.fd[1], "aa", 100, MSG_PEEK);
    peekoffeq(self.fd[1], 4);
    recveq(self.fd[1], "bbbb", 100, MSG_PEEK);
    peekoffeq(self.fd[1], 8);
    recveq(self.fd[1], "aaaa", 4, 0);
    recveq(self.fd[1], "bbbb", 4, 0);
    peekoffeq(self.fd[1], 0);
    }
    TEST_HARNESS_MAIN
