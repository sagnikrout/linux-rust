//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/uvcvideo.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

//
// Dynamic controls
//
// Data types for UVC control data
pub const UVC_CTRL_DATA_TYPE_RAW: c_int = 0;
pub const UVC_CTRL_DATA_TYPE_SIGNED: c_int = 1;
pub const UVC_CTRL_DATA_TYPE_UNSIGNED: c_int = 2;
pub const UVC_CTRL_DATA_TYPE_BOOLEAN: c_int = 3;
pub const UVC_CTRL_DATA_TYPE_ENUM: c_int = 4;
pub const UVC_CTRL_DATA_TYPE_BITMASK: c_int = 5;
pub const UVC_CTRL_DATA_TYPE_RECT: c_int = 6;
// Control flags

// Control should be saved at suspend and restored at resume.

// Control can be updated by the camera.

// Control supports asynchronous reporting

pub const UVC_MENU_NAME_LEN: c_int = 32;
// V4L2 driver-specific controls

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_menu_info {
    pub value: __u32,
    pub name: [__u8; UVC_MENU_NAME_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_xu_control_mapping {
    pub id: __u32,
    pub name: [__u8; 32],
    pub entity: [__u8; 16],
    pub selector: __u8,
    pub size: __u8,
    pub offset: __u8,
    pub v4l2_type: __u32,
    pub data_type: __u32,
    pub menu_info: *mut uvc_menu_info __user,
    pub menu_count: __u32,
    pub reserved: [__u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_xu_control_query {
    pub unit: __u8,
    pub selector: __u8,
    pub /: *mut *mut __u8 query; / Video Class-Specific Request Code,,
// defined in linux/usb/video.h A.8.
    pub size: __u16,
    pub data: *mut __u8 __user,
}

//
// Metadata node
//
// struct uvc_meta_buf - metadata buffer building block
// @ns: system timestamp of the payload in nanoseconds
// @sof: USB Frame Number
// @length: length of the payload header
// @flags: payload header flags
// @buf: optional device-specific header data
//
// UVC metadata nodes fill buffers with possibly multiple instances of this
// struct. The first two fields are added by the driver, they can be used for
// clock synchronisation. The rest is an exact copy of a UVC payload header.
// Only complete objects with complete buffers are included. Therefore it's
// always sizeof(meta->ns) + sizeof(meta->sof) + meta->length bytes large.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_meta_buf {
    pub ns: __u64,
    pub sof: __u16,
    pub length: __u8,
    pub flags: __u8,
    pub buf: [__u8; ],
    pub __packed: },
