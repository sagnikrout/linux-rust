//! Automatically rewritten from C to Rust
//! Source: arch/um/drivers/tty.c
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
pub struct tty_chan {
    pub dev: *mut c_char,
    pub raw: c_int,
    pub tt: termios,
}

    static void *tty_chan_init(char *str, int device, const struct chan_opts *opts)
    {
    struct tty_chan *data;
    if (*str != ':') {
    printk(UM_KERN_ERR "tty_init : channel type 'tty' must specify "
    "a device\n");
    return core::ptr::null_mut();
    }
    str++;
    data = uml_kmalloc(sizeof(*data), UM_GFP_KERNEL);
    if (data == core::ptr::null_mut())
    return core::ptr::null_mut();
// data = ((struct tty_chan) { .dev 	= str,
    .raw 	= opts.raw });
    return data;
    }
    static int tty_open(int input, int output, int primary, void *d,
    char **dev_out)
    {
    struct tty_chan *data = d;
    int fd, err, mode = 0;
    if (input && output)
    mode = O_RDWR;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: input) -> else {
    else if (input)
    mode = O_RDONLY;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: output) -> else {
    else if (output)
    mode = O_WRONLY;
    fd = open(data.dev, mode);
    if (fd < 0)
    return -errno;
    if (data.raw) {
    CATCH_EINTR(err = tcgetattr(fd, &data.tt));
    if (err)
    return err;
    err = raw(fd);
    if (err)
    return err;
    }
// dev_out = data->dev;
    return fd;
    }
    const struct chan_ops tty_ops = {
    .type		= "tty",
    .init		= tty_chan_init,
    .open		= tty_open,
    .close		= generic_close,
    .read		= generic_read,
    .write		= generic_write,
    .console_write	= generic_console_write,
    .window_size	= generic_window_size,
    .free		= generic_free,
    .winch		= 0,
    };
