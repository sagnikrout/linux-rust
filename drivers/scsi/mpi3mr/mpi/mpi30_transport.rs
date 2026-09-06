//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mpi3mr/mpi/mpi30_transport.h
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
pub const MPI30_TRANSPORT_H: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_version_struct {
    pub dev: u8,
    pub unit: u8,
    pub minor: u8,
    pub major: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mpi3_version_union {
    pub mpi3_version: mpi3_version_struct,
    pub word: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sysif_oper_queue_indexes {
    pub producer_index: __le16,
    pub reserved02: __le16,
    pub consumer_index: __le16,
    pub reserved06: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sysif_registers {
    pub ioc_information: __le64,
    pub version: mpi3_version_union,
    pub reserved0c: [__le32; 2],
    pub ioc_configuration: __le32,
    pub reserved18: __le32,
    pub ioc_status: __le32,
    pub reserved20: __le32,
    pub admin_queue_num_entries: __le32,
    pub admin_request_queue_address: __le64,
    pub admin_reply_queue_address: __le64,
    pub reserved38: [__le32; 2],
    pub coalesce_control: __le32,
    pub reserved44: [__le32; 1007],
    pub admin_request_queue_pi: __le16,
    pub reserved1002: __le16,
    pub admin_reply_queue_ci: __le16,
    pub reserved1006: __le16,
    pub oper_queue_indexes: [mpi3_sysif_oper_queue_indexes; 383],
    pub reserved1c00: __le32,
    pub write_sequence: __le32,
    pub host_diagnostic: __le32,
    pub reserved1c0c: __le32,
    pub fault: __le32,
    pub fault_info: [__le32; 3],
    pub reserved1c20: [__le32; 4],
    pub hcb_address: __le64,
    pub hcb_size: __le32,
    pub reserved1c3c: __le32,
    pub reply_free_host_index: __le32,
    pub sense_buffer_free_host_index: __le32,
    pub reserved1c48: [__le32; 2],
    pub diag_rw_data: __le64,
    pub diag_rw_address: __le64,
    pub diag_rw_control: __le16,
    pub diag_rw_status: __le16,
    pub reserved1c64: [__le32; 35],
    pub scratchpad: [__le32; 4],
    pub reserved1d00: [__le32; 192],
    pub device_assigned_registers: [__le32; 2048],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_default_reply_descriptor {
    pub descriptor_type_dependent1: [__le32; 2],
    pub request_queue_ci: __le16,
    pub request_queue_id: __le16,
    pub descriptor_type_dependent2: __le16,
    pub reply_flags: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_address_reply_descriptor {
    pub reply_frame_address: __le64,
    pub request_queue_ci: __le16,
    pub request_queue_id: __le16,
    pub reserved0c: __le16,
    pub reply_flags: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_success_reply_descriptor {
    pub reserved00: [__le32; 2],
    pub request_queue_ci: __le16,
    pub request_queue_id: __le16,
    pub host_tag: __le16,
    pub reply_flags: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_target_command_buffer_reply_descriptor {
    pub reserved00: __le32,
    pub initiator_dev_handle: __le16,
    pub phy_num: u8,
    pub reserved07: u8,
    pub request_queue_ci: __le16,
    pub request_queue_id: __le16,
    pub io_index: __le16,
    pub reply_flags: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_status_reply_descriptor {
    pub ioc_status: __le16,
    pub reserved02: __le16,
    pub ioc_log_info: __le32,
    pub request_queue_ci: __le16,
    pub request_queue_id: __le16,
    pub host_tag: __le16,
    pub reply_flags: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mpi3_reply_descriptors_union {
    pub default_reply: mpi3_default_reply_descriptor,
    pub address_reply: mpi3_address_reply_descriptor,
    pub success: mpi3_success_reply_descriptor,
    pub target_command_buffer: mpi3_target_command_buffer_reply_descriptor,
    pub status: mpi3_status_reply_descriptor,
    pub words: [__le32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sge_common {
    pub address: __le64,
    pub length: __le32,
    pub reserved0c: [u8; 3],
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sge_bit_bucket {
    pub reserved00: __le64,
    pub length: __le32,
    pub reserved0c: [u8; 3],
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sge_extended_eedp {
    pub user_data_size: u8,
    pub reserved01: u8,
    pub eedp_flags: __le16,
    pub secondary_reference_tag: __le32,
    pub secondary_application_tag: __le16,
    pub application_tag_translation_mask: __le16,
    pub reserved0c: __le16,
    pub extended_operation: u8,
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mpi3_sge_union {
    pub simple: mpi3_sge_common,
    pub chain: mpi3_sge_common,
    pub last_chain: mpi3_sge_common,
    pub bit_bucket: mpi3_sge_bit_bucket,
    pub eedp: mpi3_sge_extended_eedp,
    pub words: [__le32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_request_header {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub function_dependent: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_default_reply {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub ioc_use_only08: __le16,
    pub ioc_status: __le16,
    pub ioc_log_info: __le32,
}

