//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/function/u_rndis.h
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
// u_rndis.h
//
// Utility definitions for the subset function
//
// Copyright (c) 2013 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Andrzej Pietrasiewicz <andrzejtp2010@gmail.com>
//

//
// struct f_rndis_opts - RNDIS function options
// @func_inst: USB function instance.
// @vendor_id: Vendor ID.
// @manufacturer: Manufacturer string.
// @net: The net_device associated with the RNDIS function.
// @bind_count: Tracks the number of configurations the RNDIS function is
// bound to, preventing double-registration of the @net device.
// @borrowed_net: True if the net_device is shared and pre-registered during
// the legacy composite driver's bind phase (e.g., multi.c).
// If false, the RNDIS function will register the net_device
// during its own bind phase.
// @rndis_interf_group: ConfigFS group for RNDIS interface.
// @rndis_os_desc: USB OS descriptor for RNDIS.
// @rndis_ext_compat_id: Extended compatibility ID.
// @class: USB class.
// @subclass: USB subclass.
// @protocol: USB protocol.
// @lock: Protects the data from concurrent access by configfs read/write
// and create symlink/remove symlink operations.
// @refcnt: Reference counter for the function instance.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_rndis_opts {
    pub func_inst: usb_function_instance,
    pub vendor_id: u32,
    pub manufacturer: *const c_char,
    pub net: *mut net_device,
    pub bind_count: c_int,
    pub borrowed_net: bool,
    pub rndis_interf_group: *mut config_group,
    pub rndis_os_desc: usb_os_desc,
    pub rndis_ext_compat_id: [c_char; 16],
    pub class: u8,
    pub subclass: u8,
    pub protocol: u8,
    pub lock: mutex,
    pub refcnt: c_int,
}

extern "C" {
    pub fn rndis_borrow_net(f: *mut usb_function_instance, net: *mut net_device);
}
