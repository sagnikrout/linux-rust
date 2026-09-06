//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/st/cw1200/hwbus.h
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
// Common hwbus abstraction layer interface for cw1200 wireless driver
//
// Copyright (c) 2010, ST-Ericsson
// Author: Dmitry Tarnyagin <dmitry.tarnyagin@lockless.no>
//
extern "C" {
    pub fn cw1200_irq_handler(priv: *mut cw1200_common);
}
// This MUST be wrapped with hwbus_ops->lock/unlock!
extern "C" {
    pub fn __cw1200_irq_enable(priv: *mut cw1200_common, enable: c_int) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwbus_ops {
    pub count): *mut *mut void dst, int,
    pub count): *const *const void src, int,
    pub self): *mut *mut void (lock)(struct hwbus_priv,
    pub self): *mut *mut void (unlock)(struct hwbus_priv,
    pub size): *mut *mut *mut size_t (align_size)(struct hwbus_priv self, size_t,
    pub suspend): *mut *mut *mut int (power_mgmt)(struct hwbus_priv self, bool,
}
