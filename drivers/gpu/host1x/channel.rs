//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/host1x/channel.h
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
// Tegra host1x Channel
//
// Copyright (c) 2010-2013, NVIDIA Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_channel_list {
    pub channels: *mut host1x_channel,
    pub lock: mutex,
    pub allocated_channels: *mut c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_channel {
    pub refcount: kref,
    pub id: c_uint,
    pub submitlock: mutex,
    pub regs: *mut void __iomem,
    pub client: *mut host1x_client,
    pub dev: *mut device,
    pub cdma: host1x_cdma,
}

// channel list operations
extern "C" {
    pub fn host1x_channel_list_free(chlist: *mut host1x_channel_list);
}
extern "C" {
    pub fn host1x_channel_stop_all(host: *mut host1x);
}
