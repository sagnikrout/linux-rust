//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/rdma_cm_ib.h
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
// Copyright (c) 2006 Intel Corporation.  All rights reserved.
//

//
// rdma_set_ib_path - Manually sets the path record used to establish a
// connection.
// @id: Connection identifier associated with the request.
// @path_rec: Reference to the path record
//
// This call permits a user to specify routing information for rdma_cm_id's
// bound to InfiniBand devices. It is called on the client side of a
// connection and replaces the call to rdma_resolve_route.
//
// Global qkey for UDP QPs and multicast groups.
pub const RDMA_UDP_QKEY: c_uint = 0x01234567;
