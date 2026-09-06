//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/scsi_transport.h
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
// Transport specific attributes.
//
// Copyright (c) 2003 Silicon Graphics, Inc.  All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_transport_template {
// the attribute containers
    pub host_attrs: transport_container,
    pub target_attrs: transport_container,
    pub device_attrs: transport_container,
//
// If set, called from sysfs and legacy procfs rescanning code.
//
    pub u64): *mut *mut *mut int (user_scan)(struct Scsi_Host , uint, uint,,
// The size of the specific transport attribute structure (a
// space of this size will be left at the end of the
// scsi_* structure
    pub device_size: c_int,
    pub device_private_offset: c_int,
    pub target_size: c_int,
    pub target_private_offset: c_int,
    pub host_size: c_int,
// no private offset for the host; there's an alternative mechanism
//
// True if the transport wants to use a host-based work-queue
//
    pub 1: unsigned int create_work_queue :,
//
// Allows a transport to override the default error handler.
//
    pub ): *mut *mut void ( eh_strategy_handler)(struct Scsi_Host,
}

// Private area maintenance. The driver requested allocations come
// directly after the transport class allocations (if any).  The idea
// is that you *must* call these only once.  The code assumes that the
// initial values are the ones the transport specific code requires
extern "C" {
    pub fn scsi_init_limits(shost: *mut Scsi_Host, lim: *mut queue_limits);
}
