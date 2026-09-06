//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/apei/apei-internal.h
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
// apei-internal.h - ACPI Platform Error Interface internal
// definitions.
//

pub const APEI_EXEC_INS_ACCESS_REGISTER: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apei_exec_ins_type {
    pub flags: u32,
    pub run: apei_exec_ins_func_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apei_exec_context {
    pub ip: u32,
    pub value: u64,
    pub var1: u64,
    pub var2: u64,
    pub src_base: u64,
    pub dst_base: u64,
    pub ins_table: *mut apei_exec_ins_type,
    pub instructions: u32,
    pub action_table: *mut acpi_whea_header,
    pub entries: u32,
}

extern "C" {
    pub fn __apei_exec_run(ctx: *mut apei_exec_context, action: u8, optional: bool) -> c_int;
}
extern "C" {
    pub fn __apei_exec_run(_arg: ctx, _arg: action, _arg: 0) -> return;
}
// It is optional whether the firmware provides the action
extern "C" {
    pub fn __apei_exec_run(_arg: ctx, _arg: action, _arg: 1) -> return;
}
// Common instruction implementation
// IP has been set in instruction function
pub const APEI_EXEC_SET_IP: c_int = 1;
extern "C" {
    pub fn apei_map_generic_address(reg: *mut acpi_generic_address) -> c_int;
}
extern "C" {
    pub fn apei_read(val: *mut u64, reg: *mut acpi_generic_address) -> c_int;
}
extern "C" {
    pub fn apei_write(val: u64, reg: *mut acpi_generic_address) -> c_int;
}
extern "C" {
    pub fn __apei_exec_read_register(entry: *mut acpi_whea_header, val: *mut u64) -> c_int;
}
extern "C" {
    pub fn __apei_exec_write_register(entry: *mut acpi_whea_header, val: u64) -> c_int;
}
extern "C" {
    pub fn apei_exec_pre_map_gars(ctx: *mut apei_exec_context) -> c_int;
}
extern "C" {
    pub fn apei_exec_post_unmap_gars(ctx: *mut apei_exec_context) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apei_resources {
    pub iomem: list_head,
    pub ioport: list_head,
}

extern "C" {
    pub fn apei_resources_fini(resources: *mut apei_resources);
}
extern "C" {
    pub fn apei_resources_release(resources: *mut apei_resources);
}
extern "C" {
    pub fn apei_osc_setup() -> c_int;
}
extern "C" {
    pub fn einj_get_available_error_type(type: *mut u32, einj_action: c_int) -> c_int;
}
extern "C" {
    pub fn einj_is_cxl_error_type(type: u64) -> bool;
}
extern "C" {
    pub fn einj_validate_error_type(type: u64) -> c_int;
}

