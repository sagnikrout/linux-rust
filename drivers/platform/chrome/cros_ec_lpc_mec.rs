//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/chrome/cros_ec_lpc_mec.h
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
// LPC variant I/O for Microchip EC
//
// Copyright (C) 2016 Google, Inc
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cros_ec_lpc_mec_emi_access_mode {
// 8-bit access
    ACCESS_TYPE_BYTE = 0x0,
// 16-bit access
    ACCESS_TYPE_WORD = 0x1,
// 32-bit access
    ACCESS_TYPE_LONG = 0x2,
//
// 32-bit access, read or write of MEC_EMI_EC_DATA_B3 causes the
// EC data register to be incremented.
//
    ACCESS_TYPE_LONG_AUTO_INCREMENT = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cros_ec_lpc_mec_io_type {
    MEC_IO_READ,
    MEC_IO_WRITE,
}

// EMI registers are relative to base

//
// cros_ec_lpc_mec_init() - Initialize MEC I/O.
//
// @base: MEC EMI Base address
// @end: MEC EMI End address
//
extern "C" {
    pub fn cros_ec_lpc_mec_init(base: c_uint, end: c_uint);
}
//
// cros_ec_lpc_mec_acpi_mutex() - Find and set ACPI mutex for MEC
//
// @adev:     Parent ACPI device
// @pathname: Name of AML mutex
// @return:   Negative error code, or zero for success
//
extern "C" {
    pub fn cros_ec_lpc_mec_acpi_mutex(adev: *mut acpi_device, pathname: *const c_char) -> c_int;
}
//
// cros_ec_lpc_mec_in_range() - Determine if addresses are in MEC EMI range.
//
// @offset: Address offset
// @length: Number of bytes to check
//
// Return: 1 if in range, 0 if not, and -EINVAL on failure
// such as the mec range not being initialized
//
extern "C" {
    pub fn cros_ec_lpc_mec_in_range(offset: c_uint, length: c_uint) -> c_int;
}
//
// cros_ec_lpc_io_bytes_mec - Read / write bytes to MEC EMI port
//
// @io_type: MEC_IO_READ or MEC_IO_WRITE, depending on request
// @offset:  Base read / write address
// @length:  Number of bytes to read / write
// @buf:     Destination / source buffer
//
// @return:  A negative error code on error, or 8-bit checksum of all
// bytes read / written
//
