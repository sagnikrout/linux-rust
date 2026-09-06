//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/rdma_user_ioctl_cmds.h
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


//
// Copyright (c) 2018, Mellanox Technologies inc.  All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

// Documentation/userspace-api/ioctl/ioctl-number.rst
pub const RDMA_IOCTL_MAGIC: c_uint = 0x1b;

// User input
//
// Valid output bit should be ignored and considered set in
// mandatory fields. This bit is kernel output.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_attr {
    pub /: *mut *mut __u16 attr_id; / command specific type attribute,
    pub /: *mut *mut __u16 len; / only for pointers and IDRs array,
    pub /: *mut *mut __u16 flags; / combination of UVERBS_ATTR_F_XXXX,
    pub elem_id: __u8,
    pub reserved: __u8,
    pub enum_data: },
    pub reserved: __u16,
    pub attr_data: },
//
// ptr to command, inline data, idr/fd or
// ptr to __u32 array of IDRs
//
    pub data: __aligned_u64,
// Used by FD_IN and FD_OUT
    pub data_s64: __s64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_ioctl_hdr {
    pub length: __u16,
    pub object_id: __u16,
    pub method_id: __u16,
    pub num_attrs: __u16,
    pub reserved1: __aligned_u64,
    pub driver_id: __u32,
    pub reserved2: __u32,
    pub attrs: [ib_uverbs_attr; ],
}
