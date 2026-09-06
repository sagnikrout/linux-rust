//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/hidraw.h
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
// Copyright (c) 2007 Jiri Kosina
//
// This program is free software; you can redistribute it and/or modify it
// under the terms and conditions of the GNU General Public License,
// version 2, as published by the Free Software Foundation.
//
// You should have received a copy of the GNU General Public License along with
// this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin St - Fifth Floor, Boston, MA 02110-1301 USA.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hidraw_report_descriptor {
    pub size: __u32,
    pub value: [__u8; HID_MAX_DESCRIPTOR_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hidraw_devinfo {
    pub bustype: __u32,
    pub vendor: __s16,
    pub product: __s16,
}

// ioctl interface

// The first byte of SFEATURE and GFEATURE is the report number

// The first byte of SINPUT and GINPUT is the report number

// The first byte of SOUTPUT and GOUTPUT is the report number

pub const HIDRAW_FIRST_MINOR: c_int = 0;
pub const HIDRAW_MAX_DEVICES: c_int = 64;
// number of reports to buffer
pub const HIDRAW_BUFFER_SIZE: c_int = 64;
// kernel-only API declarations
