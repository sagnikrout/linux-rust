//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/usb/g_uvc.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// g_uvc.h  --  USB Video Class Gadget driver API
//
// Copyright (C) 2009-2010 Laurent Pinchart <laurent.pinchart@ideasonboard.com>
//

pub const UVC_STRING_CONTROL_IDX: c_int = 0;
pub const UVC_STRING_STREAMING_IDX: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_request_data {
    pub length: __s32,
    pub data: [__u8; 60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_event {
    pub speed: usb_device_speed,
    pub req: usb_ctrlrequest,
    pub data: uvc_request_data,
}

