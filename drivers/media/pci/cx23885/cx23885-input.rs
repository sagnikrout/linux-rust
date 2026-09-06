//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx23885/cx23885-input.h
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
// Driver for the Conexant CX23885/7/8 PCIe bridge
//
// Infrared remote control input device
//
// Copyright (C) 2009  Andy Walls <awalls@md.metrocast.net>
//
extern "C" {
    pub fn cx23885_input_rx_work_handler(dev: *mut cx23885_dev, events: u32);
}
extern "C" {
    pub fn cx23885_input_init(dev: *mut cx23885_dev) -> c_int;
}
extern "C" {
    pub fn cx23885_input_fini(dev: *mut cx23885_dev);
}
