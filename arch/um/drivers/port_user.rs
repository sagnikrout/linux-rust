//! Automatically rewritten from C to Rust
//! Source: arch/um/drivers/port_user.c
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
pub struct port_chan {
    pub raw: c_int,
    pub tt: termios,
    pub kernel_data: *mut c_void,
    pub dev: [c_char; sizeof("32768\0")],
}

    static void *port_init(char *str, int device, const struct chan_opts *opts)
    {
    struct port_chan *data;
    void *kern_data;
    char *end;
    int port;
    if (*str != ':') {
    printk(UM_KERN_ERR "port_init : channel type 'port' must "
    "specify a port number\n");
    return core::ptr::null_mut();
    }
    str++;
    port = strtoul(str, &end, 0);
    if ((*end != '\0') || (end == str)) {
    printk(UM_KERN_ERR "port_init : couldn't parse port '%s'\n",
    str);
    return core::ptr::null_mut();
    }
    kern_data = port_data(port);
    if (kern_data == core::ptr::null_mut())
    return core::ptr::null_mut();
    data = uml_kmalloc(sizeof(*data), UM_GFP_KERNEL);
    if (data == core::ptr::null_mut())
    goto err;
// data = ((struct port_chan) { .raw  		= opts->raw,
    .kernel_data 	= kern_data });
    sprintf(data.dev, "%d", port);
    return data;
    err:
    port_kern_free(kern_data);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn port_free(d: *mut c_void) {
    static void port_free(void *d)
    {
    struct port_chan *data = d;
    port_kern_free(data.kernel_data);
    kfree(data);
    }
    static int port_open(int input, int output, int primary, void *d,
    char **dev_out)
    {
    struct port_chan *data = d;
    int fd, err;
    fd = port_wait(data.kernel_data);
    if ((fd >= 0) && data.raw) {
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
#[no_mangle]
unsafe extern "C" fn port_close(fd: c_int, d: *mut c_void) {
    static void port_close(int fd, void *d)
    {
    struct port_chan *data = d;
    port_remove_dev(data.kernel_data);
    os_close_file(fd);
    }
    const struct chan_ops port_ops = {
    .type		= "port",
    .init		= port_init,
    .open		= port_open,
    .close		= port_close,
    .read	        = generic_read,
    .write		= generic_write,
    .console_write	= generic_console_write,
    .window_size	= generic_window_size,
    .free		= port_free,
    .winch		= 1,
    };
#[no_mangle]
pub unsafe extern "C" fn port_listen_fd(port: c_int) -> c_int {
    int port_listen_fd(int port)
    {
    struct sockaddr_in addr;
    int fd, err, arg;
    fd = socket(PF_INET, SOCK_STREAM, 0);
    if (fd == -1)
    return -errno;
    arg = 1;
    if (setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &arg, sizeof(arg)) < 0) {
    err = -errno;
    goto out;
    }
    addr.sin_family = AF_INET;
    addr.sin_port = htons(port);
    addr.sin_addr.s_addr = htonl(INADDR_ANY);
    if (bind(fd, (struct sockaddr *) &addr, sizeof(addr)) < 0) {
    err = -errno;
    goto out;
    }
    if (listen(fd, 1) < 0) {
    err = -errno;
    goto out;
    }
    err = os_set_fd_block(fd, 0);
    if (err < 0)
    goto out;
    return fd;
    out:
    close(fd);
    return err;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_pre_exec_data {
    pub sock_fd: c_int,
    pub pipe_fd: c_int,
}

#[no_mangle]
unsafe extern "C" fn port_pre_exec(arg: *mut c_void) {
    static void port_pre_exec(void *arg)
    {
    struct port_pre_exec_data *data = arg;
    dup2(data.sock_fd, 0);
    dup2(data.sock_fd, 1);
    dup2(data.sock_fd, 2);
    close(data.sock_fd);
    dup2(data.pipe_fd, 3);
    shutdown(3, SHUT_RD);
    close(data.pipe_fd);
    }
#[no_mangle]
pub unsafe extern "C" fn port_connection(fd: c_int, socket: *mut c_int, pid_out: *mut c_int) -> c_int {
    int port_connection(int fd, int *socket, int *pid_out)
    {
    int new, err;
    char *env;
    char *argv[] = { "in.telnetd", "-L",
    OS_LIB_PATH "/uml/port-helper", core::ptr::null_mut() };
    struct port_pre_exec_data data;
    if ((env = getenv("UML_PORT_HELPER")))
    argv[2] = env;
    new = accept(fd, core::ptr::null_mut(), 0);
    if (new < 0)
    return -errno;
    err = os_access(argv[2], X_OK);
    if (err < 0) {
    printk(UM_KERN_ERR "port_connection : error accessing port-helper "
    "executable at %s: %s\n", argv[2], strerror(-err));
    if (env == core::ptr::null_mut())
    printk(UM_KERN_ERR "Set UML_PORT_HELPER environment "
    "variable to path to uml-utilities port-helper "
    "binary\n");
    goto out_close;
    }
    err = os_pipe(socket, 0, 0);
    if (err < 0)
    goto out_close;
    data = ((struct port_pre_exec_data)
    { .sock_fd  		= new,
    .pipe_fd 		= socket[1] });
    err = run_helper(port_pre_exec, &data, argv);
    if (err < 0)
    goto out_shutdown;
// pid_out = err;
    return new;
    out_shutdown:
    shutdown(socket[0], SHUT_RDWR);
    close(socket[0]);
    shutdown(socket[1], SHUT_RDWR);
    close(socket[1]);
    out_close:
    close(new);
    return err;
    }
