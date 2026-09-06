//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hid-debug.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 2007-2009	Jiri Kosina
//

pub const HID_DEBUG_BUFSIZE: c_int = 512;
pub const HID_DEBUG_FIFOSIZE: c_int = 512;
extern "C" {
    pub fn hid_dump_input(: *mut hid_device, : *mut hid_usage, _arg: __s32);
}
extern "C" {
    pub fn hid_dump_report(: *mut hid_device, _arg: c_int, : *mut u8, _arg: c_int);
}
extern "C" {
    pub fn hid_dump_device(: *mut hid_device, : *mut seq_file);
}
extern "C" {
    pub fn hid_dump_field(: *mut hid_field, _arg: c_int, : *mut seq_file);
}
extern "C" {
    pub fn hid_debug_register(: *mut hid_device, : *const c_char);
}
extern "C" {
    pub fn hid_debug_unregister(: *mut hid_device);
}
extern "C" {
    pub fn hid_debug_init();
}
extern "C" {
    pub fn hid_debug_exit();
}
extern "C" {
    pub fn hid_debug_event(: *mut hid_device, : *mut c_char);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_debug_list {
    pub char): DECLARE_KFIFO_PTR(hid_debug_fifo,,
    pub fasync: *mut fasync_struct,
    pub hdev: *mut hid_device,
    pub node: list_head,
    pub read_mutex: mutex,
}

