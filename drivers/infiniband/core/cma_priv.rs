//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/core/cma_priv.h
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
// Copyright (c) 2005 Voltaire Inc.  All rights reserved.
// Copyright (c) 2002-2005, Network Appliance, Inc. All rights reserved.
// Copyright (c) 1999-2005, Mellanox Technologies, Inc. All rights reserved.
// Copyright (c) 2005-2006 Intel Corporation.  All rights reserved.
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_cm_state {
    RDMA_CM_IDLE,
    RDMA_CM_ADDR_QUERY,
    RDMA_CM_ADDR_RESOLVED,
    RDMA_CM_ROUTE_QUERY,
    RDMA_CM_ROUTE_RESOLVED,
    RDMA_CM_CONNECT,
    RDMA_CM_DISCONNECT,
    RDMA_CM_ADDR_BOUND,
    RDMA_CM_LISTEN,
    RDMA_CM_DEVICE_REMOVAL,
    RDMA_CM_DESTROYING,
    RDMA_CM_ADDRINFO_QUERY,
    RDMA_CM_ADDRINFO_RESOLVED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_id_private {
    pub id: rdma_cm_id,
    pub bind_list: *mut rdma_bind_list,
    pub node: hlist_node,
    pub /: *mut *mut list_head device_item; / On cma_device->id_list,
    pub /: *mut *mut list_head listen_any_item; / On listen_any_list,
}

// On rdma_id_private->listen_list
//
// Internal to RDMA/core, don't use in the drivers
//

extern "C" {
    pub fn cma_configfs_init() -> c_int;
}
extern "C" {
    pub fn cma_configfs_exit();
}

extern "C" {
    pub fn cma_dev_get(dev: *mut cma_device);
}
extern "C" {
    pub fn cma_dev_put(dev: *mut cma_device);
}
extern "C" {
    pub fn bool(: *mut *mut cma_device_filter)(struct ib_device, : *mut c_void) -> typedef;
}
extern "C" {
    pub fn cma_get_default_gid_type(dev: *mut cma_device, port: u32) -> c_int;
}
extern "C" {
    pub fn cma_get_default_roce_tos(dev: *mut cma_device, port: u32) -> c_int;
}
