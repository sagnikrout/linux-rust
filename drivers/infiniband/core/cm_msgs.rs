//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/core/cm_msgs.h
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
// Copyright (c) 2004, 2011 Intel Corporation.  All rights reserved.
// Copyright (c) 2004 Topspin Corporation.  All rights reserved.
// Copyright (c) 2004 Voltaire Corporation.  All rights reserved.
// Copyright (c) 2019, Mellanox Technologies inc.  All rights reserved.
//

//
// Parameters to routines below should be in network-byte order, and values
// are returned in network-byte order.
//

// Message REJected or MRAed
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cm_msg_response {
    CM_MSG_RESPONSE_REQ = 0x0,
    CM_MSG_RESPONSE_REP = 0x1,
    CM_MSG_RESPONSE_OTHER = 0x2
}
