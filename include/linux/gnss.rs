//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/gnss.h
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
// GNSS receiver support
//
// Copyright (C) 2018 Johan Hovold <johan@kernel.org>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gnss_type {
    GNSS_TYPE_NMEA = 0,
    GNSS_TYPE_SIRF,
    GNSS_TYPE_UBX,
    GNSS_TYPE_MTK,

    GNSS_TYPE_COUNT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnss_operations {
    pub gdev): *mut *mut int (open)(struct gnss_device,
    pub gdev): *mut *mut void (close)(struct gnss_device,
    pub count): usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnss_device {
    pub dev: device,
    pub cdev: cdev,
    pub id: c_int,
    pub type: gnss_type,
    pub flags: c_ulong,
    pub rwsem: rw_semaphore,
    pub ops: *const gnss_operations,
    pub count: c_uint,
    pub disconnected:1: c_uint,
    pub read_mutex: mutex,
    pub read_fifo: kfifo,
    pub read_queue: wait_queue_head_t,
    pub write_mutex: mutex,
    pub write_buf: *mut c_char,
}

extern "C" {
    pub fn gnss_put_device(gdev: *mut gnss_device);
}
extern "C" {
    pub fn gnss_register_device(gdev: *mut gnss_device) -> c_int;
}
extern "C" {
    pub fn gnss_deregister_device(gdev: *mut gnss_device);
}
extern "C" {
    pub fn dev_get_drvdata(_arg: &gdev->dev) -> return;
}
