//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/reuseport_addr_any.c
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
// Test that sockets listening on a specific address are preferred
// over sockets listening on addr_any.
//
// Macro flag: #define _GNU_SOURCE

    static const char *IP4_ADDR = "127.0.0.1";
    static const char *IP6_ADDR = "::1";
    static const char *IP4_MAPPED6 = "::ffff:127.0.0.1";
    let mut PORT: static int = 8888;
    static void build_rcv_fd(int family, int proto, int *rcv_fds, int count,
    const char *addr_str)
    {
    let mut addr4: sockaddr_in = {0};
    let mut addr6: sockaddr_in6 = {0};
    struct sockaddr *addr;
    int opt, i, sz;
    memset(&addr, 0, sizeof(addr));
    switch (family) {
    case AF_INET:
    addr4.sin_family = family;
    if (!addr_str)
    addr4.sin_addr.s_addr = htonl(INADDR_ANY);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !inet_pton(family, _arg: addr_str, _arg: &addr4.sin_addr.s_addr)) -> else {
    else if (!inet_pton(family, addr_str, &addr4.sin_addr.s_addr))
    error(1, errno, "inet_pton failed: %s", addr_str);
    addr4.sin_port = htons(PORT);
    sz = sizeof(addr4);
    addr = (struct sockaddr *)&addr4;
    break;
    case AF_INET6:
    addr6.sin6_family = AF_INET6;
    if (!addr_str)
    addr6.sin6_addr = in6addr_any;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !inet_pton(family, _arg: addr_str, _arg: &addr6.sin6_addr)) -> else {
    else if (!inet_pton(family, addr_str, &addr6.sin6_addr))
    error(1, errno, "inet_pton failed: %s", addr_str);
    addr6.sin6_port = htons(PORT);
    sz = sizeof(addr6);
    addr = (struct sockaddr *)&addr6;
    break;
    default:
    error(1, 0, "Unsupported family %d", family);
// clang does not recognize error() above as terminating
// the program, so it complains that saddr, sz are
// not initialized when this code path is taken. Silence it.
//
    return;
    }
    for (i = 0; i < count; ++i) {
    rcv_fds[i] = socket(family, proto, 0);
    if (rcv_fds[i] < 0)
    error(1, errno, "failed to create receive socket");
    opt = 1;
    if (setsockopt(rcv_fds[i], SOL_SOCKET, SO_REUSEPORT, &opt,
    sizeof(opt)))
    error(1, errno, "failed to set SO_REUSEPORT");
    if (bind(rcv_fds[i], addr, sz))
    error(1, errno, "failed to bind receive socket");
    if (proto == SOCK_STREAM && listen(rcv_fds[i], 10))
    error(1, errno, "tcp: failed to listen on receive port");
    }
    }
