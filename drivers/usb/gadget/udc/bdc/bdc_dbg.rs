//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/udc/bdc/bdc_dbg.h
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
// bdc_dbg.h - header for the BDC debug functions
//
// Copyright (C) 2014 Broadcom Corporation
//
// Author: Ashwini Pahuja
//

extern "C" {
    pub fn bdc_dbg_bd_list(bdc: *mut bdc, ep: *mut bdc_ep);
}
extern "C" {
    pub fn bdc_dbg_srr(bdc: *mut bdc, srr_num: u32);
}
extern "C" {
    pub fn bdc_dbg_regs(bdc: *mut bdc);
}
extern "C" {
    pub fn bdc_dump_epsts(bdc: *mut bdc);
}

