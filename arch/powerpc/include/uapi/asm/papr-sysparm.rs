//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/papr-sysparm.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct papr_sysparm_io_block {
    pub parameter: __u32,
    pub length: __u16,
    pub data: [__u8; PAPR_SYSPARM_MAX_OUTPUT],
}

//
// PAPR_SYSPARM_IOC_GET - Retrieve the value of a PAPR system parameter.
//
// Uses _IOWR because of one corner case: Retrieving the value of the
// "OS Service Entitlement Status" parameter (60) requires the caller
// to supply input data (a date string) in the buffer passed to
// firmware. So the @length and @data of the incoming
// papr_sysparm_io_block are always used to initialize the work area
// supplied to ibm,get-system-parameter. No other parameters are known
// to parameterize the result this way, and callers are encouraged
// (but not required) to zero-initialize @length and @data in the
// common case.
//
// On error the contents of the ioblock are indeterminate.
//
// Return:
// 0: Success; @length is the length of valid data in @data, not to exceed @PAPR_SYSPARM_MAX_OUTPUT.
// -EIO: Platform error. (-1)
// -EINVAL: Incorrect data length or format. (-9999)
// -EPERM: The calling partition is not allowed to access this parameter. (-9002)
// -EOPNOTSUPP: Parameter not supported on this platform (-3)
//

//
// PAPR_SYSPARM_IOC_SET - Update the value of a PAPR system parameter.
//
// The contents of the ioblock are unchanged regardless of success.
//
// Return:
// 0: Success; the parameter has been updated.
// -EIO: Platform error. (-1)
// -EINVAL: Incorrect data length or format. (-9999)
// -EPERM: The calling partition is not allowed to access this parameter. (-9002)
// -EOPNOTSUPP: Parameter not supported on this platform (-3)
//

