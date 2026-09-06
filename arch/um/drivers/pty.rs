//! Automatically rewritten from C to Rust
//! Source: arch/um/drivers/pty.c
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
// Copyright (C) 2001 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pty_chan {
    pub dev): *mut *mut *mut void (announce)(char dev_name, int,
    pub dev: c_int,
    pub raw: c_int,
    pub tt: termios,
    pub dev_name: [c_char; sizeof("/dev/pts/0123456\0")],
}

    static void *pty_chan_init(char *str, int device, const struct chan_opts *opts)
    {
    struct pty_chan *data;
    data = uml_kmalloc(sizeof(*data), UM_GFP_KERNEL);
    if (data == core::ptr::null_mut())
    return core::ptr::null_mut();
// data = ((struct pty_chan) { .announce  	= opts->announce,
    .dev  		= device,
    .raw  		= opts.raw });
    return data;
    }
    static int pts_open(int input, int output, int primary, void *d,
    char **dev_out)
    {
    struct pty_chan *data = d;
    char *dev;
    int fd, err;
    fd = get_pty();
    if (fd < 0) {
    err = -errno;
    printk(UM_KERN_ERR "open_pts : Failed to open pts\n");
    return err;
    }
    if (data.raw) {
    CATCH_EINTR(err = tcgetattr(fd, &data.tt));
    if (err)
    goto out_close;
    err = raw(fd);
    if (err)
    goto out_close;
    }
    dev = ptsname(fd);
    sprintf(data.dev_name, "%s", dev);
// dev_out = data->dev_name;
    if (data.announce)
    (*data.announce)(dev, data.dev);
    return fd;
    out_close:
    close(fd);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn getmaster(line: *mut c_char) -> c_int {
    static int getmaster(char *line)
    {
    struct stat buf;
    char *pty, *bank, *cp;
    int master, err;
    pty = &line[strlen("/dev/ptyp")];
    for (bank = "pqrs"; *bank; bank++) {
    line[strlen("/dev/pty")] = *bank;
// pty = '0';
// Did we hit the end ?
    if ((stat(line, &buf) < 0) && (errno == ENOENT))
    break;
    for (cp = "0123456789abcdef"; *cp; cp++) {
// pty = *cp;
    master = open(line, O_RDWR);
    if (master >= 0) {
    char *tp = &line[strlen("/dev/")];
// verify slave side is usable
// tp = 't';
    err = access(line, R_OK | W_OK);
// tp = 'p';
    if (!err)
    return master;
    close(master);
    }
    }
    }
    printk(UM_KERN_ERR "getmaster - no usable host pty devices\n");
    return -ENOENT;
    }
    static int pty_open(int input, int output, int primary, void *d,
    char **dev_out)
    {
    struct pty_chan *data = d;
    int fd, err;
    char dev[sizeof("/dev/ptyxx\0")] = "/dev/ptyxx";
    fd = getmaster(dev);
    if (fd < 0)
    return fd;
    if (data.raw) {
    err = raw(fd);
    if (err) {
    close(fd);
    return err;
    }
    }
    if (data.announce)
    (*data.announce)(dev, data.dev);
    sprintf(data.dev_name, "%s", dev);
// dev_out = data->dev_name;
    return fd;
    }
    const struct chan_ops pty_ops = {
    .type		= "pty",
    .init		= pty_chan_init,
    .open		= pty_open,
    .close		= generic_close,
    .read		= generic_read,
    .write		= generic_write,
    .console_write	= generic_console_write,
    .window_size	= generic_window_size,
    .free		= generic_free,
    .winch		= 0,
    };
    const struct chan_ops pts_ops = {
    .type		= "pts",
    .init		= pty_chan_init,
    .open		= pts_open,
    .close		= generic_close,
    .read		= generic_read,
    .write		= generic_write,
    .console_write	= generic_console_write,
    .window_size	= generic_window_size,
    .free		= generic_free,
    .winch		= 0,
    };
