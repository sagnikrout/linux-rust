//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/splice/splice_read.c
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
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    int fd;
    size_t size;
    ssize_t spliced;
    if (argc < 2) {
    fprintf(stderr, "Usage: %s INPUT [BYTES]\n", argv[0]);
    return EXIT_FAILURE;
    }
    fd = open(argv[1], O_RDONLY);
    if (fd < 0) {
    perror(argv[1]);
    return EXIT_FAILURE;
    }
    if (argc == 3)
    size = atol(argv[2]);
    else {
    struct stat statbuf;
    if (fstat(fd, &statbuf) < 0) {
    perror(argv[1]);
    return EXIT_FAILURE;
    }
    if (statbuf.st_size > INT_MAX) {
    fprintf(stderr, "%s: Too big\n", argv[1]);
    return EXIT_FAILURE;
    }
    size = statbuf.st_size;
    }
// splice(2) file to stdout.
    spliced = splice(fd, core::ptr::null_mut(), STDOUT_FILENO, core::ptr::null_mut(),
    size, SPLICE_F_MOVE);
    if (spliced < 0) {
    perror("splice");
    return EXIT_FAILURE;
    }
    close(fd);
    return EXIT_SUCCESS;
    }
