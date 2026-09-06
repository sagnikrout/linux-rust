//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/connector.h
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
// connector.h
//
// 2004-2005 Copyright (c) Evgeniy Polyakov <zbr@ioremap.net>
// All rights reserved.
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
// Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
//

//
// Process Events connector unique ids -- used for message routing
//
pub const CN_IDX_PROC: c_uint = 0x1;
pub const CN_VAL_PROC: c_uint = 0x1;
pub const CN_IDX_CIFS: c_uint = 0x2;
pub const CN_VAL_CIFS: c_uint = 0x1;
pub const CN_W1_IDX: c_uint = 0x3	/* w1 communication */;
pub const CN_W1_VAL: c_uint = 0x1;
pub const CN_IDX_V86D: c_uint = 0x4;
pub const CN_VAL_V86D_UVESAFB: c_uint = 0x1;
pub const CN_IDX_BB: c_uint = 0x5	/* BlackBoard, from the TSP GPL sampling framework */;
pub const CN_DST_IDX: c_uint = 0x6;
pub const CN_DST_VAL: c_uint = 0x1;
pub const CN_IDX_DM: c_uint = 0x7	/* Device Mapper */;
pub const CN_VAL_DM_USERSPACE_LOG: c_uint = 0x1;
pub const CN_IDX_DRBD: c_uint = 0x8;
pub const CN_VAL_DRBD: c_uint = 0x1;
pub const CN_KVP_IDX: c_uint = 0x9	/* HyperV KVP */;
pub const CN_KVP_VAL: c_uint = 0x1	/* queries from the kernel */;
pub const CN_VSS_IDX: c_uint = 0xA     /* HyperV VSS */;
pub const CN_VSS_VAL: c_uint = 0x1     /* queries from the kernel */;

//
// Maximum connector's message size.
//
pub const CONNECTOR_MAX_MSG_SIZE: c_int = 16384;
//
// idx and val are unique identifiers which
// are used for message routing and
// must be registered in connector.h for in-kernel usage.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cb_id {
    pub idx: __u32,
    pub val: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cn_msg {
    pub id: cb_id,
    pub seq: __u32,
    pub ack: __u32,
    pub /: *mut *mut __u16 len; / Length of the following data,
    pub flags: __u16,
    pub data: [__u8; ],
}
