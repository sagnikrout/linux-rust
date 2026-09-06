//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/isp116x.h
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
// Board initialization code should put one of these into dev->platform_data
// and place the isp116x onto platform_bus.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp116x_platform_data {
// Enable internal resistors on downstream ports
    pub sel15Kres:1: unsigned,
// On-chip overcurrent detection
    pub oc_enable:1: unsigned,
// INT output polarity
    pub int_act_high:1: unsigned,
// INT edge or level triggered
    pub int_edge_triggered:1: unsigned,
// Enable wakeup by devices on usb bus (e.g. wakeup
    pub remote_wakeup_enable:1: unsigned,
// Inter-io delay (ns). The chip is picky about access timings; it
//
    pub delay): *mut *mut *mut void (delay) (struct device dev, int,
}
