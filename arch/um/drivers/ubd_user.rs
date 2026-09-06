//! Automatically rewritten from C to Rust
//! Source: arch/um/drivers/ubd_user.c
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
// Copyright (C) 2016 Anton Ivanov (aivanov@brocade.com)
// Copyright (C) 2000, 2001, 2002 Jeff Dike (jdike@karaya.com)
// Copyright (C) 2001 Ridgerun,Inc (glonnon@ridgerun.com)
//

    static struct pollfd kernel_pollfd;
#[no_mangle]
pub unsafe extern "C" fn start_io_thread(td_out: *mut os_helper_thread, fd_out: *mut c_int) -> c_int {
    int start_io_thread(struct os_helper_thread **td_out, int *fd_out)
    {
    int fds[2], err;
    err = os_pipe(fds, 1, 1);
    if(err < 0){
    printk("start_io_thread - os_pipe failed, err = %d\n", -err);
    goto out;
    }
    kernel_fd = fds[0];
    kernel_pollfd.fd = kernel_fd;
    kernel_pollfd.events = POLLIN;
// fd_out = fds[1];
    err = os_set_fd_block(*fd_out, 0);
    err |= os_set_fd_block(kernel_fd, 0);
    if (err) {
    printk("start_io_thread - failed to set nonblocking I/O.\n");
    goto out_close;
    }
    err = os_run_helper_thread(td_out, io_thread, core::ptr::null_mut());
    if (err < 0) {
    printk("%s - failed to run helper thread, err = %d\n",
    __func__, -err);
    goto out_close;
    }
    return 0;
    out_close:
    os_close_file(fds[0]);
    os_close_file(fds[1]);
    kernel_fd = -1;
// fd_out = -1;
    out:
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn ubd_read_poll(timeout: c_int) -> c_int {
    int ubd_read_poll(int timeout)
    {
    kernel_pollfd.events = POLLIN;
    return poll(&kernel_pollfd, 1, timeout);
    }
#[no_mangle]
pub unsafe extern "C" fn ubd_write_poll(timeout: c_int) -> c_int {
    int ubd_write_poll(int timeout)
    {
    kernel_pollfd.events = POLLOUT;
    return poll(&kernel_pollfd, 1, timeout);
    }
