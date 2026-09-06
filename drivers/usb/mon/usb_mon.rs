//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/mon/usb_mon.h
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
// The USB Monitor, inspired by Dave Harding's USBMon.
//
// Copyright (C) 2005 Pete Zaitcev (zaitcev@redhat.com)
//

// #include <linux/usb.h> */	/* We use struct pointers only in this header

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mon_bus {
    pub bus_link: list_head,
    pub lock: spinlock_t,
    pub u_bus: *mut usb_bus,
    pub text_inited: c_int,
    pub bin_inited: c_int,
    pub /: *mut *mut *mut dentry dent_s; / Debugging file,
    pub /: *mut *mut *mut dentry dent_t; / Text interface file,
    pub /: *mut *mut *mut dentry dent_u; / Second text interface file,
    pub /: *mut *mut *mut device classdev; / Device in usbmon class,
// Ref
    pub /: *mut *mut int nreaders; / Under mon_lock AND mbus->lock,
    pub /: *mut *mut list_head r_list; / Chain of readers (usually one),
    pub /: *mut *mut kref ref; / Under mon_lock,
// Stats
    pub cnt_events: c_uint,
    pub cnt_text_lost: c_uint,
}

//
// An instance of a process which opened a file (but can fork later)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mon_reader {
    pub r_link: list_head,
    pub m_bus: *mut mon_bus,
    pub /: *mut *mut *mut void r_data; / Use container_of instead?,
    pub urb): *mut *mut *mut void (rnf_submit)(void data, struct urb,
    pub error): *mut *mut *mut *mut void (rnf_error)(void data, struct urb urb, int,
    pub status): *mut *mut *mut *mut void (rnf_complete)(void data, struct urb urb, int,
}

extern "C" {
    pub fn mon_reader_add(mbus: *mut mon_bus, r: *mut mon_reader);
}
extern "C" {
    pub fn mon_reader_del(mbus: *mut mon_bus, r: *mut mon_reader);
}
extern "C" {
    pub fn mon_text_del(mbus: *mut mon_bus);
}
extern "C" {
    pub fn mon_bin_del(mbus: *mut mon_bus);
}
extern "C" {
    pub fn mon_text_init() -> int __init;
}
extern "C" {
    pub fn mon_text_exit();
}
extern "C" {
    pub fn mon_bin_init() -> int __init;
}
extern "C" {
    pub fn mon_bin_exit();
}
//
