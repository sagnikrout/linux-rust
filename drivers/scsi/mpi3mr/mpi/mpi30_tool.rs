//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mpi3mr/mpi/mpi30_tool.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2016-2026 Broadcom Inc. All rights reserved.
//
pub const MPI30_TOOL_H: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_diag_buffer_post_request {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub reserved0a: __le16,
    pub type: u8,
    pub reserved0d: u8,
    pub reserved0e: __le16,
    pub address: __le64,
    pub length: __le32,
    pub reserved1c: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_diag_buffer_manage_request {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub reserved0a: __le16,
    pub type: u8,
    pub action: u8,
    pub reserved0e: __le16,
}
