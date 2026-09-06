//! Automatically rewritten from C Header to Rust Module
//! Source: tools/power/acpi/tools/acpidump/acpidump.h
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
// Module Name: acpidump.h - Include file for acpi_dump utility
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// Global variables. Defined in main.c only, externed in all other files
//

// Macro flag: #define EXTERN

// Globals
extern "C" {
    pub fn INIT_GLOBAL(_arg: gbl_summary_mode, _arg: FALSE) -> EXTERN u8;
}
extern "C" {
    pub fn INIT_GLOBAL(_arg: gbl_verbose_mode, _arg: FALSE) -> EXTERN u8;
}
extern "C" {
    pub fn INIT_GLOBAL(_arg: gbl_binary_mode, _arg: FALSE) -> EXTERN u8;
}
extern "C" {
    pub fn INIT_GLOBAL(_arg: gbl_dump_customized_tables, _arg: TRUE) -> EXTERN u8;
}
extern "C" {
    pub fn INIT_GLOBAL(_arg: gbl_do_not_dump_xsdt, _arg: FALSE) -> EXTERN u8;
}
extern "C" {
    pub fn INIT_GLOBAL(_arg: gbl_output_file, _arg: NULL) -> EXTERN ACPI_FILE;
}
extern "C" {
    pub fn INIT_GLOBAL(_arg: *mut gbl_output_filename, _arg: NULL) -> EXTERN char;
}
extern "C" {
    pub fn INIT_GLOBAL(_arg: gbl_rsdp_base, _arg: 0) -> EXTERN u64;
}
// Action table used to defer requested options
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_dump_action {
    pub argument: *mut c_char,
    pub to_be_done: u32,
}

pub const AP_MAX_ACTIONS: c_int = 32;
pub const AP_DUMP_ALL_TABLES: c_int = 0;
pub const AP_DUMP_TABLE_BY_ADDRESS: c_int = 1;
pub const AP_DUMP_TABLE_BY_NAME: c_int = 2;
pub const AP_DUMP_TABLE_BY_FILE: c_int = 3;

// Minimum FADT sizes for various table addresses

//
// apdump - Table get/dump routines
//
extern "C" {
    pub fn ap_dump_table_from_file(pathname: *mut c_char) -> c_int;
}
extern "C" {
    pub fn ap_dump_table_by_name(signature: *mut c_char) -> c_int;
}
extern "C" {
    pub fn ap_dump_table_by_address(ascii_address: *mut c_char) -> c_int;
}
extern "C" {
    pub fn ap_dump_all_tables() -> c_int;
}
extern "C" {
    pub fn ap_is_valid_header(table: *mut acpi_table_header) -> u8;
}
extern "C" {
    pub fn ap_is_valid_checksum(table: *mut acpi_table_header) -> u8;
}
extern "C" {
    pub fn ap_get_table_length(table: *mut acpi_table_header) -> u32;
}
//
// apfiles - File I/O utilities
//
extern "C" {
    pub fn ap_open_output_file(pathname: *mut c_char) -> c_int;
}
extern "C" {
    pub fn ap_write_to_binary_file(table: *mut acpi_table_header, instance: u32) -> c_int;
}
