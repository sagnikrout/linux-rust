//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/gpio/gpio-line-name.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// GPIO character device helper for reading line names.
//
// Copyright (C) 2021 Bartosz Golaszewski <brgl@bgdev.pl>
//

#[no_mangle]
unsafe extern "C" fn print_usage() {
    static void print_usage(void)
    {
    printf("usage:\n");
    printf("  gpio-line-name <chip path> <line offset>\n");
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct gpio_v2_line_info info;
    int fd, ret;
    char *endp;
    if (argc != 3) {
    print_usage();
    return EXIT_FAILURE;
    }
    fd = open(argv[1], O_RDWR);
    if (fd < 0) {
    perror("unable to open the GPIO chip");
    return EXIT_FAILURE;
    }
    memset(&info, 0, sizeof(info));
    info.offset = strtoul(argv[2], &endp, 10);
    if (*endp != '\0') {
    print_usage();
    return EXIT_FAILURE;
    }
    ret = ioctl(fd, GPIO_V2_GET_LINEINFO_IOCTL, &info);
    if (ret) {
    perror("line info ioctl failed");
    return EXIT_FAILURE;
    }
    printf("%s\n", info.name);
    return EXIT_SUCCESS;
    }
