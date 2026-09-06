//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/function/u_ncm.h
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
// u_ncm.h
//
// Utility definitions for the ncm function
//
// Copyright (c) 2013 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Andrzej Pietrasiewicz <andrzejtp2010@gmail.com>
//

//
// struct f_ncm_opts - NCM function options
// @func_inst: USB function instance.
// @net: The net_device associated with the NCM function.
// @bind_count: Tracks the number of configurations the NCM function is
// bound to, preventing double-registration of the @net device.
// @ncm_interf_group: ConfigFS group for NCM interface.
// @ncm_os_desc: USB OS descriptor for NCM.
// @ncm_ext_compat_id: Extended compatibility ID.
// @lock: Protects the data from concurrent access by configfs read/write
// and create symlink/remove symlink operations.
// @refcnt: Reference counter for the function instance.
// @max_segment_size: Maximum segment size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_ncm_opts {
    pub func_inst: usb_function_instance,
    pub net: *mut net_device,
    pub bind_count: c_int,
    pub ncm_interf_group: *mut config_group,
    pub ncm_os_desc: usb_os_desc,
    pub ncm_ext_compat_id: [c_char; 16],
    pub lock: mutex,
    pub refcnt: c_int,
    pub max_segment_size: u16,
}
