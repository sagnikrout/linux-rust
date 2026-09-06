//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/iosm/iosm_ipc_coredump.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2020-2021 Intel Corporation.
//

// Max number of bytes to receive for Coredump list structure
pub const MAX_CD_LIST_SIZE: c_uint = 0x1000;
// Max buffer allocated to receive coredump data
pub const MAX_DATA_SIZE: c_uint = 0x00010000;
// Max length
pub const MAX_SIZE_LEN: c_int = 32;
//
// struct iosm_cd_list_entry - Structure to hold coredump file info.
// @size:       Number of bytes for the entry
// @filename:   Coredump filename to be generated on host
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iosm_cd_list_entry {
    pub size: __le32,
    pub filename: [c_char; IOSM_MAX_FILENAME_LEN],
    pub __packed: },
//
// struct iosm_cd_list - Structure to hold list of coredump files
// to be collected.
// @num_entries:        Number of entries to be received
// @entry:              Contains File info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iosm_cd_list {
    pub num_entries: __le32,
    pub entry: [iosm_cd_list_entry; ],
    pub __packed: },
//
// struct iosm_cd_table - Common Coredump table
// @version:            Version of coredump structure
// @list:               Coredump list structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iosm_cd_table {
    pub version: __le32,
    pub list: iosm_cd_list,
    pub __packed: },
    pub region_size): u32,
    pub cmd): *mut *mut int ipc_coredump_get_list(struct iosm_devlink devlink, u16,
