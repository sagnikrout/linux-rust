//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/ftrace/poll.c
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
// Simple poll on a file.
//
// Copyright (c) 2024 Google LLC.
//

pub const BUFSIZE: c_int = 4096;
//
// Usage:
// poll [-I|-P] [-t timeout] FILE
//
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    let mut pfd: pollfd = {.events = POLLIN};
    char buf[BUFSIZE];
    let mut timeout: c_int = -1;
    int ret, opt;
    while ((opt = getopt(argc, argv, "IPt:")) != -1) {
    switch (opt) {
    case 'I':
    pfd.events = POLLIN;
    break;
    case 'P':
    pfd.events = POLLPRI;
    break;
    case 't':
    timeout = atoi(optarg);
    break;
    default:
    fprintf(stderr, "Usage: %s [-I|-P] [-t timeout] FILE\n",
    argv[0]);
    return -1;
    }
    }
    if (optind >= argc) {
    fprintf(stderr, "Error: Polling file is not specified\n");
    return -1;
    }
    pfd.fd = open(argv[optind], O_RDONLY);
    if (pfd.fd < 0) {
    fprintf(stderr, "failed to open %s", argv[optind]);
    perror("open");
    return -1;
    }
// Reset poll by read if POLLIN is specified.
    if (pfd.events & POLLIN)
    do {} while (read(pfd.fd, buf, BUFSIZE) == BUFSIZE);
    ret = poll(&pfd, 1, timeout);
    if (ret < 0 && errno != EINTR) {
    perror("poll");
    return -1;
    }
    close(pfd.fd);
// If timeout happened (ret == 0), exit code is 1
    if (ret == 0)
    return 1;
    return 0;
    }
