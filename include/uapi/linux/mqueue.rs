//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/mqueue.h
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


// SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note
// Copyright (C) 2003 Krzysztof Benedyczak & Michal Wronski

pub const MQ_PRIO_MAX: c_int = 32768;
// per-uid limit of kernel memory used by mqueue, in bytes
pub const MQ_BYTES_MAX: c_int = 819200;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mq_attr {
    pub /: *mut *mut __kernel_long_t mq_flags; / message queue flags,
    pub /: *mut *mut __kernel_long_t mq_maxmsg; / maximum number of messages,
    pub /: *mut *mut __kernel_long_t mq_msgsize; / maximum message size,
    pub /: *mut *mut __kernel_long_t mq_curmsgs; / number of messages currently queued,
    pub /: *mut *mut __kernel_long_t __reserved[4]; / ignored for input, zeroed for output,
}

//
// SIGEV_THREAD implementation:
// SIGEV_THREAD must be implemented in user space. If SIGEV_THREAD is passed
// to mq_notify, then
// - sigev_signo must be the file descriptor of an AF_NETLINK socket. It's not
// necessary that the socket is bound.
// - sigev_value.sival_ptr must point to a cookie that is NOTIFY_COOKIE_LEN
// bytes long.
// If the notification is triggered, then the cookie is sent to the netlink
// socket. The last byte of the cookie is replaced with the NOTIFY_?? codes:
// NOTIFY_WOKENUP if the notification got triggered, NOTIFY_REMOVED if it was
// removed, either due to a close() on the message queue fd or due to a
// mq_notify() that removed the notification.
//
pub const NOTIFY_NONE: c_int = 0;
pub const NOTIFY_WOKENUP: c_int = 1;
pub const NOTIFY_REMOVED: c_int = 2;
pub const NOTIFY_COOKIE_LEN: c_int = 32;
