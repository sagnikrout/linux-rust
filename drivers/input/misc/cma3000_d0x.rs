//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/misc/cma3000_d0x.h
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
// VTI CMA3000_D0x Accelerometer driver
//
// Copyright (C) 2010 Texas Instruments
// Author: Hemanth V <hemanthv@ti.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cma3000_bus_ops {
    pub bustype: u16,
    pub ctrl_mod: u8,
    pub ): *mut *mut *mut int (read)(struct device , u8, char,
    pub ): *mut *mut *mut int (write)(struct device , u8, u8, char,
}

extern "C" {
    pub fn cma3000_exit(: *mut cma3000_accl_data);
}
extern "C" {
    pub fn cma3000_suspend(: *mut cma3000_accl_data);
}
extern "C" {
    pub fn cma3000_resume(: *mut cma3000_accl_data);
}
