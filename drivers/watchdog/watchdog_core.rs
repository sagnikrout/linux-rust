//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/watchdog/watchdog_core.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// watchdog_core.h
//
// (c) Copyright 2008-2011 Alan Cox <alan@lxorguk.ukuu.org.uk>,
// All Rights Reserved.
//
// (c) Copyright 2008-2011 Wim Van Sebroeck <wim@iguana.be>.
//
// (c) Copyright 2021 Hewlett Packard Enterprise Development LP.
//
// This source code is part of the generic code that can be used
// by all the watchdog timer drivers.
//
// Based on source code of the following authors:
// Matt Domsch <Matt_Domsch@dell.com>,
// Rob Radez <rob@osinvestor.com>,
// Rusty Lynch <rusty@linux.co.intel.com>
// Satyam Sharma <satyam@infradead.org>
// Randy Dunlap <randy.dunlap@oracle.com>
//
// Neither Alan Cox, CymruNet Ltd., Wim Van Sebroeck nor Iguana vzw.
// admit liability nor provide warranty for any of this software.
// This material is provided "AS-IS" and at no charge.
//

//
// struct watchdog_core_data - watchdog core internal data
// @dev:	The watchdog's internal device
// @cdev:	The watchdog's Character device.
// @wdd:	Pointer to watchdog device.
// @lock:	Lock for watchdog core.
// @status:	Watchdog core internal status bits.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct watchdog_core_data {
    pub dev: device,
    pub cdev: cdev,
    pub wdd: *mut watchdog_device,
    pub lock: mutex,
    pub last_keepalive: ktime_t,
    pub last_hw_keepalive: ktime_t,
    pub open_deadline: ktime_t,
    pub timer: hrtimer,
    pub work: kthread_work,

    pub pretimeout_timer: hrtimer,

    pub /: *mut *mut unsigned long status; / Internal status bits,

}

//
// Functions/procedures to be called by the core
//
extern "C" {
    pub fn watchdog_dev_register(: *mut watchdog_device) -> c_int;
}
extern "C" {
    pub fn watchdog_dev_unregister(: *mut watchdog_device);
}
extern "C" {
    pub fn watchdog_dev_init() -> int __init;
}
extern "C" {
    pub fn watchdog_dev_exit() -> void __exit;
}

extern "C" {
    pub fn watchdog_hrtimer_pretimeout_init(wdd: *mut watchdog_device);
}
extern "C" {
    pub fn watchdog_hrtimer_pretimeout_start(wdd: *mut watchdog_device);
}
extern "C" {
    pub fn watchdog_hrtimer_pretimeout_stop(wdd: *mut watchdog_device);
}

