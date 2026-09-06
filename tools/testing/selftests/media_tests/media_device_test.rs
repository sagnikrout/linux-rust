//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/media_tests/media_device_test.c
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
// media_device_test.c - Media Controller Device ioctl loop Test
//
// Copyright (c) 2016 Shuah Khan <shuahkh@osg.samsung.com>
// Copyright (c) 2016 Samsung Electronics Co., Ltd.
//
// This file adds a test for Media Controller API.
// This test should be run as root and should not be
// included in the Kselftest run. This test should be
// run when hardware and driver that makes use Media
// Controller API are present in the system.
//
// This test opens user specified Media Device and calls
// MEDIA_IOC_DEVICE_INFO ioctl in a loop once every 10
// seconds.
//
// Usage:
// sudo ./media_device_test -d /dev/mediaX
//
// While test is running, remove the device and
// ensure there are no use after free errors and
// other Oops in the dmesg. Enable KaSan kernel
// config option for use-after-free error detection.
//

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int opt;
    char media_device[256];
    int count;
    struct media_device_info mdi;
    int ret;
    int fd;
    if (argc < 2) {
    printf("Usage: %s [-d </dev/mediaX>]\n", argv[0]);
    exit(-1);
    }
// Process arguments
    while ((opt = getopt(argc, argv, "d:")) != -1) {
    switch (opt) {
    case 'd':
    strncpy(media_device, optarg, sizeof(media_device) - 1);
    media_device[sizeof(media_device)-1] = '\0';
    break;
    default:
    printf("Usage: %s [-d </dev/mediaX>]\n", argv[0]);
    exit(-1);
    }
    }
    if (getuid() != 0)
    ksft_exit_skip("Please run the test as root - Exiting.\n");
// Generate random number of interations
    srand((unsigned int) time(core::ptr::null_mut()));
    count = rand();
// Open Media device and keep it open
    fd = open(media_device, O_RDWR);
    if (fd == -1) {
    printf("Media Device open errno %s\n", strerror(errno));
    exit(-1);
    }
    printf("\nNote:\n"
    "While test is running, remove the device and\n"
    "ensure there are no use after free errors and\n"
    "other Oops in the dmesg. Enable KaSan kernel\n"
    "config option for use-after-free error detection.\n\n");
    printf("Running test for %d iterations\n", count);
    while (count > 0) {
    ret = ioctl(fd, MEDIA_IOC_DEVICE_INFO, &mdi);
    if (ret < 0)
    printf("Media Device Info errno %s\n", strerror(errno));
    else
    printf("Media device model %s driver %s - count %d\n",
    mdi.model, mdi.driver, count);
    sleep(10);
    count--;
    }
    }
