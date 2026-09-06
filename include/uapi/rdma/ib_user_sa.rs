//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/ib_user_sa.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB)
//
// Copyright (c) 2005 Intel Corporation.  All rights reserved.
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_path_rec_data {
    pub flags: __u32,
    pub reserved: __u32,
    pub path_rec: [__u32; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_user_path_rec {
    pub dgid: [__u8; 16],
    pub sgid: [__u8; 16],
    pub dlid: __be16,
    pub slid: __be16,
    pub raw_traffic: __u32,
    pub flow_label: __be32,
    pub reversible: __u32,
    pub mtu: __u32,
    pub pkey: __be16,
    pub hop_limit: __u8,
    pub traffic_class: __u8,
    pub numb_path: __u8,
    pub sl: __u8,
    pub mtu_selector: __u8,
    pub rate_selector: __u8,
    pub rate: __u8,
    pub packet_life_time_selector: __u8,
    pub packet_life_time: __u8,
    pub preference: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_user_service_rec {
    pub id: __be64,
    pub gid: [__u8; 16],
    pub pkey: __be16,
    pub reserved: [__u8; 2],
    pub lease: __be32,
    pub key: [__u8; 16],
    pub name: [__u8; 64],
    pub data_8: [__u8; 16],
    pub data_16: [__be16; 8],
    pub data_32: [__be32; 4],
    pub data_64: [__be64; 2],
}
