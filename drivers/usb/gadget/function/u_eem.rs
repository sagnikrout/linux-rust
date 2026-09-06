//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/function/u_eem.h
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
// u_eem.h
//
// Utility definitions for the eem function
//
// Copyright (c) 2013 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Andrzej Pietrasiewicz <andrzejtp2010@gmail.com>
//

//
// struct f_eem_opts - EEM function options
// @func_inst: USB function instance.
// @net: The net_device associated with the EEM function.
// @bound: True if the net_device is shared and pre-registered during the
// legacy composite driver's bind phase (e.g., multi.c). If false,
// the EEM function will register the net_device during its own
// bind phase.
// @bind_count: Tracks the number of configurations the EEM function is
// bound to, preventing double-registration of the @net device.
// @lock: Protects the data from concurrent access by configfs read/write
// and create symlink/remove symlink operations.
// @refcnt: Reference counter for the function instance.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_eem_opts {
    pub func_inst: usb_function_instance,
    pub net: *mut net_device,
    pub bound: bool,
    pub bind_count: c_int,
    pub lock: mutex,
    pub refcnt: c_int,
}
