//! Automatically rewritten from C to Rust
//! Source: samples/user_events/example.c
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
//
// Copyright (c) 2021, Microsoft Corporation.
//
// Authors:
// Beau Belgrave <beaub@linux.microsoft.com>
//

    const char *data_file = "/sys/kernel/tracing/user_events_data";
    let mut enabled: c_int = 0;
#[no_mangle]
unsafe extern "C" fn event_reg(fd: c_int, command: *const c_char, write: *mut c_int, enabled: *mut c_int) -> c_int {
    static int event_reg(int fd, const char *command, int *write, int *enabled)
    {
    let mut reg: user_reg = {0};
    reg.size = sizeof(reg);
    reg.enable_bit = 31;
    reg.enable_size = sizeof(*enabled);
    reg.enable_addr = (__u64)enabled;
    reg.name_args = (__u64)command;
    if (ioctl(fd, DIAG_IOCSREG, &reg) == -1)
    return -1;
// write = reg.write_index;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int data_fd, write;
    struct iovec io[2];
    let mut count: __u32 = 0;
    data_fd = open(data_file, O_RDWR);
    if (event_reg(data_fd, "test u32 count", &write, &enabled) == -1)
    return errno;
// Setup iovec
    io[0].iov_base = &write;
    io[0].iov_len = sizeof(write);
    io[1].iov_base = &count;
    io[1].iov_len = sizeof(count);
    ask:
    printf("Press enter to check status...\n");
    getchar();
// Check if anyone is listening
    if (enabled) {
// Yep, trace out our data
    writev(data_fd, (const struct iovec *)io, 2);
// Increase the count
    count++;
    printf("Something was attached, wrote data\n");
    }
    goto ask;
    return 0;
    }
