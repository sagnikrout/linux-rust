//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/nhlt.h
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
// Copyright(c) 2023-2024 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
// Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

//
// acpi_nhlt_endpoint_fmtscfg - Get the formats configuration space.
// @ep:		the endpoint to retrieve the space for.
//
// Return: A pointer to the formats configuration space.
//

//
// The for_each_nhlt_*() macros rely on an iterator to deal with the
// variable length of each endpoint structure and the possible presence
// of an OED-Config used by Windows only.
//
// for_each_nhlt_endpoint - Iterate over endpoints in a NHLT table.
// @tb:		the pointer to a NHLT table.
// @ep:		the pointer to endpoint to use as loop cursor.
//

//
// for_each_nhlt_fmtcfg - Iterate over format configurations.
// @fmts:	the pointer to formats configuration space.
// @fmt:	the pointer to format to use as loop cursor.
//

//
// for_each_nhlt_endpoint_fmtcfg - Iterate over format configurations in an endpoint.
// @ep:		the pointer to an endpoint.
// @fmt:	the pointer to format to use as loop cursor.
//

//
// System-wide pointer to the first NHLT table.
//
// A sound driver may utilize acpi_nhlt_get/put_gbl_table() on its
// initialization and removal respectively to avoid excessive mapping
// and unmapping of the memory occupied by the table between streaming
// operations.
//
extern "C" {
    pub fn acpi_nhlt_get_gbl_table() -> acpi_status;
}
extern "C" {
    pub fn acpi_nhlt_put_gbl_table();
}
extern "C" {
    pub fn acpi_nhlt_endpoint_mic_count(ep: *const acpi_nhlt_endpoint) -> c_int;
}

