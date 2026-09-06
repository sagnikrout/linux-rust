//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mpi3mr/mpi/mpi30_init.h
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
pub const MPI30_INIT_H: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_scsi_io_cdb_eedp32 {
    pub cdb: [u8; 20],
    pub primary_reference_tag: __be32,
    pub primary_application_tag: __le16,
    pub primary_application_tag_mask: __le16,
    pub transfer_length: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mpi3_scsi_io_cdb_union {
    pub cdb32: [u8; 32],
    pub eedp32: mpi3_scsi_io_cdb_eedp32,
    pub sge: mpi3_sge_common,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_scsi_io_request {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub dev_handle: __le16,
    pub flags: __le32,
    pub skip_count: __le32,
    pub data_length: __le32,
    pub lun: [u8; 8],
    pub cdb: mpi3_scsi_io_cdb_union,
    pub sgl: [mpi3_sge_union; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_scsi_io_reply {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub ioc_use_only08: __le16,
    pub ioc_status: __le16,
    pub ioc_log_info: __le32,
    pub scsi_status: u8,
    pub scsi_state: u8,
    pub dev_handle: __le16,
    pub transfer_count: __le32,
    pub sense_count: __le32,
    pub response_data: __le32,
    pub task_tag: __le16,
    pub scsi_status_qualifier: __le16,
    pub eedp_error_offset: __le32,
    pub eedp_observed_app_tag: __le16,
    pub eedp_observed_guard: __le16,
    pub eedp_observed_ref_tag: __le32,
    pub sense_data_buffer_address: __le64,
}

