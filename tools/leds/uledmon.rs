//! Automatically rewritten from C to Rust
//! Source: tools/leds/uledmon.c
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
// uledmon.c
//
// This program creates a new userspace LED class device and monitors it. A
// timestamp and brightness value is printed each time the brightness changes.
//
// Usage: uledmon <device-name>
//
// <device-name> is the name of the LED class device to be created. Pressing
// CTRL+C will exit.
//

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *const c_char) -> c_int {
    int main(int argc, char const *argv[])
    {
    struct uleds_user_dev uleds_dev;
    int fd, ret;
    int brightness;
    struct timespec ts;
    if (argc != 2) {
    fprintf(stderr, "Requires <device-name> argument\n");
    return 1;
    }
    strncpy(uleds_dev.name, argv[1], LED_MAX_NAME_SIZE);
    uleds_dev.max_brightness = 100;
    fd = open("/dev/uleds", O_RDWR);
    if (fd == -1) {
    perror("Failed to open /dev/uleds");
    return 1;
    }
    ret = write(fd, &uleds_dev, sizeof(uleds_dev));
    if (ret == -1) {
    perror("Failed to write to /dev/uleds");
    close(fd);
    return 1;
    }
    while (1) {
    ret = read(fd, &brightness, sizeof(brightness));
    if (ret == -1) {
    perror("Failed to read from /dev/uleds");
    close(fd);
    return 1;
    }
    clock_gettime(CLOCK_MONOTONIC, &ts);
    printf("[%ld.%09ld] %u\n", ts.tv_sec, ts.tv_nsec, brightness);
    }
    close(fd);
    return 0;
    }
