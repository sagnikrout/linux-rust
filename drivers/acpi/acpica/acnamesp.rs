//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/acpica/acnamesp.h
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
// Name: acnamesp.h - Namespace subcomponent prototypes and defines
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// To search the entire name space, pass this as search_base

//
// Elements of acpi_ns_properties are bit significant
// and should be one-to-one with values of acpi_object_type
//
pub const ACPI_NS_NORMAL: c_int = 0;

// Flags for acpi_ns_lookup, acpi_ns_search_and_enter
pub const ACPI_NS_NO_UPSEARCH: c_int = 0;
pub const ACPI_NS_SEARCH_PARENT: c_uint = 0x0001;
pub const ACPI_NS_DONT_OPEN_SCOPE: c_uint = 0x0002;
pub const ACPI_NS_NO_PEER_SEARCH: c_uint = 0x0004;
pub const ACPI_NS_ERROR_IF_FOUND: c_uint = 0x0008;
pub const ACPI_NS_PREFIX_IS_SCOPE: c_uint = 0x0010;
pub const ACPI_NS_EXTERNAL: c_uint = 0x0020;
pub const ACPI_NS_TEMPORARY: c_uint = 0x0040;
pub const ACPI_NS_OVERRIDE_IF_FOUND: c_uint = 0x0080;
pub const ACPI_NS_EARLY_INIT: c_uint = 0x0100;
pub const ACPI_NS_PREFIX_MUST_EXIST: c_uint = 0x0200;
// Flags for acpi_ns_walk_namespace
pub const ACPI_NS_WALK_NO_UNLOCK: c_int = 0;
pub const ACPI_NS_WALK_UNLOCK: c_uint = 0x01;
pub const ACPI_NS_WALK_TEMP_NODES: c_uint = 0x02;
// Object is not a package element

// Always emit warning message, not dependent on node flags
pub const ACPI_WARN_ALWAYS: c_int = 0;
//
// nsinit - Namespace initialization
//
extern "C" {
    pub fn acpi_ns_initialize_objects() -> acpi_status;
}
extern "C" {
    pub fn acpi_ns_initialize_devices(flags: u32) -> acpi_status;
}
//
// nsload -  Namespace loading
//
extern "C" {
    pub fn acpi_ns_load_namespace() -> acpi_status;
}
//
// nswalk - walk the namespace
//
// parent,
// child);
// parent,
// child);
//
// nsparse - table parsing
//
// nsaccess - Top-level namespace access
//
extern "C" {
    pub fn acpi_ns_root_initialize() -> acpi_status;
}
//
// nsalloc - Named object allocation/deallocation
//
extern "C" {
    pub fn acpi_ns_delete_node(node: *mut acpi_namespace_node);
}
extern "C" {
    pub fn acpi_ns_remove_node(node: *mut acpi_namespace_node);
}
extern "C" {
    pub fn acpi_ns_delete_namespace_by_owner(owner_id: acpi_owner_id);
}
extern "C" {
    pub fn acpi_ns_detach_object(node: *mut acpi_namespace_node);
}
extern "C" {
    pub fn acpi_ns_delete_children(parent: *mut acpi_namespace_node);
}
extern "C" {
    pub fn acpi_ns_compare_names(name1: *mut c_char, name2: *mut c_char) -> c_int;
}
//
// nsconvert - Dynamic object conversion routines
//
// nsdump - Namespace dump/print utilities
//
extern "C" {
    pub fn acpi_ns_dump_tables(search_base: acpi_handle, max_depth: u32);
}
extern "C" {
    pub fn acpi_ns_dump_entry(handle: acpi_handle, debug_level: u32);
}
extern "C" {
    pub fn acpi_ns_print_pathname(num_segments: u32, pathname: *const c_char);
}
//
// nseval - Namespace evaluation functions
//
extern "C" {
    pub fn acpi_ns_evaluate(info: *mut acpi_evaluate_info) -> acpi_status;
}
//
// nsarguments - Argument count/type checking for predefined/reserved names
//
extern "C" {
    pub fn acpi_ns_check_argument_types(info: *mut acpi_evaluate_info);
}
//
// nspredef - Return value checking for predefined/reserved names
//
// nsprepkg - Validation of predefined name packages
//
// nsnames - Name and Scope manipulation
//
extern "C" {
    pub fn acpi_ns_opens_scope(type: acpi_object_type) -> u32;
}
extern "C" {
    pub fn acpi_ns_normalize_pathname(original_path: *mut c_char);
}
extern "C" {
    pub fn acpi_ns_get_pathname_length(node: *mut acpi_namespace_node) -> acpi_size;
}
//
// nsobject - Object management for namespace nodes
//
// node);
// obj_desc);
//
// nsrepair - General return object repair for all
// predefined methods/objects
//
// nsrepair2 - Return object repair for specific
// predefined methods/objects
//
// nssearch - Namespace searching and entry
//
// nsutils - Utility functions
//
extern "C" {
    pub fn acpi_ns_get_type(node: *mut acpi_namespace_node) -> acpi_object_type;
}
extern "C" {
    pub fn acpi_ns_local(type: acpi_object_type) -> u32;
}
extern "C" {
    pub fn acpi_ns_build_internal_name(info: *mut acpi_namestring_info) -> acpi_status;
}
extern "C" {
    pub fn acpi_ns_get_internal_name_length(info: *mut acpi_namestring_info);
}
extern "C" {
    pub fn acpi_ns_terminate();
}
