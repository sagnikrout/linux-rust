//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/hid/hid_common.h
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
// Copyright (c) 2022-2024 Red Hat

pub const SHOW_UHID_DEBUG: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhid_device {
    pub /: *mut *mut int dev_id; / uniq (random) number to identify the device,
    pub uhid_fd: c_int,
    pub /: *mut *mut int hid_id; / HID device id in the system,
    pub bus: __u16,
    pub vid: __u32,
    pub pid: __u32,
    pub /: *mut *mut pthread_t tid; / thread for reading uhid events,
}

// no need to protect uhid_stopped, only one thread accesses it
extern "C" {
    pub fn uhid_write(_arg: _metadata, _arg: fd, _arg: &ev) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhid_thread_args {
    pub fd: c_int,
    pub _metadata: *mut __test_metadata,
}

extern "C" {
    pub fn uhid_write(_arg: _metadata, _arg: hid->uhid_fd, _arg: &ev) -> return;
}
// we found the correct VID/PID, now check for phys
// it would be nice to be able to use nftw, but the no_alu32 target doesn't support it
// retry 5 times in case the system is loaded
// open hidraw node to check the other side of the pipe
extern "C" {
    pub fn open(_arg: hidraw_path, O_NONBLOCK: O_RDWR |) -> return;
}
// initialize random number generator
// locate the uevent file of the created device
