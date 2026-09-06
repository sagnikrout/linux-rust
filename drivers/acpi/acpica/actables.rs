//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/acpica/actables.h
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
// Name: actables.h - ACPI table management
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
extern "C" {
    pub fn acpi_allocate_root_table(initial_table_count: u32) -> acpi_status;
}
//
// tbxfroot - Root pointer utilities
//
extern "C" {
    pub fn acpi_tb_get_rsdp_length(rsdp: *mut acpi_table_rsdp) -> u32;
}
extern "C" {
    pub fn acpi_tb_validate_rsdp(rsdp: *mut acpi_table_rsdp) -> acpi_status;
}
//
// tbdata - table data structure management
//
extern "C" {
    pub fn acpi_tb_release_temp_table(table_desc: *mut acpi_table_desc);
}
extern "C" {
    pub fn acpi_tb_validate_temp_table(table_desc: *mut acpi_table_desc) -> acpi_status;
}
extern "C" {
    pub fn acpi_tb_is_table_loaded(table_index: u32) -> u8;
}
extern "C" {
    pub fn acpi_tb_set_table_loaded_flag(table_index: u32, is_loaded: u8);
}
//
// tbfadt - FADT parse/convert/validate
//
extern "C" {
    pub fn acpi_tb_parse_fadt();
}
extern "C" {
    pub fn acpi_tb_create_local_fadt(table: *mut acpi_table_header, length: u32);
}
//
// tbfind - find ACPI table
//
// tbinstal - Table removal and deletion
//
extern "C" {
    pub fn acpi_tb_resize_root_table_list() -> acpi_status;
}
extern "C" {
    pub fn acpi_tb_validate_table(table_desc: *mut acpi_table_desc) -> acpi_status;
}
extern "C" {
    pub fn acpi_tb_invalidate_table(table_desc: *mut acpi_table_desc);
}
extern "C" {
    pub fn acpi_tb_override_table(old_table_desc: *mut acpi_table_desc);
}
extern "C" {
    pub fn acpi_tb_uninstall_table(table_desc: *mut acpi_table_desc);
}
extern "C" {
    pub fn acpi_tb_unload_table(table_index: u32) -> acpi_status;
}
extern "C" {
    pub fn acpi_tb_notify_table(event: u32, table: *mut c_void);
}
extern "C" {
    pub fn acpi_tb_terminate();
}
extern "C" {
    pub fn acpi_tb_delete_namespace_by_owner(table_index: u32) -> acpi_status;
}
extern "C" {
    pub fn acpi_tb_allocate_owner_id(table_index: u32) -> acpi_status;
}
extern "C" {
    pub fn acpi_tb_release_owner_id(table_index: u32) -> acpi_status;
}
extern "C" {
    pub fn acpi_tb_get_owner_id(table_index: u32, owner_id: *mut acpi_owner_id) -> acpi_status;
}
//
// tbutils - table manager utilities
//
extern "C" {
    pub fn acpi_tb_initialize_facs() -> acpi_status;
}
extern "C" {
    pub fn acpi_tb_check_dsdt_header();
}
extern "C" {
    pub fn acpi_tb_parse_root_table(rsdp_address: acpi_physical_address) -> acpi_status;
}
extern "C" {
    pub fn acpi_tb_put_table(table_desc: *mut acpi_table_desc);
}
//
// tbxfload
//
extern "C" {
    pub fn acpi_tb_load_namespace() -> acpi_status;
}
