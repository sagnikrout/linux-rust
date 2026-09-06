//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dm-kcopyd.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2001 - 2003 Sistina Software
// Copyright (C) 2004 - 2008 Red Hat, Inc. All rights reserved.
//
// kcopyd provides a simple interface for copying an area of one
// block-device to one or more other block-devices, either synchronous
// or with an asynchronous completion notification.
//
// This file is released under the GPL.
//

// FIXME: make this configurable
pub const DM_KCOPYD_MAX_REGIONS: c_int = 8;
pub const DM_KCOPYD_IGNORE_ERROR: c_int = 1;
pub const DM_KCOPYD_WRITE_SEQ: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_kcopyd_throttle {
    pub throttle: c_uint,
    pub num_io_jobs: c_uint,
    pub io_period: c_uint,
    pub total_period: c_uint,
    pub last_jiffies: c_uint,
}

//
// kcopyd clients that want to support throttling must pass an initialised
// dm_kcopyd_throttle struct into dm_kcopyd_client_create().
// Two or more clients may share the same instance of this struct between
// them if they wish to be throttled as a group.
//
// This macro also creates a corresponding module parameter to configure
// the amount of throttling.
//

//
// To use kcopyd you must first create a dm_kcopyd_client object.
// throttle can be NULL if you don't want any throttling.
//
extern "C" {
    pub fn dm_kcopyd_client_destroy(kc: *mut dm_kcopyd_client);
}
extern "C" {
    pub fn dm_kcopyd_client_flush(kc: *mut dm_kcopyd_client);
}
//
// Submit a copy job to kcopyd.  This is built on top of the
// previous three fns.
//
// read_err is a boolean,
// write_err is a bitset, with 1 bit for each destination region
//
// Prepare a callback and submit it via the kcopyd thread.
//
// dm_kcopyd_prepare_callback allocates a callback structure and returns it.
// It must not be called from interrupt context.
// The returned value should be passed into dm_kcopyd_do_callback.
//
// dm_kcopyd_do_callback submits the callback.
// It may be called from interrupt context.
// The callback is issued from the kcopyd thread.
//
extern "C" {
    pub fn dm_kcopyd_do_callback(job: *mut c_void, read_err: c_int, write_err: unsigned int long);
}

