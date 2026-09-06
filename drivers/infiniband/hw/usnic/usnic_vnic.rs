//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/usnic/usnic_vnic.h
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
// Copyright (c) 2013, Cisco Systems, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// BSD license below:
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

// =USNIC_VNIC_RES_TYPE= =VNIC_RES=   =DESC=

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usnic_vnic_res_type {
    USNIC_VNIC_RES_TYPES
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usnic_vnic_res {
    pub type: usnic_vnic_res_type,
    pub vnic_idx: c_uint,
    pub vnic: *mut usnic_vnic,
    pub ctrl: *mut void __iomem,
    pub owner: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usnic_vnic_res_chunk {
    pub type: usnic_vnic_res_type,
    pub cnt: c_int,
    pub free_cnt: c_int,
    pub res: *mut usnic_vnic_res,
    pub vnic: *mut usnic_vnic,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usnic_vnic_res_desc {
    pub type: usnic_vnic_res_type,
    pub cnt: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usnic_vnic_res_spec {
    pub resources: [usnic_vnic_res_desc; USNIC_VNIC_RES_TYPE_MAX],
}

extern "C" {
    pub fn usnic_vnic_put_resources(chunk: *mut usnic_vnic_res_chunk);
}
extern "C" {
    pub fn usnic_vnic_free(vnic: *mut usnic_vnic);
}
extern "C" {
    pub fn usnic_vnic_get_index(vnic: *mut usnic_vnic) -> u16;
}
