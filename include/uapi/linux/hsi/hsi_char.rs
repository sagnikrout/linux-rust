//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/hsi/hsi_char.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Part of the HSI character device driver.
//
// Copyright (C) 2010 Nokia Corporation. All rights reserved.
//
// Contact: Andras Domokos <andras.domokos at nokia.com>
//

pub const HSC_PM_DISABLE: c_int = 0;
pub const HSC_PM_ENABLE: c_int = 1;
pub const HSC_MODE_STREAM: c_int = 1;
pub const HSC_MODE_FRAME: c_int = 2;
pub const HSC_FLOW_SYNC: c_int = 0;
pub const HSC_ARB_RR: c_int = 0;
pub const HSC_ARB_PRIO: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsc_rx_config {
    pub mode: __u32,
    pub flow: __u32,
    pub channels: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsc_tx_config {
    pub mode: __u32,
    pub channels: __u32,
    pub speed: __u32,
    pub arb_mode: __u32,
}
