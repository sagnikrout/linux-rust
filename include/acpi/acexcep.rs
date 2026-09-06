//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/acexcep.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Name: acexcep.h - Exception codes returned by the ACPI subsystem
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// This module contains all possible exception codes for acpi_status
//
// Exception code classes
//
pub const AE_CODE_ENVIRONMENTAL: c_uint = 0x0000	/* General ACPICA environment */;
pub const AE_CODE_PROGRAMMER: c_uint = 0x1000	/* External ACPICA interface caller */;
pub const AE_CODE_ACPI_TABLES: c_uint = 0x2000	/* ACPI tables */;
pub const AE_CODE_AML: c_uint = 0x3000	/* From executing AML code */;
pub const AE_CODE_CONTROL: c_uint = 0x4000	/* Internal control codes */;
pub const AE_CODE_MAX: c_uint = 0x4000;
pub const AE_CODE_MASK: c_uint = 0xF000;
//
// Macros to insert the exception code classes
//

//
// Exception info table. The "Description" field is used only by the
// ACPICA help application (acpihelp).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_exception_info {
    pub name: *mut c_char,

    pub description: *mut c_char,

}

//
// Success is always zero, failure is non-zero
//

//
// Environmental exceptions
//

pub const AE_CODE_ENV_MAX: c_uint = 0x0023;
//
// Programmer exceptions
//

pub const AE_CODE_PGM_MAX: c_uint = 0x0009;
//
// Acpi table exceptions
//

pub const AE_CODE_TBL_MAX: c_uint = 0x0005;
//
// AML exceptions. These are caused by problems with
// the actual AML byte stream
//

pub const AE_CODE_AML_MAX: c_uint = 0x0027;
//
// Internal exceptions used for control
//

pub const AE_CODE_CTRL_MAX: c_uint = 0x000C;
// Exception strings for acpi_format_exception

//
// String versions of the exception codes above
// These strings must match the corresponding defines exactly
//

