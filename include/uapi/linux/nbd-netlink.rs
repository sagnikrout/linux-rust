//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nbd-netlink.h
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
// Copyright (C) 2017 Facebook.  All rights reserved.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public
// License v2 as published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public
// License along with this program; if not, write to the
// Free Software Foundation, Inc., 59 Temple Place - Suite 330,
// Boston, MA 021110-1307, USA.
//

pub const NBD_GENL_VERSION: c_uint = 0x1;

// Configuration policy attributes, used for CONNECT

//
// This is the format for multiple devices with NBD_ATTR_DEVICE_LIST
//
// [NBD_ATTR_DEVICE_LIST]
// [NBD_DEVICE_ITEM]
// [NBD_DEVICE_INDEX]
// [NBD_DEVICE_CONNECTED]
//

//
// This is the format for multiple sockets with NBD_ATTR_SOCKETS
//
// [NBD_ATTR_SOCKETS]
// [NBD_SOCK_ITEM]
// [NBD_SOCK_FD]
// [NBD_SOCK_ITEM]
// [NBD_SOCK_FD]
//

