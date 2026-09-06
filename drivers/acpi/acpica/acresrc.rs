//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/acpica/acresrc.h
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
// Name: acresrc.h - Resource Manager function prototypes
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// Need the AML resource descriptor structs

//
// If possible, pack the following structures to byte alignment, since we
// don't care about performance for debug output. Two cases where we cannot
// pack the structures:
//
// 1) Hardware does not support misaligned memory transfers
// 2) Compiler does not support pointers within packed structures
//

//
// Individual entry for the resource conversion tables
//
// Resource conversion opcodes
// Resource Conversion sub-opcodes
pub const ACPI_RSC_COMPARE_AML_LENGTH: c_int = 0;
pub const ACPI_RSC_COMPARE_VALUE: c_int = 1;

//
// Individual entry for the resource dump tables
//
// Values for the Opcode field above
// restore default alignment

// Resource tables indexed by internal resource type
// Resource tables indexed by raw AML resource descriptor type
// acpi_gbl_convert_resource_serial_bus_dispatch[];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_vendor_walk_info {
    pub uuid: *mut acpi_vendor_uuid,
    pub buffer: *mut acpi_buffer,
    pub status: acpi_status,
}

//
// rscreate
//
// rsutils
//
// rscalc
//
// rsaddr
//
// rsmisc
//
// rsutils
//
extern "C" {
    pub fn acpi_rs_decode_bitmask(mask: u16, list: *mut *mut u8) -> u8;
}
extern "C" {
    pub fn acpi_rs_encode_bitmask(list: *mut *mut u8, count: u8) -> u16;
}
//
// rsdump - Debugger support
//

extern "C" {
    pub fn acpi_rs_dump_resource_list(resource: *mut acpi_resource);
}
extern "C" {
    pub fn acpi_rs_dump_irq_list(route_table: *mut u8);
}

//
// Resource conversion tables
//
// These resources require separate get/set tables

//
// rsinfo
//
// rsdumpinfo
//

