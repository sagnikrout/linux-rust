//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/gpio/gpio-chip-info.c
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
// GPIO character device helper for reading chip information.
//
// Copyright (C) 2021 Bartosz Golaszewski <brgl@bgdev.pl>
//

#[no_mangle]
unsafe extern "C" fn print_usage() {
    static void print_usage(void)
    {
    printf("usage:\n");
    printf("  gpio-chip-info <chip path> [name|label|num-lines]\n");
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct gpiochip_info info;
    int fd, ret;
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
    ret = ioctl(fd, GPIO_GET_CHIPINFO_IOCTL, &info);
    if (ret) {
    perror("chip info ioctl failed");
    return EXIT_FAILURE;
    }
    if (strcmp(argv[2], "name") == 0) {
    printf("%s\n", info.name);
    } else if (strcmp(argv[2], "label") == 0) {
    printf("%s\n", info.label);
    } else if (strcmp(argv[2], "num-lines") == 0) {
    printf("%u\n", info.lines);
    } else {
    fprintf(stderr, "unknown command: %s\n", argv[2]);
    return EXIT_FAILURE;
    }
    return EXIT_SUCCESS;
    }
