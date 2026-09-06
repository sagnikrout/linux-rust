//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/usnic/usnic_abi.h
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
// ABI between userspace and kernel
pub const USNIC_UVERBS_ABI_VERSION: c_int = 4;
pub const USNIC_QP_GRP_MAX_WQS: c_int = 8;
pub const USNIC_QP_GRP_MAX_RQS: c_int = 8;
pub const USNIC_QP_GRP_MAX_CQS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usnic_transport_type {
    USNIC_TRANSPORT_UNKNOWN		= 0,
    USNIC_TRANSPORT_ROCE_CUSTOM	= 1,
    USNIC_TRANSPORT_IPV4_UDP	= 2,
    USNIC_TRANSPORT_MAX		= 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usnic_transport_spec {
    pub trans_type: usnic_transport_type,
    pub port_num: u16,
    pub usnic_roce: },
    pub sock_fd: u32,
    pub udp: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usnic_ib_create_qp_cmd {
    pub spec: usnic_transport_spec,
}

// TODO: Future - usnic_modify_qp needs to pass in generic filters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usnic_ib_create_qp_resp {
    pub vfid: u32,
    pub qp_grp_id: u32,
    pub bar_bus_addr: u64,
    pub bar_len: u32,
//
// WQ, RQ, CQ are explicitly specified bc exposing a generic resources inteface
// expands the scope of ABI to many files.
//
    pub wq_cnt: u32,
    pub rq_cnt: u32,
    pub cq_cnt: u32,
    pub wq_idx: [u32; USNIC_QP_GRP_MAX_WQS],
    pub rq_idx: [u32; USNIC_QP_GRP_MAX_RQS],
    pub cq_idx: [u32; USNIC_QP_GRP_MAX_CQS],
    pub transport: u32,
    pub reserved: [u32; 9],
}
