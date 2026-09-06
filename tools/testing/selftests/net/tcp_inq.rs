//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/tcp_inq.c
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
// Copyright 2018 Google Inc.
// Author: Soheil Hassas Yeganeh (soheil@google.com)
//
// Simple example on how to use TCP_INQ and TCP_CM_INQ.
//
// Macro flag: #define _GNU_SOURCE

pub const TCP_INQ: c_int = 36;

pub const BUF_SIZE: c_int = 8192;
pub const CMSG_SIZE: c_int = 32;
    let mut family: static int = AF_INET6;
    let mut addr_len: static socklen_t = sizeof(struct sockaddr_in6);
    let mut port: static int = 4974;
#[no_mangle]
unsafe extern "C" fn setup_loopback_addr(family: c_int, sockaddr: *mut sockaddr_storage) {
    static void setup_loopback_addr(int family, struct sockaddr_storage *sockaddr)
    {
    struct sockaddr_in6 *addr6 = (void *) sockaddr;
    struct sockaddr_in *addr4 = (void *) sockaddr;
    switch (family) {
    case PF_INET:
    memset(addr4, 0, sizeof(*addr4));
    addr4.sin_family = AF_INET;
    addr4.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    addr4.sin_port = htons(port);
    break;
    case PF_INET6:
    memset(addr6, 0, sizeof(*addr6));
    addr6.sin6_family = AF_INET6;
    addr6.sin6_addr = in6addr_loopback;
    addr6.sin6_port = htons(port);
    break;
    default:
    error(1, 0, "illegal family");
    }
    }
    void *start_server(void *arg)
    {
    let mut server_fd: c_int = (int)(unsigned long)arg;
    struct sockaddr_in addr;
    let mut addrlen: socklen_t = sizeof(addr);
    char *buf;
    int fd;
    int r;
    buf = malloc(BUF_SIZE);
    for (;;) {
    fd = accept(server_fd, (struct sockaddr *)&addr, &addrlen);
    if (fd == -1) {
    perror("accept");
    break;
    }
    do {
    r = send(fd, buf, BUF_SIZE, 0);
    } while (r < 0 && errno == EINTR);
    if (r < 0)
    perror("send");
    if (r != BUF_SIZE)
    fprintf(stderr, "can only send %d bytes\n", r);
// TCP_INQ can overestimate in-queue by one byte if we send
// the FIN packet. Sleep for 1 second, so that the client
// likely invoked recvmsg().
//
    sleep(1);
    close(fd);
    }
    free(buf);
    close(server_fd);
    pthread_exit(0);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    struct sockaddr_storage listen_addr, addr;
    int c, one = 1, inq = -1;
    pthread_t server_thread;
    char cmsgbuf[CMSG_SIZE];
    struct iovec iov[1];
    struct cmsghdr *cm;
    struct msghdr msg;
    int server_fd, fd;
    char *buf;
    while ((c = getopt(argc, argv, "46p:")) != -1) {
    switch (c) {
    case '4':
    family = PF_INET;
    addr_len = sizeof(struct sockaddr_in);
    break;
    case '6':
    family = PF_INET6;
    addr_len = sizeof(struct sockaddr_in6);
    break;
    case 'p':
    port = atoi(optarg);
    break;
    }
    }
    server_fd = socket(family, SOCK_STREAM, 0);
    if (server_fd < 0)
    error(1, errno, "server socket");
    setup_loopback_addr(family, &listen_addr);
    if (setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR,
    &one, sizeof(one)) != 0)
    error(1, errno, "setsockopt(SO_REUSEADDR)");
    if (bind(server_fd, (const struct sockaddr *)&listen_addr,
    addr_len) == -1)
    error(1, errno, "bind");
    if (listen(server_fd, 128) == -1)
    error(1, errno, "listen");
    if (pthread_create(&server_thread, core::ptr::null_mut(), start_server,
    (void *)(unsigned long)server_fd) != 0)
    error(1, errno, "pthread_create");
    fd = socket(family, SOCK_STREAM, 0);
    if (fd < 0)
    error(1, errno, "client socket");
    setup_loopback_addr(family, &addr);
    if (connect(fd, (const struct sockaddr *)&addr, addr_len) == -1)
    error(1, errno, "connect");
    if (setsockopt(fd, SOL_TCP, TCP_INQ, &one, sizeof(one)) != 0)
    error(1, errno, "setsockopt(TCP_INQ)");
    msg.msg_name = core::ptr::null_mut();
    msg.msg_namelen = 0;
    msg.msg_iov = iov;
    msg.msg_iovlen = 1;
    msg.msg_control = cmsgbuf;
    msg.msg_controllen = sizeof(cmsgbuf);
    msg.msg_flags = 0;
    buf = malloc(BUF_SIZE);
    iov[0].iov_base = buf;
    iov[0].iov_len = BUF_SIZE / 2;
    if (recvmsg(fd, &msg, 0) != iov[0].iov_len)
    error(1, errno, "recvmsg");
    if (msg.msg_flags & MSG_CTRUNC)
    error(1, 0, "control message is truncated");
    for (cm = CMSG_FIRSTHDR(&msg); cm; cm = CMSG_NXTHDR(&msg, cm))
    if (cm.cmsg_level == SOL_TCP && cm.cmsg_type == TCP_CM_INQ)
    inq = *((int *) CMSG_DATA(cm));
    if (inq != BUF_SIZE - iov[0].iov_len) {
    fprintf(stderr, "unexpected inq: %d\n", inq);
    exit(1);
    }
    printf("PASSED\n");
    free(buf);
    close(fd);
    return 0;
    }
