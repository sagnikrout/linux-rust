//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/cio/trace.h
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
// Tracepoint header for the s390 Common I/O layer (CIO)
//
// Copyright IBM Corp. 2015
// Author(s): Peter Oberparleiter <oberpar@linux.vnet.ibm.com>
//

//
// s390_cio_stsch -  Store Subchannel instruction (STSCH) was performed
// @schid: Subchannel ID
// @schib: Subchannel-Information block
// @cc: Condition code
//
// s390_cio_msch -  Modify Subchannel instruction (MSCH) was performed
// @schid: Subchannel ID
// @schib: Subchannel-Information block
// @cc: Condition code
//
// s390_cio_tsch - Test Subchannel instruction (TSCH) was performed
// @schid: Subchannel ID
// @irb: Interruption-Response Block
// @cc: Condition code
//
// s390_cio_tpi - Test Pending Interruption instruction (TPI) was performed
// @addr: Address of the I/O interruption code or %NULL
// @cc: Condition code
//
// s390_cio_ssch - Start Subchannel instruction (SSCH) was performed
// @schid: Subchannel ID
// @orb: Operation-Request Block
// @cc: Condition code
//
// s390_cio_csch - Clear Subchannel instruction (CSCH) was performed
// @schid: Subchannel ID
// @cc: Condition code
//
// s390_cio_hsch - Halt Subchannel instruction (HSCH) was performed
// @schid: Subchannel ID
// @cc: Condition code
//
// s390_cio_xsch - Cancel Subchannel instruction (XSCH) was performed
// @schid: Subchannel ID
// @cc: Condition code
//
// s390_cio_rsch - Resume Subchannel instruction (RSCH) was performed
// @schid: Subchannel ID
// @cc: Condition code
//
pub const CHSC_MAX_REQUEST_LEN: c_int = 64;
pub const CHSC_MAX_RESPONSE_LEN: c_int = 64;
//
// s390_cio_chsc - Channel Subsystem Call (CHSC) instruction was performed
// @chsc: CHSC block
// @cc: Condition code
//
// s390_cio_interrupt - An I/O interrupt occurred
// @tpi_info: Address of the I/O interruption code
//
// s390_cio_adapter_int - An adapter interrupt occurred
// @tpi_info: Address of the I/O interruption code
//
// s390_cio_stcrw - Store Channel Report Word (STCRW) was performed
// @crw: Channel Report Word
// @cc: Condition code
//

// This part must be outside protection

