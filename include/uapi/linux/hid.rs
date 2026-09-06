//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/hid.h
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
// Copyright (c) 1999 Andreas Gal
// Copyright (c) 2000-2001 Vojtech Pavlik
// Copyright (c) 2006-2007 Jiri Kosina
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA
//
// Should you need to contact me, the author, you can do so either by
// e-mail - mail your message to <vojtech@ucw.cz>, or by paper mail:
// Vojtech Pavlik, Simunkova 1594, Prague 8, 182 00 Czech Republic
//
// USB HID (Human Interface Device) interface class code
//
pub const USB_INTERFACE_CLASS_HID: c_int = 3;
//
// USB HID interface subclass and protocol codes
//
pub const USB_INTERFACE_SUBCLASS_BOOT: c_int = 1;
pub const USB_INTERFACE_PROTOCOL_KEYBOARD: c_int = 1;
pub const USB_INTERFACE_PROTOCOL_MOUSE: c_int = 2;
//
// HID report types --- Ouch! HID spec says 1 2 3!
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hid_report_type {
    HID_INPUT_REPORT		= 0,
    HID_OUTPUT_REPORT		= 1,
    HID_FEATURE_REPORT		= 2,

    HID_REPORT_TYPES,
}

//
// HID class requests
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hid_class_request {
    HID_REQ_GET_REPORT		= 0x01,
    HID_REQ_GET_IDLE		= 0x02,
    HID_REQ_GET_PROTOCOL		= 0x03,
    HID_REQ_SET_REPORT		= 0x09,
    HID_REQ_SET_IDLE		= 0x0A,
    HID_REQ_SET_PROTOCOL		= 0x0B,
}

//
// HID class descriptor types
//

pub const HID_MAX_DESCRIPTOR_SIZE: c_int = 4096;
