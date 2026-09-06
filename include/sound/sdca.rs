//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sdca.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// The MIPI SDCA specification is available for public downloads at
// https://www.mipi.org/mipi-sdca-v1-0-download
//
// Copyright(c) 2024 Intel Corporation
//

pub const SDCA_MAX_FUNCTION_COUNT: c_int = 8;
//
// struct sdca_function_desc - short descriptor for an SDCA Function
// @node: firmware node for the Function.
// @func_dev: pointer to SDCA function device.
// @name: Human-readable string.
// @type: Function topology type.
// @adr: ACPI address (used for SDCA register access).
// @duplicate: Internal flag to indicate if other functions of the same type
// exist.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdca_function_desc {
    pub node: *mut fwnode_handle,
    pub func_dev: *mut sdca_dev,
    pub name: *const c_char,
    pub type: u32,
    pub adr: u8,
    pub duplicate: bool,
}

//
// struct sdca_device_data - structure containing all SDCA related information
// @interface_revision: Value read from _DSD property, mainly to check
// for changes between silicon versions.
// @num_functions: Total number of supported SDCA functions. Invalid/unsupported
// functions will be skipped.
// @function: Array of function descriptors.
// @swft: Pointer to the SWFT table, if available.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdca_device_data {
    pub interface_revision: u32,
    pub num_functions: c_int,
    pub function: [sdca_function_desc; SDCA_MAX_FUNCTION_COUNT],
    pub swft: *mut acpi_table_swft,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdca_quirk {
    SDCA_QUIRKS_RT712_VB,
    SDCA_QUIRKS_SKIP_FUNC_TYPE_PATCHING,
}

extern "C" {
    pub fn sdca_lookup_functions(slave: *mut sdw_slave);
}
extern "C" {
    pub fn sdca_lookup_swft(slave: *mut sdw_slave);
}
extern "C" {
    pub fn sdca_lookup_interface_revision(slave: *mut sdw_slave);
}
extern "C" {
    pub fn sdca_device_quirk_match(slave: *mut sdw_slave, quirk: sdca_quirk) -> bool;
}
extern "C" {
    pub fn sdca_dev_register_functions(slave: *mut sdw_slave) -> c_int;
}
extern "C" {
    pub fn sdca_dev_unregister_functions(slave: *mut sdw_slave);
}

