//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/media_tests/video_device_test.c
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
// video_device_test - Video Device Test
//
// Copyright (c) 2016 Shuah Khan <shuahkh@osg.samsung.com>
// Copyright (c) 2016 Samsung Electronics Co., Ltd.
//
// This file adds a test for Video Device. This test should not be included
// in the Kselftest run. This test should be run when hardware and driver
// that makes use of V4L2 API is present.
//
// This test opens user specified Video Device and calls video ioctls in a
// loop once every 10 seconds.
//
// Usage:
// sudo ./video_device_test -d /dev/videoX
//
// While test is running, remove the device or unbind the driver and
// ensure there are no use after free errors and other Oops in the
// dmesg.
// When possible, enable KaSan kernel config option for use-after-free
// error detection.
//

pub const PRIORITY_MAX: c_int = 4;
#[no_mangle]
pub unsafe extern "C" fn priority_test(fd: c_int) -> c_int {
    int priority_test(int fd)
    {
// This test will try to update the priority associated with a file descriptor
    enum v4l2_priority old_priority, new_priority, priority_to_compare;
    int ret;
    let mut result: c_int = 0;
    ret = ioctl(fd, VIDIOC_G_PRIORITY, &old_priority);
    if (ret < 0) {
    printf("Failed to get priority: %s\n", strerror(errno));
    return -1;
    }
    new_priority = (old_priority + 1) % PRIORITY_MAX;
    ret = ioctl(fd, VIDIOC_S_PRIORITY, &new_priority);
    if (ret < 0) {
    printf("Failed to set priority: %s\n", strerror(errno));
    return -1;
    }
    ret = ioctl(fd, VIDIOC_G_PRIORITY, &priority_to_compare);
    if (ret < 0) {
    printf("Failed to get new priority: %s\n", strerror(errno));
    result = -1;
    goto cleanup;
    }
    if (priority_to_compare != new_priority) {
    printf("Priority wasn't set - test failed\n");
    result = -1;
    }
    cleanup:
    ret = ioctl(fd, VIDIOC_S_PRIORITY, &old_priority);
    if (ret < 0) {
    printf("Failed to restore priority: %s\n", strerror(errno));
    return -1;
    }
    return result;
    }
#[no_mangle]
pub unsafe extern "C" fn loop_test(fd: c_int) -> c_int {
    int loop_test(int fd)
    {
    int count;
    struct v4l2_tuner vtuner;
    struct v4l2_capability vcap;
    int ret;
// Generate random number of interations
    srand((unsigned int) time(core::ptr::null_mut()));
    count = rand();
    printf("\nNote:\n"
    "While test is running, remove the device or unbind\n"
    "driver and ensure there are no use after free errors\n"
    "and other Oops in the dmesg. When possible, enable KaSan\n"
    "kernel config option for use-after-free error detection.\n\n");
    while (count > 0) {
    ret = ioctl(fd, VIDIOC_QUERYCAP, &vcap);
    if (ret < 0)
    printf("VIDIOC_QUERYCAP errno %s\n", strerror(errno));
    else
    printf("Video device driver %s\n", vcap.driver);
    ret = ioctl(fd, VIDIOC_G_TUNER, &vtuner);
    if (ret < 0)
    printf("VIDIOC_G_TUNER, errno %s\n", strerror(errno));
    else
    printf("type %d rangelow %d rangehigh %d\n",
    vtuner.type, vtuner.rangelow, vtuner.rangehigh);
    sleep(10);
    count--;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int opt;
    char video_dev[256];
    int fd;
    int test_result;
    if (argc < 2) {
    printf("Usage: %s [-d </dev/videoX>]\n", argv[0]);
    exit(-1);
    }
// Process arguments
    while ((opt = getopt(argc, argv, "d:")) != -1) {
    switch (opt) {
    case 'd':
    strncpy(video_dev, optarg, sizeof(video_dev) - 1);
    video_dev[sizeof(video_dev)-1] = '\0';
    break;
    default:
    printf("Usage: %s [-d </dev/videoX>]\n", argv[0]);
    exit(-1);
    }
    }
// Open Video device and keep it open
    fd = open(video_dev, O_RDWR);
    if (fd == -1) {
    printf("Video Device open errno %s\n", strerror(errno));
    exit(-1);
    }
    test_result = priority_test(fd);
    if (!test_result)
    printf("Priority test - PASSED\n");
    else
    printf("Priority test - FAILED\n");
    loop_test(fd);
    }
