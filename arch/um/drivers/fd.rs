//! Automatically rewritten from C to Rust
//! Source: arch/um/drivers/fd.c
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
// Copyright (C) 2001 - 2007 Jeff Dike (jdike@{linux.intel,addtoit}.com)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fd_chan {
    pub fd: c_int,
    pub raw: c_int,
    pub tt: termios,
    pub str: [c_char; sizeof("1234567890\0")],
}

    static void *fd_init(char *str, int device, const struct chan_opts *opts)
    {
    struct fd_chan *data;
    char *end;
    int n;
    if (*str != ':') {
    printk(UM_KERN_ERR "fd_init : channel type 'fd' must specify a "
    "file descriptor\n");
    return core::ptr::null_mut();
    }
    str++;
    n = strtoul(str, &end, 0);
    if ((*end != '\0') || (end == str)) {
    printk(UM_KERN_ERR "fd_init : couldn't parse file descriptor "
    "'%s'\n", str);
    return core::ptr::null_mut();
    }
    data = uml_kmalloc(sizeof(*data), UM_GFP_KERNEL);
    if (data == core::ptr::null_mut())
    return core::ptr::null_mut();
// data = ((struct fd_chan) { .fd  	= n,
    .raw  	= opts.raw });
    return data;
    }
#[no_mangle]
unsafe extern "C" fn fd_open(input: c_int, output: c_int, primary: c_int, d: *mut c_void, dev_out: *mut c_char) -> c_int {
    static int fd_open(int input, int output, int primary, void *d, char **dev_out)
    {
    struct fd_chan *data = d;
    int err;
    if (data.raw && isatty(data.fd)) {
    CATCH_EINTR(err = tcgetattr(data.fd, &data.tt));
    if (err)
    return err;
    err = raw(data.fd);
    if (err)
    return err;
    }
    sprintf(data.str, "%d", data.fd);
// dev_out = data->str;
    return data.fd;
    }
#[no_mangle]
unsafe extern "C" fn fd_close(fd: c_int, d: *mut c_void) {
    static void fd_close(int fd, void *d)
    {
    struct fd_chan *data = d;
    int err;
    if (!data.raw || !isatty(fd))
    return;
    CATCH_EINTR(err = tcsetattr(fd, TCSAFLUSH, &data.tt));
    if (err)
    printk(UM_KERN_ERR "Failed to restore terminal state - "
    "errno = %d\n", -err);
    data.raw = 0;
    }
    const struct chan_ops fd_ops = {
    .type		= "fd",
    .init		= fd_init,
    .open		= fd_open,
    .close		= fd_close,
    .read		= generic_read,
    .write		= generic_write,
    .console_write	= generic_console_write,
    .window_size	= generic_window_size,
    .free		= generic_free,
    .winch		= 1,
    };
