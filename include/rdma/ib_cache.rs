//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/ib_cache.h
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
// Copyright (c) 2004 Topspin Communications.  All rights reserved.
// Copyright (c) 2005 Intel Corporation. All rights reserved.
// Copyright (c) 2005 Sun Microsystems, Inc. All rights reserved.
//

//
// ib_get_cached_pkey - Returns a cached PKey table entry
// @device_handle: The device to query.
// @port_num: The port number of the device to query.
// @index: The index into the cached PKey table to query.
// @pkey: The PKey value found at the specified index.
//
// ib_get_cached_pkey() fetches the specified PKey table entry stored in
// the local software cache.
//
// ib_find_cached_pkey - Returns the PKey table index where a specified
// PKey value occurs.
// @device: The device to query.
// @port_num: The port number of the device to search for the PKey.
// @pkey: The PKey value to search for.
// @index: The index into the cached PKey table where the PKey was found.
//
// ib_find_cached_pkey() searches the specified PKey table in
// the local software cache.
//
// ib_get_cached_lmc - Returns a cached lmc table entry
// @device: The device to query.
// @port_num: The port number of the device to query.
// @lmc: The lmc value for the specified port for that device.
//
// ib_get_cached_lmc() fetches the specified lmc table entry stored in
// the local software cache.
//
// ib_get_cached_port_state - Returns a cached port state table entry
// @device: The device to query.
// @port_num: The port number of the device to query.
// @port_active: port_state for the specified port for that device.
//
// ib_get_cached_port_state() fetches the specified port_state table entry stored in
// the local software cache.
//
extern "C" {
    pub fn rdma_is_zero_gid(gid: *const ib_gid) -> bool;
}
extern "C" {
    pub fn rdma_put_gid_attr(attr: *const ib_gid_attr);
}
extern "C" {
    pub fn rdma_hold_gid_attr(attr: *const ib_gid_attr);
}
