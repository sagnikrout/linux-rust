//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/netfilter/conntrack_reverse_clash.c
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
// Needs something like:
//
// iptables -t nat -A POSTROUTING -o nomatch -j MASQUERADE
//
// so NAT engine attaches a NAT null-binding to each connection.
//
// With unmodified kernels, child or parent will exit with
// "Port number changed" error, even though no port translation
// was requested.
//

pub const LEN: c_int = 512;
pub const PORT: c_int = 56789;
pub const TEST_TIME: c_int = 5;
#[no_mangle]
unsafe extern "C" fn die(e: *const c_char) {
    static void die(const char *e)
    {
    perror(e);
    exit(111);
    }
#[no_mangle]
unsafe extern "C" fn die_port(sin: *const sockaddr_in, want: u16) {
    static void die_port(const struct sockaddr_in *sin, uint16_t want)
    {
    let mut got: u16 = ntohs(sin.sin_port);
    char str[INET_ADDRSTRLEN];
    inet_ntop(AF_INET, &sin.sin_addr, str, sizeof(str));
    fprintf(stderr, "Port number changed, wanted %d got %d from %s\n", want, got, str);
    exit(1);
    }
#[no_mangle]
unsafe extern "C" fn udp_socket() -> c_int {
    static int udp_socket(void)
    {
    static const struct timeval tv = {
    .tv_sec = 1,
    };
    let mut fd: c_int = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
    if (fd < 0)
    die("socket");
    setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof(tv));
    return fd;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    struct sockaddr_in sa1 = {
    .sin_family = AF_INET,
    };
    struct sockaddr_in sa2 = {
    .sin_family = AF_INET,
    };
    int s1, s2, status;
    time_t end, now;
    socklen_t plen;
    char buf[LEN];
    bool child;
    sa1.sin_port = htons(PORT);
    sa2.sin_port = htons(PORT + 1);
    s1 = udp_socket();
    s2 = udp_socket();
    inet_pton(AF_INET, "127.0.0.11", &sa1.sin_addr);
    inet_pton(AF_INET, "127.0.0.12", &sa2.sin_addr);
    if (bind(s1, (struct sockaddr *)&sa1, sizeof(sa1)) < 0)
    die("bind 1");
    if (bind(s2, (struct sockaddr *)&sa2, sizeof(sa2)) < 0)
    die("bind 2");
    child = fork() == 0;
    now = time(core::ptr::null_mut());
    end = now + TEST_TIME;
    while (now < end) {
    struct sockaddr_in peer;
    let mut plen: socklen_t = sizeof(peer);
    now = time(core::ptr::null_mut());
    if (child) {
    if (sendto(s1, buf, LEN, 0, (struct sockaddr *)&sa2, sizeof(sa2)) != LEN)
    continue;
    if (recvfrom(s2, buf, LEN, 0, (struct sockaddr *)&peer, &plen) < 0)
    die("child recvfrom");
    if (peer.sin_port != htons(PORT))
    die_port(&peer, PORT);
    } else {
    if (sendto(s2, buf, LEN, 0, (struct sockaddr *)&sa1, sizeof(sa1)) != LEN)
    continue;
    if (recvfrom(s1, buf, LEN, 0, (struct sockaddr *)&peer, &plen) < 0)
    die("parent recvfrom");
    if (peer.sin_port != htons((PORT + 1)))
    die_port(&peer, PORT + 1);
    }
    }
    if (child)
    return 0;
    wait(&status);
    if (WIFEXITED(status))
    return WEXITSTATUS(status);
    return 1;
    }
