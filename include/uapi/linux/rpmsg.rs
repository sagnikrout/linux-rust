//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/rpmsg.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright (c) 2016, Linaro Ltd.
//

pub const RPMSG_ADDR_ANY: c_uint = 0xFFFFFFFF;
//
// struct rpmsg_endpoint_info - endpoint info representation
// @name: name of service
// @src: local address. To set to RPMSG_ADDR_ANY if not used.
// @dst: destination address. To set to RPMSG_ADDR_ANY if not used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmsg_endpoint_info {
    pub name: [c_char; 32],
    pub src: __u32,
    pub dst: __u32,
}

//
// Instantiate a new rmpsg char device endpoint.
//

//
// Destroy a rpmsg char device endpoint created by the RPMSG_CREATE_EPT_IOCTL.
//

//
// Instantiate a new local rpmsg service device.
//

//
// Release a local rpmsg device.
//

//
// Get the flow control state of the remote rpmsg char device.
//

//
// Set the flow control state of the local rpmsg char device.
//

