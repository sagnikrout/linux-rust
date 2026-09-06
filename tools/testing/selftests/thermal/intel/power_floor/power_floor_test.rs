//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/thermal/intel/power_floor/power_floor_test.c
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
pub unsafe extern "C" fn power_floor_exit(signum: c_int) {
    void power_floor_exit(int signum)
    {
    int fd;
// Disable feature via sysfs knob
    fd = open(POWER_FLOOR_ENABLE_ATTRIBUTE, O_RDWR);
    if (fd < 0) {
    perror("Unable to open power floor enable file\n");
    exit(1);
    }
    if (write(fd, "0\n", 2) < 0) {
    perror("Can' disable power floor notifications\n");
    exit(1);
    }
    printf("Disabled power floor notifications\n");
    close(fd);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct pollfd ufd;
    char status_str[3];
    int fd, ret;
    if (signal(SIGINT, power_floor_exit) == SIG_IGN)
    signal(SIGINT, SIG_IGN);
    if (signal(SIGHUP, power_floor_exit) == SIG_IGN)
    signal(SIGHUP, SIG_IGN);
    if (signal(SIGTERM, power_floor_exit) == SIG_IGN)
    signal(SIGTERM, SIG_IGN);
// Enable feature via sysfs knob
    fd = open(POWER_FLOOR_ENABLE_ATTRIBUTE, O_RDWR);
    if (fd < 0) {
    perror("Unable to open power floor enable file\n");
    exit(1);
    }
    if (write(fd, "1\n", 2) < 0) {
    perror("Can't enable power floor notifications\n");
    exit(1);
    }
    close(fd);
    printf("Enabled power floor notifications\n");
    while (1) {
    fd = open(POWER_FLOOR_STATUS_ATTRIBUTE, O_RDONLY);
    if (fd < 0) {
    perror("Unable to power floor status file\n");
    exit(1);
    }
    if ((lseek(fd, 0L, SEEK_SET)) < 0) {
    fprintf(stderr, "Failed to set pointer to beginning\n");
    exit(1);
    }
    if (read(fd, status_str, sizeof(status_str)) < 0) {
    fprintf(stderr, "Failed to read from:%s\n",
    POWER_FLOOR_STATUS_ATTRIBUTE);
    exit(1);
    }
    ufd.fd = fd;
    ufd.events = POLLPRI;
    ret = poll(&ufd, 1, -1);
    if (ret < 0) {
    perror("poll error");
    exit(1);
    } else if (ret == 0) {
    printf("Poll Timeout\n");
    } else {
    if ((lseek(fd, 0L, SEEK_SET)) < 0) {
    fprintf(stderr, "Failed to set pointer to beginning\n");
    exit(1);
    }
    if (read(fd, status_str, sizeof(status_str)) < 0)
    exit(0);
    printf("power floor status: %s\n", status_str);
    }
    close(fd);
    }
    }
