//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-ima.h
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
// Copyright (C) 2021 Microsoft Corporation
//
// Author: Tushar Sugandhi <tusharsu@linux.microsoft.com>
//
// Header file for device mapper IMA measurements.
//
pub const DM_IMA_MEASUREMENT_BUF_LEN: c_int = 4096;
pub const DM_IMA_DEVICE_BUF_LEN: c_int = 1024;
pub const DM_IMA_TARGET_METADATA_BUF_LEN: c_int = 128;
pub const DM_IMA_TARGET_DATA_BUF_LEN: c_int = 2048;
pub const DM_IMA_DEVICE_CAPACITY_BUF_LEN: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dm_ima_table_op {
    DM_IMA_TABLE_SAVE,
    DM_IMA_TABLE_RESTORE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_ima_device_table_metadata {
//
// Contains data specific to the device which is common across
// all the targets in the table (e.g. name, uuid, major, minor, etc).
// The values are stored in comma separated list of key1=val1,key2=val2;
// pairs delimited by a semicolon at the end of the list.
//
    pub device_metadata: *mut c_char,
    pub device_metadata_len: c_uint,
    pub num_targets: c_uint,
    pub capacity: sector_t,
//
// Contains the sha256 hashes of the IMA measurements of the target
// attributes' key-value pairs from the active/inactive tables.
//
    pub hash: *mut c_char,
    pub hash_len: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_ima_context {
    pub table: dm_ima_device_table_metadata,
    pub update_idx: c_uint,
    pub dev_name: [*mut c_char; DM_NAME_LEN*2],
    pub dev_uuid: [*mut c_char; DM_UUID_LEN*2],
}

//
// This structure contains device metadata, and table hash for
// active and inactive tables for ima measurements.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_ima_measurements {
    pub update_idx: c_uint,
    pub measure_idx: c_uint,
    pub ima_wq: wait_queue_head,
    pub ima_lock: spinlock_t,
    pub active_table: dm_ima_device_table_metadata,
    pub inactive_table: dm_ima_device_table_metadata,
}

extern "C" {
    pub fn dm_ima_init(md: *mut mapped_device);
}
extern "C" {
    pub fn dm_ima_alloc_context(context: *mut dm_ima_context, noio: bool);
}
extern "C" {
    pub fn dm_ima_free_context(context: *mut dm_ima_context);
}

