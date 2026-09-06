//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/ib_addr.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2005 Voltaire Inc.  All rights reserved.
// Copyright (c) 2005 Intel Corporation.  All rights reserved.
//

//
// struct rdma_dev_addr - Contains resolved RDMA hardware addresses
// @src_dev_addr:	Source MAC address.
// @dst_dev_addr:	Destination MAC address.
// @broadcast:		Broadcast address of the device.
// @dev_type:		The interface hardware type of the device.
// @bound_dev_if:	An optional device interface index.
// @transport:		The transport type used.
// @net:		Network namespace containing the bound_dev_if net_dev.
// @sgid_attr:		GID attribute to use for identified SGID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_dev_addr {
    pub src_dev_addr: [c_uchar; MAX_ADDR_LEN],
    pub dst_dev_addr: [c_uchar; MAX_ADDR_LEN],
    pub broadcast: [c_uchar; MAX_ADDR_LEN],
    pub dev_type: c_ushort,
    pub bound_dev_if: c_int,
    pub transport: rdma_transport_type,
    pub net: *mut net,
    pub sgid_attr: *const ib_gid_attr,
    pub network: rdma_network_type,
    pub hoplimit: c_int,
}

//
// rdma_translate_ip - Translate a local IP address to an RDMA hardware
// address.
//
// The dev_addr->net field must be initialized.
//
// rdma_resolve_ip - Resolve source and destination IP addresses to
// RDMA hardware addresses.
// @src_addr: An optional source address to use in the resolution.  If a
// source address is not provided, a usable address will be returned via
// the callback.
// @dst_addr: The destination address to resolve.
// @addr: A reference to a data location that will receive the resolved
// addresses.  The data location must remain valid until the callback has
// been invoked. The net field of the addr struct must be valid.
// @timeout_ms: Amount of time to wait for the address resolution to complete.
// @callback: Call invoked once address resolution has completed, timed out,
// or been canceled.  A status of 0 indicates success.
// @resolve_by_gid_attr:	Resolve the ip based on the GID attribute from
// rdma_dev_addr.
// @context: User-specified context associated with the call.
//
extern "C" {
    pub fn rdma_addr_cancel(addr: *mut rdma_dev_addr);
}
extern "C" {
    pub fn rdma_addr_size(addr: *const sockaddr) -> c_int;
}
extern "C" {
    pub fn rdma_addr_size_in6(addr: *mut sockaddr_in6) -> c_int;
}
extern "C" {
    pub fn rdma_addr_size_kss(addr: *mut __kernel_sockaddr_storage) -> c_int;
}
// (struct in6_addr *)&gid->raw =
// Important - sockaddr should be a union of sockaddr_in and sockaddr_in6
//
// rdma_get/set_sgid/dgid() APIs are applicable to IB, and iWarp.
// They are not applicable to RoCE.
// RoCE GIDs are derived from the IP addresses.
//
// Reduce IB headers from effective IBoE MTU.
//
