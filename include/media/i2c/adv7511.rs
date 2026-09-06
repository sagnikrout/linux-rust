//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/i2c/adv7511.h
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
// Analog Devices ADV7511 HDMI Transmitter Device Driver
//
// Copyright 2013 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//
// notify events
pub const ADV7511_MONITOR_DETECT: c_int = 0;
pub const ADV7511_EDID_DETECT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv7511_monitor_detect {
    pub present: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv7511_edid_detect {
    pub present: c_int,
    pub segment: c_int,
    pub phys_addr: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv7511_platform_data {
    pub i2c_edid: u8,
    pub i2c_cec: u8,
    pub i2c_pktmem: u8,
    pub cec_clk: u32,
}
