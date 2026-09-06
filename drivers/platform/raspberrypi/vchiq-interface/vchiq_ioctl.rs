//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/raspberrypi/vchiq-interface/vchiq_ioctl.h
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

pub const VCHIQ_IOC_MAGIC: c_uint = 0xc4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_service_params {
    pub fourcc: c_int,
    pub bulk_userdata): *mut c_void,
    pub userdata: *mut void __user,
    pub /: *mut *mut short version; / Increment for non-trivial changes,
    pub /: *mut *mut short version_min; / Update for incompatible changes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_create_service {
    pub params: vchiq_service_params,
    pub is_open: c_int,
    pub is_vchi: c_int,
    pub /: *mut *mut unsigned int handle; / OUT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_queue_message {
    pub handle: c_uint,
    pub count: c_uint,
    pub elements: *const vchiq_element __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_queue_bulk_transfer {
    pub handle: c_uint,
    pub data: *mut void __user,
    pub size: c_uint,
    pub userdata: *mut void __user,
    pub mode: vchiq_bulk_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_completion_data {
    pub reason: vchiq_reason,
    pub header: *mut vchiq_header __user,
    pub service_userdata: *mut void __user,
    pub cb_userdata: *mut void __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_await_completion {
    pub count: c_uint,
    pub buf: *mut vchiq_completion_data __user,
    pub msgbufsize: c_uint,
    pub /: *mut *mut unsigned int msgbufcount; / IN/OUT,
    pub msgbufs: *mut *mut void  __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_dequeue_message {
    pub handle: c_uint,
    pub blocking: c_int,
    pub bufsize: c_uint,
    pub buf: *mut void __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_get_config {
    pub config_size: c_uint,
    pub pconfig: *mut vchiq_config __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_set_service_option {
    pub handle: c_uint,
    pub option: vchiq_service_option,
    pub value: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_dump_mem {
    pub virt_addr: *mut void __user,
    pub num_bytes: usize,
}

pub const VCHIQ_IOC_MAX: c_int = 17;
