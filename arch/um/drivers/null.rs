//! Automatically rewritten from C to Rust
//! Source: arch/um/drivers/null.c
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
// Copyright (C) 2002 - 2007 Jeff Dike (jdike@{linux.intel,addtoit}.com)
//

// This address is used only as a unique identifier
    static int null_chan;
    static void *null_init(char *str, int device, const struct chan_opts *opts)
    {
    return &null_chan;
    }
    static int null_open(int input, int output, int primary, void *d,
    char **dev_out)
    {
    int fd;
// dev_out = NULL;
    fd = open(DEV_NULL, O_RDWR);
    return (fd < 0) ? -errno : fd;
    }
#[no_mangle]
unsafe extern "C" fn null_read(fd: c_int, c_out: *mut __u8, unused: *mut c_void) -> c_int {
    static int null_read(int fd, __u8 *c_out, void *unused)
    {
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn null_free(data: *mut c_void) {
    static void null_free(void *data)
    {
    }
    const struct chan_ops null_ops = {
    .type		= "null",
    .init		= null_init,
    .open		= null_open,
    .close		= generic_close,
    .read		= null_read,
    .write		= generic_write,
    .console_write	= generic_console_write,
    .window_size	= generic_window_size,
    .free		= null_free,
    .winch		= 0,
    };
