//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/fnic/vnic_wq_copy.h
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
// Copyright 2008 Cisco Systems, Inc.  All rights reserved.
// Copyright 2007 Nuova Systems, Inc.  All rights reserved.
//

pub const VNIC_WQ_COPY_MAX: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_wq_copy {
    pub index: c_uint,
    pub vdev: *mut vnic_dev,
    pub /: *mut *mut *mut vnic_wq_ctrl __iomem ctrl; / memory-mapped,
    pub ring: vnic_dev_ring,
    pub to_use_index: unsigned,
    pub to_clean_index: unsigned,
}

// Adding write memory barrier prevents compiler and/or CPU
// reordering, thus avoiding descriptor posting before
// descriptor is initialized. Otherwise, hardware can read
// stale descriptor fields.
//
// increment the to-clean index so that we start
// with an unprocessed index next time we enter the loop
//
// we have cleaned all the entries
extern "C" {
    pub fn vnic_wq_copy_enable(wq: *mut vnic_wq_copy);
}
extern "C" {
    pub fn vnic_wq_copy_disable(wq: *mut vnic_wq_copy) -> c_int;
}
extern "C" {
    pub fn vnic_wq_copy_free(wq: *mut vnic_wq_copy);
}
