//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/uvc.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// v4l2 uvc internal API header
//
// Some commonly needed functions for uvc drivers
//
// ------------------------------------------------------------------------
// GUIDs
//
// The GUID returned by lsusb can be converted to this format with the
// following python snippet:
//
// import uuid
// id = "{01234567-89ab-cdef-0123-456789abcdef}"
// le = uuid.UUID(id).bytes_le
// print("{" + ", ".join([f"0x{b:02x}" for b in le]) + "}")
//

// https://learn.microsoft.com/en-us/windows-hardware/drivers/stream/uvc-extensions-1-5#222-extension-unit-controls
pub const UVC_MSXU_CONTROL_FOCUS: c_uint = 0x01;
pub const UVC_MSXU_CONTROL_EXPOSURE: c_uint = 0x02;
pub const UVC_MSXU_CONTROL_EVCOMPENSATION: c_uint = 0x03;
pub const UVC_MSXU_CONTROL_WHITEBALANCE: c_uint = 0x04;
pub const UVC_MSXU_CONTROL_FACE_AUTHENTICATION: c_uint = 0x06;
pub const UVC_MSXU_CONTROL_CAMERA_EXTRINSICS: c_uint = 0x07;
pub const UVC_MSXU_CONTROL_CAMERA_INTRINSICS: c_uint = 0x08;
pub const UVC_MSXU_CONTROL_METADATA: c_uint = 0x09;
pub const UVC_MSXU_CONTROL_IR_TORCH: c_uint = 0x0a;
pub const UVC_MSXU_CONTROL_DIGITALWINDOW: c_uint = 0x0b;
pub const UVC_MSXU_CONTROL_DIGITALWINDOW_CONFIG: c_uint = 0x0c;
pub const UVC_MSXU_CONTROL_VIDEO_HDR: c_uint = 0x0d;
pub const UVC_MSXU_CONTROL_FRAMERATE_THROTTLE: c_uint = 0x0e;
pub const UVC_MSXU_CONTROL_FIELDOFVIEW2_CONFIG: c_uint = 0x0f;
pub const UVC_MSXU_CONTROL_FIELDOFVIEW2: c_uint = 0x10;
pub const UVC_CROSXU_CONTROL_IQ_PROFILE: c_uint = 0x04;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_format_desc {
    pub guid: [u8; 16],
    pub fcc: u32,
}