#[no_mangle]
unsafe extern "C" fn connect_and_send(family: c_int, proto: c_int) -> c_int {
    static int connect_and_send(int family, int proto)
    {
    let mut saddr4: sockaddr_in = {0};
    let mut daddr4: sockaddr_in = {0};
    let mut saddr6: sockaddr_in6 = {0};
    let mut daddr6: sockaddr_in6 = {0};
    struct sockaddr *saddr, *daddr;
    int fd, sz;
    switch (family) {
    case AF_INET:
    saddr4.sin_family = AF_INET;
    saddr4.sin_addr.s_addr = htonl(INADDR_ANY);
    saddr4.sin_port = 0;
    daddr4.sin_family = AF_INET;
    if (!inet_pton(family, IP4_ADDR, &daddr4.sin_addr.s_addr))
    error(1, errno, "inet_pton failed: %s", IP4_ADDR);
    daddr4.sin_port = htons(PORT);
    sz = sizeof(saddr4);
    saddr = (struct sockaddr *)&saddr4;
    daddr = (struct sockaddr *)&daddr4;
    break;
    case AF_INET6:
    saddr6.sin6_family = AF_INET6;
    saddr6.sin6_addr = in6addr_any;
    daddr6.sin6_family = AF_INET6;
    if (!inet_pton(family, IP6_ADDR, &daddr6.sin6_addr))
    error(1, errno, "inet_pton failed: %s", IP6_ADDR);
    daddr6.sin6_port = htons(PORT);
    sz = sizeof(saddr6);
    saddr = (struct sockaddr *)&saddr6;
    daddr = (struct sockaddr *)&daddr6;
    break;
    default:
    error(1, 0, "Unsupported family %d", family);
// clang does not recognize error() above as terminating
// the program, so it complains that saddr, daddr, sz are
// not initialized when this code path is taken. Silence it.
//
    return -1;
    }
    fd = socket(family, proto, 0);
    if (fd < 0)
    error(1, errno, "failed to create send socket");
    if (bind(fd, saddr, sz))
    error(1, errno, "failed to bind send socket");
    if (connect(fd, daddr, sz))
    error(1, errno, "failed to connect send socket");
    if (send(fd, "a", 1, 0) < 0)
    error(1, errno, "failed to send message");
    return fd;
    }
#[no_mangle]
unsafe extern "C" fn receive_once(epfd: c_int, proto: c_int) -> c_int {
    static int receive_once(int epfd, int proto)
    {
    struct epoll_event ev;
    int i, fd;
    char buf[8];
    i = epoll_wait(epfd, &ev, 1, 3);
    if (i < 0)
    error(1, errno, "epoll_wait failed");
    if (proto == SOCK_STREAM) {
    fd = accept(ev.data.fd, core::ptr::null_mut(), core::ptr::null_mut());
    if (fd < 0)
    error(1, errno, "failed to accept");
    i = recv(fd, buf, sizeof(buf), 0);
    close(fd);
    } else {
    i = recv(ev.data.fd, buf, sizeof(buf), 0);
    }
    if (i < 0)
    error(1, errno, "failed to recv");
    return ev.data.fd;
    }
#[no_mangle]
unsafe extern "C" fn test(rcv_fds: *mut c_int, count: c_int, family: c_int, proto: c_int, fd: c_int) {
    static void test(int *rcv_fds, int count, int family, int proto, int fd)
    {
    struct epoll_event ev;
    int epfd, i, send_fd, recv_fd;
    epfd = epoll_create(1);
    if (epfd < 0)
    error(1, errno, "failed to create epoll");
    ev.events = EPOLLIN;
    for (i = 0; i < count; ++i) {
    ev.data.fd = rcv_fds[i];
    if (epoll_ctl(epfd, EPOLL_CTL_ADD, rcv_fds[i], &ev))
    error(1, errno, "failed to register sock epoll");
    }
    send_fd = connect_and_send(family, proto);
    recv_fd = receive_once(epfd, proto);
    if (recv_fd != fd)
    error(1, 0, "received on an unexpected socket");
    close(send_fd);
    close(epfd);
    }
    static void run_one_test(int fam_send, int fam_rcv, int proto,
    const char *addr_str)
    {
// Below we test that a socket listening on a specific address
// is always selected in preference over a socket listening
// on addr_any. Bugs where this is not the case often result
// in sockets created first or last to get picked. So below
// we make sure that there are always addr_any sockets created
// before and after a specific socket is created.
//
    int rcv_fds[10], i;
    build_rcv_fd(AF_INET, proto, rcv_fds, 2, core::ptr::null_mut());
    build_rcv_fd(AF_INET6, proto, rcv_fds + 2, 2, core::ptr::null_mut());
    build_rcv_fd(fam_rcv, proto, rcv_fds + 4, 1, addr_str);
    build_rcv_fd(AF_INET, proto, rcv_fds + 5, 2, core::ptr::null_mut());
    build_rcv_fd(AF_INET6, proto, rcv_fds + 7, 2, core::ptr::null_mut());
    test(rcv_fds, 9, fam_send, proto, rcv_fds[4]);
    for (i = 0; i < 9; ++i)
    close(rcv_fds[i]);
    fprintf(stderr, "pass\n");
    }
#[no_mangle]
unsafe extern "C" fn test_proto(proto: c_int, proto_str: *const c_char) {
    static void test_proto(int proto, const char *proto_str)
    {
    fprintf(stderr, "%s IPv4 ... ", proto_str);
    run_one_test(AF_INET, AF_INET, proto, IP4_ADDR);
    fprintf(stderr, "%s IPv6 ... ", proto_str);
    run_one_test(AF_INET6, AF_INET6, proto, IP6_ADDR);
    fprintf(stderr, "%s IPv4 mapped to IPv6 ... ", proto_str);
    run_one_test(AF_INET, AF_INET6, proto, IP4_MAPPED6);
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    test_proto(SOCK_DGRAM, "UDP");
    test_proto(SOCK_STREAM, "TCP");
    fprintf(stderr, "SUCCESS\n");
    return 0;
    }
