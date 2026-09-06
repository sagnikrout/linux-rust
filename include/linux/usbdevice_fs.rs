//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usbdevice_fs.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// usbdevice_fs.h  --  USB device file system.
//
// Copyright (C) 2000
// Thomas Sailer (sailer@ife.ee.ethz.ch)
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
// Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
//
// History:
// 0.1  04.01.2000  Created
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbdevfs_ctrltransfer32 {
    pub bRequestType: u8,
    pub bRequest: u8,
    pub wValue: u16,
    pub wIndex: u16,
    pub wLength: u16,
    pub /: *mut *mut u32 timeout; / in milliseconds,
    pub data: compat_caddr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbdevfs_bulktransfer32 {
    pub ep: compat_uint_t,
    pub len: compat_uint_t,
    pub /: *mut *mut compat_uint_t timeout; / in milliseconds,
    pub data: compat_caddr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbdevfs_disconnectsignal32 {
    pub signr: compat_int_t,
    pub context: compat_caddr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbdevfs_urb32 {
    pub type: c_uchar,
    pub endpoint: c_uchar,
    pub status: compat_int_t,
    pub flags: compat_uint_t,
    pub buffer: compat_caddr_t,
    pub buffer_length: compat_int_t,
    pub actual_length: compat_int_t,
    pub start_frame: compat_int_t,
    pub number_of_packets: compat_int_t,
    pub error_count: compat_int_t,
    pub signr: compat_uint_t,
    pub /: *mut *mut compat_caddr_t usercontext; / unused,
    pub iso_frame_desc: [usbdevfs_iso_packet_desc; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbdevfs_ioctl32 {
    pub ifno: i32,
    pub ioctl_code: i32,
    pub data: compat_caddr_t,
}

