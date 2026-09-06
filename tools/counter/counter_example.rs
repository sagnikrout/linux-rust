//! Automatically rewritten from C to Rust
//! Source: tools/counter/counter_example.c
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


// SPDX-License-Identifier: GPL-2.0-only
// Counter - example userspace application
//
// The userspace application opens /dev/counter0, configures the
// COUNTER_EVENT_INDEX event channel 0 to gather Count 0 count and Count
// 1 count, and prints out the data as it becomes available on the
// character device node.
//
// Copyright (C) 2021 William Breathitt Gray
//

    static struct counter_watch watches[2] = {
    {
// Component data: Count 0 count
    .component.type = COUNTER_COMPONENT_COUNT,
    .component.scope = COUNTER_SCOPE_COUNT,
    .component.parent = 0,
// Event type: Index
    .event = COUNTER_EVENT_INDEX,
// Device event channel 0
    .channel = 0,
    },
    {
// Component data: Count 1 count
    .component.type = COUNTER_COMPONENT_COUNT,
    .component.scope = COUNTER_SCOPE_COUNT,
    .component.parent = 1,
// Event type: Index
    .event = COUNTER_EVENT_INDEX,
// Device event channel 0
    .channel = 0,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    int fd;
    int ret;
    int i;
    struct counter_event event_data[2];
    fd = open("/dev/counter0", O_RDWR);
    if (fd == -1) {
    perror("Unable to open /dev/counter0");
    return 1;
    }
    for (i = 0; i < 2; i++) {
    ret = ioctl(fd, COUNTER_ADD_WATCH_IOCTL, watches + i);
    if (ret == -1) {
    fprintf(stderr, "Error adding watches[%d]: %s\n", i,
    strerror(errno));
    return 1;
    }
    }
    ret = ioctl(fd, COUNTER_ENABLE_EVENTS_IOCTL);
    if (ret == -1) {
    perror("Error enabling events");
    return 1;
    }
    for (;;) {
    ret = read(fd, event_data, sizeof(event_data));
    if (ret == -1) {
    perror("Failed to read event data");
    return 1;
    }
    if (ret != sizeof(event_data)) {
    fprintf(stderr, "Failed to read event data\n");
    return -EIO;
    }
    printf("Timestamp 0: %llu\tCount 0: %llu\n"
    "Error Message 0: %s\n"
    "Timestamp 1: %llu\tCount 1: %llu\n"
    "Error Message 1: %s\n",
    event_data[0].timestamp, event_data[0].value,
    strerror(event_data[0].status),
    event_data[1].timestamp, event_data[1].value,
    strerror(event_data[1].status));
    }
    return 0;
    }
