//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hidraw.h
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
// Copyright (c) 2007 Jiri Kosina
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hidraw {
    pub minor: c_uint,
    pub exist: c_int,
    pub open: c_int,
    pub wait: wait_queue_head_t,
    pub hid: *mut hid_device,
    pub dev: *mut device,
    pub list_lock: spinlock_t,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hidraw_report {
    pub value: *mut __u8,
    pub len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hidraw_list {
    pub buffer: [hidraw_report; HIDRAW_BUFFER_SIZE],
    pub head: c_int,
    pub tail: c_int,
    pub fasync: *mut fasync_struct,
    pub hidraw: *mut hidraw,
    pub node: list_head,
    pub read_mutex: mutex,
    pub revoked: bool,
}

extern "C" {
    pub fn hidraw_init() -> c_int;
}
extern "C" {
    pub fn hidraw_exit();
}
extern "C" {
    pub fn hidraw_report_event(: *mut hid_device, : *mut u8, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn hidraw_connect(: *mut hid_device) -> c_int;
}
extern "C" {
    pub fn hidraw_disconnect(: *mut hid_device);
}

