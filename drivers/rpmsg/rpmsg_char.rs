//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/rpmsg/rpmsg_char.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2022, STMicroelectronics
//

//
// rpmsg_chrdev_eptdev_create() - register char device based on an endpoint
// @rpdev:  prepared rpdev to be used for creating endpoints
// @parent: parent device
// @chinfo: associated endpoint channel information.
//
// This function create a new rpmsg char endpoint device to instantiate a new
// endpoint based on chinfo information.
//
// rpmsg_chrdev_eptdev_destroy() - destroy created char device endpoint.
// @data: private data associated to the endpoint device
//
// This function destroys a rpmsg char endpoint device created by the RPMSG_DESTROY_EPT_IOCTL
// control.
//
extern "C" {
    pub fn rpmsg_chrdev_eptdev_destroy(dev: *mut device, data: *mut c_void) -> c_int;
}

