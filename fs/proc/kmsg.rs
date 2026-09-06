//! Automatically rewritten from C to Rust
//! Source: fs/proc/kmsg.c
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
// linux/fs/proc/kmsg.c
//
// Copyright (C) 1992  by Linus Torvalds
//

#[no_mangle]
unsafe extern "C" fn kmsg_open(inode: *mut *mut inode, file: *mut *mut file) -> c_int {
    static int kmsg_open(struct inode * inode, struct file * file)
    {
    return do_syslog(SYSLOG_ACTION_OPEN, core::ptr::null_mut(), 0, SYSLOG_FROM_PROC);
    }
#[no_mangle]
unsafe extern "C" fn kmsg_release(inode: *mut *mut inode, file: *mut *mut file) -> c_int {
    static int kmsg_release(struct inode * inode, struct file * file)
    {
    (void) do_syslog(SYSLOG_ACTION_CLOSE, core::ptr::null_mut(), 0, SYSLOG_FROM_PROC);
    return 0;
    }
    static ssize_t kmsg_read(struct file *file, char __user *buf,
    size_t count, loff_t *ppos)
    {
    if ((file.f_flags & O_NONBLOCK) &&
    !do_syslog(SYSLOG_ACTION_SIZE_UNREAD, core::ptr::null_mut(), 0, SYSLOG_FROM_PROC))
    return -EAGAIN;
    return do_syslog(SYSLOG_ACTION_READ, buf, count, SYSLOG_FROM_PROC);
    }
#[no_mangle]
unsafe extern "C" fn kmsg_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    static __poll_t kmsg_poll(struct file *file, poll_table *wait)
    {
    poll_wait(file, &log_wait, wait);
    if (do_syslog(SYSLOG_ACTION_SIZE_UNREAD, core::ptr::null_mut(), 0, SYSLOG_FROM_PROC))
    return EPOLLIN | EPOLLRDNORM;
    return 0;
    }
    static const struct proc_ops kmsg_proc_ops = {
    .proc_flags	= PROC_ENTRY_PERMANENT,
    .proc_read	= kmsg_read,
    .proc_poll	= kmsg_poll,
    .proc_open	= kmsg_open,
    .proc_release	= kmsg_release,
    .proc_lseek	= generic_file_llseek,
    };
#[no_mangle]
unsafe extern "C" fn proc_kmsg_init() -> int __init {
    static int __init proc_kmsg_init(void)
    {
    proc_create("kmsg", S_IRUSR, core::ptr::null_mut(), &kmsg_proc_ops);
    return 0;
    }
    fs_initcall(proc_kmsg_init);
