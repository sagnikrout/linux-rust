//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/usnic/usnic_fwd.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usnic_fwd_dev {
    pub pdev: *mut pci_dev,
    pub netdev: *mut net_device,
    pub lock: spinlock_t,
//
// The following fields can be read directly off the device.
// However, they should be set by a accessor function, except name,
// which cannot be changed.
//
    pub link_up: bool,
    pub mac: [c_char; ETH_ALEN],
    pub mtu: c_uint,
    pub inaddr: __be32,
    pub name: [c_char; IFNAMSIZ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usnic_fwd_flow {
    pub flow_id: u32,
    pub ufdev: *mut usnic_fwd_dev,
    pub vnic_idx: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usnic_filter_action {
    pub vnic_idx: c_int,
    pub action: filter_action,
}

extern "C" {
    pub fn usnic_fwd_dev_free(ufdev: *mut usnic_fwd_dev);
}
extern "C" {
    pub fn usnic_fwd_set_mac(ufdev: *mut usnic_fwd_dev, mac[ETH_ALEN]: c_char);
}
extern "C" {
    pub fn usnic_fwd_add_ipaddr(ufdev: *mut usnic_fwd_dev, inaddr: __be32);
}
extern "C" {
    pub fn usnic_fwd_del_ipaddr(ufdev: *mut usnic_fwd_dev);
}
extern "C" {
    pub fn usnic_fwd_carrier_up(ufdev: *mut usnic_fwd_dev);
}
extern "C" {
    pub fn usnic_fwd_carrier_down(ufdev: *mut usnic_fwd_dev);
}
extern "C" {
    pub fn usnic_fwd_set_mtu(ufdev: *mut usnic_fwd_dev, mtu: c_uint);
}
//
// Allocate a flow on this forwarding device. Whoever calls this function,
// must monitor netdev events on ufdev's netdevice. If NETDEV_REBOOT or
// NETDEV_DOWN is seen, flow will no longer function and must be
// immediately freed by calling usnic_dealloc_flow.
//
extern "C" {
    pub fn usnic_fwd_dealloc_flow(flow: *mut usnic_fwd_flow) -> c_int;
}
extern "C" {
    pub fn usnic_fwd_enable_qp(ufdev: *mut usnic_fwd_dev, vnic_idx: c_int, qp_idx: c_int) -> c_int;
}
extern "C" {
    pub fn usnic_fwd_disable_qp(ufdev: *mut usnic_fwd_dev, vnic_idx: c_int, qp_idx: c_int) -> c_int;
}
