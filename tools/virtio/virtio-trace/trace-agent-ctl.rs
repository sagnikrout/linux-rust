//! Automatically rewritten from C to Rust
//! Source: tools/virtio/virtio-trace/trace-agent-ctl.c
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
// Controller of read/write threads for virtio-trace
//
// Copyright (C) 2012 Hitachi, Ltd.
// Created by Yoshihiro Yunomae <yoshihiro.yunomae.ez@hitachi.com>
// Masami Hiramatsu <masami.hiramatsu.pt@hitachi.com>
//
// Macro flag: #define _GNU_SOURCE

pub const HOST_MSG_SIZE: c_int = 256;
pub const EVENT_WAIT_MSEC: c_int = 100;
    static volatile sig_atomic_t global_signal_val;
    bool global_sig_receive;	/* default false */
    bool global_run_operation;	/* default false*/
// Handle SIGTERM/SIGINT/SIGQUIT to exit
#[no_mangle]
unsafe extern "C" fn signal_handler(sig: c_int) {
    static void signal_handler(int sig)
    {
    global_signal_val = sig;
    }
#[no_mangle]
pub unsafe extern "C" fn rw_ctl_init(ctl_path: *const c_char) -> c_int {
    int rw_ctl_init(const char *ctl_path)
    {
    int ctl_fd;
    ctl_fd = open(ctl_path, O_RDONLY);
    if (ctl_fd == -1) {
    pr_err("Cannot open ctl_fd\n");
    goto error;
    }
    return ctl_fd;
    error:
    exit(EXIT_FAILURE);
    }
#[no_mangle]
unsafe extern "C" fn wait_order(ctl_fd: c_int) -> c_int {
    static int wait_order(int ctl_fd)
    {
    struct pollfd poll_fd;
    let mut ret: c_int = 0;
    while (!global_sig_receive) {
    poll_fd.fd = ctl_fd;
    poll_fd.events = POLLIN;
    ret = poll(&poll_fd, 1, EVENT_WAIT_MSEC);
    if (global_signal_val) {
    global_sig_receive = true;
    pr_info("Receive interrupt %d\n", global_signal_val);
// Wakes rw-threads when they are sleeping
    if (!global_run_operation)
    pthread_cond_broadcast(&cond_wakeup);
    ret = -1;
    break;
    }
    if (ret < 0) {
    pr_err("Polling error\n");
    goto error;
    }
    if (ret)
    break;
    }
    return ret;
    error:
    exit(EXIT_FAILURE);
    }
//
// control read/write threads by handling global_run_operation
//
    void *rw_ctl_loop(int ctl_fd)
    {
    ssize_t rlen;
    char buf[HOST_MSG_SIZE];
    int ret;
// Setup signal handlers
    signal(SIGTERM, signal_handler);
    signal(SIGINT, signal_handler);
    signal(SIGQUIT, signal_handler);
    while (!global_sig_receive) {
    ret = wait_order(ctl_fd);
    if (ret < 0)
    break;
    rlen = read(ctl_fd, buf, sizeof(buf));
    if (rlen < 0) {
    pr_err("read data error in ctl thread\n");
    goto error;
    }
    if (rlen == 2 && buf[0] == '1') {
//
// If host writes '1' to a control path,
// this controller wakes all read/write threads.
//
    global_run_operation = true;
    pthread_cond_broadcast(&cond_wakeup);
    pr_debug("Wake up all read/write threads\n");
    } else if (rlen == 2 && buf[0] == '0') {
//
// If host writes '0' to a control path, read/write
// threads will wait for notification from Host.
//
    global_run_operation = false;
    pr_debug("Stop all read/write threads\n");
    } else
    pr_info("Invalid host notification: %s\n", buf);
    }
    return core::ptr::null_mut();
    error:
    exit(EXIT_FAILURE);
    }
