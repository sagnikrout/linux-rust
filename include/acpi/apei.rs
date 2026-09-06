//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/apei.h
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
// apei.h - ACPI Platform Error Interface
//

pub const APEI_ERST_INVALID_RECORD_ID: c_uint = 0xffffffffffffffffULL;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hest_status {
    HEST_ENABLED,
    HEST_DISABLED,
    HEST_NOT_FOUND,
}

extern "C" {
    pub fn acpi_ghes_init() -> void __init;
}

pub const ghes_disable: c_int = 1;

extern "C" {
    pub fn acpi_hest_init() -> void __init;
}

extern "C" {
    pub fn erst_write(record: *const cper_record_header) -> c_int;
}
extern "C" {
    pub fn erst_get_record_count() -> isize;
}
extern "C" {
    pub fn erst_get_record_id_begin(pos: *mut c_int) -> c_int;
}
extern "C" {
    pub fn erst_get_record_id_next(pos: *mut c_int, record_id: *mut u64) -> c_int;
}
extern "C" {
    pub fn erst_get_record_id_end();
}
extern "C" {
    pub fn erst_clear(record_id: u64) -> c_int;
}
extern "C" {
    pub fn arch_apei_enable_cmcff(hest_hdr: *mut acpi_hest_header, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn arch_apei_report_mem_error(sev: c_int, mem_err: *mut cper_sec_mem_err);
}

