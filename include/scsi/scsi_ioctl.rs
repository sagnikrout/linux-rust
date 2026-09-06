//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/scsi_ioctl.h
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
pub const SCSI_IOCTL_SEND_COMMAND: c_int = 1;
pub const SCSI_IOCTL_TEST_UNIT_READY: c_int = 2;
pub const SCSI_IOCTL_BENCHMARK_COMMAND: c_int = 3;

pub const SCSI_IOCTL_START_UNIT: c_int = 5;
pub const SCSI_IOCTL_STOP_UNIT: c_int = 6;
// The door lock/unlock constants are compatible with Sun constants for
pub const SCSI_IOCTL_DOORLOCK: c_uint = 0x5380		/* lock the eject mechanism */;
pub const SCSI_IOCTL_DOORUNLOCK: c_uint = 0x5381		/* unlock the mechanism	  */;
pub const SCSI_REMOVAL_PREVENT: c_int = 1;
pub const SCSI_REMOVAL_ALLOW: c_int = 0;

//
// Structures used for scsi_ioctl et al.
//
// Fibre Channel WWN, port_id struct
extern "C" {
    pub fn get_sg_io_hdr(hdr: *mut sg_io_hdr, argp: *const void __user) -> c_int;
}
extern "C" {
    pub fn put_sg_io_hdr(hdr: *const sg_io_hdr, argp: *mut void __user) -> c_int;
}
extern "C" {
    pub fn scsi_cmd_allowed(cmd: *mut c_uchar, open_for_write: bool) -> bool;
}

