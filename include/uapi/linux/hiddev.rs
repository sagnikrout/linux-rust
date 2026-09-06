//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/hiddev.h
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
// Copyright (c) 1999-2000 Vojtech Pavlik
//
// Sponsored by SuSE
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
// e-mail - mail your message to <vojtech@suse.cz>, or by paper mail:
// Vojtech Pavlik, Ucitelska 1576, Prague 8, 182 00 Czech Republic
//

//
// The event structure itself
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hiddev_event {
    pub hid: unsigned,
    pub value: signed int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hiddev_devinfo {
    pub bustype: __u32,
    pub busnum: __u32,
    pub devnum: __u32,
    pub ifnum: __u32,
    pub vendor: __s16,
    pub product: __s16,
    pub version: __s16,
    pub num_applications: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hiddev_collection_info {
    pub index: __u32,
    pub type: __u32,
    pub usage: __u32,
    pub level: __u32,
}

pub const HID_STRING_SIZE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hiddev_string_descriptor {
    pub index: __s32,
    pub value: [c_char; HID_STRING_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hiddev_report_info {
    pub report_type: __u32,
    pub report_id: __u32,
    pub num_fields: __u32,
}

// To do a GUSAGE/SUSAGE, fill in at least usage_code,  report_type and
// report_id.  Set report_id to REPORT_ID_UNKNOWN if the rest of the fields
// are unknown.  Otherwise use a usage_ref struct filled in from a previous
// successful GUSAGE call to save time.  To actually send a value to the
// device, perform a SUSAGE first, followed by a SREPORT.  An INITREPORT or a
// GREPORT isn't necessary for a GUSAGE to return valid data.
//
pub const HID_REPORT_ID_UNKNOWN: c_uint = 0xffffffff;
pub const HID_REPORT_ID_FIRST: c_uint = 0x00000100;
pub const HID_REPORT_ID_NEXT: c_uint = 0x00000200;
pub const HID_REPORT_ID_MASK: c_uint = 0x000000ff;
pub const HID_REPORT_ID_MAX: c_uint = 0x000000ff;
pub const HID_REPORT_TYPE_INPUT: c_int = 1;
pub const HID_REPORT_TYPE_OUTPUT: c_int = 2;
pub const HID_REPORT_TYPE_FEATURE: c_int = 3;
pub const HID_REPORT_TYPE_MIN: c_int = 1;
pub const HID_REPORT_TYPE_MAX: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hiddev_field_info {
    pub report_type: __u32,
    pub report_id: __u32,
    pub field_index: __u32,
    pub maxusage: __u32,
    pub flags: __u32,
    pub /: *mut *mut __u32 physical; / physical usage for this field,
    pub /: *mut *mut __u32 logical; / logical usage for this field,
    pub /: *mut *mut __u32 application; / application usage for this field,
    pub logical_minimum: __s32,
    pub logical_maximum: __s32,
    pub physical_minimum: __s32,
    pub physical_maximum: __s32,
    pub unit_exponent: __u32,
    pub unit: __u32,
}

// Fill in report_type, report_id and field_index to get the information on a
// field.
//
pub const HID_FIELD_CONSTANT: c_uint = 0x001;
pub const HID_FIELD_VARIABLE: c_uint = 0x002;
pub const HID_FIELD_RELATIVE: c_uint = 0x004;
pub const HID_FIELD_WRAP: c_uint = 0x008;
pub const HID_FIELD_NONLINEAR: c_uint = 0x010;
pub const HID_FIELD_NO_PREFERRED: c_uint = 0x020;
pub const HID_FIELD_NULL_STATE: c_uint = 0x040;
pub const HID_FIELD_VOLATILE: c_uint = 0x080;
pub const HID_FIELD_BUFFERED_BYTE: c_uint = 0x100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hiddev_usage_ref {
    pub report_type: __u32,
    pub report_id: __u32,
    pub field_index: __u32,
    pub usage_index: __u32,
    pub usage_code: __u32,
    pub value: __s32,
}

// hiddev_usage_ref_multi is used for sending multiple bytes to a control.
// It really manifests itself as setting the value of consecutive usages
pub const HID_MAX_MULTI_USAGES: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hiddev_usage_ref_multi {
    pub uref: hiddev_usage_ref,
    pub num_values: __u32,
    pub values: [__s32; HID_MAX_MULTI_USAGES],
}

// FIELD_INDEX_NONE is returned in read() data from the kernel when flags
// is set to (HIDDEV_FLAG_UREF | HIDDEV_FLAG_REPORT) and a new report has
// been sent by the device
//
pub const HID_FIELD_INDEX_NONE: c_uint = 0xffffffff;
//
// Protocol version.
//
pub const HID_VERSION: c_uint = 0x010004;
//
// IOCTLs (0x00 - 0x7f)
//

// For writing/reading to multiple/consecutive usages

//
// Flags to be used in HIDIOCSFLAG
//
pub const HIDDEV_FLAG_UREF: c_uint = 0x1;
pub const HIDDEV_FLAG_REPORT: c_uint = 0x2;
pub const HIDDEV_FLAGS: c_uint = 0x3;
// To traverse the input report descriptor info for a HID device, perform the
// following:
//
// rinfo.report_type = HID_REPORT_TYPE_INPUT;
// rinfo.report_id = HID_REPORT_ID_FIRST;
// ret = ioctl(fd, HIDIOCGREPORTINFO, &rinfo);
//
// while (ret >= 0) {
// for (i = 0; i < rinfo.num_fields; i++) {
// finfo.report_type = rinfo.report_type;
// finfo.report_id = rinfo.report_id;
// finfo.field_index = i;
// ioctl(fd, HIDIOCGFIELDINFO, &finfo);
// for (j = 0; j < finfo.maxusage; j++) {
// uref.report_type = rinfo.report_type;
// uref.report_id = rinfo.report_id;
// uref.field_index = i;
// uref.usage_index = j;
// ioctl(fd, HIDIOCGUCODE, &uref);
// ioctl(fd, HIDIOCGUSAGE, &uref);
// }
// rinfo.report_id |= HID_REPORT_ID_NEXT;
// ret = ioctl(fd, HIDIOCGREPORTINFO, &rinfo);
// }
//
