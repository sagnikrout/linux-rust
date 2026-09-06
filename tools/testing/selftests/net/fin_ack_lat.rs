//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/fin_ack_lat.c
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

    static int child_pid;
#[no_mangle]
unsafe extern "C" fn timediff(s: timeval, e: timeval) -> c_ulong {
    static unsigned long timediff(struct timeval s, struct timeval e)
    {
    unsigned long s_us, e_us;
    s_us = s.tv_sec * 1000000 + s.tv_usec;
    e_us = e.tv_sec * 1000000 + e.tv_usec;
    if (s_us > e_us)
    return 0;
    return e_us - s_us;
    }
#[no_mangle]
unsafe extern "C" fn client(port: c_int) {
    static void client(int port)
    {
    let mut sock: c_int = 0;
    struct sockaddr_in addr, laddr;
    let mut len: socklen_t = sizeof(laddr);
    struct linger sl;
    let mut flag: c_int = 1;
    int buffer;
    struct timeval start, end;
    unsigned long lat, sum_lat = 0, nr_lat = 0;
    while (1) {
    gettimeofday(&start, core::ptr::null_mut());
    sock = socket(AF_INET, SOCK_STREAM, 0);
    if (sock < 0)
    error(-1, errno, "socket creation");
    sl.l_onoff = 1;
    sl.l_linger = 0;
    if (setsockopt(sock, SOL_SOCKET, SO_LINGER, &sl, sizeof(sl)))
    error(-1, errno, "setsockopt(linger)");
    if (setsockopt(sock, IPPROTO_TCP, TCP_NODELAY,
    &flag, sizeof(flag)))
    error(-1, errno, "setsockopt(nodelay)");
    addr.sin_family = AF_INET;
    addr.sin_port = htons(port);
    if (inet_pton(AF_INET, "127.0.0.1", &addr.sin_addr) <= 0)
    error(-1, errno, "inet_pton");
    if (connect(sock, (struct sockaddr *)&addr, sizeof(addr)) < 0)
    error(-1, errno, "connect");
    send(sock, &buffer, sizeof(buffer), 0);
    if (read(sock, &buffer, sizeof(buffer)) == -1)
    error(-1, errno, "waiting read");
    gettimeofday(&end, core::ptr::null_mut());
    lat = timediff(start, end);
    sum_lat += lat;
    nr_lat++;
    if (lat < 1000000)
    goto close;
    if (getsockname(sock, (struct sockaddr *)&laddr, &len) == -1)
    error(-1, errno, "getsockname");
    printf("port: %d, lat: %lu, avg: %lu, nr: %lu\n",
    ntohs(laddr.sin_port), lat,
    sum_lat / nr_lat, nr_lat);
    close:
    fflush(stdout);
    close(sock);
    }
    }
#[no_mangle]
unsafe extern "C" fn server(sock: c_int, address: sockaddr_in) {
    static void server(int sock, struct sockaddr_in address)
    {
    int accepted;
    let mut addrlen: c_int = sizeof(address);
    int buffer;
    while (1) {
    accepted = accept(sock, (struct sockaddr *)&address,
    (socklen_t *)&addrlen);
    if (accepted < 0)
    error(-1, errno, "accept");
    if (read(accepted, &buffer, sizeof(buffer)) == -1)
    error(-1, errno, "read");
    close(accepted);
    }
    }
#[no_mangle]
unsafe extern "C" fn sig_handler(signum: c_int) {
    static void sig_handler(int signum)
    {
    if (child_pid > 0)
    kill(child_pid, SIGTERM);
    exit(0);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *const c_char) -> c_int {
    int main(int argc, char const *argv[])
    {
    int sock;
    let mut opt: c_int = 1;
    struct sockaddr_in address;
    struct sockaddr_in laddr;
    let mut len: socklen_t = sizeof(laddr);
    if (signal(SIGTERM, sig_handler) == SIG_ERR)
    error(-1, errno, "signal");
    sock = socket(AF_INET, SOCK_STREAM, 0);
    if (sock < 0)
    error(-1, errno, "socket");
    if (setsockopt(sock, SOL_SOCKET, SO_REUSEADDR | SO_REUSEPORT,
    &opt, sizeof(opt)) == -1)
    error(-1, errno, "setsockopt");
    address.sin_family = AF_INET;
    address.sin_addr.s_addr = INADDR_ANY;
// dynamically allocate unused port
    address.sin_port = 0;
    if (bind(sock, (struct sockaddr *)&address, sizeof(address)) < 0)
    error(-1, errno, "bind");
    if (listen(sock, 3) < 0)
    error(-1, errno, "listen");
    if (getsockname(sock, (struct sockaddr *)&laddr, &len) == -1)
    error(-1, errno, "getsockname");
    fprintf(stderr, "server port: %d\n", ntohs(laddr.sin_port));
    child_pid = fork();
    if (child_pid < 0)
    error(-1, errno, "fork");
    if (!child_pid)
    client(ntohs(laddr.sin_port));
    else
    server(sock, laddr);
    return 0;
    }
