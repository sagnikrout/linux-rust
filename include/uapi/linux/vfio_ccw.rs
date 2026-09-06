//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/vfio_ccw.h
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
// Interfaces for vfio-ccw
//
// Copyright IBM Corp. 2017
//
// Author(s): Dong Jia Shi <bjsdjshi@linux.vnet.ibm.com>
//

// used for START SUBCHANNEL, always present
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccw_io_region {
pub const ORB_AREA_SIZE: c_int = 12;
    pub orb_area: [__u8; ORB_AREA_SIZE],
pub const SCSW_AREA_SIZE: c_int = 12;
    pub scsw_area: [__u8; SCSW_AREA_SIZE],
pub const IRB_AREA_SIZE: c_int = 96;
    pub irb_area: [__u8; IRB_AREA_SIZE],
    pub ret_code: __u32,
    pub __packed: },
//
// used for processing commands that trigger asynchronous actions
// Note: this is controlled by a capability
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccw_cmd_region {
    pub command: __u32,
    pub ret_code: __u32,
    pub __packed: },
//
// Used for processing commands that read the subchannel-information block
// Reading this region triggers a stsch() to hardware
// Note: this is controlled by a capability
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccw_schib_region {
pub const SCHIB_AREA_SIZE: c_int = 52;
    pub schib_area: [__u8; SCHIB_AREA_SIZE],
    pub __packed: },
//
// Used for returning a Channel Report Word to userspace.
// Note: this is controlled by a capability
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccw_crw_region {
    pub crw: __u32,
    pub pad: __u32,
    pub __packed: },
