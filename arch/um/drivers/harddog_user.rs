//! Automatically rewritten from C to Rust
//! Source: arch/um/drivers/harddog_user.c
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
pub struct dog_data {
    pub stdin_fd: c_int,
    pub stdout_fd: c_int,
    pub close_me: [c_int; 2],
}

#[no_mangle]
unsafe extern "C" fn pre_exec(d: *mut c_void) {
    static void pre_exec(void *d)
    {
    struct dog_data *data = d;
    dup2(data.stdin_fd, 0);
    dup2(data.stdout_fd, 1);
    dup2(data.stdout_fd, 2);
    close(data.stdin_fd);
    close(data.stdout_fd);
    close(data.close_me[0]);
    close(data.close_me[1]);
    }
#[no_mangle]
pub unsafe extern "C" fn start_watchdog(in_fd_ret: *mut c_int, out_fd_ret: *mut c_int, sock: *mut c_char) -> c_int {
    int start_watchdog(int *in_fd_ret, int *out_fd_ret, char *sock)
    {
    struct dog_data data;
    int in_fds[2], out_fds[2], pid, n, err;
    char pid_buf[sizeof("nnnnnnn\0")], c;
    char *pid_args[] = { "/usr/bin/uml_watchdog", "-pid", pid_buf, core::ptr::null_mut() };
    char *mconsole_args[] = { "/usr/bin/uml_watchdog", "-mconsole", core::ptr::null_mut(),
    core::ptr::null_mut() };
    char **args = core::ptr::null_mut();
    err = os_pipe(in_fds, 1, 0);
    if (err < 0) {
    printk("harddog_open - os_pipe failed, err = %d\n", -err);
    goto out;
    }
    err = os_pipe(out_fds, 1, 0);
    if (err < 0) {
    printk("harddog_open - os_pipe failed, err = %d\n", -err);
    goto out_close_in;
    }
    data.stdin_fd = out_fds[0];
    data.stdout_fd = in_fds[1];
    data.close_me[0] = out_fds[1];
    data.close_me[1] = in_fds[0];
    if (sock != core::ptr::null_mut()) {
    mconsole_args[2] = sock;
    args = mconsole_args;
    }
    else {
// XXX The os_getpid() is not SMP correct
    sprintf(pid_buf, "%d", os_getpid());
    args = pid_args;
    }
    pid = run_helper(pre_exec, &data, args);
    close(out_fds[0]);
    close(in_fds[1]);
    if (pid < 0) {
    err = -pid;
    printk("harddog_open - run_helper failed, errno = %d\n", -err);
    goto out_close_out;
    }
    n = read(in_fds[0], &c, sizeof(c));
    if (n == 0) {
    printk("harddog_open - EOF on watchdog pipe\n");
    helper_wait(pid);
    err = -EIO;
    goto out_close_out;
    }
#[no_mangle]
pub unsafe extern "C" fn if(0: n <) -> else {
    printk("harddog_open - read of watchdog pipe failed, "
    "err = %d\n", errno);
    helper_wait(pid);
    err = n;
    goto out_close_out;
    }
// in_fd_ret = in_fds[0];
// out_fd_ret = out_fds[1];
    return 0;
    out_close_in:
    close(in_fds[0]);
    close(in_fds[1]);
    out_close_out:
    close(out_fds[0]);
    close(out_fds[1]);
    out:
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn stop_watchdog(in_fd: c_int, out_fd: c_int) {
    void stop_watchdog(int in_fd, int out_fd)
    {
    close(in_fd);
    close(out_fd);
    }
#[no_mangle]
pub unsafe extern "C" fn ping_watchdog(fd: c_int) -> c_int {
    int ping_watchdog(int fd)
    {
    int n;
    let mut c: c_char = '\n';
    n = write(fd, &c, sizeof(c));
    if (n != sizeof(c)) {
    printk("ping_watchdog - write failed, ret = %d, err = %d\n",
    n, errno);
    if (n < 0)
    return n;
    return -EIO;
    }
    return 1;
    }
