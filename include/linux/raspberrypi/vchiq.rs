//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/raspberrypi/vchiq.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright (c) 2010-2012 Broadcom. All rights reserved.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vchiq_reason {
    VCHIQ_SERVICE_OPENED,         /* service, -, -             */
    VCHIQ_SERVICE_CLOSED,         /* service, -, -             */
    VCHIQ_MESSAGE_AVAILABLE,      /* service, header, -        */
    VCHIQ_BULK_TRANSMIT_DONE,     /* service, -, bulk_userdata */
    VCHIQ_BULK_RECEIVE_DONE,      /* service, -, bulk_userdata */
    VCHIQ_BULK_TRANSMIT_ABORTED,  /* service, -, bulk_userdata */
    VCHIQ_BULK_RECEIVE_ABORTED    /* service, -, bulk_userdata */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vchiq_bulk_mode {
    VCHIQ_BULK_MODE_CALLBACK,
    VCHIQ_BULK_MODE_BLOCKING,
    VCHIQ_BULK_MODE_NOCALLBACK,
    VCHIQ_BULK_MODE_WAITING		/* Reserved for internal use */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vchiq_service_option {
    VCHIQ_SERVICE_OPTION_AUTOCLOSE,
    VCHIQ_SERVICE_OPTION_SLOT_QUOTA,
    VCHIQ_SERVICE_OPTION_MESSAGE_QUOTA,
    VCHIQ_SERVICE_OPTION_SYNCHRONOUS,
    VCHIQ_SERVICE_OPTION_TRACE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_header {
// The message identifier - opaque to applications.
    pub msgid: c_int,
// Size of message data.
    pub size: c_uint,
    pub /: *mut *mut char data[]; / message,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_element {
    pub data: *const void __user,
    pub size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_service_base {
    pub fourcc: c_int,
    pub cb_userdata): *mut *mut void cb_data, void __user,
    pub userdata: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_completion_data_kernel {
    pub reason: vchiq_reason,
    pub header: *mut vchiq_header,
    pub service_userdata: *mut c_void,
    pub cb_data: *mut c_void,
    pub cb_userdata: *mut void __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_service_params_kernel {
    pub fourcc: c_int,
    pub cb_userdata): *mut *mut void cb_data, void __user,
    pub userdata: *mut c_void,
    pub /: *mut *mut short version; / Increment for non-trivial changes,
    pub /: *mut *mut short version_min; / Update for incompatible changes,
}

extern "C" {
    pub fn vchiq_shutdown(instance: *mut vchiq_instance) -> c_int;
}
extern "C" {
    pub fn vchiq_connect(instance: *mut vchiq_instance) -> c_int;
}
extern "C" {
    pub fn vchiq_use_service(instance: *mut vchiq_instance, service: c_uint) -> c_int;
}
