//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hv/hv_utils_transport.h
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
// Kernel/userspace transport abstraction for Hyper-V util driver.
//
// Copyright (C) 2015, Vitaly Kuznetsov <vkuznets@redhat.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hvutil_transport_mode {
    HVUTIL_TRANSPORT_INIT = 0,
    HVUTIL_TRANSPORT_NETLINK,
    HVUTIL_TRANSPORT_CHARDEV,
    HVUTIL_TRANSPORT_DESTROY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hvutil_transport {
    pub /: *mut *mut int mode; / hvutil_transport_mode,
    pub /: *mut *mut file_operations fops; / file operations,
    pub /: *mut *mut miscdevice mdev; / misc device,
    pub /: *mut *mut *mut *mut cb_id cn_id; / CN__IDX/CN__VAL,
    pub /: *mut *mut list_head list; / hvt_list,
    pub /: *mut *mut *mut *mut int (on_msg)(void , int); / callback on new user message,
    pub /: *mut *mut *mut void (on_reset)(void); / callback when userspace drops,
    pub /: *mut *mut *mut void (on_read)(void); / callback on message read,
    pub /: *mut *mut *mut u8 outmsg; / message to the userspace,
    pub /: *mut *mut int outmsg_len; / its length,
    pub /: *mut *mut wait_queue_head_t outmsg_q; / poll/read wait queue,
    pub /: *mut *mut mutex lock; / protects members,
    pub /: *mut *mut completion release; / synchronize with fd release,
}

extern "C" {
    pub fn hvutil_transport_destroy(hvt: *mut hvutil_transport);
}
