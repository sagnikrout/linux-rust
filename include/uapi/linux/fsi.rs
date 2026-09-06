//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/fsi.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note

//
// /dev/scom "raw" ioctl interface
//
// The driver supports a high level "read/write" interface which
// handles retries and converts the status to Linux error codes,
// however low level tools an debugger need to access the "raw"
// HW status information and interpret it themselves, so this
// ioctl interface is also provided for their use case.
//
// Structure for SCOM read/write
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scom_access {
    pub /: *mut *mut __u64 addr; / SCOM address, supports indirect,
    pub /: *mut *mut __u64 data; / SCOM data (in for write, out for read),
    pub /: *mut *mut __u64 mask; / Data mask for writes,
    pub /: *mut *mut __u32 intf_errors; / Interface error flags,
pub const SCOM_INTF_ERR_PARITY: c_uint = 0x00000001 /* Parity error */;
pub const SCOM_INTF_ERR_PROTECTION: c_uint = 0x00000002 /* Blocked by secure boot */;
pub const SCOM_INTF_ERR_ABORT: c_uint = 0x00000004 /* PIB reset during access */;
pub const SCOM_INTF_ERR_UNKNOWN: c_uint = 0x80000000 /* Unknown error */;
//
// Note: Any other bit set in intf_errors need to be considered as an
// error. Future implementations may define new error conditions. The
// pib_status below is only valid if intf_errors is 0.
//
    pub /: *mut *mut __u8 pib_status; / 3-bit PIB status,

    pub pad: __u8,
}

// Flags for SCOM check
pub const SCOM_CHECK_SUPPORTED: c_uint = 0x00000001	/* Interface supported */;
pub const SCOM_CHECK_PROTECTED: c_uint = 0x00000002	/* Interface blocked by secure boot */;
// Flags for SCOM reset
pub const SCOM_RESET_INTF: c_uint = 0x00000001	/* Reset interface */;
pub const SCOM_RESET_PIB: c_uint = 0x00000002	/* Reset PIB */;

//
// /dev/sbefifo* ioctl interface
//
// FSI_SBEFIFO_CMD_TIMEOUT sets the timeout for writing data to the SBEFIFO.
//
// The command timeout is specified in seconds.  The minimum value of command
// timeout is 1 seconds (default) and the maximum value of command timeout is
// 120 seconds.  A command timeout of 0 will reset the value to the default of
// 1 seconds.
//

//
// FSI_SBEFIFO_READ_TIMEOUT sets the read timeout for response from SBE.
//
// The read timeout is specified in seconds.  The minimum value of read
// timeout is 10 seconds (default) and the maximum value of read timeout is
// 120 seconds.  A read timeout of 0 will reset the value to the default of
// (10 seconds).
//

