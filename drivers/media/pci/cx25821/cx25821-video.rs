//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx25821/cx25821-video.h
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
// Driver for the Conexant CX25821 PCIe bridge
//
// Copyright (C) 2009 Conexant Systems Inc.
// Authors  <shu.lin@conexant.com>, <hiep.huynh@conexant.com>
// Based on Steven Toth <stoth@linuxtv.org> cx23885 driver
//

pub const VIDEO_DEBUG: c_int = 0;

pub const FORMAT_FLAGS_PACKED: c_uint = 0x01;
extern "C" {
    pub fn cx25821_video_irq(dev: *mut cx25821_dev, chan_num: c_int, status: u32) -> c_int;
}
extern "C" {
    pub fn cx25821_video_unregister(dev: *mut cx25821_dev, chan_num: c_int);
}
extern "C" {
    pub fn cx25821_video_register(dev: *mut cx25821_dev) -> c_int;
}
