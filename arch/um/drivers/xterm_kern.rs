//! Automatically rewritten from C to Rust
//! Source: arch/um/drivers/xterm_kern.c
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
pub struct xterm_wait {
    pub ready: completion,
    pub fd: c_int,
    pub pid: c_int,
    pub new_fd: c_int,
}

#[no_mangle]
unsafe extern "C" fn xterm_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t xterm_interrupt(int irq, void *data)
    {
    struct xterm_wait *xterm = data;
    let mut fd: c_int = -1, n_fds = 1;
    ssize_t ret;
    ret = os_rcv_fd_msg(xterm.fd, &fd, n_fds,
    &xterm.pid, sizeof(xterm.pid));
    if (ret == -EAGAIN)
    return IRQ_NONE;
    if (ret < 0)
    fd = ret;
#[no_mangle]
pub unsafe extern "C" fn if(sizeof(xterm->pid): ret !=) -> else {
    else if (ret != sizeof(xterm.pid))
    fd = -EMSGSIZE;
    xterm.new_fd = fd;
    complete(&xterm.ready);
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn xterm_fd(socket: c_int, pid_out: *mut c_int) -> c_int {
    int xterm_fd(int socket, int *pid_out)
    {
    struct xterm_wait *data;
    int err, ret;
    data = kmalloc_obj(*data);
    if (data == core::ptr::null_mut()) {
    printk(KERN_ERR "xterm_fd : failed to allocate xterm_wait\n");
    return -ENOMEM;
    }
// This is a locked semaphore...
// data = ((struct xterm_wait) { .fd 		= socket,
    .pid 		= -1,
    .new_fd	 	= -1 });
    init_completion(&data.ready);
    err = um_request_irq(XTERM_IRQ, socket, IRQ_READ, xterm_interrupt,
    IRQF_SHARED, "xterm", data);
    if (err < 0) {
    printk(KERN_ERR "xterm_fd : failed to get IRQ for xterm, "
    "err = %d\n",  err);
    ret = err;
    goto out;
    }
// ... so here we wait for an xterm interrupt.
//
// XXX Note, if the xterm doesn't work for some reason (eg. DISPLAY
// isn't set) this will hang...
    wait_for_completion(&data.ready);
    um_free_irq(XTERM_IRQ, data);
    ret = data.new_fd;
// pid_out = data->pid;
    out:
    kfree(data);
    return ret;
    }
