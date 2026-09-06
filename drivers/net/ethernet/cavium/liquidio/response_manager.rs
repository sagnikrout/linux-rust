//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/liquidio/response_manager.h
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


//
// Author: Cavium, Inc.
//
// Contact: support@cavium.com
// Please include "LiquidIO" in the subject.
//
// Copyright (c) 2003-2016 Cavium, Inc.
//
// This file is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License, Version 2, as
// published by the Free Software Foundation.
//
// This file is distributed in the hope that it will be useful, but
// AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
// NONINFRINGEMENT.  See the GNU General Public License for more
// details.
//
// ! \file response_manager.h
// \brief Host Driver:  Response queues for host instructions.
//
// Maximum ordered requests to process in every invocation of
// lio_process_ordered_list(). The function will continue to process requests
// as long as it can find one that has finished processing. If it keeps
// finding requests that have completed, the function can run for ever. The
// value defined here sets an upper limit on the number of requests it can
// process before it returns control to the poll thread.
//
pub const MAX_ORD_REQS_TO_PROCESS: c_int = 4096;
// Head of a response list. There are several response lists in the
// system. One for each response order- Unordered, ordered
// and 1 for noresponse entries on each instruction queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_response_list {
// List structure to add delete pending entries to
    pub head: list_head,
// A lock for this response list
    pub lock: spinlock_t,
    pub pending_req_count: core::sync::atomic::AtomicI32,
}

// The type of response list.
//
// Response Order values for a Octeon Request.
// Error codes  used in Octeon Host-Core communication.
//
// 31            16 15            0
// ---------------------------------
// |               |               |
// ---------------------------------
// Error codes are 32-bit wide. The upper 16-bits, called Major Error Number,
// are reserved to identify the group to which the error code belongs. The
// lower 16-bits, called Minor Error Number, carry the actual code.
//
// So error codes are (MAJOR NUMBER << 16)| MINOR_NUMBER.
//
// ------------   Error codes used by host driver   -----------------
pub const DRIVER_MAJOR_ERROR_CODE: c_uint = 0x0000;
// ------   Error codes used by firmware (bits 15..0 set by firmware
pub const FIRMWARE_MAJOR_ERROR_CODE: c_uint = 0x0001;
// A value of 0x00000000 indicates no error i.e. success
pub const DRIVER_ERROR_NONE: c_uint = 0x00000000;
pub const DRIVER_ERROR_REQ_PENDING: c_uint = 0x00000001;
pub const DRIVER_ERROR_REQ_TIMEOUT: c_uint = 0x00000003;
pub const DRIVER_ERROR_REQ_EINTR: c_uint = 0x00000004;
pub const DRIVER_ERROR_REQ_ENXIO: c_uint = 0x00000006;
pub const DRIVER_ERROR_REQ_ENOMEM: c_uint = 0x0000000C;
pub const DRIVER_ERROR_REQ_EINVAL: c_uint = 0x00000016;
pub const DRIVER_ERROR_REQ_FAILED: c_uint = 0x000000ff;
// Status for a request.
// If a request is not queued to Octeon by the driver, the driver returns
// an error condition that's describe by one of the OCTEON_REQ_ERR_* value
// below. If the request is successfully queued, the driver will return
// a OCTEON_REQUEST_PENDING status. OCTEON_REQUEST_TIMEOUT and
// OCTEON_REQUEST_INTERRUPTED are only returned by the driver if the
// response for request failed to arrive before a time-out period or if
// the request processing * got interrupted due to a signal respectively.
//

// Initialize the response lists. The number of response lists to create is
// given by count.
// @param octeon_dev      - the octeon device structure.
//
extern "C" {
    pub fn octeon_setup_response_list(octeon_dev: *mut octeon_device) -> c_int;
}
extern "C" {
    pub fn octeon_delete_response_list(octeon_dev: *mut octeon_device);
}
// Check the status of first entry in the ordered list. If the instruction at
// that entry finished processing or has timed-out, the entry is cleaned.
// @param octeon_dev  - the octeon device structure.
// @param force_quit - the request is forced to timeout if this is 1
// @return 1 if the ordered list is empty, 0 otherwise.
//
