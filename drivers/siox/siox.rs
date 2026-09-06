//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/siox/siox.h
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
// Copyright (C) 2015-2017 Pengutronix, Uwe Kleine-König <kernel@pengutronix.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siox_master {
// these fields should be initialized by the driver
    pub busno: c_int,
    pub getbuf[]): size_t getbuf_len, u8,
// might be initialized by the driver, if 0 it is set to HZ / 40
    pub /: *mut *mut unsigned long poll_interval; / in jiffies,
// framework private stuff
    pub lock: mutex,
    pub active: bool,
    pub owner: *mut module,
    pub dev: device,
    pub num_devices: c_uint,
    pub devices: list_head,
    pub getbuf_len: size_t setbuf_len,,
    pub buf_len: usize,
    pub buf: *mut u8,
    pub status: u8,
    pub last_poll: c_ulong,
    pub poll_thread: *mut task_struct,
}

extern "C" {
    pub fn dev_get_drvdata(_arg: &smaster->dev) -> return;
}
extern "C" {
    pub fn siox_master_register(smaster: *mut siox_master) -> c_int;
}
extern "C" {
    pub fn siox_master_unregister(smaster: *mut siox_master);
}
extern "C" {
    pub fn devm_siox_master_register(dev: *mut device, smaster: *mut siox_master) -> c_int;
}
