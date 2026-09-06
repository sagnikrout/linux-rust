//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/af_unix/scm_inq.c
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

pub const NR_CHUNKS: c_int = 100;
pub const MSG_LEN: c_int = 256;
pub const NR_PARTIAL_READS: c_int = 3;
    FIXTURE(scm_inq)
    {
    int fd[2];
    };
    FIXTURE_VARIANT(scm_inq)
    {
    int type;
    };
    FIXTURE_VARIANT_ADD(scm_inq, stream)
    {
    .type = SOCK_STREAM,
    };
    FIXTURE_VARIANT_ADD(scm_inq, dgram)
    {
    .type = SOCK_DGRAM,
    };
    FIXTURE_VARIANT_ADD(scm_inq, seqpacket)
    {
    .type = SOCK_SEQPACKET,
    };
    FIXTURE_SETUP(scm_inq)
    {
    int err;
    err = socketpair(AF_UNIX, variant.type | SOCK_NONBLOCK, 0, self.fd);
    ASSERT_EQ(0, err);
    }
    FIXTURE_TEARDOWN(scm_inq)
    {
    close(self.fd[0]);
    close(self.fd[1]);
    }
    static void send_chunks(struct __test_metadata *_metadata,
    FIXTURE_DATA(scm_inq) *self)
    {
    char buf[MSG_LEN] = {};
    int i, ret;
    for (i = 0; i < NR_CHUNKS; i++) {
    ret = send(self.fd[0], buf, sizeof(buf), 0);
    ASSERT_EQ(sizeof(buf), ret);
    }
    }
    static void recv_chunks(struct __test_metadata *_metadata,
    FIXTURE_DATA(scm_inq) *self)
    {
    char cmsg_buf[CMSG_SPACE(sizeof(int))];
    let mut msg: msghdr = {};
    let mut iov: iovec = {};
    struct cmsghdr *cmsg;
    char buf[MSG_LEN];
    int i, ret;
    int inq;
    msg.msg_iov = &iov;
    msg.msg_iovlen = 1;
    msg.msg_control = cmsg_buf;
    msg.msg_controllen = sizeof(cmsg_buf);
    iov.iov_base = buf;
    iov.iov_len = sizeof(buf);
    for (i = 0; i < NR_CHUNKS; i++) {
    memset(buf, 0, sizeof(buf));
    memset(cmsg_buf, 0, sizeof(cmsg_buf));
    ret = recvmsg(self.fd[1], &msg, 0);
    ASSERT_EQ(MSG_LEN, ret);
    cmsg = CMSG_FIRSTHDR(&msg);
    ASSERT_NE(core::ptr::null_mut(), cmsg);
    ASSERT_EQ(CMSG_LEN(sizeof(int)), cmsg.cmsg_len);
    ASSERT_EQ(SOL_SOCKET, cmsg.cmsg_level);
    ASSERT_EQ(SCM_INQ, cmsg.cmsg_type);
    ret = ioctl(self.fd[1], SIOCINQ, &inq);
    ASSERT_EQ(0, ret);
    ASSERT_EQ(*(int *)CMSG_DATA(cmsg), inq);
    }
    }
    TEST_F(scm_inq, basic)
    {
    int err, inq;
    err = setsockopt(self.fd[1], SOL_SOCKET, SO_INQ, &(int){1}, sizeof(int));
    if (variant.type != SOCK_STREAM) {
    ASSERT_EQ(-ENOPROTOOPT, -errno);
    return;
    }
    ASSERT_EQ(0, err);
    err = ioctl(self.fd[1], SIOCINQ, &inq);
    ASSERT_EQ(0, err);
    ASSERT_EQ(0, inq);
    send_chunks(_metadata, self);
    recv_chunks(_metadata, self);
    }
    TEST_F(scm_inq, partial_read)
    {
    char buf[MSG_LEN * NR_PARTIAL_READS] = {};
    char cmsg_buf[CMSG_SPACE(sizeof(int))];
    let mut msg: msghdr = {};
    let mut iov: iovec = {};
    struct cmsghdr *cmsg;
    int err, inq, ret, i;
    int remain;
    err = setsockopt(self.fd[1], SOL_SOCKET, SO_INQ, &(int){1}, sizeof(int));
    if (variant.type != SOCK_STREAM) {
    ASSERT_EQ(-ENOPROTOOPT, -errno);
    return;
    }
    ASSERT_EQ(0, err);
    ret = send(self.fd[0], buf, sizeof(buf), 0);
    ASSERT_EQ(sizeof(buf), ret);
    msg.msg_iov = &iov;
    msg.msg_iovlen = 1;
    msg.msg_control = cmsg_buf;
    msg.msg_controllen = sizeof(cmsg_buf);
    iov.iov_base = buf;
    iov.iov_len = MSG_LEN;
    for (i = 0; i < NR_PARTIAL_READS; i++) {
    remain = MSG_LEN * (NR_PARTIAL_READS - 1 - i);
    memset(buf, 0, MSG_LEN);
    memset(cmsg_buf, 0, sizeof(cmsg_buf));
    ret = recvmsg(self.fd[1], &msg, 0);
    ASSERT_EQ(MSG_LEN, ret);
    cmsg = CMSG_FIRSTHDR(&msg);
    ASSERT_NE(core::ptr::null_mut(), cmsg);
    ASSERT_EQ(CMSG_LEN(sizeof(int)), cmsg.cmsg_len);
    ASSERT_EQ(SOL_SOCKET, cmsg.cmsg_level);
    ASSERT_EQ(SCM_INQ, cmsg.cmsg_type);
    ASSERT_EQ(remain, *(int *)CMSG_DATA(cmsg));
    ret = ioctl(self.fd[1], SIOCINQ, &inq);
    ASSERT_EQ(0, ret);
    ASSERT_EQ(remain, inq);
    }
    }
    TEST_HARNESS_MAIN
