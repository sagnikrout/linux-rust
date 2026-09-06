//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/netfilter/connect_close.c
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

pub const PORT: c_int = 12345;
pub const RUNTIME: c_int = 10;
    static struct {
    unsigned int timeout;
    unsigned int port;
    } opts = {
    .timeout = RUNTIME,
    .port = PORT,
    };
#[no_mangle]
unsafe extern "C" fn handler(sig: c_int) {
    static void handler(int sig)
    {
    _exit(sig == SIGALRM ? 0 : 1);
    }
#[no_mangle]
unsafe extern "C" fn set_timeout() {
    static void set_timeout(void)
    {
    struct sigaction action = {
    .sa_handler = handler,
    };
    sigaction(SIGALRM, &action, core::ptr::null_mut());
    alarm(opts.timeout);
    }
#[no_mangle]
unsafe extern "C" fn do_connect(dst: *const sockaddr_in) {
    static void do_connect(const struct sockaddr_in *dst)
    {
    let mut s: c_int = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    if (s >= 0)
    fcntl(s, F_SETFL, O_NONBLOCK);
    connect(s, (struct sockaddr *)dst, sizeof(*dst));
    close(s);
    }
#[no_mangle]
unsafe extern "C" fn do_accept(src: *const sockaddr_in) {
    static void do_accept(const struct sockaddr_in *src)
    {
    int c, one = 1, s = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    if (s < 0)
    return;
    setsockopt(s, SOL_SOCKET, SO_REUSEADDR, &one, sizeof(one));
    setsockopt(s, SOL_SOCKET, SO_REUSEPORT, &one, sizeof(one));
    bind(s, (struct sockaddr *)src, sizeof(*src));
    listen(s, 16);
    c = accept(s, core::ptr::null_mut(), core::ptr::null_mut());
    if (c >= 0)
    close(c);
    close(s);
    }
#[no_mangle]
unsafe extern "C" fn accept_loop() -> c_int {
    static int accept_loop(void)
    {
    struct sockaddr_in src = {
    .sin_family = AF_INET,
    .sin_port = htons(opts.port),
    };
    inet_pton(AF_INET, "127.0.0.1", &src.sin_addr);
    set_timeout();
    for (;;)
    do_accept(&src);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn connect_loop() -> c_int {
    static int connect_loop(void)
    {
    struct sockaddr_in dst = {
    .sin_family = AF_INET,
    .sin_port = htons(opts.port),
    };
    inet_pton(AF_INET, "127.0.0.1", &dst.sin_addr);
    set_timeout();
    for (;;)
    do_connect(&dst);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn parse_opts(argc: c_int, argv: *mut c_char) {
    static void parse_opts(int argc, char **argv)
    {
    int c;
    while ((c = getopt(argc, argv, "t:p:")) != -1) {
    switch (c) {
    case 't':
    opts.timeout = atoi(optarg);
    break;
    case 'p':
    opts.port = atoi(optarg);
    break;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    pid_t p;
    parse_opts(argc, argv);
    p = fork();
    if (p < 0)
    return 111;
    if (p > 0)
    return accept_loop();
    return connect_loop();
    }
