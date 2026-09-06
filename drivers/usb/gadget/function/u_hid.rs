//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/function/u_hid.h
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
// u_hid.h
//
// Utility definitions for the hid function
//
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Andrzej Pietrasiewicz <andrzejtp2010@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_hid_opts {
    pub func_inst: usb_function_instance,
    pub minor: c_int,
    pub subclass: c_uchar,
    pub protocol: c_uchar,
    pub no_out_endpoint: c_uchar,
    pub report_length: c_ushort,
    pub report_desc_length: c_ushort,
    pub report_desc: *mut c_uchar,
    pub report_desc_alloc: bool,
    pub interval: c_uchar,
    pub interval_user_set: bool,
//
// Protect the data form concurrent access by read/write
// and create symlink/remove symlink.
//
    pub lock: mutex,
    pub refcnt: c_int,
}

extern "C" {
    pub fn ghid_setup(g: *mut usb_gadget, count: c_int) -> c_int;
}
extern "C" {
    pub fn ghid_cleanup();
}
