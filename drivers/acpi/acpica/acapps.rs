//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/acpica/acapps.h
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
// Module Name: acapps - common include for ACPI applications/tools
//
// Copyright (C) 2000 - 2026, Intel Corp.
//

// Common info for tool signons

// Macros for signons and file headers

// Macros for usage messages

// Check for unexpected exceptions

// Check for unexpected non-AE_OK errors

// acfileio
extern "C" {
    pub fn ac_delete_table_list(list_head: *mut acpi_new_table_desc);
}
extern "C" {
    pub fn ac_is_file_binary(file: *mut *mut FILE) -> u8;
}
extern "C" {
    pub fn ac_validate_table_header(file: *mut *mut FILE, table_offset: c_long) -> acpi_status;
}
// Values for get_only_aml_tables

//
// getopt
//
extern "C" {
    pub fn acpi_getopt(argc: c_int, argv: *mut c_char, opts: *mut c_char) -> c_int;
}
extern "C" {
    pub fn acpi_getopt_argument(argc: c_int, argv: *mut c_char) -> c_int;
}
//
// cmfsize - Common get file size function
//
extern "C" {
    pub fn cm_get_file_size(file: ACPI_FILE) -> u32;
}
//
// adwalk
//
extern "C" {
    pub fn acpi_dm_dump_tree(origin: *mut acpi_parse_object);
}
extern "C" {
    pub fn acpi_dm_find_orphan_methods(origin: *mut acpi_parse_object);
}
//
// adfile
//
extern "C" {
    pub fn ad_initialize() -> acpi_status;
}
