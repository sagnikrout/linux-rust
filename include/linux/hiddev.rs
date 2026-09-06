//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hiddev.h
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
// Copyright (c) 1999-2000 Vojtech Pavlik
//
// Sponsored by SuSE
//
// Should you need to contact me, the author, you can do so either by
// e-mail - mail your message to <vojtech@suse.cz>, or by paper mail:
// Vojtech Pavlik, Ucitelska 1576, Prague 8, 182 00 Czech Republic
//

//
// In-kernel definitions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hiddev {
    pub minor: c_int,
    pub exist: c_int,
    pub open: c_int,
    pub existancelock: mutex,
    pub wait: wait_queue_head_t,
    pub hid: *mut hid_device,
    pub list: list_head,
    pub list_lock: spinlock_t,
    pub initialized: bool,
}

extern "C" {
    pub fn hiddev_connect(hid: *mut hid_device, force: c_uint) -> c_int;
}
extern "C" {
    pub fn hiddev_disconnect(: *mut hid_device);
}
extern "C" {
    pub fn hiddev_report_event(hid: *mut hid_device, report: *mut hid_report);
}

