//! Automatically rewritten from C to Rust
//! Source: tools/leds/led_hw_brightness_mon.c
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
// led_hw_brightness_mon.c
//
// This program monitors LED brightness level changes having its origin
// in hardware/firmware, i.e. outside of kernel control.
// A timestamp and brightness value is printed each time the brightness changes.
//
// Usage: led_hw_brightness_mon <device-name>
//
// <device-name> is the name of the LED class device to be monitored. Pressing
// CTRL+C will exit.
//

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *const c_char) -> c_int {
    int main(int argc, char const *argv[])
    {
    int fd, ret;
    char brightness_file_path[LED_MAX_NAME_SIZE + 11];
    struct pollfd pollfd;
    struct timespec ts;
    char buf[11];
    if (argc != 2) {
    fprintf(stderr, "Requires <device-name> argument\n");
    return 1;
    }
    snprintf(brightness_file_path, LED_MAX_NAME_SIZE,
    "/sys/class/leds/%s/brightness_hw_changed", argv[1]);
    fd = open(brightness_file_path, O_RDONLY);
    if (fd == -1) {
    printf("Failed to open %s file\n", brightness_file_path);
    return 1;
    }
//
// read may fail if no hw brightness change has occurred so far,
// but it is required to avoid spurious poll notifications in
// the opposite case.
//
    read(fd, buf, sizeof(buf));
    pollfd.fd = fd;
    pollfd.events = POLLPRI;
    while (1) {
    ret = poll(&pollfd, 1, -1);
    if (ret == -1) {
    printf("Failed to poll %s file (%d)\n",
    brightness_file_path, ret);
    ret = 1;
    break;
    }
    clock_gettime(CLOCK_MONOTONIC, &ts);
    ret = read(fd, buf, sizeof(buf));
    if (ret < 0)
    break;
    ret = lseek(pollfd.fd, 0, SEEK_SET);
    if (ret < 0) {
    printf("lseek failed (%d)\n", ret);
    break;
    }
    printf("[%ld.%09ld] %d\n", ts.tv_sec, ts.tv_nsec, atoi(buf));
    }
    close(fd);
    return ret;
    }
