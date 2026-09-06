//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fw_table.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// fw_tables.h - Parsing support for ACPI and ACPI-like tables provided by
// platform or device firmware
//
// Copyright (C) 2001 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
// Copyright (C) 2023 Intel Corp.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_subtable_proc {
    pub id: c_int,
    pub handler: acpi_tbl_entry_handler,
    pub handler_arg: acpi_tbl_entry_handler_arg,
    pub arg: *mut c_void,
    pub count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_table_header {
    pub acpi: acpi_table_header,
    pub cdat: acpi_table_cdat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union acpi_subtable_headers {
    pub common: acpi_subtable_header,
    pub hmat: acpi_hmat_structure,
    pub prmt: acpi_prmt_module_header,
    pub cedt: acpi_cedt_header,
    pub cdat: acpi_cdat_header,
}

// CXL is the only non-ACPI consumer of the FIRMWARE_TABLE library

// Macro flag: #define __init_or_fwtbl_lib

