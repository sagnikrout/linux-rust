//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/uapi/asm/monwriter.h
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
// Copyright IBM Corp. 2006
// Character device driver for writing z/VM APPLDATA monitor records
// Version 1.0
// Author(s): Melissa Howland <melissah@us.ibm.com>
//
// mon_function values
pub const MONWRITE_START_INTERVAL: c_uint = 0x00 /* start interval recording */;
pub const MONWRITE_STOP_INTERVAL: c_uint = 0x01 /* stop interval or config recording */;
pub const MONWRITE_GEN_EVENT: c_uint = 0x02 /* generate event record */;
pub const MONWRITE_START_CONFIG: c_uint = 0x03 /* start configuration recording */;
// the header the app uses in its write() data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct monwrite_hdr {
    pub mon_function: c_uchar,
    pub applid: c_ushort,
    pub record_num: c_uchar,
    pub version: c_ushort,
    pub release: c_ushort,
    pub mod_level: c_ushort,
    pub datalen: c_ushort,
    pub hdrlen: c_uchar,
    pub __attribute__((packed)): },
