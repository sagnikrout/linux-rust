//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/firmware/efi/test/efi_test.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// EFI Test driver Header
//
// Copyright(C) 2012-2016 Canonical Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_getvariable {
    pub variable_name: *mut efi_char16_t,
    pub vendor_guid: *mut efi_guid_t,
    pub attributes: *mut u32,
    pub data_size: *mut c_ulong,
    pub data: *mut c_void,
    pub status: *mut efi_status_t,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_setvariable {
    pub variable_name: *mut efi_char16_t,
    pub vendor_guid: *mut efi_guid_t,
    pub attributes: u32,
    pub data_size: c_ulong,
    pub data: *mut c_void,
    pub status: *mut efi_status_t,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_getnextvariablename {
    pub variable_name_size: *mut c_ulong,
    pub variable_name: *mut efi_char16_t,
    pub vendor_guid: *mut efi_guid_t,
    pub status: *mut efi_status_t,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_queryvariableinfo {
    pub attributes: u32,
    pub maximum_variable_storage_size: *mut u64,
    pub remaining_variable_storage_size: *mut u64,
    pub maximum_variable_size: *mut u64,
    pub status: *mut efi_status_t,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_gettime {
    pub time: *mut efi_time_t,
    pub capabilities: *mut efi_time_cap_t,
    pub status: *mut efi_status_t,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_settime {
    pub time: *mut efi_time_t,
    pub status: *mut efi_status_t,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_getwakeuptime {
    pub enabled: *mut efi_bool_t,
    pub pending: *mut efi_bool_t,
    pub time: *mut efi_time_t,
    pub status: *mut efi_status_t,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_setwakeuptime {
    pub enabled: efi_bool_t,
    pub time: *mut efi_time_t,
    pub status: *mut efi_status_t,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_getnexthighmonotoniccount {
    pub high_count: *mut u32,
    pub status: *mut efi_status_t,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_querycapsulecapabilities {
    pub capsule_header_array: *mut efi_capsule_header_t,
    pub capsule_count: c_ulong,
    pub maximum_capsule_size: *mut u64,
    pub reset_type: *mut c_int,
    pub status: *mut efi_status_t,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_resetsystem {
    pub reset_type: c_int,
    pub status: efi_status_t,
    pub data_size: c_ulong,
    pub data: *mut efi_char16_t,
    pub __packed: },

