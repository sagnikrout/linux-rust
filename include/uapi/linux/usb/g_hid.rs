//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/usb/g_hid.h
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

// Maximum HID report length for High-Speed USB (i.e. USB 2.0)
pub const MAX_REPORT_LENGTH: c_int = 64;
//
// struct usb_hidg_report - response to GET_REPORT
// @report_id: report ID that this is a response for
// @userspace_req:
// !0 this report is used for any pending GET_REPORT request
// but wait on userspace to issue a new report on future requests
// 0  this report is to be used for any future GET_REPORT requests
// @length: length of the report response
// @data: report response
// @padding: padding for 32/64 bit compatibility
//
// Structure used by GADGET_HID_WRITE_GET_REPORT ioctl on /dev/hidg*.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_hidg_report {
    pub report_id: __u8,
    pub userspace_req: __u8,
    pub length: __u16,
    pub data: [__u8; MAX_REPORT_LENGTH],
    pub padding: [__u8; 4],
}

// The 'g' code is used by gadgetfs and hid gadget ioctl requests.
// Don't add any colliding codes to either driver, and keep
// them in unique ranges.
//

