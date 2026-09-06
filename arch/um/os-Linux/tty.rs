//! Automatically rewritten from C to Rust
//! Source: arch/um/os-Linux/tty.c
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
// Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct grantpt_info {
    pub fd: c_int,
    pub res: c_int,
    pub err: c_int,
}

#[no_mangle]
unsafe extern "C" fn grantpt_cb(arg: *mut c_void) {
    static void grantpt_cb(void *arg)
    {
    struct grantpt_info *info = arg;
    info.res = grantpt(info.fd);
    info.err = errno;
    }
#[no_mangle]
pub unsafe extern "C" fn get_pty() -> c_int {
    int get_pty(void)
    {
    struct grantpt_info info;
    int fd, err;
    fd = open("/dev/ptmx", O_RDWR);
    if (fd < 0) {
    err = -errno;
    printk(UM_KERN_ERR "get_pty : Couldn't open /dev/ptmx - "
    "err = %d\n", errno);
    return err;
    }
    info.fd = fd;
    initial_thread_cb(grantpt_cb, &info);
    if (info.res < 0) {
    err = -info.err;
    printk(UM_KERN_ERR "get_pty : Couldn't grant pty - "
    "errno = %d\n", -info.err);
    goto out;
    }
    if (unlockpt(fd) < 0) {
    err = -errno;
    printk(UM_KERN_ERR "get_pty : Couldn't unlock pty - "
    "errno = %d\n", errno);
    goto out;
    }
    return fd;
    out:
    close(fd);
    return err;
    }
